extern crate carcasian;

use std::io;
use std::io::{Read,Write};
use std::net::TcpListener;
use std::thread;
use carcasian::*;
use std::collections::HashMap;

fn main() {
	let mut host;
	let mut port;

	parse_args(&host, &port);

	println!("Welcome to Carcasian database! Listening on {}", server);

	let mut data: HashMap<String, String> = database::storage::new();

	server(&host, &port, &mut data);
}

fn server(&host: String, &port: String, &data: HashMap<String, String>) -> u8
{
	let listener = TcpListener::bind(host, port).unwrap();

	for stream in listener.incoming() {
		thread::spawn(|| {
			let mut command: Vec<&str> = input.trim_right_matches("\n").split(" ").collect();
			let command_name: &str = command.first().unwrap();

			command.remove(0);

			if command_name == "SET" {
				stream.write(database::setget::SetGet.set(&mut data, command[0], command[1]));
			} else if command_name == "GET" {
				stream.write(database::setget::SetGet.get(&mut data, command[0]));
			} else if command_name == "EXIT" {
				stream.write("Bye");
			}
		});
	}
}

fn parse_args(&host: String, &port: String) -> ()
{
	let mut ap = ArgumentParser::new();

	ap.set_description("Carcasian database server");

	ap.refer(&mut host).add_option(
		&["-h", "--host"],
		Store,
		"127.0.0.1"
	);

	ap.refer(&mut name).add_option(
		&["-p", "--port"],
		Store,
		"8991"
	);

	ap.parse_args_or_exit();
}