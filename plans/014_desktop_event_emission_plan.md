# Metis Desktop Event Emission Plan (Phase 2.2)

Audience: coding agent wiring live desktop event emission from adapter command flows.

## Goal

Publish typed desktop events for successful mutating command operations so frontend consumers can react without polling.

## Scope

- Implement Tauri emission helper using existing `DesktopEvent` definitions.
- Emit events from command adapter layer after successful service calls.
- Keep event emission best-effort for this phase (do not fail successful writes if emit fails).

## Initial operations to emit

- `desktop_channels_create` -> `channel-created`
- `desktop_tasks_enqueue` -> `task-enqueued`

## Design requirements

- Use existing event constants and payload types in `adapters/desktop/events.rs`.
- Emit only after service operation success.
- Log emit failures with context; return command success payload regardless.
- Keep service/repository layers transport-agnostic.

## Out of scope

- Durable/replayable event delivery
- Strict delivery guarantees and retries
- Headless transport event adapters
- Emission for all future mutate commands not yet exposed

## Deliverables

1. Tauri-backed event emitter helper in desktop adapter events module.
2. Command-layer wiring for initial mutating commands.
3. Notes update documenting best-effort emit policy and covered operations.

## Acceptance criteria

- Workspace compiles and tests pass.
- Successful channel create and task enqueue commands attempt Tauri emission.
- Emit failures are surfaced in logs, not as command write failures.
