-- Your SQL goes here
CREATE TABLE carte (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    description VARCHAR(256) NOT NULL,
    category VARCHAR(32) NOT NULL,
    price VARCHAR(32) NOT NULL
)