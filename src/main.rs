use env_logger::Env;
use mailrocket::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    run(TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port"))?.await
}
