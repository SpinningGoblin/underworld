-- Add migration script here
CREATE TABLE IF NOT EXISTS api_tokens (
  id serial PRIMARY KEY,
  email text NOT NULL,
  token text NOT NULL,
  deleted_after timestamptz
);
