use dotenvy::dotenv;
use sqlx::SqlitePool;
use std::env;
use warp::Filter;
use std::net::SocketAddr;

mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load the database URL from environment
    let db_url = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_url).await?;

    // Shared filter for the database pool
    let pool_filter = warp::any().map(move || pool.clone());

    // Index route handler
    let index = warp::path::end()
        .and(pool_filter.clone())
        .and_then(|pool| async move { Ok::<_, std::convert::Infallible>(handlers::index_handler(pool).await) });

    // Read host and port from environment variables or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "80".into())  // Default to 80 if PORT is not set
        .parse()
        .expect("Failed to parse PORT environment variable");

    // Format the address to bind the server
    let addr = format!("{}:{}", host, port);

    println!("Server running at http://{}", addr);

    // Ensure that addr can be parsed into a SocketAddr
    let socket_addr: SocketAddr = addr
        .parse()
        .map_err(|e| format!("Failed to parse address {}: {}", addr, e))?;

    // Bind to the desired address and port
    warp::serve(index).run(socket_addr).await;

    Ok(())
}
