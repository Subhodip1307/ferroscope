-- droping old migrations
DROP TABLE IF EXISTS node_permissions,users_assigned_nodes,user_node_level_permissions;
-- 1. Which nodes a user can access
CREATE TABLE user_node_access (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL
        REFERENCES users(id) ON DELETE CASCADE,

    node_id BIGINT NOT NULL
        REFERENCES nodes(id) ON DELETE CASCADE,

    -- TRUE = everything on the node
    -- FALSE = use user_node_metric_access
    is_full_access BOOLEAN NOT NULL DEFAULT FALSE,

   UNIQUE (user_id, node_id)
);

-- 2. Fine-grained metric access
CREATE TABLE user_node_metric_access (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL
        REFERENCES users(id) ON DELETE CASCADE,

    node_id BIGINT NOT NULL
        REFERENCES nodes(id) ON DELETE CASCADE,

    -- metric means RAM,which service
    metric_name TEXT NOT NULL,


   UNIQUE (user_id, node_id, metric_name)
);

-- 3. Fine-grained service access
CREATE TABLE user_node_service_access (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL
        REFERENCES users(id) ON DELETE CASCADE,

      -- service_monitor contains the node relationship
    service_id BIGINT NOT NULL 
        REFERENCES service_monitor(id) ON DELETE CASCADE,


   UNIQUE (user_id,service_id)
);
-- adding is null to the users table
ALTER TABLE users ADD is_admin bool  NOT NULL DEFAULT false;
