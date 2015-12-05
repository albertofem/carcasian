extern crate carcasian;

use std::io;
use std::io::{Read,Write};
use std::process::exit;
use carcasian::*;
use std::net::TcpStream;

fn main() {
	let mut host: String = "".to_string();
	let mut port: String = "".to_string();

	util::args::parse_host_args(&mut host, &mut port, "Carcasian command line interface");

	cli(&host, &port);
}

fn cli(host: &String, port: &String) -> u8
{
	let server = format!("{}:{}", host, port);
	let mut stream = TcpStream::connect(&*server);

	match stream {
		Err(e) => { println!("Unable to connect: {}", e) }
		Ok(mut stream) => {
			loop {
				cli_loop(host, port, &mut stream);
			}
		}
	}

	0x000
}

fn cli_loop(host: &String, port: &String, stream: &mut TcpStream) -> ()
{
	print!("{}:{}> ", host, port);
	io::stdout().flush();

	let mut input = util::io::read_line();

	println!("Sending to server: {}", input);

    // convert string command to redis protocol (using module)

    // send command to server

    // read response from server and parse using redis protocl (module)

    // convert back to string for human representation

    // write response to client

	stream.write(input.as_bytes());

	let mut buf = [0];
	stream.read(&mut buf);

	println!("{:?}", buf);
}