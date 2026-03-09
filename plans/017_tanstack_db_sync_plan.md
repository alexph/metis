# Metis TanStack DB Sync Plan (Phase 2.5)

Audience: coding agent integrating TanStack DB entity synchronization.

## Goal

Use TanStack DB to maintain normalized in-memory entity state synchronized from invoke/query results.

## Scope

- Define entity collections matching contract DTOs.
- Upsert query and mutation responses into TanStack DB.
- Ensure relationships remain coherent (channel -> branch/task/history, task -> worker).

## Design requirements

- Keep entity IDs aligned with backend UUIDv7 strings.
- Use deterministic update semantics for merges/upserts.
- Avoid leaking transport-specific metadata into entity model.

## Synchronization strategy

- Query responses hydrate/refresh collections.
- Mutation responses patch affected entities immediately.
- Query invalidation remains as a safety net, not sole consistency mechanism.

## Out of scope

- Full offline persistence strategy
- Realtime conflict reconciliation
- Background agent stream ingestion

## Deliverables

1. TanStack DB collection definitions.
2. Sync helpers for hydrate/upsert/merge.
3. Integration with existing query/mutation hooks.

## Acceptance criteria

- Entity collections reflect backend responses accurately.
- Mutations update visible state without requiring full page refresh.
- Relationship views (channel/task/history) remain consistent.
