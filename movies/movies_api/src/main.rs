use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, get}, Json, Router};
use movies_logic::{IdentifyableMovie, Movie, MovieStore};
use tokio::{net::TcpListener, sync::RwLock};

type MovieStoreState = Arc<RwLock<MovieStore>>;

#[tokio::main]
async fn main() {
    let movie_store: MovieStoreState = Arc::new(RwLock::new(MovieStore::new()));

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/movies", get(get_movies).post(add_movie))
        .route("/movies/{id}", delete(delete_movie))
        .with_state(movie_store);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn get_movies(State(store): State<MovieStoreState>) -> Json<Vec<IdentifyableMovie>> {
    let movies  = store.read().await.get_movies();
    Json(movies)
}

pub async fn add_movie(State(store): State<MovieStoreState>, Json(movie): Json<Movie>) -> Json<IdentifyableMovie> {
    let movie = store.write().await.add_movie(movie);
    Json(movie)
}

pub async fn delete_movie(State(store): State<MovieStoreState>, Path(id): Path<usize>) -> impl IntoResponse {
    store.write().await.delete_movie(id);
    (StatusCode::OK, "Movie deleted")
}
