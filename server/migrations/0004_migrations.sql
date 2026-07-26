-- adding is null to the users table
ALTER TABLE users ADD is_admin bool  NOT NULL DEFAULT false;
