use rand::Rng;
use core::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
	println!("Input your guess:");

	let mut guess = String::new();

	 // get user input
	io::stdin()
	    .read_line(&mut guess)
	    .expect("Failed to read line");

	println!("Input your guess:");
	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};


	match guess.cmp(&secret_number){
	    Ordering::Less => println!("Too low"),
	    Ordering::Greater => println!("Too high"),
	    Ordering::Equal => {
		println!("winner winner chicken dinner");
		break;
	    }
	}
    }
}
