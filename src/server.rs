extern crate carcasian;
extern crate argparse;

use carcasian::database::storage::Storage;
use carcasian::tcp::server;
use std::sync::{Arc, Mutex};

fn main() {
    let mut host: String = "".to_string();
    let mut port: String = "".to_string();

    carcasian::util::args::parse_host_args(&mut host, &mut port, "Carcasian server");

    println!("Welcome to Carcasian database!");
    println!("Listening on {}:{}", host, port);

    // We need to wrap our storage in a Arc with Mutex:
    // Arc in order to have atomic reference counting and
    // Mutex to prevent data races between threads
    let storage = Arc::new(Mutex::new(Storage::new()));

    server::run(storage);
}