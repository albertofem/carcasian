extern crate mioco;

use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{Read, Write};
use self::mioco::mio::tcp::{TcpSocket};
use std::str;

pub fn run() -> u8 {
    mioco::start(move |mioco| {
        let addr = "127.0.0.1:8991".parse().unwrap();

        let sock = try!(TcpSocket::v4());
        try!(sock.bind(&addr));
        let sock = try!(sock.listen(1024));

        let sock = mioco.wrap(sock);

        loop {
            let conn = try!(sock.accept());

            mioco.spawn(move |mioco| {
                let mut conn = mioco.wrap(conn);
                let mut buf = [0u8; 1024 * 16];

                loop {
                    let size = try!(conn.read(&mut buf));

                    if size == 0 {
                        /* eof */
                        break;
                    }

                    println!("Sent to server!: {}", str::from_utf8(&mut buf[0..size]).unwrap());

                    try!(conn.write_all(&mut buf[0..size]))
                }

                Ok(())
            });
        }
    });

    0x000
}