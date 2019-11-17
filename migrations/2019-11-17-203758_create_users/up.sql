-- Your SQL goes here
CREATE TABLE users
(
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL UNIQUE,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  role VARCHAR(16) NOT NULL,
  password VARCHAR(64) NOT NULL,
  created_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX users_email ON users (email);