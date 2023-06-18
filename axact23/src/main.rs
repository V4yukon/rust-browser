use axum::{Router, handler::HandlerWithoutStateExt, ServiceExt, Server, routing::get};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root_get));

    let server = Server::bind(&"0.0.0.0:8032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    print!("Listening on {addr}");

    server.await.unwrap();

    println!("Hello, world!")
}
async fn root_get() ->&'static str {
    "Hi from Axum!"
}
