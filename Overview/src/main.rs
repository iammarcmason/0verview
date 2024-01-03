use axum::{response::Html, routing::get, Router};
use serde::Serialize;
use sqlx::{sqlite::SqlitePool, Row};

use std::{net::SocketAddr, path::Path};

#[derive(Serialize)]
struct ListItem {
    category: String,
    sub_category: String,
    title: String,
    link: String,
}

async fn connect(filename: impl AsRef<Path>) -> Result<SqlitePool, sqlx::Error> {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    sqlx::sqlite::SqlitePool::connect_with(options).await
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut db_path = std::env::current_exe()?;

    db_path.pop();
    db_path.push("data");

    std::fs::create_dir_all(&db_path)?;

    db_path.push("my_database.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    // Create or open the database file
    let pool = connect(db_path).await?;

    // Create the table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS list_items (
                id INTEGER PRIMARY KEY,
                category TEXT,
                sub_category TEXT,
                title TEXT,
                link TEXT
            )",
    )
    .execute(&pool)
    .await?;

    // Insert test data
    insert_test_data(&pool).await?;

    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path_str)).await?;

    let app = Router::new()
        .route(
            "/",
            get(|| async { Html(include_str!("static/index.html")) }),
        )
        .route(
            "/list",
            get(move || {
                let pool = pool.clone();
                async move {
                    let result = get_list(&pool).await;
                    match result {
                        Ok(data) => Html(serde_json::to_string(&data).unwrap()),
                        Err(_) => Html("Error retrieving data".to_string()), // Customize error response
                    }
                }
            }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // Change the port as needed

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_list(pool: &SqlitePool) -> Result<Vec<ListItem>, sqlx::Error> {
    let mut items = vec![];

    let mut conn = pool.acquire().await?;
    let rows = sqlx::query("SELECT category, sub_category, title, link FROM list_items")
        .fetch_all(&mut *conn)
        .await?;

    for row in rows {
        items.push(ListItem {
            category: row.get(0),
            sub_category: row.get(1),
            title: row.get(2),
            link: row.get(3),
        });
    }

    Ok(items)
}

async fn insert_test_data(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for i in 1..=5 {
        let category = format!("Category {}", i);
        let sub_category = format!("Subcategory {}", i);
        let title = format!("Title {}", i);
        let link = format!("https://example.com/link{}", i);

        sqlx::query(
            "INSERT INTO list_items (category, sub_category, title, link) VALUES (?, ?, ?, ?)",
        )
        .bind(&category)
        .bind(&sub_category)
        .bind(&title)
        .bind(&link)
        .execute(pool)
        .await?;
    }

    Ok(())
}
