use actix_web::HttpResponse;
use chrono::{Duration, Local};
use jwt::{decode, encode, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: i32,
  email: String,
  first_name: String,
  last_name: String,
  exp: usize,
}

// We're using a struct so we can implement a conversion from
// Claims to SlimUser, useful in the decode function.
#[derive(Serialize)]
pub struct SlimUser {
  pub id: i32,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

impl From<Claims> for SlimUser {
  fn from(claims: Claims) -> Self {
    SlimUser {
      id: claims.sub,
      email: claims.email,
      first_name: claims.first_name,
      last_name: claims.last_name,
    }
  }
}

impl Claims {
  fn with_email(id: i32, email: &str, first_name: &str, last_name: &str) -> Self {
    Claims {
      sub: id,
      email: email.into(),
      first_name: first_name.into(),
      last_name: last_name.into(),
      exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
    }
  }
}

pub fn create_token(
  id: i32,
  email: &str,
  first_name: &str,
  last_name: &str,
) -> Result<String, HttpResponse> {
  let claims = Claims::with_email(id, email, first_name, last_name);
  encode(&Header::default(), &claims, get_secret()).map_err(|e| {
    println!("create_token failure");
    HttpResponse::InternalServerError().json(e.to_string())
  })
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
  decode::<Claims>(token, get_secret(), &Validation::default())
    .map(|data| data.claims.into())
    .map_err(|e| {
      println!("decode_token failure");
      HttpResponse::Unauthorized().json(e.to_string())
    })
}

fn get_secret<'a>() -> &'a [u8] {
  dotenv!("JWT_SECRET").as_bytes()
}
