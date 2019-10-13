use crate::db_connection::{PgPool, PgPooledConnection};
use crate::models::book::{Book, ListOfBooks, NewBook};
use actix_web::{web, HttpRequest, HttpResponse};

fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
  pool
    .get()
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn index(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Ok(HttpResponse::Ok().json(ListOfBooks::get_list(&pg_pool)))
}

pub fn create(
  new_product: web::Json<NewBook>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  new_product
    .create(&pg_pool)
    .map(|book| HttpResponse::Ok().json(book))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn find_by_id(
  id: web::Path<i32>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Book::find_by_id(&id, &pg_pool)
    .map(|book| HttpResponse::Ok().json(book))
    .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
}

pub fn delete_by_id(
  id: web::Path<i32>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Book::delete_by_id(&id, &pg_pool)
    .map(|_| HttpResponse::Ok().json(()))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn update_by_id(
  id: web::Path<i32>,
  pool: web::Data<PgPool>,
  new_product: web::Json<NewBook>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Book::update_by_id(&id, &pg_pool, &new_product)
    .map(|_| HttpResponse::Ok().json(()))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
