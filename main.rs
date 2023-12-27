use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    
    println!("Guess the number!");
    let mut guessed: bool = false;
    
    loop {
        let secret_number: i8 = rand::thread_rng().gen_range(1..=5);
        println!("Please input your guess.");

        let mut guess: String = String::new();

        get_input(&mut guess);

        guess_it(secret_number, guess.trim().parse::<i8>().expect("Invalid input"), &mut guessed);
        if guessed {
            break;
        }
        else {
            println!("Try again!");
        }
    }

}

fn guess_it(secret: i8, input: i8, guessed: &mut bool) {
    println!("The number is {} and you guessed {}", secret, input);

    match input.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            *guessed = true;
        },
    }
}

fn get_input(guess: &mut String) {
    io::stdin()
        .read_line(guess)
        .expect("Failed to read line");
}