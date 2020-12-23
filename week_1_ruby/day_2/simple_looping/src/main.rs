use std::io;


fn main() {
    println!("Please provide a minimum number:");

    let mut min_num = String::new();

    io::stdin()
    .read_line(&mut min_num)
    .expect("Not an input");

    let min_num: i32 = min_num.trim().parse().expect("Not an integer");

    println!("Please provide a maximum number:");

    let mut max_num = String::new();

    io::stdin()
    .read_line(&mut max_num)
    .expect("Not a correct input");

    let max_num: i32 = max_num.trim().parse().expect("Not an integer");

    let return_value = min_num_less_max_num(min_num, max_num);

    

    println!("return: {}", &return_value);


}

fn min_num_less_max_num(min_num: i32, max_num: i32) -> i32 {
    if min_num > max_num {
        let result: i32 = -1;
        result
    }
    else {
        let sum: i32 = max_num - min_num + 1;
        
        sum
    }
 
}