extern crate bytesize;

use bytesize::ByteSize;
use std;
use std::fs::File;
// use std::thread;
// use std::time::Duration;
use std::io::Read;
use std::time;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::prelude::*;

macro_rules! clrscr {
    () => {
        //print!("\x1B[2J\x1B[1;1H"); // clear entire screen
        print!("\x1b[0F"); // Prev Line: \u001b[{n}F moves cursor to beginning of line n lines down
        print!("\x1b[2K"); // Clear Line: clears entire line
    };
}

pub async fn send() -> io::Result<usize> {
    let addr = "45.82.137.212:6000";
    //let addr = "127.0.0.1:6000";

    let mut buf = Vec::new();
    let total = get_file("mock/Dariush - Delkhosham.mp3", &mut buf)?;

    let mut stream = TcpStream::connect(addr).await?;
    println!("created stream");

    let mut sum_duration: u128 = 0;
    let start_upload = Instant::now();

    let mut sent: usize = 0;
    while sent < total {
        let b = &buf[sent..total];

        let start = Instant::now();
        let result = stream.write(b).await?;
        let duration = start.elapsed();

        sent += result;
        sum_duration += duration.as_nanos();

        let speed = calc_speed(result, duration.as_nanos());
        let sum_speed = calc_speed(sent, sum_duration);

        clrscr!();
        println!(
            "Sent {} of {} Speed Sent: {:?}/s Speed Total: {:?}/s Remaining: {}s",
            ByteSize(sent as u64),
            ByteSize(total as u64),
            ByteSize(speed),
            ByteSize(sum_speed),
            (total - sent) as u64 / sum_speed
        );
    }

    let total_duration = start_upload.elapsed();
    let total_speed = calc_speed(sent, total_duration.as_nanos());
    // clrscr!();
    println!(
        "Sent: {}\nSpeed: {}/s\nDuration: {:?} -> {:?}",
        ByteSize(total as u64),
        ByteSize(total_speed),
        total_duration,
        time::Duration::from_nanos(sum_duration as u64),
    );

    Ok(total)
}

fn calc_speed(len: usize, nanos: u128) -> u64 {
    (len as u128 * 1_000_000_000 / nanos) as u64
}

fn get_file(file_name: &str, buf: &mut Vec<u8>) -> io::Result<usize> {
    let mut f = File::open(file_name)?;
    f.read_to_end(buf)
}
