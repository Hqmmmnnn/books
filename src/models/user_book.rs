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

#[derive(Insertable, Deserialize, AsChangeset, Clone)]
#[table_name = "users_books"]
pub struct NewUserBook {
  pub user_id: i32,
  pub book_id: i32,
  pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ListOfUserBook(pub Vec<UserBook>);

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

impl ListOfUserBook {
  pub fn get_all_books(param_user_id: i32, connection: &PgConnection) -> ListOfBooks {
    use crate::schema::books::dsl::{id as identity, user_id as u_id};

    let books = schema::books::table
      .filter(u_id.eq(param_user_id))
      .filter(
        identity.eq_any(
          users_books::table
            .select(book_id)
            .filter(user_id.eq(param_user_id)),
        ),
      )
      .load::<Book>(connection)
      .expect("Error loading users bnooks");

    /*&let result = users_books::table
      .filter(user_id.eq(param_user_id))
      .limit(10)
      .load::<UserBook>(connection)
      .expect("Error loading users books");

    ListOfUserBook(result)*/
    ListOfBooks(books)
  }

  pub fn proverka(param_user_id: i32, connection: &PgConnection) -> Self {
    let result = users_books::table
      .filter(user_id.eq(param_user_id))
      .limit(10)
      .load::<UserBook>(connection)
      .expect("Error loading users books");

    ListOfUserBook(result)
  }
}

impl NewUserBook {
  pub fn take_book(
    &self,
    param_user_id: i32,
    connection: &PgConnection,
  ) -> Result<UserBook, diesel::result::Error> {
    let new_user_book = NewUserBook {
      user_id: param_user_id,
      book_id: self.book_id,
      amount: self.amount,
    };

    diesel::insert_into(users_books::table)
      .values(new_user_book)
      .get_result(connection)
  }
}

/*impl NewUserBook {
  pub fn take_book(&self, param_user_id: i32, connection: &PgConnection) {
    let book = find_book_by_id(self.book_id, param_user_id, connection);
    let new_user_book = NewUserBook {
      user_id: param_user_id,
      book_id: self.book_id,
      amount: 0,
    };
    /*diesel::update(
      users_books::table
        .filter(user_id.eq(param_user_id))
        .find(self.book_id),
    )
    .set(updated_book);*/
    let result = match book {
      Ok(book) => diesel::update(
        users_books::table
          .filter(user_id.eq(param_user_id))
          .find(self.book_id),
      )
      .set(UserBook {
        id: book.id,
        user_id: book.user_id,
        book_id: book.book_id,
        amount: book.amount + 1,
      })
      .execute(connection),
      Err(_) => diesel::insert_into(users_books::table)
        .values(new_user_book)
        .get_result(connection),
    };

    /*diesel::insert_into(users_books::table)
    .values(new_user_book)
    .get_result(connection)*/
  }
}
*/

/*
pub fn take_book(
   _book_id: i32,
   param_user_id: i32,
   connection: &PgConnection,
 ) -> Result<UserBook, diesel::result::Error> {
   let book = UserBook::find_book_by_id(_book_id, param_user_id, connection);
   let new_user_book = NewUserBook {
     user_id: param_user_id,
     book_id: _book_id,
     amount: 1,
   };
   let result = match book {
     Ok(book) => diesel::update(
       users_books::table
         .filter(user_id.eq(param_user_id))
         .find(book_id),
     )
     .set(UserBook {
       id: book.id,
       user_id: book.user_id,
       book_id: book.book_id,
       amount: book.amount + 1,
     })
     .execute(connection),
     Err(_) => take_bookkk(_book_id, param_user_id, connection),
   };
   Result(result)
 }*/
