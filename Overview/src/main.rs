use axum::{
    routing::get,
    Router,
    response::Html,
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct ListItem {
    category: String,
    sub_category: String,
    title: String,
    link: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Html(include_str!("static/index.html")) }))
        .route("/list", get(get_list));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // Change the port as needed

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_list() -> Html<String> {
    // Simulated data - Replace this with your actual data retrieval logic
    let data = vec![
        ListItem {
            category: "Category 1".to_string(),
            sub_category: "Subcategory A".to_string(),
            title: "Title 1".to_string(),
            link: "https://example.com/link1".to_string(),
        },
        ListItem {
            category: "Category 2".to_string(),
            sub_category: "Subcategory B".to_string(),
            title: "Title 2".to_string(),
            link: "https://example.com/link2".to_string(),
        },
        // Add more items as needed
    ];

    Html(serde_json::to_string(&data).unwrap())
}
