## Context

Metis currently has strong local persistence and typed command boundaries, but command handlers still contain execution-adjacent behavior (including direct event emission) and no explicit runtime handoff model for processing new user turns. The prototype goal is to model runtime behavior clearly, keep command surfaces stable, and avoid introducing heavyweight orchestration infrastructure.

Constraints:
- Single-user desktop-first product today, with optional embedded server mode later.
- Tasks are planning artifacts for agents and SHALL NOT be the orchestration trigger primitive.
- Crash-hardening and durable outbox semantics are explicitly out of scope for this change.

Stakeholders:
- Core runtime modeling and architecture evolution for Metis.
- Frontend integration that depends on stable command envelopes and names.
- Future transport adapters (server endpoints) reusing the same application behaviors.

## Goals / Non-Goals

**Goals:**
- Make Tauri command modules thin transport adapters.
- Introduce one clear runtime handoff path from persisted history to asynchronous runtime turn processing.
- Introduce one domain-event fanout path that emits to Tauri and additional sinks.
- Preserve existing desktop command naming and response envelope compatibility.

**Non-Goals:**
- Implement durable queue/outbox/retry guarantees.
- Introduce enterprise event bus patterns or distributed orchestration abstractions.
- Redesign task semantics; task records remain agent-planning data, not runtime dispatch.
- Finalize transport protocol for server mode (REST vs tRPC).

## Decisions

1. Thin adapter boundary for transport commands
- Decision: Tauri command functions delegate to shared app use-cases and stop containing orchestration logic.
- Rationale: This makes behavior reusable by future server adapters and removes command-level coupling.
- Alternatives considered:
  - Keep orchestration logic in Tauri commands: rejected because it duplicates behavior for server mode.
  - Introduce separate command framework for each transport: rejected as unnecessary complexity.

2. Runtime handoff keyed off history append
- Decision: A successful user-message history append triggers runtime handoff via an internal runtime inbox command (for example, ProcessTurn).
- Rationale: History is the source of truth for replay and aligns with user mental model of conversation-driven execution.
- Alternatives considered:
  - Task-based orchestration trigger: rejected because tasks are planning artifacts, not execution dispatch.
  - Polling history table for new turns: deferred; direct enqueue is simpler and sufficient for prototype velocity.

3. Domain events fan out to sinks
- Decision: Use a minimal internal domain-event publisher with multiple sink implementations (Tauri sink now, other sinks later).
- Rationale: One emitted event should naturally publish to multiple channels without adapter-specific branching in use-cases.
- Alternatives considered:
  - Keep Tauri event emission in command handlers: rejected due to transport coupling.
  - Add full external message bus abstraction: rejected as over-engineered for single-user desktop scope.

4. Best-effort asynchronous trigger semantics
- Decision: Runtime inbox enqueue and event fanout are best-effort after successful persistence.
- Rationale: Replay from stored history is acceptable and preferred over adding durable dispatch mechanics now.
- Alternatives considered:
  - Durable outbox with recovery replay: deferred until reliability requirements justify added complexity.

## Risks / Trade-offs

- [In-memory runtime handoff may be dropped on process crash between write and enqueue] -> Mitigation: rely on history replay strategy and keep history as source of truth.
- [Dual event vocabularies (internal domain vs adapter payloads) can drift] -> Mitigation: define explicit mapping at sink boundary and add focused tests for event-name/payload routing.
- [Refactor may temporarily blur ownership across commands/service/runtime modules] -> Mitigation: enforce module responsibilities in tasks and complete one vertical path (`history_append`) first.
- [Additional sinks can increase failure surface] -> Mitigation: keep fanout best-effort and isolate sink failures from command success.

## Migration Plan

1. Introduce app-level use-case boundary and move `history_append` orchestration-adjacent behavior there.
2. Introduce runtime inbox command for turn processing and wire use-case to enqueue after persistence.
3. Introduce domain-event fanout publisher and Tauri sink adapter.
4. Update Tauri command handlers to delegate to use-cases and remove direct fanout logic.
5. Keep command names and response envelopes unchanged for frontend compatibility.
6. Add focused tests for handoff, fanout mapping, and thin-adapter delegation.

Rollback strategy:
- Revert command delegation to previous command-service flow while preserving persisted history behavior.
- Event fanout and runtime inbox wiring can be disabled behind internal composition boundaries without data migration.

## Open Questions

- Which non-Tauri sink should be implemented first (embedded server stream, logging sink, or in-process observer)?
- Should runtime handoff trigger only on user-role message events or on a configurable subset of event types?
- Should duplicate turn enqueue prevention be added now (idempotency key/correlation check) or deferred to later runtime hardening?
