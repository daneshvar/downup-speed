use net2::TcpStreamExt;
use std::io;
use std::net;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

impl super::stream::Stream for TcpStream {
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

pub fn connect(
    addr: &str,
    timeout: time::Duration,
) -> Result<Box<dyn super::stream::Stream>, String> {
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

    Ok(Box::new(stream))
}

pub fn listen(addr: &str, f: fn(stream: Box<dyn super::stream::Stream>)) -> Result<(), String> {
    let listener;
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
                thread::spawn(move || {
                    // connection succeeded
                    f(Box::new(stream));
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
