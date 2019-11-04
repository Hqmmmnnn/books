use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::handlers::LoggedUser;
use crate::models::book::{Book, ListOfBooks, NewBook};

use actix_web::{web, HttpResponse, Result};

pub fn get_all_books(pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Ok(HttpResponse::Ok().json(ListOfBooks::get_list(1, &pg_pool)))
}

pub fn get_books_by_id(
  _user: LoggedUser,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Ok(HttpResponse::Ok().json(ListOfBooks::get_list(_user.id, &pg_pool)))
}

pub fn create(
  _user: LoggedUser,
  new_product: web::Json<NewBook>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  new_product
    .create(_user.id, &pg_pool)
    .map(|book| HttpResponse::Ok().json(book))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn find_by_id(
  _user: LoggedUser,
  _book_id: web::Path<i32>,

  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Book::find_by_id(_user.id, &_book_id, &pg_pool)
    .map(|book_with_author| HttpResponse::Ok().json(book_with_author))
    .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
}

pub fn delete_by_id(
  _user: LoggedUser,
  id: web::Path<i32>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Book::delete_by_id(_user.id, &id, &pg_pool)
    .map(|_| HttpResponse::Ok().json(()))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn update_by_id(
  _user: LoggedUser,
  id: web::Path<i32>,
  pool: web::Data<PgPool>,
  new_product: web::Json<NewBook>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Book::update_by_id(_user.id, &id, &pg_pool, &new_product)
    .map(|_| HttpResponse::Ok().json(()))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
