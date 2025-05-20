#![allow(unused_variables, dead_code)]

mod math;

const NEARLY_THE_ANSWER: i32 = 41;
const ANSWER: i32 = NEARLY_THE_ANSWER + 1;
const NUMBERS: [i32; 5] = five_numbers();

const fn five_numbers() -> [i32; 5] {
    let mut numbers = [0i32; 5];

    let mut i = 0;
    while i < 5 {
        numbers[i] = i as i32 + 1;
        i += 1;
    }

    numbers
}

fn main() {
    let numbers = five_numbers();

    math::div(10, 5);

    let a = 10;
    let b = 20;
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    let c = c + a;
    println!("{} + {} = {}", c, a, c);

    let user_input = "42";
    let mut user_input: i32 = user_input.parse().unwrap();
    println!("{}", user_input);

    user_input += 1;
    println!("{}", user_input);

    let user_input = user_input;
    // ...
    let mut user_input = user_input;
    user_input += 1;
    println!("{}", user_input);

    let add = |a: i32, b: i32| a + b;
    let result = add(1, 2);
    println!("{}", result);
}
