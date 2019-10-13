pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{web, App, HttpServer};
use db_connection::establish_connection;

fn main() {
    let sys = actix::System::new("books_api");
    let uri = "127.0.0.1:8001";

    HttpServer::new(|| {
        App::new()
            .data(establish_connection())
            .service(web::resource("/").route(web::get().to(handlers::default::index)))
            .service(
                web::resource("/books")
                    .route(web::get().to(handlers::books::index))
                    .route(web::post().to(handlers::books::create)),
            )
            .service(
                web::resource("/books/{id}")
                    .route(web::get().to(handlers::books::find_by_id))
                    .route(web::delete().to(handlers::books::delete_by_id))
                    .route(web::patch().to(handlers::books::update_by_id)),
            )
    })
    .bind(uri)
    .unwrap()
    .start();

    println!("Started http server: {}", uri);
    let _ = sys.run();
}
