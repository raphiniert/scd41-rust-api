use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    // use environment variables or default fallbacks
    let database_url = env::var("DATABASE_POSTGRES_URL").unwrap_or_else(|_| "postgres://admin:quest@questdb:8812/qdb".to_string());
    let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
    .unwrap_or_else(|_| "5".to_string()) // Default to 5 if not set
    .parse::<u32>()
    .expect("DATABASE_MAX_CONNECTIONS must be a positive integer");
    let pool = PgPoolOptions::new().max_connections(max_connections).connect(&database_url).await?;
    Ok(pool)
}