use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::handlers::LoggedUser;
use crate::models::genre::{ListOfGenres, NewGenre};

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
      .map(|author| HttpResponse::Ok().json(author))
      .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
  } else {
    Err(HttpResponse::InternalServerError().json("access denied".to_string()))
  }
}
