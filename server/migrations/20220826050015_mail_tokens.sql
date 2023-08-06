-- Add migration script here
CREATE TABLE IF NOT EXISTS mail_tokens (
  id serial PRIMARY KEY,
  email text NOT NULL,
  token text NOT NULL,
  created_at timestamptz,
  deleted_after timestamptz
);
