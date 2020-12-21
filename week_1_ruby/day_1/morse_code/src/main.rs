use std::collections::HashMap;
use std::io;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("A"), ".-");
    scores.insert(String::from("B"), "-...");
    scores.insert(String::from("C"), "-.-.");
    scores.insert(String::from("D"), "-..");
    scores.insert(String::from("E"), ".");
    scores.insert(String::from("F"), "..-.");
    scores.insert(String::from("G"), "--.");
    scores.insert(String::from("H"), "....");
    scores.insert(String::from("I"), "..");
    scores.insert(String::from("J"), ".---");
    scores.insert(String::from("K"), "-.-");
    scores.insert(String::from("L"), ".-..");
    scores.insert(String::from("M"), "--");
    scores.insert(String::from("N"), "-.");
    scores.insert(String::from("O"), "---");
    scores.insert(String::from("P"), ".--.");
    scores.insert(String::from("Q"), "--.-");
    scores.insert(String::from("R"), ".-.");
    scores.insert(String::from("S"), "...");
    scores.insert(String::from("T"), "-");
    scores.insert(String::from("U"), "..-");
    scores.insert(String::from("V"), "...-");
    scores.insert(String::from("W"), ".--");
    scores.insert(String::from("X"), "-..-");
    scores.insert(String::from("Y"), "-.--");
    scores.insert(String::from("Z"), "--..");

    println!("What word would you like to translate into Morse ?");

    let mut word = String::new();
        
    io::stdin()
    .read_line(&mut word)
    .expect("New word added");

    let mut word = word.trim();

    let chars = word.chars();

    let mut translated_word = String::new();

    let v = ["?", "!", "."];

    for char_word in chars {
        let string = &char_word.to_uppercase().to_string();
        let slice: &str = &string;
        if string == " " {
            translated_word.push_str("|"); 
        }        
        else if v.contains(&slice) {
            
        }
        else {
            let mut morse = scores[string];
            translated_word.push_str(morse); 
            translated_word.push_str(" "); 
        }
        
    }

    println!("Letter A is {}", translated_word);
}
