use pkg::app;

#[tokio::main]
async fn main() {
    app::SimpleServer::run().await
}
