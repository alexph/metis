# Metis Repository Read Path Expansion Plan (Phase 1.8)

Audience: coding agent implementing additional concrete repository methods.

## Goal

Expand real persistence coverage so service and desktop command flows can return meaningful task/worker/history data from SQLite.

## Scope

Implement concrete SQLx behavior for:

- `TaskRepository::enqueue`
- `TaskRepository::get`
- `WorkerRepository::get_by_task`
- `HistoryRepository::list_by_channel`
- `HistoryRepository::list_by_branch`

## Design requirements

- Use typed SQLx row decoding (`FromRow`) for queried records.
- Reuse explicit mapping functions in `storage::mappings`.
- Preserve deterministic ordering (`created_at ASC`) for list methods.
- Keep unimplemented methods as explicit stubs where out of scope.

## Out of scope

- Worker create/update/heartbeat write workflows
- History append write workflow
- Task state update lifecycle persistence
- Cross-entity transaction orchestration

## Deliverables

1. Concrete SQLx implementations for listed methods.
2. Record model derives needed for row decoding.
3. Compileable service/adapter path with expanded real read coverage.

## Acceptance criteria

- Workspace compiles successfully.
- Task enqueue/get persists and retrieves DB-backed task data.
- Worker list-by-task returns mapped DTOs.
- History list queries return mapped DTOs for channel and branch scopes.
- Typed mapping/storage errors continue to propagate to adapter envelope mapping.
