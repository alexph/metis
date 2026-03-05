# Metis Bootstrap Plan (Phase 1)

Audience: coding agent implementing scaffolding only.

## Purpose

Metis is a Rust-based orchestrator for personal AI agents. It should support long-running channels where agents coordinate tasks and workers.

## Scope (Phase 1)

Build only the foundational structure of the engine:

- local persistence model
- runtime skeleton (event loop + process/task orchestration)
- module boundaries for channels, tasks, workers, and history
- minimal agent stub integration

Do not implement production behavior yet.

## Product model

- A user interacts with Metis through **channels** (long-running conversations, IRC-like).
- Channels can be created from:
  - an external source (file, ingest event, etc)
  - an empty/manual creation flow
- Each channel may have **branches** (sub-contexts).
- The system orchestrates **tasks**, which may spawn background **workers**.
- A top-level “main agent” exists outside channel scope (name TBD; e.g. overseer/oracle/architect).

## Data contracts

Define canonical, transport-agnostic contract types shared across delivery modes.

- Keep shared DTO/schema types in a dedicated contract module/crate (for example `metis-contract`).
- Make contract types portable with `serde` (`Serialize`/`Deserialize`).
- Keep contract types free of transport/runtime concerns (no Tauri- or HTTP-specific fields).
- Keep persistence models separate from contract types; map DB models <-> contract types explicitly.
- Use adapters to expose the same contract types through:
  - desktop mode (Tauri commands/events)
  - future headless mode (HTTP/IPC/API)

This prevents duplication and allows UI and server surfaces to reuse the same data schema.

## Storage plan

Use a local database under `~/.metis/`.

Candidate backends:

- SQLite (default target)
- Turso (optional future backend)

Migration approach:

- Use `sqlx` migrations.
- Embed migrations in the binary (no external migration files required at runtime).

Persist enough state for deterministic runtime behavior from DB state alone.

Required entities:

- `metadata`: installation-level info, including random instance ID
- `channels`: user channels
- `branches`: sub-contexts within channels
- `tasks`: queued or active work within channels
- `workers`: background agents/processes
- `history`: messages/events linked to channel/branch/task/worker

Note: message schema should remain compatible with common LLM chat formats.

## Runtime/agent plan

- Rust + Tokio runtime
- Event loop abstraction
- Pub/sub abstraction (only if needed by module boundaries)
- Process and worker lifecycle abstractions
- Agent integration via Rig crate
- Agent implementation for now: stub response (`"hello"`)

## Project structure target

Current baseline:

- `src/`: frontend
- `src-tauri/`: Rust system

Phase 1 should define additional Rust module layout under `src-tauri/` for:

- `storage`
- `channels`
- `branches`
- `tasks`
- `workers`
- `history`
- `runtime` (event loop / orchestration)
- `agents`

## Implementation constraints (important)

For this phase, implement **structure only**:

- create folders/modules/types/interfaces
- create placeholder functions with signatures
- wire module boundaries and basic dependency direction

Do not implement real feature logic.

## Deliverables

1. Compilable Rust module tree for engine scaffolding.
2. Stubbed domain models for required entities.
3. Stubbed storage interface and backend placeholders.
4. Migration scaffold using `sqlx` with embedded migrations.
5. Stubbed runtime/orchestration interfaces.
6. Stubbed agent integration that returns `"hello"`.
7. Short developer notes explaining where real logic will be added later.

## Acceptance criteria

- Codebase compiles.
- No production behavior beyond stubs.
- Module boundaries reflect the product model above.
- All required entities exist as explicit types/interfaces.
- Migration setup is defined via `sqlx` and designed to run from embedded assets.
- A future coding pass can implement behavior without restructuring the tree.
