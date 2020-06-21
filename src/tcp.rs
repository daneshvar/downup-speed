use crate::stream::{NetStream, Stream};
use net2::TcpStreamExt;
use std::io;
use std::net;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

impl NetStream for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::Write::write(self, buf)
    }
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        io::Read::read(self, buf)
    }
    fn write_buffer_size(&self) -> io::Result<usize> {
        self.send_buffer_size()
    }
    fn read_buffer_size(&self) -> io::Result<usize> {
        self.recv_buffer_size()
    }
}

pub fn connect(addr: &str, timeout: time::Duration) -> Result<Stream, String> {
    let socket_addr: net::SocketAddr = addr.parse().expect("Addr Parse Error");
    let stream;
    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(s) => stream = s,
        Err(e) => return Err(format!("TCP Connection: {}", e)),
    }

    if let Err(e) = stream.set_write_timeout(Some(timeout)) {
        return Err(format!("Set Write Timeout: {}", e));
    }

    if let Err(e) = stream.set_read_timeout(Some(timeout)) {
        return Err(format!("Set Read Timeout: {}", e));
    }

    Ok(Stream::new(Box::new(stream)))
}

pub fn listen(
    addr: &str,
    incoming_fn: fn(stream: &mut Stream, remote: String),
) -> Result<(), String> {
    let listener;
    match TcpListener::bind(addr) {
        Ok(l) => listener = l,
        Err(err) => return Err(format!("{}", err)),
    }

    // accept connections and process them, spawning a new thread for each one
    println!("Server listening: {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    let remote = format!("{}", stream.peer_addr().unwrap());
                    incoming_fn(&mut Stream::new(Box::new(stream)), remote)
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
