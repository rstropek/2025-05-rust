#![allow(unused_variables, dead_code)]

use std::collections::HashSet;

fn main() -> Result<(), i32> {
    // Arrays
    let numbers = [1, 2, 3, 4, 5];
    let numbers = [0i32; 5];
    let numbers: [i16; 5] = [1, 2, 3, 4, 5];
    let mut numbers = [1i16, 2, 3, 4, 5];
    numbers[0] = 10;
    println!("{:?}", numbers[0]);
    
    // Vectors
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers[0] = 10;
    println!("{:?}", numbers[0]);
    numbers.push(6);

    let mut numbers;
    numbers = Vec::new();
    numbers.push(42);

    let my_range = 0..10;
    let numbers: HashSet<i32> = my_range.collect();
    let my_range = 0..10;
    let numbers = my_range.collect::<HashSet<i32>>();

    let user_input = "42";
    let user_input: i16 = user_input.parse().unwrap();
    let user_input = "42";
    let user_input = user_input.parse::<i16>().unwrap();

    let numbers: Vec<i64> = (0..10).collect::<Vec<_>>().into_iter()
        .filter(|&x| x % 2 == 0)
        .collect();
    println!("{numbers:?}");

    // Tuples
    let mut numbers = (1u8, 2u16, 3u32, 4u64, 5u128);
    numbers.0 = 10;
    println!("{:?}", numbers.0);
    let void = ();
  
    // Slices
    let numbers = [1, 2, 3, 4, 5];
    let sum = calculate_sum(&numbers);
    let numbers_slice = &numbers[0..2];
    println!("{:?}", numbers_slice);

    let numbers = "1,2,3,4,5";
    let number = numbers.as_bytes();
    let number = number[4];
    let number = '🤣' as u8;
    println!("{}", number as char);

    Ok(())
    //Err(-1)
}

fn calculate_sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}

fn convert(input: &str) -> i64 {
    let result = input.parse().unwrap();
    result
}

