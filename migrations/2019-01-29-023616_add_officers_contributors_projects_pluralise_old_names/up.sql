-- Change member to members
ALTER TABLE member RENAME TO members;

-- Change event to events
ALTER TABLE event RENAME TO events;

-- Change attendance to attendances
ALTER TABLE attendance RENAME TO attendances;

-- Create an officers table
CREATE TABLE officers (
	ufl_username TEXT REFERENCES members,
	PRIMARY KEY (ufl_username),
	position TEXT
);

-- Create an projects table
CREATE TABLE projects (
	github_url TEXT PRIMARY KEY,
	name TEXT,
	description TEXT,
	technologies TEXT[],
	discord_channel TEXT,
	is_active BOOLEAN,
	next_milestone_date TIMESTAMPTZ,
	image BYTEA
);

-- Create an contributors table
CREATE TABLE contributors (
	ufl_username TEXT REFERENCES members,
	github_url TEXT REFERENCES projects,
	PRIMARY KEY (ufl_username, github_url),
	is_project_lead BOOLEAN
);
