-- Add down migration script here
ALTER TABLE point_conditions DROP COLUMN swell_wave_height_unit;
ALTER TABLE point_conditions DROP COLUMN swell_wave_direction_unit;
ALTER TABLE point_conditions DROP COLUMN rain_unit;
ALTER TABLE point_conditions DROP COLUMN temperature_unit;
ALTER TABLE point_conditions DROP COLUMN weather_code_unit;
ALTER TABLE point_conditions DROP COLUMN wind_speed_unit;


ALTER TABLE point_conditions DROP COLUMN rain;
ALTER TABLE point_conditions DROP COLUMN temperature;
ALTER TABLE point_conditions DROP COLUMN weather_code;
ALTER TABLE point_conditions DROP COLUMN wind_speed;

