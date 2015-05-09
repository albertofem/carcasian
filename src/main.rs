extern crate carcasian;

use std::io;
use std::io::{Read,Write};
use std::net::TcpListener;
use std::thread;
use carcasian::*;
use std::collections::HashMap;
use carcasian::database::command::Command;

fn main() {
	let host = "127.0.0.1";
	let port = "9821";

	let server = format!("{}:{}", host, port);

	let listener = TcpListener::bind("127.0.0.1:9821").unwrap(); // TODO: as_str() when RFC is approved

	println!("Welcome to Carcasian database! Listening on {}", server);

	let mut data: HashMap<String, String> = database::storage::new();
	let mut input;

	let mut command_result: &str = "(nil)";

	loop {
		io::stdout().write("127.0.0.1:9821> ".as_bytes());
		io::stdout().flush();

		input = read_line();

		let mut command: Vec<&str> = input.trim_right_matches("\n").split(" ").collect();
		let command_name: &str = command.first().unwrap();

		command.remove(0);

		if command_name == "SET" {
			command_result = database::set::Set::handle(&data, command_name, command);
		}

		println!("{}", command_result);

		command_result = "(nil)";
	}
}

fn read_line() -> String {
	let mut line = String::new();

	io::stdin().read_line(&mut line)
		.ok()
		.expect("failed to read line");

	return line;
}