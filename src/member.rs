use diesel::prelude::*;

use super::database;
use super::schema::member;

#[derive(Queryable)]
pub struct Member {
	pub ufl_username: String,
	pub is_info_filled_out: bool,
	pub first_name: String,
	pub last_name: String,
	pub discord_username: String,
	pub github_username: String,
	pub server_username: String,
	pub server_key: String,
	pub is_acm_shareable: bool,
	pub is_in_email_list: bool,
}

pub fn test() {
	let connection = database::establish_connection();
	let results = member::table
		.filter(member::columns::is_info_filled_out.eq(true))
		.limit(5)
		.load::<Member>(&connection)
		.expect("Error loading posts");

	println!("Displaying {} posts", results.len());
	for member in results {
		println!("{}", member.ufl_username);
		println!("----------\n");
		println!("{}", member.is_info_filled_out);
	}
}
