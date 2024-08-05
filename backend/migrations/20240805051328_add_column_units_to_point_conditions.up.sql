-- Add up migration script here
ALTER TABLE point_conditions ADD COLUMN swell_wave_height_unit VARCHAR(255);
ALTER TABLE point_conditions ADD COLUMN swell_wave_direction_unit VARCHAR(255);

ALTER TABLE point_conditions ADD COLUMN rain REAL DEFAULT 0.0;
ALTER TABLE point_conditions ADD COLUMN temperature REAL DEFAULT 0.0;
ALTER TABLE point_conditions ADD COLUMN weather_code INTEGER DEFAULT 0;
ALTER TABLE point_conditions ADD COLUMN wind_speed REAL DEFAULT 0.0;

ALTER TABLE point_conditions ADD COLUMN rain_unit VARCHAR(255);
ALTER TABLE point_conditions ADD COLUMN temperature_unit VARCHAR(255);
ALTER TABLE point_conditions ADD COLUMN weather_code_unit VARCHAR(255);
ALTER TABLE point_conditions ADD COLUMN wind_speed_unit VARCHAR(255);
