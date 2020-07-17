#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

// Necessary imports here 
use actix_web::{App, HttpServer};

// importing student module(s)
mod students;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initializing server
    HttpServer::new(|| {
        App::new()
        // Associating service(s)/route_handler(s)
         .configure(students::init_routes)
    })
    // Binding socket address server will receive requests on
    .bind("127.0.0.1:5000")?
    .run()
    .await
}