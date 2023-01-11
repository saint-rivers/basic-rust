use rand::Rng;
use std::{
    cmp::Ordering,
    io::{stdin, stdout, Write},
};

fn generate_number() -> i32 {
    let mut rng = rand::thread_rng();
    let target_number = rng.gen_range(0..24);
    target_number
}

fn get_user_input() -> i32 {
    print!("Guess the number: ");
    stdout().flush().unwrap(); // not sure why but i needed this for print!() but not println!()

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .ok()
        .expect("did not enter something valid");

    // convert user input
    let input: i32 = user_input.trim().parse().expect("not a number");

    input
}

fn check_result(target: i32, input: i32) -> bool {
    // check guessed number
    let result: bool = match target.cmp(&input) {
        Ordering::Less => {
            println!("Guess lower");
            false
        }
        Ordering::Greater => {
            println!("Guess higher");
            false
        }
        Ordering::Equal => {
            println!("Correct. The number was {}", target);
            true
        }
    };
    result
}

fn main() {
    // program start
    let target_number = generate_number();

    let mut is_game_won = false;
    let mut guess_count = 0;

    while !is_game_won {
        let user_input = get_user_input();
        guess_count += 1;
        is_game_won = check_result(target_number, user_input);
    }

    println!("Total guesses: {}", guess_count);
}
