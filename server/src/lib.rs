extern crate bytesize;

use std;
use std::io;
use std::io::Read;
use std::fs::File;
use std::time;
use std::time::Instant;
use bytesize::ByteSize;

pub mod stream;
mod tcp;
mod udp;

pub async fn run(protocol: &str, addr: &str) -> Result<(), String> {
    let mut server;
    match protocol {
        "tcp" => server = &Server{stream: tcp::connect(addr)?},
        "udp" => server = &Server{stream: udp::connect(addr)?},
        _ => return Err(String::from("protocol not defined")),
    }

    server.listen()

    Ok(())
}

// macro_rules! clrscr {
//     () => {
//         print!("\x1B[2J\x1B[1;1H"); // clear entire screen
//     };
// }

macro_rules! clrline {
    () => {
        print!("\x1b[0F"); // Prev Line: \u001b[{n}F moves cursor to beginning of line n lines down
        print!("\x1b[2K"); // Clear Line: clears entire line
    };
}

pub struct Server {
    stream: Box<dyn stream::Client>,
}

impl Server {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn command(&mut self, command: String) -> Result<(), String> {
        let buf = command.as_bytes();
        let size = buf.len();
        let mut sent: usize = 0;
        while sent < size {
            let b = &buf[sent..size];
            match self.write(b) {
                Ok(r) => sent += r,
                Err(e) => return Err(format!("Error on command: {}", e)),
            }
        }

        Ok(())
    }

    fn cmd_upload(&mut self, size: usize) -> Result<(), String> {
        self.command(format!("UPLOAD:{}", size))
    }

    // fn cmd_close(&mut self) -> Result<(), String> {
    //     self.command(String::from("CLOSE"))
    // }

    async fn Server();
}

impl Client {
    fn read_file(file_name: &str, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut f = File::open(file_name)?;
        f.read_to_end(buf)
    }

    fn calc_speed(len: usize, nanos: u128) -> u64 {
        (len as u128 * 1_000_000_000 / nanos) as u64
    }

    pub fn upload_file(&mut self, file_name: &str) -> Result<(), String> {
        let size;
        let mut buf = Vec::new();
        match Self::read_file(file_name, &mut buf) {
            Ok(s) => size = s,
            Err(e) => return Err(format!("{}", e)),
        }
        self.upload(&buf, size)
    }

    pub fn upload(&mut self, buf: &[u8], size: usize) -> Result<(), String> {
        self.cmd_upload(size)?;

        let mut sum_duration: u128 = 0;
        let start_upload = Instant::now();
        let mut sent: usize = 0;
        while sent < size {
            let b = &buf[sent..size];

            let start = Instant::now();
            let result ;
            match self.write(b) {
                Ok(r) => result = r,
                Err(e) => return Err(format!("Error on write: {}", e)),
            }

            let duration = start.elapsed();

            sent += result;
            sum_duration += duration.as_nanos();

            let speed = Self::calc_speed(result, duration.as_nanos());
            let sum_speed = Self::calc_speed(sent, sum_duration);

            clrline!();
            println!(
                "Sent {} of {} Speed Sent: {:?}/s Speed Total: {:?}/s Remaining: {}s",
                ByteSize(sent as u64),
                ByteSize(size as u64),
                ByteSize(speed),
                ByteSize(sum_speed),
                (size - sent) as u64 / sum_speed
            );
        }

        let total_duration = start_upload.elapsed();
        let total_speed = Self::calc_speed(sent, total_duration.as_nanos());
        // clrscr!();
        println!(
            "Sent: {}\nSpeed: {}/s\nDuration: {:?} -> {:?}",
            ByteSize(size as u64),
            ByteSize(total_speed),
            total_duration,
            time::Duration::from_nanos(sum_duration as u64),
        );

        Ok(())
    }
}

impl Client {
    pub fn download(&self, buf: &mut [u8]) -> Result<(), String> {
        self.stream.download(buf)
    }
}
