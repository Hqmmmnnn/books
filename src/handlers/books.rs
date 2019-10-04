use crate::models::book::Book;
use crate::models::book::ListOfBooks;
use crate::models::book::NewBook;

use actix_web::{web, HttpRequest, HttpResponse};

pub fn index(_req: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().json(ListOfBooks::get_list())
}

pub fn create(new_product: web::Json<NewBook>) -> Result<HttpResponse, HttpResponse> {
  new_product
    .create()
    .map(|book| HttpResponse::Ok().json(book))
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn find_by_id(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
  Book::find_by_id(&id)
    .map(|book| HttpResponse::Ok().json(book))
    .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
}
