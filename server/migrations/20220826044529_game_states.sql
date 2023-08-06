-- Add migration script here
CREATE TABLE IF NOT EXISTS game_states (
  username TEXT NOT NULL,
  game_state_id TEXT UNIQUE NOT NULL,
  id serial PRIMARY KEY,
  game_state JSONB NOT NULL
);
