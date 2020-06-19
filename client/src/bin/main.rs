#[tokio::main]
async fn main() {
    println!("Starting");

    let r = client::send().await;
    match r {
        Ok(t) => println!("Sent by Byte: {}", t),
        Err(e) => println!("Error: {:?}", e),
    }
}
