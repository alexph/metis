# Metis Channel Repository Implementation Plan (Phase 1.5)

Audience: coding agent implementing first real repository behavior.

## Goal

Implement the first end-to-end real data path from desktop command -> service -> storage for channel operations while preserving Phase 1 boundary design.

## Scope

- Implement real SQLite behavior for `ChannelRepository` methods:
  - `create`
  - `get`
  - `list`
  - `update_status`
- Reuse explicit storage mapping functions for record <-> contract conversion.
- Keep other repositories as stubs.

## Design notes

- Use SQLx queries against existing `channels` schema.
- Keep method signatures unchanged (synchronous trait methods for now).
- Bridge async DB calls using runtime blocking boundary in repository layer.
- Ensure status updates refresh `updated_at` timestamp in UTC RFC3339 format.

## Out of scope

- Async trait migration
- Transaction batching
- Pagination/filtering beyond simple ordered list
- Implementing branches/tasks/workers/history repositories

## Deliverables

1. Concrete `ChannelRepository` method implementations.
2. SQLx row mapping into `ChannelRecord` and contract types.
3. First command surface (`desktop_channels_list`, `desktop_channels_create`) capable of returning real DB-backed results.

## Acceptance criteria

- Workspace compiles successfully.
- Channels can be inserted and listed through repository/service delegation path.
- Channel status updates persist and update timestamps.
- Mapping errors continue to surface via typed error envelope.
