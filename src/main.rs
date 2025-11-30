mod hscraper;
mod server;

use server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1", 3000);

    if let Err(e) = server.start().await {
        eprintln!("Server error: {}", e);
    }
}
