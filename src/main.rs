use mailrocket::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port"))?.await
}
