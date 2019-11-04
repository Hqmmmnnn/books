-- Your SQL goes here
CREATE TABLE books
(
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  author_id INTEGER NOT NULL REFERENCES authors(id),
  name VARCHAR NOT NULL,
  price INTEGER
);

CREATE INDEX index_user_id ON books(user_id);
CREATE INDEX index_author_id ON books(author_id);