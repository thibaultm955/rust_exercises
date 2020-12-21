use std::io;

fn main() {

    let mut first_name = String::new();

    let mut middle_name = String::new();

    let mut last_name = String::new();

    println!("What is your First Name ?");

    io::stdin()
    .read_line(&mut first_name)
    .expect("Please enter a name");

    println!("What is your Middle Name ?");

    io::stdin()
    .read_line(&mut middle_name)
    .expect("Please enter a name");

    println!("What is your Last Name ?");

    io::stdin()
    .read_line(&mut last_name)
    .expect("Please enter a name");

    first_name = first_name.trim().to_string();

    middle_name = middle_name.trim().to_string();

    last_name = last_name.trim().to_string();

    println!("Hi {} {} {}", first_name, middle_name, last_name);
}
