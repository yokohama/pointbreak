-- Add down migration script here
ALTER TABLE point_conditions DROP COLUMN time;
ALTER TABLE point_conditions DROP COLUMN swell_wave_height;
ALTER TABLE point_conditions DROP COLUMN swell_wave_direction;
