use std::io;

pub trait Server {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn download(&self, buf: &mut [u8]) -> Result<(), String>;
}
