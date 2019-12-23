use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub role: String,
  #[serde(skip)]
  pub password: String,
  pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct ListOfUsers(pub Vec<User>);

impl ListOfUsers {
  pub fn get_all(connection: &PgConnection) -> Self {
    let users = users::table
      .load::<User>(connection)
      .expect("Error loading users");

    ListOfUsers(users)
  }
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub role: String,
  pub password: String,
  pub created_at: NaiveDateTime,
}

use crate::errors::MyStoreError;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use diesel::PgConnection;

impl User {
  pub fn create(
    register_user: RegisterUser,
    connection: &PgConnection,
  ) -> Result<User, MyStoreError> {
    Ok(
      diesel::insert_into(users::table)
        .values(NewUser {
          email: register_user.email,
          first_name: register_user.first_name,
          last_name: register_user.last_name,
          role: String::from("user"),
          password: Self::hash_password(register_user.password)?,
          created_at: Local::now().naive_local(),
        })
        .get_result(connection)?,
    )
  }

  pub fn delete_by_id(
    _user_id: &i32,
    connection: &PgConnection,
  ) -> Result<(), diesel::result::Error> {
    diesel::delete(users::table.find(_user_id)).execute(connection)?;
    Ok(())
  }

  pub fn hash_password(plain: String) -> Result<String, MyStoreError> {
    Ok(hash(plain, DEFAULT_COST)?)
  }
}

#[derive(Deserialize)]
pub struct RegisterUser {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password: String,
  pub password_confirmation: String,
}

impl RegisterUser {
  pub fn validates(self) -> Result<RegisterUser, MyStoreError> {
    if self.password == self.password_confirmation {
      Ok(self)
    } else {
      Err(MyStoreError::PasswordNotMatch(
        "Password and Password Confirmation does not match".to_string(),
      ))
    }
  }
}

#[derive(Deserialize)]
pub struct AuthUser {
  pub email: String,
  pub password: String,
}

impl AuthUser {
  pub fn login(&self, connection: &PgConnection) -> Result<User, MyStoreError> {
    use crate::schema::users::dsl::email;
    use bcrypt::verify;

    let mut records = users::table
      .filter(email.eq(&self.email))
      .load::<User>(connection)?;

    let user = records
      .pop()
      .ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;

    let verify_password = verify(&self.password, &user.password).map_err(|_error| {
      MyStoreError::WrongPassword("Wrong password, check again please".to_string())
    })?;

    if verify_password {
      Ok(user)
    } else {
      Err(MyStoreError::WrongPassword(
        "Wrong password, check again please".to_string(),
      ))
    }
  }
}
