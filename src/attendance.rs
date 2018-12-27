use diesel::prelude::*;
use diesel::sql_types::Timestamptz;

use super::database;
use super::schema::attendance;

#[derive(Queryable)]
pub struct Attendance {
	pub ufl_username: String,
	pub start_timestamp: Timestamptz,
}
