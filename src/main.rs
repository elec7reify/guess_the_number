use std;
use std::cmp::Ordering;
use std::io::stdin;
use std::process::exit;
use rand::Rng;

fn main() {
    start_game()
}

fn start_game() {
    let random_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("Guess the number from 1 to 100:");
    loop {
        let mut input_text = String::new();
        stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin");

        match input_text.trim().parse::<u32>() {
            Ok(input) => {
                match input.cmp(&random_number) {
                    Ordering::Less => println!("Not quite! Try guessing a higher number."),
                    Ordering::Greater => println!("Almost there! Try a lower number this time."),
                    Ordering::Equal => {
                        println!("Well done! You've found the right number!");
                        play_again()
                    },
                }
            },
            Err(e) => println!("The input was not a valid integer.")
        }
    }
}

fn play_again() {
    println!("Would you like to play again? [Y/N]");
    let mut input_text = String::from("");

    stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");
    match input_text.trim().to_lowercase().as_str() {
        "y" | "yes" => start_game(),
        _ => {
            println!("Thanks for playing!");
            exit(0);
        }
    }
}
