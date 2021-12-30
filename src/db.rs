use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn get_pool() -> anyhow::Result<PgPool, anyhow::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .min_connections(1)
        .max_lifetime(std::time::Duration::from_secs(10))
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    let url = &env::var("DATABASE_URL")?;
    let mut split = url.split("@");
    let mut creds = split.next().unwrap().split("://");
    creds.next();
    let username = creds.next().unwrap().split(":").next().unwrap();
    let cleaned_url = split.next().unwrap().split("/").next().unwrap();
    println!("Connected to the database at {} as {}", &cleaned_url, username);
    Ok(pool)
}