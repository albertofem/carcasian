extern crate carcasian;
extern crate argparse;

use carcasian::database::storage::Storage;
use carcasian::tcp::server;

fn main() {
    let mut host: String = "".to_string();
    let mut port: String = "".to_string();

    carcasian::util::args::parse_host_args(&mut host, &mut port, "Carcasian server");

    println!("Welcome to Carcasian database!");
    println!("Listening on {}:{}", host, port);

    server::run();
}