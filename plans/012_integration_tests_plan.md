# Metis Repository and Service Integration Test Plan (Phase 2.0)

Audience: coding agent adding focused verification for newly real persistence paths.

## Goal

Add targeted integration tests that validate repository and domain-service behavior for task/worker state flows and history persistence ordering.

## Scope

Create integration tests that exercise:

- task state transition persistence timestamps
- worker state transition + heartbeat persistence
- history append and ordered list retrieval by channel/branch
- service-level lifecycle conflict handling for invalid transitions

## Test harness requirements

- Use isolated temporary SQLite database per test.
- Run embedded migrations for each test DB.
- Enable SQLite foreign keys.
- Use real repository/service wiring (no mocks).

## Out of scope

- Broad end-to-end UI adapter tests
- Load/performance testing
- Property-based fuzzing

## Deliverables

1. Integration test module under `src-tauri/tests/`.
2. Temporary DB setup utility and migration bootstrap in tests.
3. Assertions for timestamp and ordering semantics.
4. Service conflict test for invalid task transition.

## Acceptance criteria

- New test suite passes reliably with isolated DB state.
- Tests verify both repository and service logic paths.
- `cargo test -p metis --test repository_service_flows` succeeds.
