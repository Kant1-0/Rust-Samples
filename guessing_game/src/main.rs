extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number within 10 tries!");

	let secret_number = rand::thread_rng().gen_range(0, 1000);

	let mut _i : u8 = 1;

	loop{
		if _i > 10 {
			println!("You loose!");
			break;
		}
		println!("{}. Please input your guess: ", _i);

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_)	=> continue,
		};

		match guess.cmp(&secret_number) {
			Ordering::Less		=> println!("Too small!\n"),
			Ordering::Greater	=> println!("Too big!\n"),
			Ordering::Equal		=> {
				println!("\nYou win!");
				break;
			}
		}

		_i += 1;
	}
}
