use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}