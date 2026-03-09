# Metis Desktop Adapter Command Boundary Test Plan (Phase 2.1)

Audience: coding agent validating Tauri desktop command boundary behavior.

## Goal

Add focused tests for desktop command adapter behavior so command response contracts and error-envelope mapping remain stable as service/repository implementations evolve.

## Scope

Validate desktop command functions in `src-tauri/src/adapters/desktop/commands.rs` for:

- response envelope shape (`CommandResponse<T>`)
- successful delegation through `DesktopCommandService`
- service/storage error mapping to shared `ErrorEnvelope`
- not-implemented fallback behavior when stub service is used

## Test approach

- Prefer unit-style tests at command module/service-boundary level.
- Use controlled test doubles for `DesktopCommandService` trait where practical.
- For error mapping checks, assert stable `code` values and core message semantics.

## Candidate test cases

1. `desktop_channels_list` returns `ok` payload when service returns data.
2. `desktop_channels_create` returns `err` envelope for validation/service conflict.
3. `desktop_tasks_list_by_channel` maps storage error to envelope code.
4. Stub command service path emits `desktop_not_implemented` envelope.

## Out of scope

- Full UI/invoke integration through frontend runtime
- Cross-process IPC behavior
- Load/performance characteristics of command execution

## Deliverables

1. Adapter-boundary tests covering success + error pathways.
2. Stable assertions around envelope codes and payload structure.
3. Notes update capturing expected command contract behavior.

## Acceptance criteria

- Tests pass consistently.
- Command boundary contract regressions are caught by CI/local runs.
- Error envelope behavior remains transport-agnostic and predictable.
