use dotenvy::dotenv;
use sqlx::SqlitePool;
use std::env;
use warp::Filter;

mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_url).await?;

    let pool_filter = warp::any().map(move || pool.clone());

    let index = warp::path::end()
        .and(pool_filter.clone())
        .and_then(|pool| async move { Ok::<_, std::convert::Infallible>(handlers::index_handler(pool).await) });

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8080".into()).parse()?;
    let addr = format!("{}:{}", host, port);

    println!("Server running at http://{}", addr);

    warp::serve(index).run(([127, 0, 0, 1], port)).await;
    Ok(())
}