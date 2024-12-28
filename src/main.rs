use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::Rng;

static OPERATORS: &[&str] = &["+", "-", "*", "/"];
static FUNCTIONS: &[fn(i32, i32) -> i32] = &[
    |a, b| a + b,
    |a, b| a - b,
    |a, b| a * b,
    |a, b| a / b,
];

fn get_guess_simple(operator:fn(i32, i32)->i32, num1:i32, num2:i32) -> bool {
    // only supports two numbers
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    if tokens.len() != 3 {
        return false;
    }
    let x1 = tokens[0].parse::<i32>().unwrap();
    let x2 = tokens[2].parse::<i32>().unwrap();
    let op_str = tokens[1];
    let operation_idx = OPERATORS.iter().position(|&x| x == op_str);
    if operation_idx.is_none() {
        return false;
    }
    let f = FUNCTIONS[operation_idx.unwrap()];
    return f(x1, x2) == operator(num1, num2);
}

fn run_round() {
    let mut rng = rand::thread_rng();
    let operation_idx = rng.gen_range(0..OPERATORS.len());
    let operation = FUNCTIONS[operation_idx];
    let num1 = rng.gen_range(1..100);
    let num2 = rng.gen_range(1..100);
    let expected_result = operation(num1, num2);
    println!("Try getting the result {} from numbers {} and {}", expected_result, num1, num2);
    while ! get_guess_simple(operation, num1, num2)
    {
        println!("Try again!");
    }
    println!("Correct!");
}

fn main() {
    println!("Hello, world!");
    run_round();
}
