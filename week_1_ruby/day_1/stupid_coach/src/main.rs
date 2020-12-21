use std::io;

fn main() {

    stupid_coach();
       
    println!("Have a nice day!");
}

fn stupid_coach() {
    let mut input = String::new();

    while input != "I am going to work right now!" {
        input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Expect a string");

        input = input.trim().to_string();
        println!(" string: {} {}", input, input == "I am going to work right now!");

        if input.contains("?") {
            println!("Silly question, get dressed and go to work!");        
        }
        else {
            println!("I don't care, get dressed and go to work!");
        }
        
    }
}
