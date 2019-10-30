-- Your SQL goes here
CREATE TABLE books
(
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  name VARCHAR NOT NULL,
  author VARCHAR NOT NULL,
  price INTEGER
);

CREATE INDEX index_user_id ON books(user_id);