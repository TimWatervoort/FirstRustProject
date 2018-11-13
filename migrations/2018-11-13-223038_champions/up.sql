-- Your SQL goes here
CREATE TABLE champions (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  role VARCHAR NOT NULL,
  comfort VARCHAR NOT NULL
);
