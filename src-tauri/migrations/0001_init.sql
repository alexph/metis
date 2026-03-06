CREATE TABLE IF NOT EXISTS metadata (
    id TEXT PRIMARY KEY,
    instance_id TEXT NOT NULL UNIQUE,
    schema_version INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS channels (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_ref TEXT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_channels_created_at ON channels(created_at);
CREATE INDEX IF NOT EXISTS idx_channels_status ON channels(status);

CREATE TABLE IF NOT EXISTS branches (
    id TEXT PRIMARY KEY,
    channel_id TEXT NOT NULL,
    parent_branch_id TEXT NULL,
    name TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE RESTRICT,
    FOREIGN KEY (parent_branch_id) REFERENCES branches(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_branches_channel_id ON branches(channel_id);
CREATE INDEX IF NOT EXISTS idx_branches_parent_branch_id ON branches(parent_branch_id);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    channel_id TEXT NOT NULL,
    branch_id TEXT NULL,
    kind TEXT NOT NULL,
    state TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    payload_json TEXT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    started_at TEXT NULL,
    finished_at TEXT NULL,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE RESTRICT,
    FOREIGN KEY (branch_id) REFERENCES branches(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_tasks_channel_id ON tasks(channel_id);
CREATE INDEX IF NOT EXISTS idx_tasks_branch_id ON tasks(branch_id);
CREATE INDEX IF NOT EXISTS idx_tasks_state ON tasks(state);
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);

CREATE TABLE IF NOT EXISTS workers (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    worker_type TEXT NOT NULL,
    state TEXT NOT NULL,
    attempt INTEGER NOT NULL DEFAULT 0,
    last_heartbeat_at TEXT NULL,
    started_at TEXT NULL,
    finished_at TEXT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_workers_task_id ON workers(task_id);
CREATE INDEX IF NOT EXISTS idx_workers_state ON workers(state);
CREATE INDEX IF NOT EXISTS idx_workers_last_heartbeat_at ON workers(last_heartbeat_at);

CREATE TABLE IF NOT EXISTS history (
    id TEXT PRIMARY KEY,
    channel_id TEXT NOT NULL,
    branch_id TEXT NULL,
    task_id TEXT NULL,
    worker_id TEXT NULL,
    event_type TEXT NOT NULL,
    role TEXT NULL,
    content_json TEXT NOT NULL,
    correlation_id TEXT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE RESTRICT,
    FOREIGN KEY (branch_id) REFERENCES branches(id) ON DELETE RESTRICT,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE RESTRICT,
    FOREIGN KEY (worker_id) REFERENCES workers(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_history_channel_created ON history(channel_id, created_at);
CREATE INDEX IF NOT EXISTS idx_history_branch_created ON history(branch_id, created_at);
CREATE INDEX IF NOT EXISTS idx_history_task_created ON history(task_id, created_at);
CREATE INDEX IF NOT EXISTS idx_history_worker_created ON history(worker_id, created_at);
CREATE INDEX IF NOT EXISTS idx_history_correlation_id ON history(correlation_id);
