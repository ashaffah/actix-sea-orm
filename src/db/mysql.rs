use sea_orm::{ Database, DatabaseConnection };
use std::env;

pub async fn init_db() -> DatabaseConnection {
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        env::var("MYSQL_USER").expect("MYSQL_USER must be set"),
        env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set"),
        env::var("MYSQL_HOST").expect("MYSQL_HOST must be set"),
        env::var("MYSQL_PORT").expect("MYSQL_PORT must be set"),
        env::var("MYSQL_DB").expect("MYSQL_DB must be set")
    );

    println!("Connecting to database at: {}", database_url);
    Database::connect(&database_url).await.expect("Failed to connect to database")
}
