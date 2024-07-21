use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod error_handler;
mod posts;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(posts::init_routes));

    server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        let host = env::var("HOST").expect("Please set HOST in .env");
        let port = env::var("PORT").expect("Please set PORT in .env");
        server.bind(format!("{}:{}", host, port))?
    };

    server.run().await
}
