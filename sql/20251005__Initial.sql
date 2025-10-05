CREATE SCHEMA dwelling;

CREATE TABLE dwelling.accounts (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE dwelling.threads (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    hru TEXT NOT NULL 
);

CREATE INDEX thread_hru_idx ON dwelling.threads(hru);

CREATE TABLE dwelling.posts (
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL REFERENCES dwelling.accounts(id),
    thread_id UUID NOT NULL REFERENCES dwelling.threads(id),
    created TIMESTAMP NOT NULL,
    body TEXT NOT NULL 
);