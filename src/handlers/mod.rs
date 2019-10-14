pub mod books;
pub mod default;
pub mod register;

use crate::db_connection::{PgPool, PgPooledConnection};
use actix_web::web;
use actix_web::HttpResponse;

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
  pool
    .get()
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
