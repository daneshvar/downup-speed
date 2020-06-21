use crate::utils::Command;
use std::io;

pub trait NetStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn write_buffer_size(&self) -> io::Result<usize>;
    fn read_buffer_size(&self) -> io::Result<usize>;
}

pub struct Stream {
    net: Box<dyn NetStream>,
}

impl Stream {
    pub fn new(net: Box<dyn NetStream>) -> Stream {
        Stream { net }
    }

    pub fn write_buffer_size(&self) -> io::Result<usize> {
        self.net.write_buffer_size()
    }

    pub fn read_buffer_size(&self) -> io::Result<usize> {
        self.net.read_buffer_size()
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        match self.net.read(buf) {
            Ok(0) => Err(String::from("Zero Read")),
            Ok(n) => Ok(n),
            Err(e) => Err(format!("{}", e)),
        }
    }

    // pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    //     self.net.write(buf)
    // }

    // write buf and return size of wrote
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, String> {
        match self.net.write(&buf) {
            Ok(n) => Ok(n),
            Err(e) => Err(format!("{}", e)),
        }
    }

    // write entire buf
    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), String> {
        let size: usize = buf.len();
        let mut sent: usize = 0;
        while sent < size {
            sent += self.write(&buf[sent..size])?;
        }

        Ok(())
    }

    pub fn command(&mut self, command: Command) -> Result<(), String> {
        let buf = [command as u8; 1];
        self.write(&buf)?;
        Ok(())
    }
}
