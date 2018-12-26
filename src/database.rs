extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

#[database("club_data")]
pub struct ClubDbConn(diesel::PgConnection);

pub fn establish_connection() -> PgConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}