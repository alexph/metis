# Metis Query Integration Plan (Phase 2.4)

Audience: coding agent integrating TanStack Query with invoke client.

## Goal

Wire TanStack Query around typed invoke wrappers for deterministic fetch/mutate flows with clear invalidation strategy.

## Scope

- Define query key factory for core entities.
- Add query functions for list/get paths.
- Add mutation hooks for create/update/append operations.
- Configure cache invalidation and/or direct cache updates per mutation.

## Query surfaces

- channels list
- branches by channel
- tasks by channel
- workers by task
- history by channel/branch

## Mutation surfaces

- channel create/update status
- task enqueue/update state
- worker create/update state/heartbeat
- history append

## Design requirements

- Keep query key shape stable and composable.
- Avoid over-fetching by using scoped invalidation.
- Surface backend `ErrorEnvelope` fields through query error states.

## Out of scope

- Offline sync/conflict handling
- Event-driven cache updates
- Agent UX flows

## Deliverables

1. Query key definitions.
2. Query/mutation hook modules.
3. Invalidation/update strategy documented in code comments or notes.

## Acceptance criteria

- Queries/mutations execute successfully through invoke wrappers.
- Cache updates are predictable after mutations.
- UI can render loading/error/success states from Query.
