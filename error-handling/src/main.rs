#![allow(unused_variables, dead_code)]

use std::fmt::Display;

use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Deserialize)]
struct Config {
    foo: String,
}

async fn read_and_parse(file_name: &str) -> String {
    let mut file = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await.unwrap();
    let file_content = String::from_utf8(buf).unwrap();
    let json_config: Value = serde_json::from_str(file_content.as_str()).unwrap();
    let foo_setting = json_config.get("foo").unwrap();
    foo_setting.as_str().unwrap().to_string()
}

enum ParseError {
    Io(std::io::Error),
    Encoding,
    Json(serde_json::Error),
    Config
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(e) => write!(f, "IO error: {}", e),
            ParseError::Encoding => write!(f, "Encoding error"),
            ParseError::Json(e) => write!(f, "JSON error: {}", e),
            ParseError::Config => write!(f, "Config error"),
        }
    }
}

async fn read_and_parse_2(file_name: &str) -> Result<String, ParseError> {
    let mut file = File::open(file_name).await.map_err(|e| ParseError::Io(e))?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await.map_err(|e| ParseError::Io(e))?;
    let file_content = String::from_utf8(buf).map_err(|_| ParseError::Encoding)?;
    let json_config: Value = serde_json::from_str(file_content.as_str()).map_err(|e| ParseError::Json(e))?;
    match json_config.get("foo") {
        Some(foo_setting) => Ok(foo_setting.to_string()),
        None => Err(ParseError::Config)
    }
}

async fn read_and_parse_3(file_name: &str) -> anyhow::Result<String> {
    let mut file = File::open(file_name).await?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await?;
    let file_content = String::from_utf8(buf)?;
    let json_config: Value = serde_json::from_str(file_content.as_str())?;
    match json_config.get("foo") {
        Some(foo_setting) => Ok(foo_setting.to_string()),
        None => Err(anyhow::anyhow!("Config error"))
    }
}

#[derive(Error, Debug)]
enum ParseErrorForThiserror {
    #[error("error while accessing file")]
    Io(#[from] std::io::Error),
    #[error("utf-8 encoding error")]
    Encoding(#[from] std::string::FromUtf8Error),
    #[error("json error")]
    Json(#[from] serde_json::Error),
    #[error("config error")]
    Config,
    #[error("unknown error")]
    Unknown
}

/// Read and parse a file and return the foo setting as a string.
/// 
/// ## Errors
/// 
/// - `Io` if the file cannot be opened or read.
/// - `Encoding` if the file is not valid UTF-8.
/// - `Json` if the file is not valid JSON.
/// 
/// ## Example
/// 
/// ```
/// let foo_value = read_and_parse_4("config.json").await;
/// ```
async fn read_and_parse_4(file_name: &str) -> Result<String, ParseErrorForThiserror> {
    let mut file = File::open(file_name).await?;
    let mut buf = Vec::new();
    file.read_buf(&mut buf).await?;
    let file_content = String::from_utf8(buf)?;
    let json_config: Value = serde_json::from_str(file_content.as_str())?;
    match json_config.get("foo") {
        Some(foo_setting) => Ok(foo_setting.to_string()),
        None => Err(ParseErrorForThiserror::Config)
    }
}

#[tokio::main]
async fn main() {
    let foo_value = read_and_parse_4("confg.json").await;
    match foo_value {
        Ok(value) => println!("foo_value: {}", value),
        Err(e) => match e {
            ParseErrorForThiserror::Io(e) => println!("IO error: {}", e),
            ParseErrorForThiserror::Encoding(e) => println!("Encoding error: {}", e),
            ParseErrorForThiserror::Json(e) => println!("JSON error: {}", e),
            ParseErrorForThiserror::Config => println!("Config error"),
            ParseErrorForThiserror::Unknown => println!("Unknown error"),
        }
    }
}
