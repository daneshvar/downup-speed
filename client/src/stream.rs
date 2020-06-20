use std::io;

pub trait Client {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn send_buffer_size(&self) -> io::Result<usize>;
    fn recv_buffer_size(&self) -> io::Result<usize>;
}
