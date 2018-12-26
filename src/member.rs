use diesel::prelude::*;

use super::database;
use super::schema::member;

#[derive(Insertable, Queryable)]
#[table_name = "member"]
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

impl Member {
	fn new(ufl_username: & str) -> Self {
		Member {
			ufl_username: ufl_username.to_string(),
			..Default::default()
		}
	}
}

impl Default for Member {
	fn default() -> Member {
		Member {
			ufl_username: "".to_string(),
			is_info_filled_out: false,
			first_name: "".to_string(),
			last_name: "".to_string(),
			github_username: "".to_string(),
			discord_username: "".to_string(),
			server_username: "".to_string(),
			server_key: "".to_string(),
			is_acm_shareable: false,
			is_in_email_list: false,
		}
	}
}

pub fn list_members() {
	let connection = database::establish_connection();
	let results = member::table
		//.filter(member::columns::is_info_filled_out.eq(true))
		// .limit(5)
		.load::<Member>(&connection)
		.expect("Error loading posts");

	println!("Displaying {} posts", results.len());
	for member in results {
		println!("{}", member.ufl_username);
		println!("----------\n");
		println!("{}", member.is_info_filled_out);
	}
}

pub fn add_member(ufl_username: &str) {
	let connection = database::establish_connection();

	let new_member = Member::new(&ufl_username);

	diesel::insert_into(member::table)
		.values(&new_member)
		.get_result::<Member>(&connection)
		.expect("Error saving new post");
}
