use std::net::TcpListener;
use zero2prod::run;
use zero2prod::configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuraiton = configuration::get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port 8080");
    let port = listener.local_addr().unwrap().port();
    run(listener)?.await
}
