-- Create a member table
CREATE TABLE member (
	ufl_username TEXT PRIMARY KEY,
	is_info_filled_out BOOLEAN NOT NULL DEFAULT FALSE,
	first_name TEXT NOT NULL DEFAULT '',
	last_name TEXT NOT NULL DEFAULT '',
	discord_username TEXT NOT NULL DEFAULT '',
	github_username TEXT NOT NULL DEFAULT '',
	server_username TEXT NOT NULL DEFAULT '',
	server_key TEXT NOT NULL DEFAULT '',
	is_acm_shareable BOOLEAN NOT NULL DEFAULT FALSE,
	is_in_email_list BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create an event table
CREATE TABLE event (
	start_timestamp TIMESTAMPTZ PRIMARY KEY, -- Events can not start at the same time and day
	title TEXT NOT NULL, -- Must have title
	location TEXT NOT NULL, -- Must have location
	description TEXT NOT NULL DEFAULT '',
	end_timestamp TIMESTAMPTZ NOT NULL, -- Events must have an end time
	image BYTEA NOT NULL DEFAULT '\000'-- Image binary defaults to 0
);

-- Create a many to many attendance table
CREATE TABLE attendance (
	ufl_username TEXT REFERENCES member,
	start_timestamp TIMESTAMPTZ REFERENCES event,
	PRIMARY KEY (ufl_username, start_timestamp)
);
