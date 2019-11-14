pub mod authentication;
pub mod authors;
pub mod books;
pub mod default;
pub mod get_current_account;
pub mod register;
pub mod users;

use crate::db_connection::{PgPool, PgPooledConnection};
use actix_web::web;
use actix_web::HttpResponse;

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
  pool
    .get()
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

use crate::utils::jwt::{decode_token, SlimUser};
use actix_identity::Identity;
use actix_web::{dev, FromRequest, HttpRequest};
pub type LoggedUser = SlimUser;

use csrf_token::CsrfTokenGenerator;
use hex;

impl FromRequest for LoggedUser {
  type Error = HttpResponse;
  type Config = ();
  type Future = Result<Self, HttpResponse>;

  fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
    let generator = req.app_data::<CsrfTokenGenerator>().ok_or({
      println!("req.app_data failure");
      HttpResponse::InternalServerError()
    })?;

    let csrf_token = req.headers().get("x-csrf-token").ok_or({
      println!("get x-src-token failure");
      HttpResponse::Unauthorized()
    })?;

    let decoded_token = hex::decode(&csrf_token).map_err(|error| {
      println!("decoded_token failure");
      HttpResponse::InternalServerError().json(error.to_string())
    })?;

    generator.verify(&decoded_token).map_err(|_| {
      println!("verify decoded_token failure");
      HttpResponse::Unauthorized()
    })?;

    if let Some(identity) = Identity::from_request(req, payload)?.identity() {
      let user: SlimUser = decode_token(&identity)?;
      return Ok(user as LoggedUser);
    }
    Err({
      println!("some(itentity) failure");
      HttpResponse::Unauthorized().into()
    })
  }
}
