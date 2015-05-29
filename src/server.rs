extern crate carcasian;
extern crate argparse;
extern crate mio;

use std::io;
use std::io::{Read,Write,BufReader};
use std::collections::{HashSet, VecDeque};
use std::thread;
use std::str;
use std::collections::HashMap;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use mio::tcp::{TcpListener, TcpStream};
use mio::util::Slab;
use mio::{
	EventLoop,
	Handler,
	Interest,
	PollOpt,
	ReadHint,
	Token,
	TryRead,
	TryWrite,
};
use mio::buf::{Buf, RingBuf};

const SERVER: Token = Token(0);

fn main() {
	let mut host: String = "".to_string();
	let mut port: String = "".to_string();

	carcasian::util::args::parse_host_args(&mut host, &mut port, "Carcasian server");

	println!("Welcome to Carcasian database! Listening on {}:{}", host, port);

	let addr = "127.0.0.1:8991".parse().unwrap();

	let listener = TcpListener::bind(&addr).unwrap();

	// Create an event loop
	let mut event_loop = EventLoop::new().unwrap();

	// Start listening for incoming connections
	event_loop.register(&listener, SERVER).unwrap();

	let mut server = Server {
		listener: listener,
		connections: mio::util::Slab::new_starting_at(Token(3), 128)
	};

	// Start handling events
	event_loop.run(&mut server).unwrap();
}

// Define a handler to process the events
struct Server {
	listener: TcpListener,
	connections: Slab<Connection>
}

impl Handler for Server {
	type Timeout = Token;
	type Message = RingBuf;

	fn readable(&mut self, reactor: &mut EventLoop<Self>, token: Token, _: ReadHint)
	{
		match token {
			SERVER => {
				let stream = match self.listener.accept().unwrap() {
					Some(s) => s,
					None => return,
				};

				let connection = Connection::new(stream);

				let tok = self.connections.insert(connection)
					.ok().expect("Could not add connection to slab.");

				self.connections[tok].token = tok;

				reactor.register_opt(
					&self.connections[tok].stream,
					tok,
					Interest::readable(),
					PollOpt::edge() | PollOpt::oneshot()
				).ok().expect("Could not register socket with event loop.");
			},
			tok => {
				self.connections[tok].readable(reactor);
			}
		}
	}

	fn timeout(&mut self, event_loop: &mut EventLoop<Self>, timeout: Self::Timeout)
	{
	}

	fn interrupted(&mut self, event_loop: &mut EventLoop<Self>)
	{
	}

	fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message)
	{
	}
}

impl Connection
{
	fn new(sock: TcpStream) -> Connection {
		Connection {
			stream: sock,
			token: Token(0),
			interest: Interest::hup(),
			current_read: BufReader::new(RingBuf::new(4096)),
			current_write: BufReader::new(RingBuf::new(4096)),
			next_write: VecDeque::with_capacity(10),
		}
	}

	fn readable(&mut self, reactor: &mut EventLoop<Server>) -> ()
	{
		let mut read = 0;

		match self.stream.read(self.current_read.get_mut()) {
			Ok(Some(r)) => {
				read = r;
			},
			Ok(None) => panic!("We just got readable, but were unable to read from the socket?"),
			Err(e) => return,
		};

		if read > 0 {
			println!("{}", read);
		}
	}
}

struct Connection {
	stream: TcpStream,
	token: Token,
	interest: Interest,
	current_read: BufReader<RingBuf>,
	current_write: BufReader<RingBuf>,
	next_write: VecDeque<RingBuf>,
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