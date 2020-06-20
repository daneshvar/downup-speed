use net2::TcpStreamExt;
use std::io;
use std::io::{Read, Write};
use std::net;
use std::net::TcpStream;
use std::time;

pub struct TCPClient {
    stream: TcpStream,
}

pub fn connect(
    addr: &str,
    timeout: time::Duration,
) -> Result<Box<dyn super::stream::Client>, String> {
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

    Ok(Box::new(TCPClient { stream }))
}

impl super::stream::Client for TCPClient {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
    fn send_buffer_size(&self) -> io::Result<usize> {
        self.stream.send_buffer_size()
    }
    fn recv_buffer_size(&self) -> io::Result<usize> {
        self.stream.recv_buffer_size()
    }
}

// impl Drop for TcpStream {
//     fn drop(&mut self) {
//         println!("Close TCP")
//     }
// }
