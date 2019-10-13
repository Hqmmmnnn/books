use crate::schema::books;
use diesel::PgConnection;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Book {
  pub id: i32,
  pub name: String,
  pub author: String,
  pub price: Option<i32>,
}

impl Book {
  pub fn find_by_id(id: &i32, connection: &PgConnection) -> Result<Book, diesel::result::Error> {
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    books::table.find(id).first(connection)
  }

  pub fn delete_by_id(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::books::dsl;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    diesel::delete(dsl::books.find(id)).execute(connection)?;
    Ok(())
  }

  pub fn update_by_id(
    id: &i32,
    connection: &PgConnection,
    new_book: &NewBook,
  ) -> Result<(), diesel::result::Error> {
    use crate::schema::books::dsl;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    diesel::update(dsl::books.find(id))
      .set(new_book)
      .execute(connection)?;
    Ok(())
  }
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "books"]
pub struct NewBook {
  pub name: Option<String>,
  pub author: Option<String>,
  pub price: Option<i32>,
}

impl NewBook {
  pub fn create(&self, connection: &PgConnection) -> Result<Book, diesel::result::Error> {
    use diesel::RunQueryDsl;

    diesel::insert_into(books::table)
      .values(self)
      .get_result(connection)
  }
}

#[derive(Serialize, Deserialize)]
pub struct ListOfBooks(pub Vec<Book>);

impl ListOfBooks {
  pub fn get_list(connection: &PgConnection) -> Self {
    use crate::schema::books::dsl::*;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    let result = books
      .limit(10)
      .load::<Book>(connection)
      .expect("Error loading products");

    ListOfBooks(result)
  }
}
