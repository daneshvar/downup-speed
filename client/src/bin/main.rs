extern crate parse_duration;

use serde_derive::Deserialize;
use std::time;
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
    let config;
    match load_config() {
        Ok(c) => config = c,
        Err(e) => {
            println!("Error on load config: {}", e);
            return;
        }
    }

    let protocol = config.server.protocol.as_str();
    let addr = config.server.address.as_str();
    let timeout = parse_duration::parse(config.server.timeout.as_str())
        .expect("Error on Parse config server timeout");
    let upload_duration = parse_duration::parse(config.test.upload_time.as_str())
        .expect("Error on Parse config upload time");
    let download_duration = parse_duration::parse(config.test.download_time.as_str())
        .expect("Error on Parse config upload time");

    println!(
        "{}Server  : {}\nProtocol: {}\nTimeout : {}\n",
        client::CLEAR_SCREEN,
        addr,
        protocol,
        config.server.timeout.as_str()
    );

    if upload_duration.as_secs() > 0 {
        println!("Starting upload test ...");
        if let Err(e) = upload(protocol, addr, timeout, upload_duration) {
            println!("Error in Upload on {}://{}: {:?}", protocol, addr, e);
        }
    }

    if download_duration.as_secs() > 0 {
        println!("Starting download test ...");
        if let Err(e) = download(protocol, addr, timeout, download_duration) {
            println!("Error in Download on {}://{}: {:?}", protocol, addr, e);
        }
    }
}

fn upload(
    protocol: &str,
    addr: &str,
    timeout: time::Duration,
    duration: time::Duration,
) -> Result<(), String> {
    client::connect(protocol, addr, timeout)?.upload(duration)
}

fn download(
    protocol: &str,
    addr: &str,
    timeout: time::Duration,
    duration: time::Duration,
) -> Result<(), String> {
    client::connect(protocol, addr, timeout)?.download(duration)
}

fn load_config() -> Result<Config, String> {
    let r = std::fs::read_to_string("/etc/config.toml");
    if r.is_err() {
        return Err(format!("Erro on open config file: {}", r.unwrap_err()));
    }

    match toml::from_str(r.unwrap().as_str()) {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Erro on parsing config file: {}", e)),
    }
}
