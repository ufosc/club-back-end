#![feature(proc_macro_hygiene, decl_macro)]
// Temporarily silence warnings caused by Diesel (https://github.com/diesel-rs/diesel/issues/1785)
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod database;
mod schema;

mod attendance;
mod event;
mod member;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

fn main() {
	rocket::ignite()
		.attach(database::ClubDbConn::fairing())
		.mount("/", routes![index])
		.launch();
}
