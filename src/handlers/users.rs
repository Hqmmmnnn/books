use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::handlers::LoggedUser;
use crate::models::user::{ListOfUsers, User};
use actix_web::{web, HttpResponse, Result};

pub fn get_all_users(
  _user: LoggedUser,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  if _user.role == String::from("admin") {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(ListOfUsers::get_all(&pg_pool)))
  } else {
    Err(HttpResponse::InternalServerError().json("access denied".to_string()))
  }
}

pub fn delete_user_by_id(
  _user: LoggedUser,
  _id: web::Path<i32>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  if _user.role == String::from("admin") {
    let pg_pool = pg_pool_handler(pool)?;
    User::delete_by_id(&_id, &pg_pool)
      .map(|_| HttpResponse::Ok().json(()))
      .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
  } else {
    Err(HttpResponse::InternalServerError().json("access denied".to_string()))
  }
}
