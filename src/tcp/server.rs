extern crate mioco;

use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{Read, Write};
use self::mioco::mio::tcp::{TcpSocket};
use std::str;
use database::storage::Storage;
use std::sync::{Arc, Mutex};
use redis::protocol;

pub fn run(storage: Arc<Mutex<Storage>>) -> u8 {
    mioco::start(move |mioco| {
        let addr = "127.0.0.1:8991".parse().unwrap();

        let sock = try!(TcpSocket::v4());
        try!(sock.bind(&addr));
        let sock = try!(sock.listen(1024));

        let sock = mioco.wrap(sock);

        loop {
            let conn = try!(sock.accept());
            let thread_storage = storage.clone();

            mioco.spawn(move |mioco| {
                let mut conn = mioco.wrap(conn);
                let mut buf = [0u8; 1024 * 16];

                let mut data = thread_storage.lock().unwrap();

                loop {
                    let size = try!(conn.read(&mut buf));

                    if size == 0 {
                        /* eof */
                        break;
                    }

                    let human_command = protocol::get_human_command_from_redis_command(
                        str::from_utf8(&mut buf[0..size]).unwrap().trim().to_string()
                    );

                    let words: Vec<&str> = human_command.split(" ").collect();

                    if words[0] == "SET" {
                        data.set(words[1].to_string(), words[2].to_string());
                    } else if words[0] == "GET" {
                        data.get(words[1].to_string());
                    }

                    try!(conn.write_all(&mut buf[0..size]))
                }

                Ok(())
            });
        }
    });

    0x000
}