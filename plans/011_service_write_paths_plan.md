# Metis Service Write Path Completion Plan (Phase 1.9)

Audience: coding agent implementing remaining service-backed write/state flows.

## Goal

Complete the remaining end-to-end write and state-mutation paths so domain services can execute their intended operations through concrete repository behavior.

## Scope

Implement concrete repository methods required by existing services:

- `BranchRepository::create`
- `TaskRepository::update_state`
- `WorkerRepository::create`
- `WorkerRepository::update_state`
- `WorkerRepository::heartbeat`
- `HistoryRepository::append`

Then verify service methods now run against real persistence paths:

- `BranchDomainService::create_branch`
- `TaskDomainService::update_state`
- `WorkerDomainService::{create_worker, update_state, heartbeat}`
- `HistoryDomainService::append_event`

## Design requirements

- Reuse mapping layer (`storage::mappings`) for all record/contract conversions.
- Use UTC RFC3339 timestamps for update/heartbeat fields.
- Preserve explicit lifecycle checks in services; repositories only persist state.
- Keep deterministic ordering behavior in existing list methods unchanged.
- Keep adapter/service/repository boundaries intact.

## Repository behavior details

### Branch create

- Insert full row into `branches`.
- Return inserted row mapped to contract.

### Task state update

- Update `state` and `updated_at`.
- Set `started_at` when transitioning to `running` if unset.
- Set `finished_at` when transitioning to terminal state (`completed`, `failed`, `cancelled`).

### Worker create/state/heartbeat

- Insert full worker row on create.
- Update `state` + `updated_at` on state changes.
- Set `started_at` / `finished_at` with same terminal semantics as task where applicable.
- Update `last_heartbeat_at` + `updated_at` on heartbeat.

### History append

- Insert full event row.
- Return inserted row mapped to contract.

## Service adjustments (minimal)

- Keep current validation and not-found checks.
- Improve worker state update path to resolve current worker state from persisted data (not placeholder assumption).

## Out of scope

- Transaction orchestration across multiple repositories
- Event bus emission on writes
- Retry policies and idempotency guarantees
- Pagination/search extensions

## Deliverables

1. Concrete write/state repository implementations for branches/tasks/workers/history.
2. Service write methods fully backed by persistence.
3. Worker state transition check based on persisted current state.
4. Compileable desktop command path with expanded real write support.

## Acceptance criteria

- Workspace compiles successfully.
- All currently exposed service write methods persist expected changes.
- Timestamp fields are updated consistently for state/heartbeat flows.
- Errors continue to map through service -> adapter -> `ErrorEnvelope` without boundary leaks.
