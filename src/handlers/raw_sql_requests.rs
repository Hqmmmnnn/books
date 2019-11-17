/*use crate::db_connection::PgPool;
use crate::diesel::RunQueryDsl;
use crate::handlers::pg_pool_handler;
use actix_web::{web, HttpResponse, Result};
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::PgConnection;

#[derive(Queryable, Serialize, Deserialize, Clone, PartialEq, QueryableByName)]
pub struct RawSql {
  #[sql_type = "Text"]
  pub response: String,
}

impl RawSql {
  pub fn raw_sql(request: &String, connection: &PgConnection) -> Self {
    let response = sql_query("SELECT * FROM ?")
      .bind::<Text, _>(request)
      .get_result::<RawSql>(connection)
      .expect("error raw sql request");

    response
  }
}

pub fn raw_sql_execute(
  _raw_sql: web::Json<RawSql>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  Ok(HttpResponse::Ok().json(RawSql::raw_sql(&_raw_sql.response, &pg_pool)))
}

      .service(
                web::resource("/rawSql")
                    .route(web::get().to(handlers::raw_sql_requests::raw_sql_execute)),
            )
*/
