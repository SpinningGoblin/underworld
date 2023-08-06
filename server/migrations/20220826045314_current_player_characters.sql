-- Add migration script here
CREATE TABLE IF NOT EXISTS current_player_characters (
  username TEXT UNIQUE NOT NULL,
  pc_id TEXT NOT NULL
);
