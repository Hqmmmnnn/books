use crate::schema;
use crate::schema::genres;

use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone, QueryableByName)]
#[table_name = "genres"]
pub struct Genre {
  pub id: i32,
  pub name: String,
}

impl Genre {
  pub fn find_by_id(genre_id: &i32, connection: &PgConnection) -> Self {
    let result: Genre = schema::genres::table
      .find(genre_id)
      .first(connection)
      .expect("Error loading genre");

    result
  }

  pub fn delete_by_id(
    genre_id: &i32,
    connection: &PgConnection,
  ) -> Result<(), diesel::result::Error> {
    diesel::delete(schema::genres::table.find(genre_id)).execute(connection)?;
    Ok(())
  }

  pub fn update_by_id(
    genre_id: &i32,
    connection: &PgConnection,
    new_genre: &NewGenre,
  ) -> Result<(), diesel::result::Error> {
    diesel::update(schema::genres::table.find(genre_id))
      .set(new_genre)
      .execute(connection)?;
    Ok(())
  }
}

#[derive(Serialize, Deserialize)]
pub struct ListOfGenres(pub Vec<Genre>);

impl ListOfGenres {
  pub fn get_all(connection: &PgConnection) -> Self {
    let genres = schema::genres::table
      .load::<Genre>(connection)
      .expect("Error loading genres");

    ListOfGenres(genres)
  }
}

#[derive(AsChangeset, Deserialize, Insertable, Clone, QueryableByName)]
#[table_name = "genres"]
pub struct NewGenre {
  pub name: String,
}

impl NewGenre {
  pub fn create(&self, connection: &PgConnection) -> Result<Genre, diesel::result::Error> {
    Ok(
      diesel::insert_into(genres::table)
        .values(NewGenre { ..self.clone() })
        .get_result(connection)?,
    )
  }
}
