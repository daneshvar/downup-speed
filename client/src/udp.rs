use std::io;
use std::net::UdpSocket;
use std::time;

// pub struct UDPClient {
//     socket: UdpSocket,
// }

pub fn connect(
    addr: &str,
    _timeout: time::Duration,
) -> Result<Box<dyn super::stream::Client>, String> {
    match UdpSocket::bind(addr) {
        Ok(socket) => Ok(Box::new(socket)),
        Err(err) => Err(format!("{}", err)),
    }
}

impl super::stream::Client for UdpSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.send(buf)
    }
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
    fn send_buffer_size(&self) -> io::Result<usize> {
        Ok(1024 * 500)
    }
    fn recv_buffer_size(&self) -> io::Result<usize> {
        Ok(1024 * 500)
    }
}

// impl Drop for UdpSocket {
//     fn drop(&mut self) {
//         println!("Close UDP")
//     }
// }
