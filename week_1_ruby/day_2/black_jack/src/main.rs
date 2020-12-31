extern crate rand;

use std::io;
use rand::thread_rng;
use rand::Rng;

fn main() {
    
    let bank_score = pick_bank_score();
    
    let good_answers = ["y", "yes"];

    let mut player_score: i32 = 0;

    loop {
        println!("Card? \'y\' or \'yes\' to get a new card");

        let mut answer = String::new();

        io::stdin()
        .read_line(&mut answer)
        .expect("Not correct");

        let answer = answer.trim();
        
        if !good_answers.contains(&answer) {
            if player_score > 21 {
                println!("You lost");
            }
            else if player_score == 21 {
                println!("Black Jack");
            }
            else if player_score > bank_score {
                println!("You beat the bank, you win !");
            }
            else if player_score < bank_score {
                println!("You lost");
            }
            else if player_score == bank_score {
                println!("Push");
            }
            break;
        } 
        else {
            let player_card = pick_player_card();

            player_score += player_card;

            println!("Your score is {}, bank is {}", player_score, bank_score);
        }
    }

}


fn pick_bank_score() -> i32 {
    let mut rng = thread_rng();
    let i: i32 = rng.gen_range(16, 21);
    i
}

fn pick_player_card() -> i32 {
    let mut rng = thread_rng();
    let i: i32 = rng.gen_range(1, 11);
    i
}
