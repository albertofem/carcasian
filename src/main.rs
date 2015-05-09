extern crate carcasian;

use std::io;
use std::io::Read;
use carcasian::*;
use std::collections::HashMap;

fn main() {
	println!("Welcome to Carcasian database! (Version: 0.0.1)");

	let mut data: HashMap<String, String> = database::storage::new();

	let mut input;

	loop {
		input = read_line();

		let mut command = input.trim_right_matches("\r\t");

		let mut tokens = command.split(" ");

		for token in tokens {
			println! ("{}", token);
		}
	}
}

fn read_line() -> String {
	let mut line = String::new();

	io::stdin().read_line(&mut line)
		.ok()
		.expect("failed to read line");

	return line;
}