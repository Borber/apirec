#[derive(sqlx::FromRow)]
pub struct AppRec {
    pub name: String,
    pub count: i64,
}
