extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

// club_data data is defined in Rocket.toml
#[database("club_data")]
pub struct ClubDbConn(diesel::PgConnection);

// Create connection for other functions to use
pub fn establish_connection() -> PgConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
