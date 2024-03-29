use crate::utils::jwt::create_token;
use actix_identity::Identity;
use actix_web::web;
use actix_web::HttpResponse;
use csrf_token::CsrfTokenGenerator;
use hex;

use crate::db_connection::PgPool;
use crate::errors::MyStoreError;
use crate::handlers::pg_pool_handler;
use crate::models::user::AuthUser;

pub fn login(
  auth_user: web::Json<AuthUser>,
  id: Identity,
  pool: web::Data<PgPool>,
  generator: web::Data<CsrfTokenGenerator>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  let user = auth_user.login(&pg_pool).map_err(|e| match e {
    MyStoreError::DBError(diesel::result::Error::NotFound) => {
      HttpResponse::NotFound().json(e.to_string())
    }
    _ => HttpResponse::InternalServerError().json(e.to_string()),
  })?;

  let token = create_token(
    user.id,
    &user.email,
    &user.first_name,
    &user.last_name,
    &user.role,
  )?;

  id.remember(token);

  let response = HttpResponse::Ok()
    .header("X-CSRF-TOKEN", hex::encode(generator.generate()))
    .json(user);
  Ok(response)
}

pub fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
  id.forget();
  Ok(HttpResponse::Ok().into())
}
