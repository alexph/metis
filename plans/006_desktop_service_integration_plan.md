# Metis Desktop Service Integration Plan (Phase 1.4)

Audience: coding agent implementing service-layer command integration.

## Goal

Make desktop command handlers look production-shaped by routing through a concrete service implementation that composes repository boundaries, while keeping repository behavior scaffold-only.

## Scope

- Add a concrete desktop command service implementation backed by SQLite repository placeholders.
- Keep command handlers unchanged at boundary level (`CommandResponse<T>` + `ErrorEnvelope`).
- Wire bootstrap to initialize DB/migrations and register real command service state.
- Preserve fallback to stub service if bootstrap fails.

## Service design

- `DesktopCommandService` remains the command-facing trait.
- New concrete implementation should:
  - compose `SqliteChannelRepository`
  - compose `SqliteBranchRepository`
  - compose `SqliteTaskRepository`
  - compose `SqliteWorkerRepository`
  - compose `SqliteHistoryRepository`
- Methods delegate directly to repository interfaces.

## Error mapping

- Add adapter-level conversion from `StorageError` to `DesktopAdapterError`.
- Continue mapping to shared `ErrorEnvelope` at command boundary.

## Bootstrap wiring

- Bootstrap returns initialized command services when DB setup succeeds.
- App startup registers real service state.
- On bootstrap failure, use stub service to keep app startup resilient.

## Out of scope

- Implementing repository SQL behavior
- Transaction orchestration
- Performance tuning or threading policy changes

## Deliverables

1. Concrete desktop command service implementation module.
2. Updated adapter error mapping for storage propagation.
3. Bootstrap-driven managed state using real service container.
4. Build passes with end-to-end command -> service -> repository delegation path.

## Acceptance criteria

- Commands route through concrete service implementation.
- Service implementation composes repository boundaries without leaking internals.
- Errors flow through adapter envelope mapping consistently.
- Workspace compiles with no production logic beyond scaffolding.
