extern crate bytesize;

use crate::{stream::Stream, tcp, udp, utils, utils::Command};
use std;
use std::time;

fn connect(protocol: &str, addr: &str, timeout: time::Duration) -> Result<Stream, String> {
    match protocol {
        "tcp" => Ok(tcp::connect(addr, timeout)?),
        "udp" => Ok(udp::connect(addr, timeout)?),
        _ => Err(String::from("protocol not defined")),
    }
}

pub fn upload(
    protocol: &str,
    addr: &str,
    timeout: time::Duration,
    duration: time::Duration,
) -> Result<(), String> {
    let mut stream = connect(protocol, addr, timeout)?;

    stream.command(Command::Upload)?;

    let mut size: usize = 1024 * 500; // 500 KiB
    let mut send: usize = size;
    let mut sent: usize = 0;
    let mut buf: Vec<u8> = vec![65; size]; // 'A'

    let mut remain: time::Duration = duration;
    let mut elapse = time::Duration::new(0, 0);
    let mut b = true;
    while b {
        let instant = time::Instant::now();
        stream.write_all(&buf[0..send])?;
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
            utils::CLEAR_PREV_LINE,
            utils::fmt_bytes(sent),
            utils::fmt_speed(sent, elapse),
            utils::fmt_duration(elapse),
            utils::fmt_duration(remain),
        );

        // adjust buffer size to TCP Window Size
        if let Ok(s) = stream.write_buffer_size() {
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

    stream.command(Command::Finish)
}

pub fn download(
    protocol: &str,
    addr: &str,
    timeout: time::Duration,
    duration: time::Duration,
) -> Result<(), String> {
    let mut stream = connect(protocol, addr, timeout)?;

    stream.command(Command::Download)?;

    let size: usize = 1024 * 500; // 500 KiB
    let mut download: usize = 0;
    let mut buf: Vec<u8> = vec![0; size];
    let mut remain: time::Duration = duration;
    let mut elapse = time::Duration::new(0, 0);
    let mut b = true;
    while b {
        let read: usize;
        let instant = time::Instant::now();
        match stream.read(&mut buf) {
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
            utils::CLEAR_PREV_LINE,
            utils::fmt_bytes(download),
            utils::fmt_speed(download, elapse),
            utils::fmt_duration(elapse),
            utils::fmt_duration(remain),
        );

        // adjust buffer size to TCP Window Size
        if let Ok(s) = stream.read_buffer_size() {
            // ToDo: resize buffer
            buf = vec![0; s];
        }
    }
    stream.command(Command::Finish)
}
