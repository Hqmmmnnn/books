use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::handlers::LoggedUser;
use crate::models::author::{Author, NewAuthor};

use actix_web::{web, HttpResponse, Result};

pub fn create(
  _user: LoggedUser,
  new_author: web::Json<NewAuthor>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  new_author
    .create(&pg_pool)
    .map(|author| HttpResponse::Ok().json(author))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn get_author_by_id(
  _user: LoggedUser,
  author_id: web::Path<i32>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Ok(HttpResponse::Ok().json(Author::get_author_by_id(&author_id, &pg_pool)))
}
