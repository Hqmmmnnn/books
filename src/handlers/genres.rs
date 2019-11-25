use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::handlers::LoggedUser;
use crate::models::genre::{Genre, ListOfGenres, NewGenre};

use actix_web::{web, HttpResponse, Result};

pub fn get_all(_user: LoggedUser, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Ok(HttpResponse::Ok().json(ListOfGenres::get_all(&pg_pool)))
}

pub fn create(
  _user: LoggedUser,
  new_genre: web::Json<NewGenre>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  if _user.role == String::from("admin") {
    let pg_pool = pg_pool_handler(pool)?;
    new_genre
      .create(&pg_pool)
      .map(|genre| HttpResponse::Ok().json(genre))
      .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
  } else {
    Err(HttpResponse::InternalServerError().json("access denied".to_string()))
  }
}

pub fn delete_by_id(
  _user: LoggedUser,
  genre_id: web::Path<i32>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  if _user.role == String::from("admin") {
    let pg_pool = pg_pool_handler(pool)?;
    Genre::delete_by_id(&genre_id, &pg_pool)
      .map(|_| HttpResponse::Ok().json(()))
      .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
  } else {
    Err(HttpResponse::InternalServerError().json("access denied".to_string()))
  }
}

pub fn update_by_id(
  _user: LoggedUser,
  genre_id: web::Path<i32>,
  pool: web::Data<PgPool>,
  new_genre: web::Json<NewGenre>,
) -> Result<HttpResponse, HttpResponse> {
  if _user.role == String::from("admin") {
    let pg_pool = pg_pool_handler(pool)?;
    Genre::update_by_id(&genre_id, &pg_pool, &new_genre)
      .map(|_| HttpResponse::Ok().json(()))
      .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
  } else {
    Err(HttpResponse::InternalServerError().json("access denied".to_string()))
  }
}
