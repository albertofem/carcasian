extern crate mioco;

use std::io::{Read, Write};
use self::mioco::mio::tcp::{TcpSocket};
use std::str;
use database::storage::Storage;
use std::sync::{Arc, Mutex};
use redis::driver;

/// This is the main TCP server structure
///
/// We have the data (which is wrapped in an Arc<Mutex<...>>
/// because we are going to use it in multiple threads at
/// the same time. Also containing host and port to the
/// TcpListener
pub struct Server {
    data: Arc<Mutex<Storage>>,
    host: String,
    port: String
}

impl Server {
    /// Creates a new server instance with given storage
    ///
    /// # Examples
    ///
    /// ```
    /// use carcasian::tcp::server::Server;
    /// use carcasian::database::storage::Storage;
    ///
    /// let server = Server::new(Storage::new(), "127.0.0.1".to_string(), "8888".to_string());
    /// ```
    pub fn new(storage: Storage, host: String, port: String) -> Server {
        Server {
            data: Arc::new(Mutex::new(storage)),
            host: host,
            port: port
        }
    }

    /// Run server
    ///
    /// This will block the main thread preventing any more
    /// action from taking place
    ///
    pub fn run(self) -> bool {
        // We 'move' everything from outside the closure
        // inside of it, in this case, 'storage'
        mioco::start(move |mioco| {
            let addr = format!("{}:{}", self.host, self.port).parse().unwrap();

            let sock = try!(TcpSocket::v4());
            try!(sock.bind(&addr));
            let sock = try!(sock.listen(1024));

            let sock = mioco.wrap(sock);

            // Enter in a infinite loop to accept
            // al incoming connections
            loop {
                let conn = sock.accept().unwrap();

                // After accepting one connection, we first clone
                // the reference to the storage (thus increasing the
                // atomic reference counting in the Arc abstraction)
                let data = self.data.clone();

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

                        match response {
                            Ok(driver::DriverResponse::Response(r)) => try!(write!(conn, "{}", r)),
                            Ok(driver::DriverResponse::Quit) => break,
                            _ => unreachable!()
                        }
                    }

                    Ok(())
                });
            }
        });

        true
    }
}
