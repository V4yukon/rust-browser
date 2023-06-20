use axum::{Router, Server, routing::get, extract::State, response::{IntoResponse, Html}, Json, http::Response};
use sysinfo::{System, SystemExt, CpuExt};
use std::sync::{Arc, Mutex};
#[tokio::main]
async fn main() {
    let router = Router::new()
    .route("/", get(root_get))
    .route("/index.js", get(indexjs_get))
    .route("/api/cpus", get(get_cpus))
    .with_state(AppState {sys: Arc::new(Mutex::new(System::new()))});


    let server = Server::bind(&"0.0.0.0:8032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();

    println!("Listening on {addr}");

    server.await.unwrap();
}

#[derive(Clone)]

struct AppState {
    sys: Arc<Mutex<System>>,
}
#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup)
    
}
async fn indexjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.js").await.unwrap();
    Response::builder()
    .status(200)
    .header("content-type", "application/javascript;charset=utf-8")
    .body(markup)
    .unwrap()
}
#[axum::debug_handler]
async fn get_cpus(State(state): State<AppState>) -> impl IntoResponse {
    
    

    let mut sys = state.sys.lock().unwrap();
    //let mut sys = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything(),));
    sys.refresh_cpu();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(v)

    // for (i, cpu) in sys.cpus().iter().enumerate() {
    //     let i = i + 1;
    //     let usage = cpu.cpu_usage();

    //     writeln!(&mut s, "CPU {i} {usage}%").unwrap();
    // }

    

}
