# Metis Mapping Layer Plan (Phase 1.1)

Audience: coding agent implementing mapping boundaries for storage scaffolding.

## Goal

Define and implement explicit conversions between storage records and shared contract types so persistence models stay internal and adapters operate on transport-agnostic DTOs.

This plan is retroactive to capture the mapping pass completed after Phase 1 scaffolding.

## Scope

Implement mapping functions for these entities:

- `metadata`
- `channels`
- `branches`
- `tasks`
- `workers`
- `history`

Include both directions where applicable:

- DB record -> contract DTO
- contract DTO -> DB record

## Mapping conventions

- Keep mappings in `src-tauri/src/storage/mappings.rs`.
- Persistence record structs remain in `src-tauri/src/storage/models`.
- Contract types remain in `metis-contract`.
- Return typed mapping errors when persisted enum/string values are invalid.
- Do not add transport-specific fields to contracts.

## Enum/string translation requirements

### Channels

- `source_type` string <-> `ChannelSourceType`
- `status` string <-> `ChannelStatus`

### Tasks

- `state` string <-> `TaskState`

### Workers

- `state` string <-> `WorkerState`

### History

- optional `role` string <-> optional `HistoryRole`

## Error model

- Add a storage mapping error variant in storage error types.
- Ensure this error can be converted to shared `ErrorEnvelope` fields:
  - `code`
  - `message`
  - optional `details`

## Out of scope

- Repository query implementation
- SQL optimization or caching
- Runtime replay behavior
- Additional schema changes

## Deliverables

1. Concrete mapping functions for all Phase 1 entities.
2. Bidirectional conversion for contract <-> record where needed for repository writes.
3. Typed mapping error for invalid enum/string values.
4. Build passes with mapping signatures ready for repository integration.

## Acceptance criteria

- No storage model types leak outside storage boundaries.
- All enum-backed fields are parsed/serialized explicitly.
- Invalid persisted values fail with typed mapping errors.
- Workspace compiles successfully.
