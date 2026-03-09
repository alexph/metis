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

## 005 implementation update (command service wiring)

- Added `DesktopCommandService` trait in `src-tauri/src/adapters/desktop/commands.rs` for the full command operation surface.
- Added `StubDesktopCommandService` and `DesktopCommandServices` container using `Arc<dyn DesktopCommandService + Send + Sync>`.
- Updated Tauri commands to resolve managed state (`tauri::State<DesktopCommandServices>`) and delegate calls through the service trait.
- Added centralized result mapping helper to convert adapter `Result<T, DesktopAdapterError>` into `CommandResponse<T>`.
- Registered stub command services in app bootstrap via `.manage(DesktopCommandServices::new_stub())`.
- Added planning artifact `plans/005_desktop_command_stubs_plan.md` to capture this boundary as a formal phase.

## 006 implementation update (concrete desktop service)

- Added concrete command service implementation at `src-tauri/src/adapters/desktop/service.rs`.
- Service composes SQLite repository placeholders and delegates command operations through repository traits:
  - channels, branches, tasks, workers, history
- Added storage error propagation into adapter surface via `DesktopAdapterError::Storage` and `From<StorageError>`.
- Updated bootstrap in `src-tauri/src/lib.rs` to return initialized `DesktopCommandServices` using real DB-backed service when setup succeeds.
- Kept resilience fallback: app still starts with stub service if bootstrap fails.
- Added planning artifact `plans/006_desktop_service_integration_plan.md`.

## 007 implementation update (first real repository methods)

- Added first concrete repository behavior in `src-tauri/src/storage/repositories.rs` for channels:
  - `create`
  - `get`
  - `list`
  - `update_status`
- Implemented SQLx query path with explicit mapping usage (`channel_contract_to_record`, `channel_record_to_contract`).
- Added `sqlx::FromRow` derive for `ChannelRecord` in `src-tauri/src/storage/models/channel.rs`.
- Added exported mapping helper `channel_status_to_storage` for status persistence consistency.
- Added planning artifact `plans/007_channels_repository_plan.md` to capture this first real data-path phase.

## 008 implementation update (channel-scoped branch/task reads)

- Implemented concrete `list_by_channel` reads in `src-tauri/src/storage/repositories.rs` for:
  - `SqliteBranchRepository`
  - `SqliteTaskRepository`
- Added SQLx row decoding derives to support typed queries:
  - `src-tauri/src/storage/models/branch.rs`
  - `src-tauri/src/storage/models/task.rs`
- Reused mapping boundaries:
  - `branch_record_to_contract`
  - `task_record_to_contract`
- Added planning artifact `plans/008_channel_related_queries_plan.md`.

## Planning memory update (009)

- Added `plans/009_services_implementation_plan.md` to define the next phase for concrete domain service implementations.
- Plan establishes service responsibilities, validation/error boundaries, and dependency direction from adapters -> services -> repositories.

## 009 implementation update (domain services)

- Implemented concrete domain service layers with validation and typed service errors:
  - `src-tauri/src/channels/mod.rs`
  - `src-tauri/src/branches/mod.rs`
  - `src-tauri/src/tasks/mod.rs`
  - `src-tauri/src/workers/mod.rs`
  - `src-tauri/src/history/mod.rs`
- Added shared service error type at `src-tauri/src/core/service_error.rs` and exported it from `src-tauri/src/core/mod.rs`.
- Updated desktop adapter error mapping to include `DesktopAdapterError::Service` and conversion from `ServiceError`.
- Refactored `src-tauri/src/adapters/desktop/service.rs` to delegate through domain services instead of calling repositories directly.
- Added concrete lifecycle transition guards for task/worker state helper functions and used them in service update methods.

## 010 implementation update (repository read-path expansion)

- Implemented additional concrete repository methods in `src-tauri/src/storage/repositories.rs`:
  - `tasks.enqueue`
  - `tasks.get`
  - `workers.get_by_task`
  - `history.list_by_channel`
  - `history.list_by_branch`
- Added SQLx row decoding support for queried models:
  - `src-tauri/src/storage/models/worker.rs`
  - `src-tauri/src/storage/models/history.rs`
- Reused explicit mapping functions for all returned contract DTOs.
- Added planning artifact `plans/010_repository_read_paths_plan.md`.

## Planning memory update (011)

- Added `plans/011_service_write_paths_plan.md` to define the remaining write/state completion pass for services and repositories.
- Plan targets branch create, task state update, worker create/state/heartbeat, and history append so all existing service write methods can run end-to-end.

## 011 implementation update (write/state paths)

- Implemented remaining write/state repository methods in `src-tauri/src/storage/repositories.rs`:
  - `branches.create`
  - `tasks.update_state`
  - `workers.create`
  - `workers.get`
  - `workers.update_state`
  - `workers.heartbeat`
  - `history.append`
- Added storage mapping helpers in `src-tauri/src/storage/mappings.rs`:
  - `task_state_to_storage`
  - `worker_state_to_storage`
- Added SQLx row decoding derives for write/readback models:
  - `src-tauri/src/storage/models/worker.rs`
  - `src-tauri/src/storage/models/history.rs`
- Updated worker domain service transition check to use persisted current worker state via `WorkerRepository::get` before applying state transition rules.

## 012 implementation update (integration tests)

- Added focused integration tests in `src-tauri/tests/repository_service_flows.rs` covering:
  - task state transition timestamp persistence
  - worker state transition + heartbeat persistence
  - history append + ordered list retrieval by channel/branch
  - service-level invalid task transition conflict handling
- Added per-test temporary SQLite harness with migrations and FK enforcement.
- Added test planning artifact `plans/012_integration_tests_plan.md`.

## Planning memory update (013)

- Added `plans/013_desktop_adapter_command_tests_plan.md` to define adapter-command boundary tests for `CommandResponse<T>` shape, delegation behavior, and shared `ErrorEnvelope` mapping guarantees.

## 013 implementation update (desktop adapter command tests)

- Added unit tests in `src-tauri/src/adapters/desktop/commands.rs` to verify desktop command boundary behavior:
  - `CommandResponse<T>` success shape serialization (`status: ok`)
  - service error mapping to shared envelope (`service_validation_error`)
  - storage error mapping passthrough (`storage_not_implemented`)
  - stub service not-implemented mapping (`desktop_not_implemented`)
- Refactored command handlers to delegate through internal `handle_*` helper functions so command behavior can be tested without Tauri runtime wiring.

## Cleanup update

- Removed now-unused domain stub service structs/impls from:
  - `src-tauri/src/channels/mod.rs`
  - `src-tauri/src/branches/mod.rs`
  - `src-tauri/src/tasks/mod.rs`
  - `src-tauri/src/workers/mod.rs`
  - `src-tauri/src/history/mod.rs`
- Kept desktop adapter stub service (`StubDesktopCommandService`) as an intentional bootstrap fallback path.

## 014 implementation update (desktop event emission)

- Added event plan `plans/014_desktop_event_emission_plan.md`.
- Implemented a Tauri-backed event emitter helper in `src-tauri/src/adapters/desktop/events.rs`:
  - `emit_tauri_event(app, event)` emits using the existing typed event names.
- Wired best-effort emission from desktop command layer in `src-tauri/src/adapters/desktop/commands.rs`:
  - `desktop_channels_create` emits `DesktopEvent::ChannelCreated` on success
  - `desktop_tasks_enqueue` emits `DesktopEvent::TaskEnqueued` on success
- Emit policy for this phase is non-blocking: command success still returns even if event emission fails; failures are logged with `tracing::warn`.

### 014 extension update (additional mutating command emissions)

- Expanded desktop command surface in `src-tauri/src/adapters/desktop/commands.rs` with additional mutating operations:
  - `desktop_channels_update_status`
  - `desktop_tasks_update_state`
  - `desktop_workers_create`
  - `desktop_workers_update_state`
  - `desktop_workers_heartbeat`
  - `desktop_history_append`
- Added matching request DTOs and `DesktopCommandService` trait methods.
- Wired these commands into `src-tauri/src/lib.rs` invoke handler.
- Extended event emission coverage (best-effort) to emit on successful mutation results:
  - `ChannelUpdated`
  - `TaskStateChanged`
  - `WorkerCreated`
  - `WorkerStateChanged`
  - `WorkerHeartbeat`
  - `HistoryAppended`
- Updated `SqliteDesktopCommandService` in `src-tauri/src/adapters/desktop/service.rs` to implement these operations and return post-mutation entities for event payloads.
- Extended worker domain service trait with `get(worker_id)` to support post-update/heartbeat fetches for emitted payloads.

### 014 test coverage update

- Added adapter command tests in `src-tauri/src/adapters/desktop/commands.rs` for event routing helpers:
  - verifies success responses map to expected event names for all emitted mutation events
  - verifies error responses map to no event emission (`None`)
- This gives fast regression coverage for command -> event-name/payload routing semantics without requiring a live Tauri app handle in tests.

## Planning memory update (015-019)

- Added forward plans for frontend-first Phase 1 completion sequence:
  - `plans/015_frontend_invoke_client_plan.md`
  - `plans/016_query_integration_plan.md`
  - `plans/017_tanstack_db_sync_plan.md`
  - `plans/018_ui_vertical_slice_plan.md`
  - `plans/019_phase1_completion_hardening_plan.md`
- This sequence intentionally targets full non-agent data loop completion (frontend send/receive + sync + hardening) before agent-specific UX/runtime work.
