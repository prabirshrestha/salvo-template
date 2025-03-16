REMOVE INDEX idx_queue_polling ON TABLE queue;
REMOVE INDEX idx_queue_unique ON TABLE queue;
REMOVE INDEX idx_queue_kind ON TABLE queue;
REMOVE INDEX idx_queue_scheduled ON TABLE queue;
REMOVE INDEX idx_queue_locked ON TABLE queue;
REMOVE INDEX idx_queue_updated ON TABLE queue;

REMOVE TABLE queue;
