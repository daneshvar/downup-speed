fn main() {
    println!("Starting");

    let protocol = "tcp";
    //let addr = "45.82.137.212:6000";
    let addr = "127.0.0.1:6000";


    if let Err(e) = upload(protocol, addr) {
        println!("Error in Upload on {}://{}: {:?}", protocol, addr, e);
    }
    // let r = client::send().await;
    // match r {
    //     Ok(t) => println!("Sent by Byte: {}", t),
    //     Err(e) => println!("Error: {:?}", e),
    // }
}

fn upload(protocol: &str, addr: &str) -> Result<(), String> {
    client::connect(protocol, addr)?.upload_file("mock/Dariush - Delkhosham.mp3")
}
