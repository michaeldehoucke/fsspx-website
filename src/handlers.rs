use axum::{extract::State, response::Html, routing::get, Router};
use askama::Template;
use sqlx::SqlitePool;
use crate::models::User;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    users: Vec<User>,
}

pub fn app_router(db: SqlitePool) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .with_state(db)
}

async fn index_handler(State(db): State<SqlitePool>) -> Html<String> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&db)
        .await
        .unwrap_or_default();

    let body = IndexTemplate { users }
        .render()
        .unwrap_or_else(|e| format!("Template error: {}", e));
    Html(body)
}