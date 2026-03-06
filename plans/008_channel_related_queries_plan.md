# Metis Channel-Scoped Query Plan (Phase 1.6)

Audience: coding agent implementing additional real repository reads.

## Goal

Extend the first real data path by implementing channel-scoped branch/task reads so desktop commands can return related entities from persisted state.

## Scope

- Implement real SQLite behavior for:
  - `BranchRepository::list_by_channel`
  - `TaskRepository::list_by_channel`
- Keep write/update methods for branches/tasks as scaffolding where not already implemented.
- Reuse mapping layer for record -> contract translation.

## Design notes

- Use SQLx `query_as` with typed record structs.
- Add `sqlx::FromRow` derives to record structs needed for query decoding.
- Return rows ordered by `created_at ASC` to keep deterministic replay-friendly ordering.

## Out of scope

- Branch/task create workflow implementation
- Task state transition logic
- Pagination/filtering/search
- Runtime orchestration behavior

## Deliverables

1. Concrete read implementations for channel-scoped branch/task lists.
2. Record model updates needed for SQLx row decoding.
3. Compileable command path for branch/task list operations through desktop service.

## Acceptance criteria

- Workspace compiles successfully.
- `branches.list_by_channel` returns mapped branch DTOs from DB.
- `tasks.list_by_channel` returns mapped task DTOs from DB.
- Mapping errors remain typed and propagate through adapter envelope flow.
