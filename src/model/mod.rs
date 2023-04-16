pub mod dto;
pub mod vo;

#[derive(sqlx::FromRow)]
pub struct App {
    pub app: String,
}

#[derive(sqlx::FromRow)]
pub struct Api {
    pub api: String,
    pub count: i64,
}
