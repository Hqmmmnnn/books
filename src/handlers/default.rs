use actix_web::{HttpRequest, HttpResponse};

pub fn index(_req: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().body("<h1>Hiii!</h1>")
}
