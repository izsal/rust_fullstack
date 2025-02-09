use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::response::Redirect;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
// Import ini penting untuk `.into_service()`

#[tokio::main]
async fn main() {
    // old Router
    // Buat router dengan satu route "/"
    // let app = Router::new().route("/", get(handler));

    // Router hanya dengan ServeDir
    let app = Router::new().nest_service("/", ServeDir::new("static"));

    // Tentukan alamat server (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server berjalan di http://{}", addr);

    // Jalankan server menggunakan tokio::net::TcpListener
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app) // Menggunakan `.into_service()` bukan `.into_make_service()`
        .await
        .unwrap();
}

// Handler untuk route "/" from old Route
// async fn handler() -> Html<&'static str> {
//     Html("<b>Hello</b>, world! This is Isal speaking")
// }
