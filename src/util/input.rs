use std::io;

pub fn read_line() -> String {
	let mut line = String::new();

	io::stdin().read_line(&mut line)
		.ok()
		.expect("failed to read line");

	return line;
}