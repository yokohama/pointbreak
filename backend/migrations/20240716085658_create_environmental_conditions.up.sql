-- Add up migration script here
CREATE TABLE point_conditions (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  lat FLOAT NOT NULL,
  lon FLOAT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_lat_lon ON point_conditions (lat, lon);
