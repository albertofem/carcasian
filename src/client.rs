extern crate carcasian;
extern crate argparse;

use std::io;
use std::io::{Read,Write};
use std::process::exit;
use carcasian::*;
use argparse::{ArgumentParser, Store};

fn main() {
	let mut host: String;
	let mut port: String;

	parse_args(&mut host, &mut port);

	cli(&host, &port);
}

fn cli(&host: String, &port: String) -> u8
{
	// conect to tcp socket

	print!("{}:{}> ", host, port);
	io::stdout().flush();

	let mut input = carcasian::util::read_line();

	// send input message to socket

	// read and print answer
}

fn parse_args(&host: String, &port: String) -> ()
{
	let mut ap = ArgumentParser::new();

	ap.set_description("Carcasian database cli client");

	ap.refer(&mut host).add_option(
			&["-h", "--host"],
			Store,
			"127.0.0.1"
		);

	ap.refer(&mut port).add_option(
		&["-p", "--port"],
		Store,
		"8991"
	);

	ap.parse_args_or_exit();
}