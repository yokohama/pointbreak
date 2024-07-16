-- Add up migration script here
ALTER TABLE users ALTER COLUMN name DROP NOT NULL;
