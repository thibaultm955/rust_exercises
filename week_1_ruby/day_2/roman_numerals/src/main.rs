use std::io;

fn main() {
    println!("What is your number ?");

    let mut user_input = String::new();

    io::stdin()
    .read_line(&mut user_input)
    .expect("Not correct");

    let user_input: i32 = user_input.trim().parse::<i32>().expect("Not a number");

    let result = old_roman_numerals(user_input);

    println!("{}", result);
}

fn old_roman_numerals(x: i32) -> String {
    let mut output = String::new();
    let mut y = x;
    loop {
        if y >= 1000 {
            output.push_str("M");
            y = y - 1000;
        }
        else if y >= 500 {
            output.push_str("D");

            y = y - 500;
        }
        else if y >= 100 {
            output.push_str("C");

            y = y - 100;
        }
        else if y >= 50 {
            output.push_str("X");

            y = y - 50;
        }
        else if y >=10 {
            output.push_str("X");

            y = y - 10;
        }
        else if y >= 5 {
            output.push_str("V");

            y = y - 5;
        }
        else if y >= 1 {
            output.push_str("I");

            y -= 1;
        }
        else {
            break
        }
    }

    output.to_string()
}