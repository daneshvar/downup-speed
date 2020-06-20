use std::io;

pub trait Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn write_buffer_size(&self) -> io::Result<usize>;
    fn read_buffer_size(&self) -> io::Result<usize>;
}
