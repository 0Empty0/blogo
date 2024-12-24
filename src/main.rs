mod handlers;
mod models;
mod routes;

use ntex::web::{App, HttpServer};
use std::io::Result;

#[ntex::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().configure(routes::configure_routes))
        .bind(("localhost", 8080))?
        .run()
        .await
}
