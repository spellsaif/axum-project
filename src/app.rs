
use crate::routes::routes;


pub struct SimpleServer;

impl SimpleServer {
    pub async fn run(){
        let app = routes();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listener, app).await.unwrap();

    }
}
