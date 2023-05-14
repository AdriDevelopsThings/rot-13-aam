use std::{env, net::SocketAddr, str::FromStr};

use axum::{extract, routing::get, Router, Server};

mod rot13;
#[cfg(test)]
mod tests;

async fn rot13(extract::Path(message): extract::Path<String>) -> String {
    rot13::rot13(&message)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:message", get(rot13));

    let listen_address =
        env::var("LISTEN_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1:8000"));
    let addr = SocketAddr::from_str(&listen_address).expect("Invalid listen address");
    println!("Server listing on {listen_address}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Error while listening");
}
