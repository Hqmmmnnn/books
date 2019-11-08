-- Your SQL goes here
CREATE TABLE authors
(
id SERIAL PRIMARY KEY,
fio VARCHAR NOT NULL,
date_of_birth VARCHAR NOT NULL,
country VARCHAR NOT NULL
);