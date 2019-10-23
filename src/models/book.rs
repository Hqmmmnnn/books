use crate::schema::books;
use diesel::PgConnection;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[table_name = "books"]
pub struct Book {
  pub id: i32,
  pub user_id: i32,
  pub name: String,
  pub author: String,
  pub price: Option<i32>,
}

impl Book {
  pub fn find_by_id(
    param_user_id: i32,
    _id: &i32,
    connection: &PgConnection,
  ) -> Result<Book, diesel::result::Error> {
    use crate::schema;
    use crate::schema::books::dsl::*;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    let product: Book = schema::books::table
      .filter(user_id.eq(param_user_id))
      .find(_id)
      .first(connection)?;

    Ok(product)
  }

  pub fn delete_by_id(
    param_user_id: i32,
    _id: &i32,
    connection: &PgConnection,
  ) -> Result<(), diesel::result::Error> {
    use crate::schema;
    use crate::schema::books::dsl::*;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    diesel::delete(
      schema::books::table
        .filter(user_id.eq(param_user_id))
        .find(_id),
    )
    .execute(connection)?;
    Ok(())
  }

  pub fn update_by_id(
    param_user_id: i32,
    _id: &i32,
    connection: &PgConnection,
    new_book: &NewBook,
  ) -> Result<(), diesel::result::Error> {
    use crate::schema;
    use crate::schema::books::dsl::*;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    diesel::update(
      schema::books::table
        .filter(user_id.eq(param_user_id))
        .find(_id),
    )
    .set(new_book)
    .execute(connection)?;
    Ok(())
  }
}

#[derive(Insertable, Deserialize, AsChangeset, Clone)]
#[table_name = "books"]
pub struct NewBook {
  pub user_id: Option<i32>,
  pub name: Option<String>,
  pub author: Option<String>,
  pub price: Option<i32>,
}

impl NewBook {
  pub fn create(
    &self,
    param_user_id: i32,
    connection: &PgConnection,
  ) -> Result<Book, diesel::result::Error> {
    use diesel::RunQueryDsl;

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
  pub fn get_list(param_user_id: i32, connection: &PgConnection) -> Self {
    use crate::schema::books::dsl::*;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    let result = books
      .filter(user_id.eq(param_user_id))
      .limit(10)
      .load::<Book>(connection)
      .expect("Error loading products");

    ListOfBooks(result)
  }
}
