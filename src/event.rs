use diesel::prelude::*;
use diesel::sql_types::Bytea;
use diesel::sql_types::Timestamptz;

use super::database;
use super::schema::event;
use chrono::prelude::*;

#[derive(Queryable)]
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
	fn new(title: &str, start_timestamp: DateTime<Utc>, end_timestamp: ) -> Self {
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
			image: Vec[0u8, 1, 2, 3, 4, 5, 6, 7],
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
	let results = event::table
		.load::<Event>(&connection)
		.expect("Error loading events");
	results
}

// Add an event with the minimum required fields
// Necessary: title, start and end timestamps
pub fn add_event(title: &str, start_timestamp: Timestamptz, end_timestamp: Timestamptz){
	let connection = database::establish_connection();

	let new_event = Event::new(&title, start_timestamp, end_timestamp);

	diesel::insert_into(event::table)
		.values(&new_event)
		.get_result::<Event>(&conenction)
		.expect("Error saving new event");
}

pub fn remove_event(title: &str) {
	let connection = database::establish_connection();

	let num_deleted =
		diesel::delete(event::table::filter(event::columns::title.eq(title)))
			.execute(&connection)
			.expect("Error deleting members");

	println! ("Deleted {} events", num_deleted);
}

// TODO: Be able to find modify an event. Find it by the start_timestamp and let them pass in all the values
// Note: Might want to consider a way of specfying only certain values to change. Might need a macro or something

// TODO: Add unit tests
