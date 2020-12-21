use std::io;
use std::str::Chars;

fn main() {
    println!("Press 1 for Visa\nPress 2 for Mastercard");

    let mut card_type = String::new();

    io::stdin()
    .read_line(&mut card_type)
    .expect("Not an input");

    let card_type = card_type.trim();

    let mut card_number = String::new();

    println!("What is your card number?");

    io::stdin()
    .read_line(&mut card_number)
    .expect("Incorrect Number");

    let card_number = card_number.trim().replace(" ", "");

    let mut chars = card_number.chars();

    let first_number = chars.next().expect("Not complete enough");

    if card_type == "1" {
        if first_number.to_string() == "4" {
            println!("{}", true);
            let result = valid_card(chars);
            println!("Validity card number {}", &result);
        }
        else {
            println!("{}", false);
        }
        
    }
    else if card_type == "2" {
        if first_number.to_string() == "5" {
            println!("{}", true);
            let result = valid_card(chars);
            println!("Validity card number {}", &result);
        }
        else {
            println!("{}", false);
        }

    }
    else {
        println!("You didn't select a correct type!");
    }
}

fn valid_card(chars: Chars) -> bool {
    
    let mut result = 0;

    let mut j = 0;

    for i in chars {
        
        let u: u32 = i.to_digit(10).expect("Not a number");
        if j == 0 || (j % 2) == 0 {
            let mut sum = u * 2;
            if sum > 9 {
                sum = sum - 9;
                result += sum;
            }
            else {
                result += sum;
            }
        }
        else {
            result += u;
        }
        j += 1;
    }

    if ( &result % 10 ) == 0 {
        true
    }
    else {
        false
    }
    
}
