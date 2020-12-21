use std::io;

fn main() {

    let mut first_num = String::new();

    println!("Please input your number as comma separated value");

    io::stdin()
    .read_line(&mut first_num)
    .expect("Require an input");

    let u = first_num.replace(" ", ""); 

    let u = u.trim();

    let v: Vec<&str>  = u.split(',').collect();

    if v.len() == 1 {
        println!("Not Colourful!");
    }
    else if v.len() == 2{
        let v_0: u32 = v[0].parse().unwrap();
        let v_1: u32 = v[1].parse().unwrap();
        let multiplication =  v_0 * v_1;
        let multiplication = multiplication.to_string();
        let multiplication: &str = &multiplication;
        if v.contains(&multiplication) {
            println!("Not colourful!");
        }
        else {
            println!("Colourful");
        }
    }
    else if v.len() == 3 {
        let v_0: u32 = v[0].parse().unwrap();
        let v_1: u32 = v[1].parse().unwrap();
        let v_2: u32 = v[2].parse().unwrap();
        let multiplication_1 = v_0 * v_1;
        let multiplication_2 = v_1 * v_2;
        let multiplication_3 = v_0 * v_1 * v_2;
        let multiplication_1: &str = &multiplication_1.to_string();
        let multiplication_2: &str = &multiplication_2.to_string();
        let multiplication_3: &str = &multiplication_3.to_string();
        if v.contains(&multiplication_1) | v.contains(&multiplication_2) | v.contains(&multiplication_3) {
            println!("Not colourful!");
        }
        else {
            println!("Colourful");
        }

    }
    else {
        println!("You provided a too small or too big number of values. We accept from 1 up to 3 values !");
    }
    



    colourful_algorithm();

    
}

fn colourful_algorithm() {
    println!("Hello, world!");
}
