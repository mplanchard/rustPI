-- Your SQL goes here

CREATE TABLE IF NOT EXISTS packages (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    location TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_name_version ON packages (name, version);
