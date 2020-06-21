use crate::stream::{NetStream, Stream};
use net2::UdpSocketExt;
use std::io;
use std::net::UdpSocket;
use std::time;

impl NetStream for UdpSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.send(buf)
    }
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv(buf)
    }
    fn write_buffer_size(&self) -> io::Result<usize> {
        self.send_buffer_size()
    }
    fn read_buffer_size(&self) -> io::Result<usize> {
        self.recv_buffer_size()
    }
}

pub fn connect(addr: &str, _timeout: time::Duration) -> Result<Stream, String> {
    match UdpSocket::bind(addr) {
        Ok(socket) => Ok(Stream::new(Box::new(socket))),
        Err(err) => Err(format!("{}", err)),
    }
}

pub fn listen(_addr: &str, _f: fn(stream: &mut Stream, remote: String)) -> Result<(), String> {
    Err(String::from("UDP Listen doesn't implement"))
}
