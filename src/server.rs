use crate::{stream::Stream, tcp, udp, utils::Command};
use std::convert::TryInto;
use std::time;

pub fn listen(protocol: &str, addr: &str, _timeout: time::Duration) -> Result<(), String> {
    match protocol {
        "tcp" => tcp::listen(addr, incoming),
        "udp" => udp::listen(addr, incoming),
        _ => Err(String::from("protocol not defined")),
    }
}

fn incoming(stream: &mut Stream) {
    println!("incoming");
    let s = "Helloooooo\n";
    // let mut stream = stream_in;

    let mut cmd: [u8; 1] = [0; 1];
    loop {
        if let Err(e) = stream.read(&mut cmd) {
            println!("Error on read command: {}", e);
            break;
        }

        let c = cmd[0];

        let r = if c == Command::Upload as u8 {
            upload(stream)
        } else if c == Command::Download as u8 {
            download(stream)
        } else if c == Command::Finish as u8 {
            Ok(())
        } else {
            Ok(())
        };

        if let Err(e) = r {
            println!("Error on incoming: {}", e);
            break;
        }
    }
}

fn upload(stream: &mut Stream) -> Result<(), String> {
    println!("Client Uploading");

    let size: usize = 1024 * 10;
    let mut buf: Vec<u8> = vec![0; size];
    loop {
        let read = stream.read(&mut buf)?;
        for i in 0..read {
            if buf[i] == Command::Finish as u8 {
                println!("Client Upload Finish");
                return Ok(());
            }
        }
    }
}

fn download(stream: &mut Stream) -> Result<(), String> {
    println!("Client Downloading");

    let size: usize = 1024 * 10;
    let mut buf: Vec<u8> = vec![65; size]; // 'A'
    loop {
        stream.write_all(&mut buf)?;
        // for i in 0..read {
        //     if buf[i] == Command::Finish as u8 {
        //         println!("Client Download Finish");
        //         return Ok(());
        //     }
        // }
    }
}
