-- Add migration script here
CREATE TABLE IF NOT EXISTS "todo" (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(255)
)
