use std::io;

fn main() {

    let mut age = String::new();

    println!("What is your age ?");

    io::stdin()
    .read_line(&mut age)
    .expect("Not an input");

    let age: u32 = age.trim().parse().expect("Not a number");

    if age >= 18 {
        println!("{}", true);
    }
    else {
        println!("{}", false);
    }

}
