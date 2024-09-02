use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("We are playing russian rulette, you guess, you live. You miss, you die!");
    println!("Your guess: ");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(0..=964);
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Idk, something happened.");
    
    let input_int: i32 = guess.trim().parse().expect("Please type a valid number!");
    println!("Your guess is {guess}");
    println!("The secret number is {secret_number}");
    match input_int.cmp(&secret_number){
        Ordering::Less => println!("Too lil *user gets killed cuz he missed*"),
        Ordering::Greater => println!("Too much *user gets killed cuz he missed*"),
        Ordering::Equal => println!("You will live for the next day. too bad!")
    };
}