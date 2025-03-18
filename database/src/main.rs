use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to your local database
    let pool: Pool<Postgres> = sqlx::PgPool::connect(&database_url).await?;

    // Your code to interact with the database here.
    println!("Connected to the local database successfully!");

    Ok(())
}
