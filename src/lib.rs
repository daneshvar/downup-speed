extern crate bytesize;

use crate::Command::{Download, Finish, Upload};
use bytesize::ByteSize;
use std;
use std::time;

pub mod stream;
mod tcp;
mod udp;

pub const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";
pub const CLEAR_PREV_LINE: &str = "\x1b[0F\x1b[2K";

enum Command {
    Upload = 0x02,   // ASCII control characters: "Start of Text"
    Download = 0x03, // ASCII control characters: "End of Text"
    Finish = 0x04,   // ASCII control characters: "End of Transmission"
}

pub fn connect(protocol: &str, addr: &str, timeout: time::Duration) -> Result<Client, String> {
    match protocol {
        "tcp" => Ok(Client {
            stream: tcp::connect(addr, timeout)?,
        }),
        "udp" => Ok(Client {
            stream: udp::connect(addr, timeout)?,
        }),
        _ => Err(String::from("protocol not defined")),
    }
}

pub struct Client {
    stream: Box<dyn stream::Client>,
}

impl Client {
    // write buf and return size of wrote
    fn write_buf(&mut self, buf: &[u8]) -> Result<usize, String> {
        match self.stream.write(&buf) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Error on write: {}", e)),
        }
    }

    // write entire buf
    fn write(&mut self, buf: &[u8]) -> Result<(), String> {
        let size: usize = buf.len();
        let mut sent: usize = 0;
        while sent < size {
            match self.write_buf(&buf[sent..size]) {
                Ok(0) => return Err(String::from("failed to write")),
                Ok(s) => sent += s,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn command(&mut self, command: Command) -> Result<(), String> {
        let buf = [command as u8; 1];
        match self.write(&buf) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Error on command: {}", e)),
        }
    }
}

fn fmt_speed(s: usize, t: time::Duration) -> String {
    fmt_bytes((s as u128 * 1_000_000_000 / t.as_nanos()) as usize) + "/s"
}

fn fmt_bytes(s: usize) -> String {
    ByteSize(s as u64).to_string_as(true)
}

fn fmt_duration(t: time::Duration) -> String {
    format!("{:.1}s", t.as_secs_f32())
}

impl Client {
    // Upload a ASCII Buffer to the server at duration
    pub fn upload(&mut self, duration: time::Duration) -> Result<(), String> {
        self.command(Upload)?;

        // print title
        // println!(
        //     "[Upload] {:<12} | {:<12} | {:<12} | {}\n",
        //     "Sent", "Speed", "Elapse", "Remain",
        // );

        let mut size: usize = 1024 * 500; // 500 KiB
        let mut send: usize = size;
        let mut sent: usize = 0;
        let mut buf: Vec<u8> = vec![65; size]; // 'A'

        let mut remain: time::Duration = duration;
        let mut elapse = time::Duration::new(0, 0);
        let mut b = true;
        while b {
            let instant = time::Instant::now();
            self.write(&buf[0..send])?;
            let duration = instant.elapsed();

            sent += send;
            elapse += duration;

            if remain < duration {
                b = false;
                remain -= remain;
            } else {
                remain -= duration;
            }

            // ToDo: Speed Avg, Max, Min
            println!(
                "{}[Upload  ] {:<12} | {:<12} | {:<12} | {}",
                CLEAR_PREV_LINE,
                fmt_bytes(sent),
                fmt_speed(sent, elapse),
                fmt_duration(elapse),
                fmt_duration(remain),
            );

            // adjust buffer size to TCP Window Size
            if let Ok(s) = self.stream.send_buffer_size() {
                if s < send {
                    send = s;
                } else if s > send {
                    send = s + 1024 * 100; // Inc 100 KiB;
                    if send > size {
                        size = send;
                        // ToDo: resize buffer
                        buf = vec![65; size]; // 'A'
                    }
                }
            }
        }

        self.command(Finish)
    }
}

impl Client {
    pub fn download(&mut self, duration: time::Duration) -> Result<(), String> {
        self.command(Download)?;

        let size: usize = 1024 * 500; // 500 KiB
        let mut download: usize = 0;
        let mut buf: Vec<u8> = vec![0; size];
        let mut remain: time::Duration = duration;
        let mut elapse = time::Duration::new(0, 0);
        let mut b = true;
        while b {
            let read: usize;
            let instant = time::Instant::now();
            match self.stream.read(&mut buf) {
                Ok(0) => return Err(format!("Download Read Zero")),
                Ok(s) => read = s,
                Err(e) => return Err(format!("Download: {}", e)),
            }
            let duration = instant.elapsed();

            download += read;
            elapse += duration;

            if remain < duration {
                b = false;
                remain -= remain;
            } else {
                remain -= duration;
            }

            println!(
                "{}[Download] {:<12} | {:<12} | {:<12} | {}",
                CLEAR_PREV_LINE,
                fmt_bytes(download),
                fmt_speed(download, elapse),
                fmt_duration(elapse),
                fmt_duration(remain),
            );

            // adjust buffer size to TCP Window Size
            if let Ok(s) = self.stream.recv_buffer_size() {
                // ToDo: resize buffer
                buf = vec![0; s];
            }
        }
        self.command(Finish)
    }
}
