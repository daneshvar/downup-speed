use std::net::UdpSocket;
use std::io;

pub struct UDPServer {
    socket: UdpSocket,
}

pub fn connect(addr: &str) -> Result<Box<dyn super::stream::Client>, String> {
    match UdpSocket::bind(addr) {
        Ok(socket) => Ok(Box::new(UDPClient { socket })),
        Err(err) => Err(format!("{}", err)),
    }
}

impl super::stream::Server for UDPServer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.socket.send(buf)
    }

    fn download(&self, _buf: &mut [u8]) -> Result<(), String> {
        Ok(())
    }
}

impl Drop for UDPServer {
    fn drop(&mut self) {
        println!("Close UDP")
    }
}
