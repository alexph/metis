# Metis Frontend Invoke Client Plan (Phase 2.3)

Audience: coding agent implementing frontend API access layer.

## Goal

Create a typed frontend invoke client that wraps Tauri command calls and normalizes `CommandResponse<T>` + `ErrorEnvelope` handling.

## Scope

- Add a frontend module for desktop command invocation wrappers.
- Define typed request/response helpers matching Rust command DTOs.
- Normalize adapter errors into a single frontend error type.
- Keep transport details isolated from UI components.

## Command coverage

- channels list/create/update status
- branches list by channel
- tasks enqueue/list/update state
- workers list/create/update state/heartbeat
- history list by channel/branch and append

## Design requirements

- Use a single invoke helper for consistent serialization and error mapping.
- Preserve server-provided error envelope fields (`code`, `message`, `details`).
- Keep command names centralized to avoid string drift.

## Out of scope

- UI state management and cache wiring
- Event subscription handling
- Agent invocation workflows

## Deliverables

1. Typed invoke client module for all desktop commands.
2. Shared frontend error abstraction for envelope handling.
3. Minimal usage examples for query/mutation integration.

## Acceptance criteria

- Frontend compiles with typed command wrappers.
- Successful invokes return typed data.
- Error responses map predictably to frontend error objects.
