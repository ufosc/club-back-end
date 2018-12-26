-- Remove the member, event, and attendance table
DROP TABLE attendance; -- Remove this first, because it depends on the other tables
DROP TABLE member;
DROP TABLE event;
