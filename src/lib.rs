use sqlx::{MySqlPool, mysql::MySqlPoolOptions, FromRow};

use std::time::Instant;

use serde::{Serialize, Deserialize};

pub async fn connect_db() -> Result<MySqlPool, sqlx::Error> {
    let start = Instant::now();
    println!("connecting to db...");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    println!(".env: {}", database_url);
    println!("Connected");

    let duration = start.elapsed();
    let ms = (duration.as_secs_f64() * 1000.0).ceil() / 1000.0;
    println!("Time elapsed: {:.3} ms\n", ms);

    Ok(pool)
    
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Players {
    id: i64,
    username: String,
    highscore: i64,
    last_game: i64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
