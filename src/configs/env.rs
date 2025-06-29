use std::env;
use dotenvy::dotenv;
use env_logger::Env;

pub fn init_env() {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("Environment: {}", get_env());
    println!("Host: {}", get_server_host());
    println!("Port: {}", get_server_port());
}

pub fn get_env() -> String {
    env::var("APP_ENV").unwrap_or_else(|_| "development".into())
}

pub fn get_server_host() -> String {
    env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into())
}

pub fn get_server_port() -> u16 {
    env::var("SERVER_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(8080)
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env")
}
