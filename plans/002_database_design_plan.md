# Metis Database Design Plan (Phase 1)

Audience: coding agent implementing storage scaffolding only.

## Goal

Define a local-first database design that supports deterministic runtime reconstruction, clean module boundaries, and future headless/desktop adapters without requiring schema restructuring.

## Scope (Phase 1)

Create schema and migration scaffolding for these entities only:

- `metadata`
- `channels`
- `branches`
- `tasks`
- `workers`
- `history`

This plan is for structure and contracts only. Do not implement production workflows, query optimization tuning, or advanced retention policies yet.

## Storage location and backend strategy

- Base directory: `~/.metis/`
- Primary DB: SQLite file at `~/.metis/metis.db`
- Future optional backend: Turso-compatible adapter behind the same storage interfaces
- Logs/config (non-DB): `~/.metis/logs/`, `~/.metis/config.toml`

## Cross-cutting conventions

- IDs: use `TEXT` UUIDv7 values for all primary IDs (portable across SQLite/Turso)
- Time: store UTC timestamps as RFC3339 strings (`TEXT`) for consistency with contracts
- Versioning:
  - `metadata.schema_version` tracks DB schema generation
  - contract version tracked in shared contract crate constants
- Soft deletion: not enabled in Phase 1 (explicitly add later if needed)
- Error envelope compatibility: storage layer returns typed errors mappable to `{ code, message, details? }`

## Entity schema outline

### 1) `metadata`

Single-row installation table.

Fields:

- `id` (TEXT PK) - fixed logical key (for example `installation`)
- `instance_id` (TEXT NOT NULL UNIQUE) - random UUID generated once
- `schema_version` (INTEGER NOT NULL)
- `created_at` (TEXT NOT NULL)
- `updated_at` (TEXT NOT NULL)

### 2) `channels`

Top-level user conversation containers.

Fields:

- `id` (TEXT PK)
- `title` (TEXT NOT NULL)
- `source_type` (TEXT NOT NULL) - `manual` | `external`
- `source_ref` (TEXT NULL) - external locator/path/event id
- `status` (TEXT NOT NULL) - placeholder lifecycle enum
- `created_at` (TEXT NOT NULL)
- `updated_at` (TEXT NOT NULL)

Indexes:

- `idx_channels_created_at`
- `idx_channels_status`

### 3) `branches`

Sub-contexts within a channel.

Fields:

- `id` (TEXT PK)
- `channel_id` (TEXT NOT NULL FK -> `channels.id`)
- `parent_branch_id` (TEXT NULL FK -> `branches.id`)
- `name` (TEXT NOT NULL)
- `is_active` (INTEGER NOT NULL DEFAULT 1)
- `created_at` (TEXT NOT NULL)
- `updated_at` (TEXT NOT NULL)

Indexes:

- `idx_branches_channel_id`
- `idx_branches_parent_branch_id`

### 4) `tasks`

Orchestrated units of work, channel-scoped.

Fields:

- `id` (TEXT PK)
- `channel_id` (TEXT NOT NULL FK -> `channels.id`)
- `branch_id` (TEXT NULL FK -> `branches.id`)
- `kind` (TEXT NOT NULL) - placeholder discriminator
- `state` (TEXT NOT NULL) - queued/running/completed/failed/cancelled (enum in Rust)
- `priority` (INTEGER NOT NULL DEFAULT 0)
- `payload_json` (TEXT NULL) - transport-agnostic task payload
- `created_at` (TEXT NOT NULL)
- `updated_at` (TEXT NOT NULL)
- `started_at` (TEXT NULL)
- `finished_at` (TEXT NULL)

Indexes:

- `idx_tasks_channel_id`
- `idx_tasks_branch_id`
- `idx_tasks_state`
- `idx_tasks_created_at`

### 5) `workers`

Background processes/agents executing tasks.

Fields:

- `id` (TEXT PK)
- `task_id` (TEXT NOT NULL FK -> `tasks.id`)
- `worker_type` (TEXT NOT NULL)
- `state` (TEXT NOT NULL) - pending/running/completed/failed/stopped (enum in Rust)
- `attempt` (INTEGER NOT NULL DEFAULT 0)
- `last_heartbeat_at` (TEXT NULL)
- `started_at` (TEXT NULL)
- `finished_at` (TEXT NULL)
- `created_at` (TEXT NOT NULL)
- `updated_at` (TEXT NOT NULL)

Indexes:

- `idx_workers_task_id`
- `idx_workers_state`
- `idx_workers_last_heartbeat_at`

### 6) `history`

Canonical event/message stream for replay and audit.

Fields:

- `id` (TEXT PK)
- `channel_id` (TEXT NOT NULL FK -> `channels.id`)
- `branch_id` (TEXT NULL FK -> `branches.id`)
- `task_id` (TEXT NULL FK -> `tasks.id`)
- `worker_id` (TEXT NULL FK -> `workers.id`)
- `event_type` (TEXT NOT NULL) - message/event discriminator
- `role` (TEXT NULL) - `system` | `user` | `assistant` | `tool` where applicable
- `content_json` (TEXT NOT NULL) - LLM-chat-compatible content envelope
- `correlation_id` (TEXT NULL) - trace/link id across runtime boundaries
- `created_at` (TEXT NOT NULL)

Indexes:

- `idx_history_channel_created`
- `idx_history_branch_created`
- `idx_history_task_created`
- `idx_history_worker_created`
- `idx_history_correlation_id`

## Referential integrity and constraints

- Enable SQLite foreign keys (`PRAGMA foreign_keys = ON` in connection setup)
- Use `ON DELETE RESTRICT` by default in Phase 1 to avoid accidental cascade loss
- Add stricter business constraints in Rust domain services (not DB triggers) during later phases

## Migration strategy (`sqlx`)

- Keep SQL migrations under `src-tauri/migrations/`
- Embed at compile time via `sqlx::migrate!()`
- Phase 1 migration files:
  - `0001_init.sql` -> creates all core tables and indexes
  - `0002_seed_metadata.sql` -> inserts installation metadata row if absent
- Runtime startup path:
  1. Open DB connection
  2. Ensure `PRAGMA foreign_keys = ON`
  3. Run embedded migrations
  4. Proceed to runtime bootstrap

## Contract mapping boundaries

- Persistence records remain internal to `storage`
- Shared DTOs live in contract module/crate and derive `Serialize`/`Deserialize`
- Mapping layer handles DB <-> contract conversion explicitly
- Avoid leaking adapter-specific types into contracts (no Tauri or HTTP transport fields)

## Minimal repository interface shape (Phase 1 stubs)

- `MetadataRepository`: `get()`, `upsert_instance()`
- `ChannelRepository`: `create()`, `get()`, `list()`, `update_status()`
- `BranchRepository`: `create()`, `list_by_channel()`
- `TaskRepository`: `enqueue()`, `get()`, `list_by_channel()`, `update_state()`
- `WorkerRepository`: `create()`, `get_by_task()`, `update_state()`, `heartbeat()`
- `HistoryRepository`: `append()`, `list_by_channel()`, `list_by_branch()`

All methods return stubs/placeholders in Phase 1 but should compile and enforce dependency direction.

## Out of scope (explicit)

- Full-text search, vector indexes, message chunking
- Archival/retention policy and compaction
- Multi-tenant separation
- Sync/replication conflict handling
- Production-grade retry semantics and queue guarantees

## Developer notes for next implementation pass

- Implement SQLx models and row mapping in `src-tauri/storage/models`
- Add lifecycle transition validators for `task.state` and `worker.state`
- Add deterministic replay loader from `history` + entity state tables
- Add adapter tests to verify same contract output for desktop and headless modes
