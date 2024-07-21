-- Add up migration script here
ALTER TABLE point_conditions ADD COLUMN time VARCHAR(255); 
ALTER TABLE point_conditions ADD COLUMN swell_wave_height REAL DEFAULT 0.0;
ALTER TABLE point_conditions ADD COLUMN swell_wave_direction INTEGER DEFAULT 0;
