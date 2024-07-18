-- Automatic updatedAt timestamp.
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updatedAt = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(24) NOT NULL,
  password VARCHAR(72) NOT NULL,
  createdAt TIMESTAMP NOT NULL DEFAULT NOW(),
  updatedAt TIMESTAMP NOT NULL DEFAULT NOW()
);
-- Enable auto updatedAt.
CREATE TRIGGER set_updated_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE keys (
  id VARCHAR(128) PRIMARY KEY,
  ownerId INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  tags INTEGER NOT NULL,
  createdAt TIMESTAMP NOT NULL DEFAULT NOW(),
  expiresAt TIMESTAMP
);

CREATE TABLE resources (
  id SERIAL PRIMARY KEY,
  kind TEXT NOT NULL,
  data json,
  createdAt TIMESTAMP NOT NULL DEFAULT NOW(),
  updatedAt TIMESTAMP NOT NULL DEFAULT NOW()
);
-- Enable auto updatedAt.
CREATE TRIGGER set_updated_timestamp
BEFORE UPDATE ON resources
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE permissions (
  id SERIAL PRIMARY KEY,
  keyId VARCHAR(128) NOT NULL REFERENCES keys(id) ON DELETE CASCADE,
  resourceId INTEGER NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
  permissions BIGINT NOT NULL,
  createdAt TIMESTAMP NOT NULL DEFAULT NOW(),
  updatedAt TIMESTAMP NOT NULL DEFAULT NOW()
);
-- Enable auto updatedAt.
CREATE TRIGGER set_updated_timestamp
BEFORE UPDATE ON permissions
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
