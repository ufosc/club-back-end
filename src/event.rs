use diesel::prelude::*;
use diesel::sql_types::Bytea;
use diesel::sql_types::Timestamptz;

use super::database;
use super::schema::events;

#[derive(Queryable)]
pub struct Event {
	pub start_timestamp: Timestamptz,
	pub title: String,
	pub location: String,
	pub description: String,
	pub end_timestamp: Timestamptz,
	pub image: Bytea,
}
