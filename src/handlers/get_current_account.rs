use crate::handlers::LoggedUser;
use actix_web::HttpResponse;

pub fn get_current_account(logged_user: LoggedUser) -> HttpResponse {
  HttpResponse::Ok().json(logged_user)
}
