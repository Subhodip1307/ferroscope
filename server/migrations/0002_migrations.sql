-- adding email option in user table
ALTER TABLE users
ADD email VARCHAR(255);

ALTER TABLE users
ADD CONSTRAINT unique_user_email UNIQUE (email);


CREATE TABLE rules (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    event_type TEXT NOT NULL,

    condition_json JSONB NOT NULL,     -- logic
    action_json JSONB NOT NULL,        -- what to do

    created_by TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
/*
 -- =========================
-- CPU RULES
-- =========================

INSERT INTO rules (name, is_active, event_type, condition_json, action_json, created_by)
VALUES
(
  'CPU > 80 Alert',
  true,
  'cpu',
  '{"operator": ">", "value": 80}',
  '{"type": "notify", "channel": "email", "to": ["ops@company.com"], "message": "CPU usage high"}',
  'system'
),
(
  'CPU Critical > 90',
  true,
  'cpu',
  '{"operator": ">", "value": 90}',
  '{"type": "notify", "channel": "pagerduty", "message": "CPU CRITICAL"}',
  'system'
);

-- =========================
-- MEMORY RULES
-- =========================

INSERT INTO rules (name, is_active, event_type, condition_json, action_json, created_by)
VALUES
(
  'Memory Warning',
  true,
  'memory',
  '{"and": [{"operator": ">", "value": 70}, {"operator": "<", "value": 90}]}',
  '{"channel": "email", "to": ["ops@company.com"], "message": "Memory warning"}',
  'system'
),
(
  'Memory Critical',
  true,
  'memory',
  '{"operator": ">", "value": 90}',
  '{"channel": "slack", "webhook": "https://hooks.slack.com/xxx", "message": "Memory critical"}',
  'system'
);

-- =========================
-- SERVICE STATUS RULES
-- =========================

INSERT INTO rules (name, is_active, event_type, condition_json, action_json, created_by)
VALUES
(
  'Service Down Alert',
  true,
  'service_status',
  '{"field": "status", "operator": "=", "value": "down"}',
  '{"channel": "email", "to": ["admin@company.com"], "message": "Service is down"}',
  'system'
),
(
  'Auto Restart Nginx',
  true,
  'service_status',
  '{"field": "status", "operator": "=", "value": "down"}',
  '{"type": "restart_service", "service": "nginx"}',
  'system'
);




-- =========================
-- CUSTOM COMPLEX RULE
-- =========================

INSERT INTO rules (name, is_active, event_type, condition_json, action_json, created_by)
VALUES
(
  'CPU + Memory Both High',
  true,
  'cpu',
  '{
    "and": [
      {"operator": ">", "value": 80},
      {"external_check": "memory", "operator": ">", "value": 75}
    ]
  }',
  '{"type": "notify", "channel": "email", "message": "CPU and Memory both high"}',
  'system'
);



*/
