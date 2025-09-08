-- Add migration script here
ALTER TABLE posts
ALTER COLUMN created_at TYPE TIMESTAMPTZ;
