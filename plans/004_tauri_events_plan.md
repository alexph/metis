# Metis Tauri Event Boundary Plan (Phase 1.2)

Audience: coding agent implementing desktop adapter boundaries only.

## Goal

Scaffold a Tauri-facing command/event boundary that exposes shared contract types without embedding runtime/storage internals into UI transport payloads.

The focus is adapter shape and compileable structure, not production behavior.

## Problem framing

Metis supports a desktop mode now and plans for future headless adapters. We need a clear boundary where Tauri commands/events map to the shared contract schema and shared error envelope.

## Scope

Create a minimal desktop adapter module under `src-tauri` that includes:

- command handler stubs for read/create/list operations
- event publishing abstraction for channel/task/worker/history updates
- adapter-level request/response wrappers that use `metis-contract` DTOs
- explicit error mapping from internal errors -> `ErrorEnvelope`

No real orchestration, DB querying, or live streaming behavior yet.

## Proposed module shape

- `src-tauri/src/adapters/mod.rs`
- `src-tauri/src/adapters/desktop/mod.rs`
- `src-tauri/src/adapters/desktop/commands.rs`
- `src-tauri/src/adapters/desktop/events.rs`
- `src-tauri/src/adapters/desktop/errors.rs`

Optional: keep lightweight DTO wrappers local to adapter if needed for command ergonomics, but prefer direct `metis-contract` types.

## Event model scaffold

Define typed event names and payload boundaries for:

- channel created/updated
- task enqueued/state changed
- worker created/state changed/heartbeat
- history appended
- runtime status changed (placeholder)

Use a single publish interface so runtime/services do not depend on Tauri directly.

## Command model scaffold

Stub commands should include signatures for:

- channel list/create
- branch list by channel
- task enqueue/list by channel
- worker list by task
- history list by channel/branch

Commands return stub placeholders or not-implemented envelopes but compile.

## Error mapping requirements

- All adapter-facing errors must map into contract `ErrorEnvelope`.
- Avoid leaking `sqlx`, `tauri`, or internal enum details directly to UI surface.
- Keep stable error `code` values for future UI handling.

## Out of scope

- Full event bus implementation
- Real-time subscriptions beyond scaffold
- Authentication/authorization
- Production retries/queue guarantees

## Deliverables

1. Compileable desktop adapter module tree.
2. Command stub signatures wired into Tauri invoke handler.
3. Event publishing boundary with typed event identifiers.
4. Shared error envelope mapping used by command stubs.

## Acceptance criteria

- Tauri layer compiles with new adapter modules.
- Commands/events use shared contract types at boundary.
- Internal services remain decoupled from Tauri-specific types.
- No production workflow behavior added.
