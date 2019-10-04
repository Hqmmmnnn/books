use crate::schema::books;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Book {
  pub id: i32,
  pub name: String,
  pub author: String,
  pub price: Option<i32>,
}

impl Book {
  pub fn find_by_id(id: &i32) -> Result<Book, diesel::result::Error> {
    use crate::db_connection::establish_connection;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    let connection = establish_connection();

    books::table.find(id).first(&connection)
  }
}

#[derive(Insertable, Deserialize)]
#[table_name = "books"]
pub struct NewBook {
  pub name: Option<String>,
  pub author: Option<String>,
  pub price: Option<i32>,
}

impl NewBook {
  pub fn create(&self) -> Result<Book, diesel::result::Error> {
    use crate::db_connection::establish_connection;
    use diesel::RunQueryDsl;

    let connection = establish_connection();
    diesel::insert_into(books::table)
      .values(self)
      .get_result(&connection)
  }
}

#[derive(Serialize, Deserialize)]
pub struct ListOfBooks(pub Vec<Book>);

impl ListOfBooks {
  pub fn get_list() -> Self {
    use crate::db_connection::establish_connection;
    use crate::schema::books::dsl::*;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    let connection = establish_connection();

    let result = books
      .limit(10)
      .load::<Book>(&connection)
      .expect("Error loading products");

    ListOfBooks(result)
  }
}
