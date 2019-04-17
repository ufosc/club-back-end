use diesel::prelude::*;

use super::database;
use super::schema::members;

/* Struct Setup */

// Struct for interacting with the member table
#[derive(Insertable, Queryable, Clone, AsChangeset)]
#[table_name = "members"]
#[primary_key(ufl_username)]

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
	let results = members::table
		.load::<Member>(&connection)
		.expect("Error loading members");
	results
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

/* Other Function */
pub fn does_member_exist(ufl_username: &str) -> bool {
	let connection = database::establish_connection();

	let result = member::table
		.filter(member::ufl_username.eq(&ufl_username))
		.load::<Member>(&connection);
	match result {
		Ok(v) => {
			for member in v {
				if member.ufl_username.eq(&ufl_username) {
					return true;
				}
			}
			false
		}
		Err(_e) => false,
	}
	// result.contains(&ufl_username)
}

// TODO: Be able to find modify a member. Find it by the ufl_username and let them pass in all the member values
// Note: Might want to consider a way of specifying only certain values to change. Might need a macro or something
// Replace old member with a modified member
fn replace_member(username: &str, modifier: &Member) {
	let connection = database::establish_connection();

	// TODO: Check to handle errors with result
	let update_result = diesel::update(
		members::table
	)
	.set(modifier)

	.get_result::<Member>(&connection);
}

// Modify a single member's first name
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

// Modify a single member's last name
fn modify_last_name(username: &str, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		last_name: string_replace.to_string(),
		..temp_member.unwrap()[0].clone()
	};

	replace_member(&username, &modifier);
}

// Modify a single member's UFL username
fn modify_ufl_username(username: &str, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username));

	let update_result = diesel::update(temp_member)
		.set(members::ufl_username.eq(string_replace))
		.get_result::<Member>(&connection);

}

// Modify a single member's Discord username
fn modify_discord_username(username: &str, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		discord_username: string_replace.to_string(),
		..temp_member.unwrap()[0].clone()
	};

	replace_member(&username, &modifier);
}

// Modify a single member's Github username
fn modify_github_username(username: &str, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		github_username: string_replace.to_string(),
		..temp_member.unwrap()[0].clone()
	};

	replace_member(&username, &modifier);
}

// Modify a single member's server username
fn modify_server_username(username: &str, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		server_username: string_replace.to_string(),
		..temp_member.unwrap()[0].clone()
	};

	replace_member(&username, &modifier);
}

// Modify a single member's server key
fn modify_server_key(username: &str, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		server_key: string_replace.to_string(),
		..temp_member.unwrap()[0].clone()
	};

	replace_member(&username, &modifier);
}

// Modify a single member's information status
fn modify_infofilledout(username: &str, bool_replace: &bool) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		is_info_filled_out: bool_replace.clone(),
		..temp_member.unwrap()[0].clone()
	};

	replace_member(&username, &modifier);
}

// Modify a single member's ACM shareability status
fn modify_acmshareable(username: &str, bool_replace: &bool) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		is_acm_shareable: bool_replace.clone(),
		..temp_member.unwrap()[0].clone()
	};

	replace_member(&username, &modifier);
}

// Modify a single member's email list status
fn modify_inemaillist(username: &str, bool_replace: &bool) {
	let connection = database::establish_connection();

	let temp_member = members::table
		.filter(members::ufl_username.eq(&username))
		.load::<Member>(&connection);

	let modifier = Member {
		is_in_email_list: bool_replace.clone(),
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
	// Check to see if a member exits after being added
	fn member_exist() {
		clear_table();
		add_member("member_exist_test@email.com");
		assert_eq!(does_member_exist("member_exist_test@email.com"), true);
	}

	#[test]
	// Check to see if a member exists without being added
	fn member_does_not_exist() {
		clear_table();
		add_member("member_does_not_exist_test@email.com");
		assert_eq!(does_member_exist("member_exist_test@email.com"), false);
	}

	// Check that a single member's first name can be modified
	#[test]
	fn modify_memberFirstName(){
	clear_table();
		let connection = database::establish_connection();

        add_member("modify_memberFirstName@email.com");


        modify_first_name("modify_memberFirstName@email.com", "changed");

	let result = members::table
		.filter(members::ufl_username.eq("modify_memberFirstName@email.com"))
		.load::<Member>(&connection);
		assert_eq!("changed", result.unwrap()[0].first_name);
	}

	// Check that a single member's last name can be modified
	#[test]
	fn modify_memberLastName(){
	clear_table();
		let connection = database::establish_connection();

        add_member("modify_memberLastName@email.com");
        modify_last_name("modify_memberLastName@email.com", "changed");

	let result = members::table
		.filter(members::ufl_username.eq("modify_memberLastName@email.com"))
		.load::<Member>(&connection);
       assert_eq!("changed", result.unwrap()[0].last_name);

	}

	// Check that a single member's UFL username can be modified
	#[test]
	fn modify_memberUserName(){
	clear_table();
		let connection = database::establish_connection();

        add_member("two_member_test_one@email.com");
        let new_member = Member {
			ufl_username:"two_member_test_one@email.com".to_string(),
			            ..Default::default()
		};

        modify_ufl_username(&new_member.ufl_username, "changed");

	let result = members::table
		.filter(members::ufl_username.eq("changed"))
		.load::<Member>(&connection);
       assert_eq!("changed", result.unwrap()[0].ufl_username);

	}

	// Check that a single member's Discord username can be modified
	#[test]
	fn modify_discordUserName(){
	clear_table();
		let connection = database::establish_connection();

        add_member("two_member_test_one@email.com");
        let new_member = Member {
			ufl_username:"two_member_test_one@email.com".to_string(),
			discord_username: "test".to_string(),
			            ..Default::default()
		};

        modify_discord_username(&new_member.ufl_username, "changed");

	let result = members::table
		.filter(members::ufl_username.eq("two_member_test_one@email.com"))
		.load::<Member>(&connection);
       assert_eq!("changed", result.unwrap()[0].discord_username);

	}

	// Check that a single member's Github username can be modified
	#[test]
	fn modify_githubUserName(){
	clear_table();
		let connection = database::establish_connection();

        add_member("two_member_test_one@email.com");
        let new_member = Member {
			ufl_username:"two_member_test_one@email.com".to_string(),
			github_username: "test".to_string(),
			            ..Default::default()
		};

        modify_github_username(&new_member.ufl_username, "changed");

	let result = members::table
		.filter(members::ufl_username.eq("two_member_test_one@email.com"))
		.load::<Member>(&connection);
       assert_eq!("changed", result.unwrap()[0].github_username);

	}

	// Check that a single member's server username can be modified
    #[test]
	fn modify_serverUserName(){
	clear_table();
		let connection = database::establish_connection();

        add_member("two_member_test_one@email.com");
        let new_member = Member {
			ufl_username:"two_member_test_one@email.com".to_string(),
		    server_username: "test".to_string(),
			            ..Default::default()
		};

        modify_server_username(&new_member.ufl_username, "changed");

	let result = members::table
		.filter(members::ufl_username.eq("two_member_test_one@email.com"))
		.load::<Member>(&connection);
       assert_eq!("changed", result.unwrap()[0].server_username);

	}

	// Check that a single member's server key can be modified
	#[test]
	fn modify_serverKey(){
	clear_table();
		let connection = database::establish_connection();

        add_member("two_member_test_one@email.com");
        let new_member = Member {
			ufl_username:"two_member_test_one@email.com".to_string(),
		    server_key: "test".to_string(),
			            ..Default::default()
		};

        modify_server_key(&new_member.ufl_username, "changed");

	let result = members::table
		.filter(members::ufl_username.eq("two_member_test_one@email.com"))
		.load::<Member>(&connection);
       assert_eq!("changed", result.unwrap()[0].server_key);

	}

	// Check that a member's information status can be modified
	#[test]
	fn modify_info(){
	clear_table();
		let connection = database::establish_connection();

        add_member("two_member_test_one@email.com");
        let new_member = Member {
			ufl_username:"two_member_test_one@email.com".to_string(),
		    is_info_filled_out: false,
			            ..Default::default()
		};

        modify_infofilledout(&new_member.ufl_username, &true);

	let result = members::table
		.filter(members::ufl_username.eq("two_member_test_one@email.com"))
		.load::<Member>(&connection);
       assert_eq!(true, result.unwrap()[0].is_info_filled_out);

	}

	// Check that a member's ACM shareability status can be modified
	#[test]
	fn modify_acm(){
	clear_table();
		let connection = database::establish_connection();

        add_member("two_member_test_one@email.com");
        let new_member = Member {
			ufl_username:"two_member_test_one@email.com".to_string(),
		    is_acm_shareable: false,
			            ..Default::default()
		};

        modify_acmshareable(&new_member.ufl_username, &true);

	let result = members::table
		.filter(members::ufl_username.eq("two_member_test_one@email.com"))
		.load::<Member>(&connection);
       assert_eq!(true, result.unwrap()[0].is_acm_shareable);

	}

	// Check that a member's email list status can be modified
	#[test]
	fn modify_inemail(){
	clear_table();
		let connection = database::establish_connection();

        add_member("two_member_test_one@email.com");
        let new_member = Member {
			ufl_username:"two_member_test_one@email.com".to_string(),
		    is_in_email_list: false,
			            ..Default::default()
		};

        modify_inemaillist(&new_member.ufl_username, &true);

	let result = members::table
		.filter(members::ufl_username.eq("two_member_test_one@email.com"))
		.load::<Member>(&connection);
       assert_eq!(true, result.unwrap()[0].is_in_email_list);

	}
}
