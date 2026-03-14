## Why

The current prototype persists channel history correctly, but execution intent stops at the Tauri command boundary and does not clearly hand off into runtime orchestration. This makes it hard to reason about "what happens next" and will get harder as additional entry channels (embedded server mode) are introduced.

## What Changes

- Refactor Tauri command handlers to be thin transport adapters that delegate to shared app use-cases.
- Introduce a runtime handoff flow based on history events (not task records) so a user message append can trigger background turn processing.
- Add a small internal domain-event fanout mechanism so a single event can publish to multiple sinks (Tauri plus non-Tauri channels).
- Preserve current response envelopes and command naming compatibility while moving orchestration behavior out of command modules.

## Capabilities

### New Capabilities
- `runtime-turn-handoff`: Defines how persisted history events trigger asynchronous runtime turn processing via an internal runtime inbox.
- `multi-sink-event-fanout`: Defines transport-agnostic event publication where one domain event is emitted to Tauri and additional channels.
- `thin-transport-adapters`: Defines adapter behavior where Tauri/server command handlers only map requests/responses and delegate execution logic to shared use-cases.

### Modified Capabilities
- None.

## Impact

- Affected backend modules in `src-tauri/src/commands`, `src-tauri/src/runtime`, and new app-level orchestration/event modules.
- Tauri invoke command names and `CommandResponse` envelope stay stable for frontend compatibility.
- Future embedded server endpoints can reuse the same use-cases and event fanout without duplicating orchestration logic.
