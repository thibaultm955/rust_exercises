use std::io;

fn main() {
    println!("Type a student name:");

    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Not correct");

    let name = name.trim();

    println!("Your name is {}", name);

    let mut names = String::new();

    names.push_str(name);

    loop {
        println!("Type another student name or press enter to finish:");

        let mut name = String::new();

        io::stdin()
        .read_line(&mut name)
        .expect("Not correct");

        if name == "\n" {
            break
        }
        else {
            let mut name = name.trim().to_string();

            name.insert_str(0, " ");

            names.push_str(&name);
        }
        
    }

    let ss = names.split(" ");
    
    let mut vec: Vec<String> = ss.map(|s| s.parse::<String>().unwrap()).collect();

    vec.sort_by(|a, b| (a.get(0..1).unwrap().to_uppercase()).cmp(&b.get(0..1).unwrap().to_uppercase()));

   let mut name_final = String::new();

   let length = vec.len();

   let mut i = 1;

    for v in vec {
        if i == 1 {
            name_final.push_str(&v);
        }
        else if i == length {
            name_final.push_str(" and ");
            name_final.push_str(&v);
        }
        else {
            name_final.push_str(", ");
            name_final.push_str(&v);
        }
        i += 1;
    }

    println!("Congratulations! Your Wagon has {} students:", length);
    
    println!("{}" , name_final);

    
}
