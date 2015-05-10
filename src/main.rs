extern crate carcasian;

use std::io;
use std::io::{Read,Write};
use std::net::TcpListener;
use std::thread;
use carcasian::*;
use std::collections::HashMap;

fn main() {
	let host = "127.0.0.1";
	let port = "9821";

	let server = format!("{}:{}", host, port);

	let listener = TcpListener::bind("127.0.0.1:9821").unwrap(); // TODO: as_str() when RFC is approved

	println!("Welcome to Carcasian database! Listening on {}", server);

	let mut data: HashMap<String, String> = database::storage::new();
	let mut input;

	loop {
		io::stdout().write("127.0.0.1:9821> ".as_bytes());
		io::stdout().flush();

		input = read_line();

		let mut command: Vec<&str> = input.trim_right_matches("\n").split(" ").collect();
		let command_name: &str = command.first().unwrap();

		command.remove(0);

		if command_name == "SET" {
			println!("{}", database::setget::SetGet.set(&mut data, command[0], command[1]));
		} else if command_name == "GET" {
			println!("{}", database::setget::SetGet.get(&mut data, command[0]))
		} else if command_name == "EXIT" {
			break;
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