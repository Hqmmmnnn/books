-- Your SQL goes here
CREATE TABLE users_books
(
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  book_id INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
  amount INTEGER NOT NULL DEFAULT 0,
  UNIQUE(user_id, book_id)
)