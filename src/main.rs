#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket_contrib::databases::diesel;
use std::env;

#[database("club_data")]
struct ClubDbConn(diesel::PgConnection);

pub fn establish_connection() -> PgConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

fn main() {
	rocket::ignite()
		.attach(ClubDbConn::fairing())
		.mount("/", routes![index])
		.launch();
}
