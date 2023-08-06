-- Add migration script here
CREATE TABLE IF NOT EXISTS player_characters (
  id serial PRIMARY KEY,
  username TEXT NOT NULL,
  pc_id TEXT UNIQUE NOT NULL,
  pc JSONB NOT NULL
);
