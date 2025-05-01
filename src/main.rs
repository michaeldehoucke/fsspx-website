use axum::Server;
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use sqlx::SqlitePool;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_url).await?;

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8080".into()).parse()?;
    let addr = SocketAddr::new(host.parse()?, port);

    println!("Listening on http://{}", addr);

    let app = handlers::app_router(pool);
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}