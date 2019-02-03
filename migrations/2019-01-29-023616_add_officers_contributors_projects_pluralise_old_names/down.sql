-- Remove the member, event, and attendance table
DROP TABLE contributors; -- Remove this first, because it depends on the other tables
DROP TABLE officers; -- Remove this next, because it depends on the other tables
DROP TABLE projects;

-- Rename tables back to member, event, and attendance
ALTER TABLE members RENAME TO member;

ALTER TABLE events RENAME TO event;

ALTER TABLE attendances RENAME TO attendance;
