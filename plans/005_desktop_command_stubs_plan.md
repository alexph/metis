# Metis Desktop Command Stub Wiring Plan (Phase 1.3)

Audience: coding agent implementing desktop command scaffolding integration.

## Goal

Wire Tauri command handlers to an explicit adapter service boundary so command signatures are stable and ready for later repository/service implementations.

## Scope

- Add a desktop command service trait that represents command operations.
- Add a stub implementation that returns not-implemented adapter errors.
- Register the command service container in Tauri managed state.
- Update command handlers to delegate through the service boundary.
- Keep command response shape stable with shared `ErrorEnvelope` mapping.

## Design requirements

- Tauri command functions should not directly call SQL/storage internals.
- The adapter service boundary should be `Send + Sync` for runtime safety.
- Command handlers should use a shared helper for `Result<T, AdapterError> -> CommandResponse<T>` mapping.
- Keep implementation scaffolding-only; no business behavior.

## Command operations covered

- channels: list/create
- branches: list by channel
- tasks: enqueue/list by channel
- workers: list by task
- history: list by channel/branch

## Out of scope

- Real service/repository implementations
- Live subscriptions or push stream semantics
- Domain validation logic beyond shape checks

## Deliverables

1. `DesktopCommandService` trait with full stubbed operation surface.
2. `DesktopCommandServices` state container managed by Tauri.
3. Tauri command handlers delegating to service trait methods.
4. Shared response/error envelope behavior preserved.

## Acceptance criteria

- Tauri app compiles with command handlers wired through managed state.
- No command handler contains storage/runtime implementation logic.
- Not-implemented behavior remains explicit and structured.
