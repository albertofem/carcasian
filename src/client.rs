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
	let mut stream = TcpStream::connect("127.0.0.1:8991");

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

	stream.write(input.as_bytes());
}