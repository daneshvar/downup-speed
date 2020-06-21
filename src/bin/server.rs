use downup::server;
use serde_derive::Deserialize;
use std::env;
use toml;

extern crate parse_duration;

#[derive(Debug, Deserialize)]
struct Config {
    server: Server,
}

#[derive(Debug, Deserialize)]
struct Server {
    protocol: String,
    ip: String,
    port: u16,
    timeout: String,
}

fn main() {
    let config = load_config().expect("Error on load config");

    let protocol = config.server.protocol.as_str();
    let addr = format!("{}:{}", config.server.ip, config.server.port);
    let timeout = parse_duration::parse(config.server.timeout.as_str())
        .expect("Error on Parse config server timeout");

    println!(
        "Listen  : {}\nProtocol: {}\nTimeout : {:?}\n",
        addr, protocol, timeout
    );

    server::listen(protocol, addr.as_str(), timeout).expect("Listen");
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
