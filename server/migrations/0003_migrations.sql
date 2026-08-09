-- defining the permissions node wise
-- CREATE TABLE  node_permissions(
--     id BIGSERIAL PRIMARY KEY,
--     nodes_id BIGINT NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
--     name VARCHAR(300) NOT NULL,
--     des TEXT,
--     UNIQUE(nodes_id, name)
-- );
-- -- assigned nodes to the users
-- CREATE TABLE users_assigned_nodes(
--     id BIGSERIAL PRIMARY KEY,
--     nodes_id BIGINT NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
--     user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
--     UNIQUE(user_id, nodes_id)
-- );

-- -- Giving fine grade permission on the user per node wise
-- CREATE TABLE user_node_level_permissions(
--     assigned_node_id BIGINT NOT NULL REFERENCES users_assigned_nodes(id) ON DELETE CASCADE,
--     permission_id BIGINT NOT NULL REFERENCES node_permissions(id) ON DELETE CASCADE,
--     PRIMARY KEY(assigned_node_id, permission_id)
-- );

-- INSERT INTO users (username, password_hash)
-- SELECT
--     'admin',
--     '$argon2id$v=19$m=19456,t=2,p=1$jMOWOTT5rPXW9SDZtEbT2A$2vvID0W1tKM0GwhN078735EkixUV5EHME1FqpO+b1zA'
-- WHERE NOT EXISTS (
--     SELECT 1 FROM users
-- );
-- not safe password 


-- will remove this migration in next update