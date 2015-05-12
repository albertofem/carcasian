extern crate carcasian;
extern crate argparse;

use std::io;
use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::thread;
use carcasian::*;
use std::collections::HashMap;
use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
	let mut host: String = "".to_string();
	let mut port: String = "".to_string();

	util::args::parse_host_args(&mut host, &mut port, "Carcasian server");

	println!("Welcome to Carcasian database! Listening on {}:{}", host, port);

	let mut data: HashMap<String, String> = database::storage::new();

	server(&host, &port, &mut data);
}

fn server(host: &String, port: &String, data: &HashMap<String, String>) -> u8
{
	let server: String = format!("{}:{}", host, port);

	let listener = TcpListener::bind(&*server).unwrap();

	for stream in listener.incoming() {
		match stream {
			Err(e) => { println!("failed: {}", e) }
			Ok(stream) => {
				thread::spawn(move || {
					handle_client(stream)
				});
			}
		}

		/*let mut command: Vec<&str> = input.trim_right_matches("\n").split(" ").collect();
		let command_name: &str = command.first().unwrap();

		command.remove(0);

		if command_name == "SET" {
			stream.write(database::setget::SetGet.set(&mut data, command[0], command[1]));
		} else if command_name == "GET" {
			stream.write(database::setget::SetGet.get(&mut data, command[0]));
		} else if command_name == "EXIT" {
			stream.write("Bye");
		}*/
	}

	0x000
}

fn handle_client(mut stream: TcpStream)
{
	let mut buf;

	println!("Client connected");

	loop {
		buf = [0; 512];
		let message = match stream.read(&mut buf) {
			Err(e) => panic!("Reading error: {}", e),
			Ok(m) => {
				if m == 0 {
					break;
				}
				m
			},
		};

		match stream.write(&buf) {
			Err(_) => break,
			Ok(_) => continue,
		}
	}
}