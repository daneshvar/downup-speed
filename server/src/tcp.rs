use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io;
use std::thread;

pub fn listen(addr: &str, f: fn(stream: &mut super::stream::Server)) -> Result<(), String> {
    let mut listener;
    match TcpListener::bind(addr) {
        Ok(l) => listener = l,
        Err(err) => return Err(format!("{}", err)),
    }

    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    f(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);

    Ok(())
}

impl super::stream::Server for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }

    fn download(&self, _buf: &mut [u8]) -> Result<(), String> {
        Ok(())
    }
}

