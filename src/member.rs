use diesel::prelude::*;

use super::database;
use super::schema::members;

/* Struct Setup */

// Struct for interacting with the member table
#[derive(Insertable, Queryable, Clone)]
#[table_name = "members"]

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

// Support creating new members that supply just a UFL username
impl Member {
	fn new(ufl_username: &str) -> Self {
		Member {
			ufl_username: ufl_username.to_string(),
			..Default::default()
		}
	}
}

// Set default values for Member
// Note: the UFL username should never be not set (Rust requires all values to have a default)
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


/* CRUD and other functions */

// Return all members
pub fn list_members() -> Vec<Member> {
	let connection = database::establish_connection();
	members::table
		.load::<Member>(&connection)
		.expect("Error loading members")
}

// Add a member with a UFL username
pub fn add_member(ufl_username: &str) {
	let connection = database::establish_connection();

	let new_member = Member::new(&ufl_username);

	diesel::insert_into(members::table)
		.values(&new_member)
		.get_result::<Member>(&connection)
		.expect("Error saving new member");
}

// Remove a member by their username
pub fn remove_member(ufl_username: &str) {
	let connection = database::establish_connection();

	let num_deleted =
		diesel::delete(members::table.filter(members::columns::ufl_username.eq(ufl_username))) //.like(ufl_username)))
			.execute(&connection)
			.expect("Error deleting members");

	println!("Deleted {} members", num_deleted);
}

fn replace_member(username: &str, modifier: &Member) {
	let connection = database::establish_connection();

	// TODO: Check to handle errors with result
	let update_result = diesel::update(
		members::table.filter(members::columns::ufl_username.eq(&username)),
	)
	.set(members::columns::first_name.eq(&modifier.first_name))
	.get_result::<Member>(&connection);
}

fn modify_first_name(username: &str, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		first_name: string_replace.to_string(),
		..temp_member.unwrap()[0].clone()
	};

	replace_member(&username, &modifier);
}
/* Unit testing */

// Note: Do run the test as `cargo test -- --test-threads=1` to run the database calls in order
#[cfg(test)]
mod tests {
	use super::*;

	// Utility function to clear out the whole member table
	fn clear_table() {
		let connection = database::establish_connection();

		diesel::delete(members::table)
			.execute(&connection)
			.expect("Error deleting all members");
	}

	// Check to make sure no members exist by default
	#[test]
	fn no_members() {
		clear_table();
		assert_eq!(Vec::len(&list_members()), 0);
	}

	// Check that one member exists after they are created
	#[test]
	fn one_member() {
		clear_table();
		add_member("one_member_test@email.com");
		assert_eq!(Vec::len(&list_members()), 1);
	}

	// Check that two members exist after they are both created
	#[test]
	fn two_member() {
		clear_table();
		add_member("two_member_test_one@email.com");
		add_member("two_member_test_two@email.com");
		assert_eq!(Vec::len(&list_members()), 2);
	}

	// Check that a single member can be deleted after being created
	#[test]
	fn delete_member() {
		clear_table();
		add_member("delete_member_test@email.com");
		remove_member("delete_member_test@email.com");
		assert_eq!(Vec::len(&list_members()), 0);
	}

	#[test]
	fn modify_member() {
		clear_table();
		let connection = database::establish_connection();

		add_member("two_member_test_one@email.com");
		let new_member = Member {
			ufl_username: "two_member_test_one@email.com".to_string(),
			first_name: "test".to_string(),
			..Default::default()
		};

		modify_first_name(&new_member.ufl_username, "changed");

		let result = members::table
			.filter(members::ufl_username.eq("two_member_test_one@email.com"))
			.load::<Member>(&connection);
		assert_eq!("changed", result.unwrap()[0].first_name);
	}


}
