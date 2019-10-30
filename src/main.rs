pub mod db_connection;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate actix;
extern crate actix_web;
extern crate bcrypt;
extern crate csrf_token;
extern crate jsonwebtoken as jwt;

#[macro_use]
extern crate dotenv_codegen;

extern crate actix_http;
extern crate env_logger;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use chrono::Duration;
use csrf_token::CsrfTokenGenerator;
use db_connection::establish_connection;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix::System::new("books_api");
    let uri = "127.0.0.1:8001";

    let csrf_token_header = header::HeaderName::from_lowercase(b"x-csrf-token").unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                    .domain(dotenv!("MYSTOREDOMAIN"))
                    .name("mystorejwt")
                    .path("/")
                    .max_age(Duration::days(1).num_seconds())
                    .secure(dotenv!("COOKIE_SECURE").parse().unwrap()),
            ))
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::CONTENT_TYPE,
                        header::ACCEPT,
                        csrf_token_header.clone(),
                    ])
                    .expose_headers(vec![csrf_token_header.clone()])
                    .max_age(3600),
            )
            .data(CsrfTokenGenerator::new(
                dotenv!("CSRF_TOKEN_KEY").as_bytes().to_vec(),
                Duration::hours(2),
            ))
            .data(establish_connection())
            .service(web::resource("/").route(web::get().to(handlers::default::index)))
            .service(web::resource("/register").route(web::post().to(handlers::register::register)))
            .service(
                web::resource("/auth")
                    .route(web::post().to(handlers::authentication::login))
                    .route(web::delete().to(handlers::authentication::logout)),
            )
            .service(
                web::resource("/books")
                    .route(web::get().to(handlers::books::index))
                    .route(web::post().to(handlers::books::create)),
            )
            .service(
                web::resource("/allBooks").route(web::get().to(handlers::books::get_all_books)),
            )
            .service(
                web::resource("/books/{id}")
                    .route(web::get().to(handlers::books::find_by_id))
                    .route(web::delete().to(handlers::books::delete_by_id))
                    .route(web::patch().to(handlers::books::update_by_id)),
            )
            .service(
                web::resource("/getCurrentAccount")
                    .route(web::get().to(handlers::get_current_account::get_current_account)),
            )
    })
    .bind(uri)
    .unwrap()
    .start();

    println!("Started http server: {}", uri);
    let _ = sys.run();
}
