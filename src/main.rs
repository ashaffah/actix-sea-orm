use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    apps::run().await
}
