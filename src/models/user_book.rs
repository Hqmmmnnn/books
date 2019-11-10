use crate::models::book::{Book, ListOfBooks};
use crate::schema;
use crate::schema::users_books;
use crate::schema::users_books::dsl::*;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Identifiable, Queryable, Serialize, Deserialize, AsChangeset, Clone, PartialEq)]
#[table_name = "users_books"]
pub struct UserBook {
  pub id: i32,
  pub user_id: i32,
  pub book_id: i32,
  pub amount: i32,
}

impl UserBook {
  pub fn find_book_by_id(
    _book_id: i32,
    param_user_id: i32,
    connection: &PgConnection,
  ) -> Result<UserBook, diesel::result::Error> {
    users_books::table
      .filter(user_id.eq(param_user_id))
      .find(_book_id)
      .first(connection)
  }
}

#[derive(Serialize, Deserialize)]
pub struct ListOfUserBook(pub Vec<UserBook>);

impl ListOfUserBook {
  pub fn get_all_books(param_user_id: i32, connection: &PgConnection) -> ListOfBooks {
    use crate::schema::books::dsl::id as identity;

    let books = schema::books::table
      .filter(
        identity.eq_any(
          users_books::table
            .select(book_id)
            .filter(user_id.eq(param_user_id)),
        ),
      )
      .load::<Book>(connection)
      .expect("Error loading users books");

    ListOfBooks(books)
  }
}

#[derive(Insertable, Deserialize, AsChangeset, Clone)]
#[table_name = "users_books"]
pub struct NewUserBook {
  pub user_id: Option<i32>,
  pub book_id: Option<i32>,
  pub amount: Option<i32>,
}

impl NewUserBook {
  pub fn take_book(
    &self,
    param_user_id: i32,
    connection: &PgConnection,
  ) -> Result<UserBook, diesel::result::Error> {
    let new_user_book = NewUserBook {
      user_id: Some(param_user_id),
      ..self.clone()
    };

    diesel::insert_into(users_books::table)
      .values(new_user_book)
      .get_result(connection)
  }
}
