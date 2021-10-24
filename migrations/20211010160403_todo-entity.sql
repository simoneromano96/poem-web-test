-- Add migration script here
CREATE TABLE IF NOT EXISTS "todo" (
  id VARCHAR(21) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(255)
)
