#![feature(proc_macro_hygiene, decl_macro)]
// Temporarily silence warnings caused by Diesel (https://github.com/diesel-rs/diesel/issues/1785)
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

// Utility local dependencies
mod database;
mod schema;

// Table specifc local dependencies
mod attendance;
mod event;
mod member;

// Check to see if the server is working
#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

// Launch the REST server with the database connection
fn main() {
	rocket::ignite()
		.attach(database::ClubDbConn::fairing())
		// Note: Be sure to mount all the routes from differnt modules
		.mount("/", routes![index])
		.launch();
}
