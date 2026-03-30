-- adding email option in user table
ALTER TABLE users
ADD email VARCHAR(255);

ALTER TABLE users
ADD CONSTRAINT unique_user_email UNIQUE (email);