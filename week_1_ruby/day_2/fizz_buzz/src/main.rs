use std::io;
use std::env;

fn main() {
    println!("What number would you like to have ?");

    let test: Vec<String> = env::args().collect();

    let number = test[1].trim().parse::<i32>().expect("Not a number");

    let result = fizz_buzz(number);

    let result: Vec<&str> = result.trim().split(" ").collect();
    
    println!("{:?}", result);
}

fn fizz_buzz(x: i32) -> String {
    let iterator = x;

    let mut result_string = String::new();

    let mut i = 0;
    
    loop {
        i += 1;

        if i % 3 == 0 && i % 5 == 0 {
            result_string.push_str(" FizzBuzz");
        }
        else if i % 3 == 0 {
            result_string.push_str(" Fizz");
        }
        else if i % 5 == 0 {
            result_string.push_str(" Buzz");
        }
        else if i == iterator {
            result_string.push_str(" ");
            result_string.push_str(&i.to_string());
            break
        }
        else {
            result_string.push_str(" ");
            result_string.push_str(&i.to_string());
        }
    }
    result_string
}