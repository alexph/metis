# Metis Phase 1 Notes

This pass only scaffolds module boundaries and placeholders.

- `metis-contract/` contains transport-agnostic DTOs and shared error/version contracts.
- `src-tauri/src/storage` contains repository interfaces, DB model stubs, and migration bootstrap.
- `src-tauri/migrations` defines the SQLx schema baseline and metadata seed pass.
- `src-tauri/src/runtime` defines event loop boundary placeholders with Tokio channels.
- `src-tauri/src/agents` contains a Rig integration boundary with a stub client that returns `"hello"`.

Real query implementations, state transitions, and replay behavior should be added in a future implementation phase.

## Mapping pass update

- Completed explicit DB-model to contract mappings in `src-tauri/src/storage/mappings.rs`.
- Added reverse mappings (contract to DB-model) for all Phase 1 entities to enforce separation boundaries.
- Added enum/string translation for:
  - `channels.source_type`
  - `channels.status`
  - `tasks.state`
  - `workers.state`
  - `history.role`
- Added a typed `StorageError::Mapping` variant for invalid persisted enum values so adapters can map it to a shared error envelope.

## Planning memory update

- Added retroactive mapping documentation plan at `plans/003_mappings_plan.md` to capture decisions and acceptance criteria for the completed mapping pass.
- Added next-step desktop adapter/event boundary plan at `plans/004_tauri_events_plan.md` to guide Tauri command/event scaffolding while preserving contract boundaries.

## 004 implementation update (desktop adapter scaffold)

- Added desktop adapter modules:
  - `src-tauri/src/adapters/mod.rs`
  - `src-tauri/src/adapters/desktop/mod.rs`
  - `src-tauri/src/adapters/desktop/commands.rs`
  - `src-tauri/src/adapters/desktop/events.rs`
  - `src-tauri/src/adapters/desktop/errors.rs`
- Added Tauri command stubs for planned boundary operations (channels, branches, tasks, workers, history).
- Wired command stubs into Tauri invoke handling in `src-tauri/src/lib.rs`.
- Added typed event identifiers and a `DesktopEvent` enum payload boundary with a publish trait (`DesktopEventPublisher`) and no-op publisher.
- Adapter command surface returns structured `CommandResponse<T>` with shared `ErrorEnvelope` for not-implemented responses.
