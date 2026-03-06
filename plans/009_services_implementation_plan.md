# Metis Service Layer Implementation Plan (Phase 1.7)

Audience: coding agent implementing domain/application services on top of repository boundaries.

## Goal

Implement concrete service behavior so command handlers call stable service APIs that enforce domain rules, lifecycle checks, and cross-repository coordination while keeping adapters/storage decoupled.

## Scope

Implement concrete service modules for:

- `channels`
- `branches`
- `tasks`
- `workers`
- `history`

Each service should consume repository traits (not SQLx directly) and return contract DTOs/errors suitable for adapter mapping.

## Service responsibilities

### Channel service

- validate create input shape (required title/source type)
- create channel via repository
- list/get channels
- update channel status via explicit service method

### Branch service

- create branch with channel existence guard
- list branches by channel
- validate parent branch linkage when provided

### Task service

- enqueue task with channel/branch consistency checks
- list/get tasks
- update task state with lifecycle transition validation boundary

### Worker service

- create worker for existing task
- list workers by task
- update worker state with lifecycle transition validation boundary
- heartbeat update path

### History service

- append history event with entity reference validation where applicable
- list history by channel/branch with deterministic ordering

## Error and validation model

- Introduce service-level error type(s) with stable codes (validation/not_found/conflict/internal).
- Map repository/storage errors into service errors explicitly.
- Keep adapter-facing conversion path to `ErrorEnvelope` intact.

## Dependency direction

- adapters -> services -> repositories -> storage/sqlx
- services depend on repository traits and mapping/lifecycle helpers.
- services must not import Tauri-specific types.

## Out of scope

- full production policy behavior (retries, scheduling guarantees)
- long-running orchestration/event-loop semantics
- auth/multi-tenant model
- advanced pagination/search

## Deliverables

1. Concrete service implementations for all five domains.
2. Service errors and conversion helpers.
3. Command service integration updated to delegate to domain services (not repositories directly).
4. Compileable boundary with current stubbed methods progressively replaced.

## Acceptance criteria

- Workspace compiles and command path remains stable.
- Core service operations perform input/domain checks before repository calls.
- Lifecycle transition helpers are used for task/worker state changes.
- Adapter responses still return shared contract envelope semantics.
