use askama::Template;
use sqlx::SqlitePool;
use crate::models::User;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    users: Vec<User>,
}

pub async fn index_handler(pool: SqlitePool) -> impl warp::Reply {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let template = IndexTemplate { users };
    warp::reply::html(template.render().unwrap_or_else(|e| format!("Template error: {}", e)))
}