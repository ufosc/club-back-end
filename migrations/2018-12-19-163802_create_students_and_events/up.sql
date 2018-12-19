-- Create a member table
CREATE TABLE member (
	ufl_username TEXT PRIMARY KEY,
	is_info_filled_out BOOLEAN DEFAULT FALSE,
	first_name TEXT DEFAULT '',
	last_name TEXT DEFAULT '',
	discord_username TEXT DEFAULT '',
	github_username TEXT DEFAULT '',
	server_username TEXT DEFAULT '',
	server_key TEXT DEFAULT '',
	is_acm_shareable BOOLEAN,
	is_in_email_list BOOLEAN DEFAULT FALSE
)

-- Create an event table
CREATE TABLE event (
	start_timestamp TIMESTAMPTZ PRIMARY KEY, -- Events can not start at the same time and day
	title TEXT NOT NULL, -- Must have title
	location TEXT NOT NULL, -- Must have location
	description TEXT DEFAULT '',
	end_timestamp TIMESTAMPTZ NOT NULL, -- Events must have an end time
	image BYTEA DEFAULT '\000'-- Image binary defaults to 0
)

-- Create a many to many attendance table
CREATE TABLE attendance (
	ufl_username TEXT REFERENCES member,
	start_timestamp TIMESTAMPTZ REFERENCES event,
	PRIMARY KEY (ufl_username, start_timestamp)
)
