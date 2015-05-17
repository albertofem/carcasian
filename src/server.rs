extern crate carcasian;
extern crate argparse;
extern crate mio;

use std::io;
use std::io::{Read,Write};
use std::thread;
use std::str;
use std::collections::HashMap;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use mio::*;
use mio::tcp::{TcpListener, TcpStream};

const SERVER: Token = Token(0);

fn main() {
	let mut host: String = "".to_string();
	let mut port: String = "".to_string();

	carcasian::util::args::parse_host_args(&mut host, &mut port, "Carcasian server");

	println!("Welcome to Carcasian database! Listening on {}:{}", host, port);

	let server: String = format!("{}:{}", host, port);

	let server = TcpListener::bind(&*server).unwrap();

	// Create an event loop
	let mut event_loop = EventLoop::new().unwrap();

	// Start listening for incoming connections
	event_loop.register(&server, SERVER).unwrap();

	// Start handling events
	event_loop.run(&mut MyHandler(server)).unwrap();
}

// Define a handler to process the events
struct MyHandler(TcpListener);

impl Handler for MyHandler {
	type Timeout = u8;
	type Message = String;

	fn readable(&mut self, event_loop: &mut EventLoop<Self>, token: Token, _: ReadHint)
	{
		match token {
			SERVER => {
				let MyHandler(ref mut server) = *self;
				let _ = server.accept();

				println!("Client connected");
			}
			_ => panic!("unexpected token"),
		}
	}

	fn timeout(&mut self, event_loop: &mut EventLoop<Self>, timeout: Self::Timeout)
	{
		println!("Client timeout");
	}

	fn interrupted(&mut self, event_loop: &mut EventLoop<Self>)
	{
		println!("Client disconnected");
	}

	fn notify(&mut self, event_loop: &mut EventLoop<MyHandler>, msg: Self::Message)
	{
		println!("Hello there: {}", msg);
	}
}

/*
fn server(host: &String, port: &String, data: Arc<Mutex<HashMap<String, String>>>) -> u8
{
	let server: String = format!("{}:{}", host, port);

	let listener = TcpListener::bind(&*server).unwrap();

	for stream in listener.incoming() {
		match stream {
			Err(e) => { println!("failed: {}", e) }
			Ok(stream) => {
				thread::spawn(|| {
					handle_client(&mut stream, data)
				});
			}
		}
	}

	0x000
}

fn handle_client(stream: &mut TcpStream, database: Arc<Mutex<HashMap<String, String>>>)
{
	let (tx, rx) = channel();
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

		let (data, tx) = (database.clone(), tx.clone());

		handle_message(str::from_utf8(&mut buf).unwrap().to_string(), &mut stream, data);

		tx.send(()).unwrap();
		rx.recv().unwrap();
	}
}

fn handle_message(input: String, stream: &mut TcpStream, data: Arc<Mutex<HashMap<String, String>>>)
{
	let mut database = data.lock().unwrap();

	let mut command: Vec<&str> = input.trim_right_matches("\n").split(" ").collect();
	let command_name: &str = command.first().unwrap();

	command.remove(0);

	if command_name == "SET" {
		stream.write(database::setget::SetGet.set(&mut database, command[0], command[1]).as_bytes());
	} else if command_name == "GET" {
		stream.write(database::setget::SetGet.get(&mut database, command[0]).as_bytes());
	} else if command_name == "EXIT" {
		stream.write(b"Bye");
	}
}*/