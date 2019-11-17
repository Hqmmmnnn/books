use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::handlers::LoggedUser;
use crate::models::author::{Author, ListOfAuthors, NewAuthor};

use actix_web::{web, HttpResponse, Result};

pub fn create(
  _user: LoggedUser,
  new_author: web::Json<NewAuthor>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  if _user.role == String::from("admin") {
    let pg_pool = pg_pool_handler(pool)?;
    new_author
      .create(&pg_pool)
      .map(|author| HttpResponse::Ok().json(author))
      .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
  } else {
    Err(HttpResponse::InternalServerError().json("access denied".to_string()))
  }
}

pub fn get_by_id(
  _user: LoggedUser,
  author_id: web::Path<i32>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Ok(HttpResponse::Ok().json(Author::get_by_id(&author_id, &pg_pool)))
}

pub fn delete_by_id(
  _user: LoggedUser,
  author_id: web::Path<i32>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  if _user.role == String::from("admin") {
    let pg_pool = pg_pool_handler(pool)?;
    Author::delete_by_id(&author_id, &pg_pool)
      .map(|_| HttpResponse::Ok().json(()))
      .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
  } else {
    Err(HttpResponse::InternalServerError().json("access denied".to_string()))
  }
}

pub fn get_all(_user: LoggedUser, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Ok(HttpResponse::Ok().json(ListOfAuthors::get_all(&pg_pool)))
}
