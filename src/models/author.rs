use crate::errors::MyStoreError;
use crate::schema::authors;

use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone, PartialEq, QueryableByName)]
#[table_name = "authors"]
pub struct Author {
  pub id: i32,
  pub fio: String,
  pub date_of_birth: String,
  pub country: String,
}

impl Author {
  pub fn get_by_id(author_id: &i32, connection: &PgConnection) -> Self {
    let result: Author = authors::table
      .find(author_id)
      .first(connection)
      .expect("Error loading author");
    result
  }

  pub fn delete_by_id(
    author_id: &i32,
    connection: &PgConnection,
  ) -> Result<(), diesel::result::Error> {
    diesel::delete(authors::table.find(author_id)).execute(connection)?;
    Ok(())
  }

  pub fn update_by_id(
    author_id: &i32,
    connection: &PgConnection,
    new_author: &NewAuthor,
  ) -> Result<(), diesel::result::Error> {
    diesel::update(authors::table.find(author_id))
      .set(new_author)
      .execute(connection)?;
    Ok(())
  }
}

#[derive(Serialize, Deserialize)]
pub struct ListOfAuthors(pub Vec<Author>);

impl ListOfAuthors {
  pub fn get_all(connection: &PgConnection) -> Self {
    let authors = authors::table
      .load::<Author>(connection)
      .expect("Error loading authors");

    ListOfAuthors(authors)
  }
}

#[derive(AsChangeset, Insertable, Deserialize, Clone, QueryableByName)]
#[table_name = "authors"]
pub struct NewAuthor {
  pub fio: String,
  pub date_of_birth: String,
  pub country: String,
}

impl NewAuthor {
  pub fn create(&self, connection: &PgConnection) -> Result<Author, MyStoreError> {
    Ok(
      diesel::insert_into(authors::table)
        .values(NewAuthor { ..self.clone() })
        .get_result(connection)?,
    )
  }
}
