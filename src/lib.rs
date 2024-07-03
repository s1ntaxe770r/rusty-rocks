pub mod models;
pub mod schema;

use std::env;
use diesel::{
    pg::PgConnection, Connection, RunQueryDsl, SelectableHelper,
};
use dotenvy::dotenv;
use models::{NewRock, Rock};
use schema::rocks;

pub fn establish_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").map_err(|_| diesel::result::ConnectionError::BadConnection("DATABASE_URL not set".into()))?;
    PgConnection::establish(&db_url)
}

pub fn insert_rock(
    conn: &mut PgConnection,
    name: &str,
    kind: &str,
) -> Result<Rock, diesel::result::Error> {
    let new_rock = NewRock { name, kind };
    diesel::insert_into(rocks::table)
        .values(new_rock)
        .returning(Rock::as_returning())
        .get_result(conn)
}

pub fn get_rocks(conn: &mut PgConnection) -> Result<Vec<Rock>, diesel::result::Error> {
    use schema::rocks::dsl::*;
    rocks.load::<Rock>(conn)
}