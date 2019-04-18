use diesel::prelude::*;
use diesel::sql_types::Bytea;
use diesel::sql_types::Timestamptz;

use super::database;
use super::schema::events;
use chrono::prelude::*;

#[derive(Insertable, Queryable, Clone, AsChangeset)]
#[table_name = "events"]
#[primary_key(start_timestamp)]

pub struct Event {
	pub start_timestamp: DateTime<Utc>,
	pub title: String,
	pub location: String,
	pub description: String,
	pub end_timestamp: DateTime<Utc>,
	pub image: Vec<u8>,
}

// Support for creating events with only title, start, and end timestamps
impl Event{
	fn new(title: &str, start_timestamp: DateTime<Utc>, end_timestamp: DateTime<Utc>) -> Self {
		Event {
			start_timestamp: start_timestamp,
			title: title.to_string(),
			end_timestamp: end_timestamp,
			..Default::default()
		}
	}
}

impl Default for Event {
	fn default() -> Event {
		Event {
			start_timestamp: Utc::now(),
			end_timestamp: Utc::now(),
			image: Vec::new(),
			description: "".to_string(),
			location: "".to_string(),
			title: "".to_string(),
		}
	}
}

// CRUD functions

// Return a list of all events
pub fn list_events() -> Vec<Event> {
	let connection = database::establish_connection();
	let results = events::table
		.load::<Event>(&connection)
		.expect("Error loading events");
	results
}

// Add an event with the minimum required fields
// Necessary: title, start and end timestamps
pub fn add_event(title: &str, start_timestamp: DateTime<Utc>, end_timestamp: DateTime<Utc>){
	let connection = database::establish_connection();

	let new_event = Event::new(&title, start_timestamp, end_timestamp);

	diesel::insert_into(events::table)
		.values(&new_event)
		.get_result::<Event>(&connection)
		.expect("Error saving new event");
}

pub fn remove_event(title: &str) {
	let connection = database::establish_connection();

	let num_deleted =
		diesel::delete(events::table.filter(events::columns::title.eq(title)))
			.execute(&connection)
			.expect("Error deleting members");

	println! ("Deleted {} events", num_deleted);
}

// TODO: Be able to find modify an event. Find it by the start_timestamp and let them pass in all the values
// Note: Might want to consider a way of specfying only certain values to change. Might need a macro or something
fn replace_event(time: &DateTime<Utc>, modifier: &Event) {
	let connection = database::establish_connection();

	// TODO: Check to handle errors with result (such as user not existing)
	let update_result = diesel::update(events::table)
		.set(modifier)
		.get_result::<Event>(&connection);
}

fn modify_location(time: &DateTime<Utc>, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_event = events::table
		.filter(events::start_timestamp.eq(&time))
		.load::<Event>(&connection);

	let modifier = Event {
		location: string_replace.to_string(),
		..temp_event.unwrap()[0].clone()
	};

	replace_event(&time, &modifier);
}

fn modify_description(time: &DateTime<Utc>, string_replace: &str) {
	let connection = database::establish_connection();

let temp_event = events::table
		.filter(events::start_timestamp.eq(&time))
		.load::<Event>(&connection);

	let modifier = Event {
		description: string_replace.to_string(),
		..temp_event.unwrap()[0].clone()
	};

	replace_event(&time, &modifier);
}

fn modify_title(time: &DateTime<Utc>, string_replace: &str) {
	let connection = database::establish_connection();

	let temp_event = events::table
		.filter(events::start_timestamp.eq(&time))
		.load::<Event>(&connection);

	let modifier = Event {
		title: string_replace.to_string(),
		..temp_event.unwrap()[0].clone()
	};

	replace_event(&time, &modifier);
}


fn modify_start_timestamp(time: &DateTime<Utc>, replace: &DateTime<Utc>) {
	let connection = database::establish_connection();

	let temp_event = events::table
		.filter(events::start_timestamp.eq(&time));

	replace.clone();

		let update_start_timestamp = diesel::update(temp_event)
		.set(events::start_timestamp.eq(*replace))
		.get_result::<Event>(&connection);

}

fn modify_end_timestamp(time: &DateTime<Utc>, replace: &DateTime<Utc>) {
	let connection = database::establish_connection();

	let temp_event = events::table
		.filter(events::start_timestamp.eq(&time))
		.load::<Event>(&connection);

	replace.clone();

	let modifier = Event {
		end_timestamp: *replace,
		..temp_event.unwrap()[0].clone()
	};

	replace_event(&time, &modifier);
}

fn modify_image(time: &DateTime<Utc>, replace: &Vec<u8>) {
	let connection = database::establish_connection();

	let temp_event = events::table
		.filter(events::start_timestamp.eq(&time))
		.load::<Event>(&connection);

   replace.clone();

	let modifier = Event {
		image: replace.to_vec(),
		..temp_event.unwrap()[0].clone()
	};

	replace_event(&time, &modifier);
}

#[cfg(test)]
mod tests {
	use super::*;

		fn clear_table() {
		let connection = database::establish_connection();

		diesel::delete(events::table)
			.execute(&connection)
			.expect("Error deleting all events");
	}

#[test]
	fn modify_event_location() {
		clear_table();
		let connection = database::establish_connection();

		let time1 = Utc::now().round_subsecs(2);

		add_event("sampleTitle", time1, time1);


		modify_location(&time1, "library");

		let result = events::table
			.filter(events::start_timestamp.eq(time1))
			.load::<Event>(&connection);
		assert_eq!("library", result.unwrap()[0].location);
	}

	#[test]
	fn modify_event_description() {
		clear_table();
		let connection = database::establish_connection();

		let time1 = Utc::now();

		add_event("sampleTitle", time1, time1);


		modify_description(&time1, "library");

		let result = events::table
			.filter(events::start_timestamp.eq(time1))
			.load::<Event>(&connection);
		assert_eq!("library", result.unwrap()[0].description);
	}

	#[test]
	fn modify_event_startTimestamp() {
		clear_table();
		let connection = database::establish_connection();

		let time1 = Utc::now().round_subsecs(2);

		add_event("sampleTitle", time1, time1);


		modify_start_timestamp(&time1, &time1);

		let result = events::table
			.filter(events::start_timestamp.eq(time1))
			.load::<Event>(&connection);
		assert_eq!(time1, result.unwrap()[0].start_timestamp);
	}

	#[test]
	fn modify_event_endTimestamp() {
		clear_table();
		let connection = database::establish_connection();

		let time1 = Utc::now().round_subsecs(2);


		add_event("sampleTitle", time1, time1);



		modify_end_timestamp(&time1, &time1);

		let result = events::table
			.filter(events::start_timestamp.eq(time1))
			.load::<Event>(&connection);
		assert_eq!(time1, result.unwrap()[0].end_timestamp);
	}

	#[test]
	fn modify_event_image() {
		clear_table();
		let connection = database::establish_connection();


		let time1 = Utc::now();

		add_event("sampleTitle", time1, time1);

		let mut image1 =  vec![0; 24];
		modify_image(&time1, &image1);

		let result = events::table
			.filter(events::start_timestamp.eq(time1))
			.load::<Event>(&connection);
		assert_eq!(image1, result.unwrap()[0].image);
	}
}
// TODO: Be able to find modify an event. Find it by the start_timestamp and let them pass in all the values
// Note: Might want to consider a way of specfying only certain values to change. Might need a macro or something

// TODO: Add unit tests
