use diesel::prelude::*;
use diesel::sql_types::Timestamptz;

use super::database;
use super::schema::attendance;

#[derive(Insertable, Queryable)]
#[table_name = "attendance"]
pub struct Attendance {
	pub ufl_username: String,
	pub start_timestamp: Timestamptz,
}

// TODO: One sign in function that will check to see if an event is occuring and add them to the table
/*
Logic for sign in:
1. The backend gets the email and does two checks.

    a. It will check to see if the student exists in the system or not. If they don't exist it will create the student with the UFL email and a flag (is_info_filled_out) set so we know to ask for more questions.

    b. It will see if there is an event going on at that moment, if there has been one recently (let's say the past hour), or will be one (let's say in the next 30 minutes).

		c. If there is an event, the backend will check to see if the student has already signed in for it.

    d. Finally, if there is an event and the student has not already signed in, an entry with both the student and event are added into the attendance table. If there isn't an event, it will return a response saying so and not record anything in the attendance table. If the student is new, it will send a response prompting for more information (and will always ask for more information if the is_info_filled_out flag is till up). If they have already signed in, return a response saying so.
*/

// TODO: Remove faulty sign-ins

// TODO: Unit tests
