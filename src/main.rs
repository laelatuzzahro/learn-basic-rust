use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");

    loop {
        println!("Please input your guess number.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("Your guess : {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
   
}

