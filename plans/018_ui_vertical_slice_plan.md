# Metis UI Vertical Slice Plan (Phase 2.6)

Audience: coding agent delivering first end-to-end interactive UI slice.

## Goal

Ship a thin but complete UI flow proving invoke -> query/db sync -> rendered updates works for non-agent operations.

## Scope

Implement a vertical slice that supports:

- list channels
- create channel
- list tasks by selected channel
- enqueue task
- list history and append history event

## UX requirements

- loading states for query fetches
- actionable empty states
- mutation pending/disabled states
- visible error rendering using envelope message/code

## Data flow requirements

- use typed invoke client only (no direct invoke from components)
- use TanStack Query for async orchestration
- use TanStack DB for local entity coherence

## Out of scope

- Advanced navigation/branching UX
- Agent orchestration panels
- Final visual polish/theme system

## Deliverables

1. Working frontend screen(s) for the vertical slice.
2. Hooked query/mutation + entity sync pipeline.
3. Basic UI interactions verified against backend commands.

## Acceptance criteria

- User can create a channel and see it reflected immediately.
- User can enqueue a task and observe updated task list.
- User can append history and observe ordered history updates.
- No manual reload required for normal flow.
