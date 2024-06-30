-- Your SQL goes here
CREATE TABLE devices (
    id SERIAL PRIMARY KEY,
    device_name VARCHAR NOT NULL,
    status VARCHAR NOT NULL
);
