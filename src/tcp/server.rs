extern crate mioco;

use std::io::{Read, Write};
use self::mioco::mio::tcp::{TcpSocket};
use std::str;
use database::storage::Storage;
use std::sync::{Arc, Mutex};
use redis::driver;

pub fn run(storage: Arc<Mutex<Storage>>) -> u8 {
    // We 'move' everything from outside the closure
    // inside of it, in this case, 'storage'
    mioco::start(move |mioco| {
        // TODO: make this parametrizable
        let addr = "127.0.0.1:8991".parse().unwrap();

        let sock = try!(TcpSocket::v4());
        try!(sock.bind(&addr));
        let sock = try!(sock.listen(1024));

        let sock = mioco.wrap(sock);

        // Enter in a infinite loop to accept
        // al incoming connections
        loop {
            let conn = try!(sock.accept());

            // After accepting one connection, we first clone
            // the reference to the storage (thus increasing the
            // atomic reference counting in the Arc abstraction)
            let data = storage.clone();

            // Move everything to another thread, including the
            // Arc and the connection
            mioco.spawn(move |mioco| {
                let mut conn = mioco.wrap(conn);

                // Initialize a buffer to store data from the
                // socket
                let mut buf = [0u8; 1024 * 16];

                // Look and construct a command from the
                // socket data stream
                loop {
                    let size = try!(conn.read(&mut buf));

                    if size == 0 {
                        // We hit EOF, it's time to start
                        // constructing another command
                        break;
                    }

                    let command = str::from_utf8(&mut buf[0..size]).unwrap().trim().to_string();

                    // we need to clone again because Arc is non copyable (yeah, it has references!)
                    let response = driver::handle_command(command, data.clone());

                    // Return response to client
                    try!(write!(conn, "{}", response))
                }

                Ok(())
            });
        }
    });

    0x000
}
