DEFINE TABLE queue SCHEMAFULL;
DEFINE FIELD created_at     ON queue TYPE datetime;
DEFINE FIELD updated_at     ON queue TYPE datetime;
DEFINE FIELD scheduled_at   ON queue TYPE datetime;
DEFINE FIELD locked_at      ON queue TYPE option<datetime>;
DEFINE FIELD queue          ON queue TYPE string;
DEFINE FIELD kind           ON queue TYPE string;
DEFINE FIELD max_attempts   ON queue TYPE number;
DEFINE FIELD attempts       ON queue TYPE number;
DEFINE FIELD priority       ON queue TYPE number;
DEFINE FIELD unique_key     ON queue TYPE option<string>;
DEFINE FIELD lease_time     ON queue TYPE number;
DEFINE FIELD payload        ON queue FLEXIBLE TYPE object;
DEFINE FIELD error_reason   ON queue FLEXIBLE TYPE option<object>;

DEFINE INDEX idx_queue_polling ON TABLE queue
FIELDS queue, attempts, scheduled_at, locked_at, priority, updated_at;

DEFINE INDEX idx_queue_unique ON TABLE queue
FIELDS queue, kind, unique_key
UNIQUE;

DEFINE INDEX idx_queue_kind ON TABLE queue
FIELDS queue, kind;

DEFINE INDEX idx_queue_scheduled ON TABLE queue
FIELDS scheduled_at;

DEFINE INDEX idx_queue_locked ON TABLE queue
FIELDS locked_at;

DEFINE INDEX idx_queue_updated ON TABLE queue
FIELDS updated_at;
