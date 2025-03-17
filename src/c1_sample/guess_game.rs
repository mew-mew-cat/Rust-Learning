use std::cmp::Ordering;
use std::io;
use rand::Rng;

#[allow(dead_code)]
pub fn start_game() {
    println!("Guess game started!");

    let secret = rand::rng().random_range(1..=100);

    println!("A secret number from [1, 100] generated!");

    let mut round = 1;
    loop {
        println!("[Round {}] Please input your guess integer:", round);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");

        // Shadowing + type translation
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[Error] You should input an integer!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        round += 1;
    }
}
