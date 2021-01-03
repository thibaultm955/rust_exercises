use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1].to_string();

    let query = query.parse::<i32>().expect("Not a number");

    let mut x = query;

    loop {
        
        if x == 0 {
            break
        }
        else if x == 1 {
            println!("{} bottles of beer on the wall, {} bottles of beer!", x, x);

            x -= 1;

            println!("Take one down, pass it around, no more bottles of beer on the wall!");
        }
        else {
            println!("{} bottles of beer on the wall, {} bottles of beer!", x, x);

            x -= 1;

            println!("Take one down, pass it around, {} bottles of beer on the wall!", x);
        }
    }
}
