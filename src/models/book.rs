use crate::schema;
use crate::schema::books;
use diesel::PgConnection;

use crate::schema::books::dsl::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone, PartialEq)]
#[table_name = "books"]
pub struct Book {
  pub id: i32,
  pub user_id: i32,
  pub author_id: i32,
  pub genre_id: i32,
  pub name: String,
  pub price: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BookWithAuthorName {
  pub book: Book,
  pub author_name: String,
}

impl Book {
  pub fn find_by_id(
    _param_user_id: i32,
    _book_id: &i32,
    connection: &PgConnection,
  ) -> Result<BookWithAuthorName, diesel::result::Error> {
    use crate::schema::authors::dsl::{fio, id as author_ident};

    let book: Book = schema::books::table.find(_book_id).first(connection)?;

    let author_name = schema::authors::table
      .filter(author_ident.eq(book.author_id))
      .select(fio)
      .first::<String>(connection)?;

    let book_with_author_name = BookWithAuthorName { book, author_name };

    Ok(book_with_author_name)
  }

  pub fn delete_by_id(
    book_id: &i32,
    connection: &PgConnection,
  ) -> Result<(), diesel::result::Error> {
    diesel::delete(schema::books::table.find(book_id)).execute(connection)?;
    Ok(())
  }

  pub fn update_by_id(
    _id: &i32,
    connection: &PgConnection,
    new_book: &NewBook,
  ) -> Result<(), diesel::result::Error> {
    diesel::update(schema::books::table.find(_id))
      .set(new_book)
      .execute(connection)?;
    Ok(())
  }
}

#[derive(Insertable, Deserialize, AsChangeset, Clone)]
#[table_name = "books"]
pub struct NewBook {
  pub user_id: Option<i32>,
  pub author_id: Option<i32>,
  pub genre_id: Option<i32>,
  pub name: Option<String>,
  pub price: Option<i32>,
}

impl NewBook {
  pub fn create(
    &self,
    param_user_id: i32,
    connection: &PgConnection,
  ) -> Result<Book, diesel::result::Error> {
    let new_book = NewBook {
      user_id: Some(param_user_id),
      ..self.clone()
    };

    diesel::insert_into(books::table)
      .values(new_book)
      .get_result(connection)
  }
}

#[derive(Serialize, Deserialize)]
pub struct ListOfBooks(pub Vec<Book>);

impl ListOfBooks {
  pub fn get_list(connection: &PgConnection) -> Self {
    let result = books
      .load::<Book>(connection)
      .expect("Error loading products");

    ListOfBooks(result)
  }

  pub fn get_user_books(param_user_id: &i32, connection: &PgConnection) -> Self {
    let result = books
      .filter(user_id.eq(param_user_id))
      .load::<Book>(connection)
      .expect("Error loading products");

    ListOfBooks(result)
  }
}
