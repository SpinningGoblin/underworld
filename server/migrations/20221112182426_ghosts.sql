-- Add migration script here
CREATE TABLE IF NOT EXISTS ghosts (
  id serial PRIMARY KEY,
  ghost JSONB NOT NULL
);
