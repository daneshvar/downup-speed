#[tokio::main]
async fn main() {
    println!("Starting Server");
    server::listen().await;
}
