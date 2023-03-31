
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let's play!");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read stdin");
        // let guess: u32 = guess.trim().parse().expect("Value error, not a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("Secret number is: {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("{}", "Yay... equal"),
            Ordering::Greater => println!("Hmm... greater"),
            Ordering::Less => {
                println!("Oops... less");
                break;
            }
        }
    }
    
}
