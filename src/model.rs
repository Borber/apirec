#[derive(sqlx::FromRow)]
pub struct Apps {
    pub name: String,
}

#[derive(sqlx::FromRow)]
pub struct Apis {
    pub name: String,
    pub count: i64,
}
