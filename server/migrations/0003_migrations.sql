-- defining the permissions node wise
CREATE TABLE  node_permissions(
    id BIGSERIAL PRIMARY KEY,
    nodes_id BIGINT NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    name VARCHAR(300) NOT NULL,
    des TEXT,
    UNIQUE(nodes_id, name)
);
-- assigned nodes to the users
CREATE TABLE users_assigned_nodes(
    id BIGSERIAL PRIMARY KEY,
    nodes_id BIGINT NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, nodes_id)
);

-- Giving fine grade permission on the user per node wise
CREATE TABLE user_node_level_permissions(
    assigned_node_id BIGINT NOT NULL REFERENCES users_assigned_nodes(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES node_permissions(id) ON DELETE CASCADE,
    PRIMARY KEY(assigned_node_id, permission_id)
);


