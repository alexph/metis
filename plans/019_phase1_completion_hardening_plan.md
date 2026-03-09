# Metis Phase 1 Completion Hardening Plan (Phase 2.7)

Audience: coding agent finalizing Phase 1 readiness before agent integration work.

## Goal

Stabilize contracts, tests, and developer ergonomics so non-agent product loop is ready for iterative UI work and future agent-specific treatment.

## Scope

- Add frontend tests for invoke client and query mapping behavior.
- Add backend command tests for newly exposed mutate commands.
- Review command/request/response naming for consistency.
- Confirm developer notes and plans reflect implemented architecture.

## Hardening checklist

- command contract consistency check (Rust <-> frontend)
- envelope code audit for stable UI handling
- test coverage for critical write paths and adapter routing
- docs/notes update for onboarding next implementation pass

## Out of scope

- Agent runtime integration and tool execution UX
- Production observability dashboards
- Multi-backend (Turso) adapter implementation

## Deliverables

1. Added/updated tests across backend and frontend boundaries.
2. Contract consistency pass and cleanup commits.
3. Updated phase summary notes indicating Phase 1 non-agent loop complete.

## Acceptance criteria

- Core non-agent command loop is test-backed and stable.
- Frontend can send/receive data through typed boundaries reliably.
- Remaining major work is primarily agent-specific behavior.
