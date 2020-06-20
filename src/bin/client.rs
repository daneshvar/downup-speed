extern crate parse_duration;

use serde_derive::Deserialize;
use std::env;
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    server: Server,
    test: Test,
}

#[derive(Debug, Deserialize)]
struct Server {
    protocol: String,
    address: String,
    timeout: String,
}

#[derive(Debug, Deserialize)]
struct Test {
    download_time: String,
    upload_time: String,
}

fn main() {
    let config = load_config().expect("Error on load config");

    let protocol = config.server.protocol.as_str();
    let addr = config.server.address.as_str();
    let timeout = parse_duration::parse(config.server.timeout.as_str())
        .expect("Error on Parse config server timeout");
    let upload_duration = parse_duration::parse(config.test.upload_time.as_str())
        .expect("Error on Parse config upload time");
    let download_duration = parse_duration::parse(config.test.download_time.as_str())
        .expect("Error on Parse config upload time");

    println!(
        "Server  : {}\nProtocol: {}\nTimeout : {:?}\nUpload  : {:?}\nDownload: {:?}\n",
        addr, protocol, timeout, upload_duration, download_duration
    );

    if upload_duration.as_secs() > 0 {
        println!("[Upload  ] ...");
        let mut conn = downup::connect(protocol, addr, timeout).expect("connect");
        if let Err(e) = conn.upload(upload_duration) {
            println!("Error in Upload on {}://{}: {:?}", protocol, addr, e);
        }
    }

    if download_duration.as_secs() > 0 {
        println!("[Download] ...");
        let mut conn = downup::connect(protocol, addr, timeout).expect("connect");
        if let Err(e) = conn.download(download_duration) {
            println!("Error in Download on {}://{}: {:?}", protocol, addr, e);
        }
    }
}

fn load_config() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    let r = std::fs::read_to_string(if args.len() > 1 {
        args[1].as_str()
    } else {
        "/etc/config.toml"
    });

    if r.is_err() {
        return Err(format!("Erro on open config file: {}", r.unwrap_err()));
    }

    match toml::from_str(r.unwrap().as_str()) {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Erro on parsing config file: {}", e)),
    }
}
