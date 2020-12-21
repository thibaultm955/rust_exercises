use std::io;
use std::convert::TryInto;

fn main() {
    println!("Type 1 if you want to translate your IP address\nType 2 if you would like to translate your bits to an IP address");

    let mut answer = String::new();

    io::stdin()
    .read_line(&mut answer)
    .expect("Not an input");

    let answer: u32 = answer.trim().parse().expect("Not a number");

    if answer == 1 {
        translate_ip_address_to_bits();
        
    }
    else if answer == 2 {
        translate_bits_to_ip_address();
        
    }

}

fn translate_ip_address_to_bits() {
    println!("Please enter the IP address:");
    
    let mut ip_address = String::new();

    io::stdin()
    .read_line(&mut ip_address)
    .expect("Incorrect input");

    ip_address = ip_address.trim().to_string();

    println!("The IP is: {}", ip_address);

    let ip_address: Vec<&str> = ip_address.split('.').collect();

    let mut bit_result = Vec::new();

    let number_bit: Vec<u32> = vec![128, 64, 32, 16, 8, 4, 2, 1];

    for i in &ip_address {
        let mut i: u32 = i.parse().expect("Not a number");
        for bit in &number_bit {
            if i >= *bit {
                bit_result.push("1");
                i = i % *bit;
            }
            else {
                bit_result.push("0");
            }
        }
    }

    let mut length = 0;

    let mut results: u64 = 0;

    bit_result.reverse();

    for bit in &bit_result {
        let bit: u64 = bit.parse().expect("Not a number");
        results += bit * 2_u64.pow(length.try_into().unwrap());
        length += 1;
    }
    
    println!("results is {}", results );
}

fn translate_bits_to_ip_address() {
    println!("Please enter the number to transform in IP address: ");

    let mut number = String::new();

    io::stdin()
    .read_line(&mut number)
    .expect("Not correct input");

    let mut number: i64 = number.trim().parse().expect("Not a number");

    let mut result = Vec::new();

    let mut i: u32 = 0;

    while i <= 31 {
        if number >= 2_i64.pow(31 - i) {
            number = number - 2_i64.pow(31 - i);
            result.push("1");
        } 
        else {
            result.push("0");
        }
        i += 1;
    }
    result.insert(8, ".");
    result.insert(17, ".");
    result.insert(26, ".");
    let result = result.concat();

    let result_splitted: Vec<&str>= result.split(".").collect();

    let mut k: u32 = 0;

    let mut ip_address = String::new();

    for word in &result_splitted {
        let mut result: i64 = 0;
        while k < 8 {
            let chars = word.chars().nth(k as usize).unwrap();
            let chars: i64 = chars.to_string().parse().expect("Not a number");
            result += chars * 2_i64.pow(7 - k);
            k += 1;
        }
        let amount: String = result.to_string();
        ip_address.push_str(&amount);
        let point = ".".to_string();
        ip_address.push_str(&point);
        k = 0;
        
    }

    

    println!("Output is {}", &ip_address);
}