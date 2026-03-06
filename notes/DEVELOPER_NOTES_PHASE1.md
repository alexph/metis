# Metis Phase 1 Notes

This pass only scaffolds module boundaries and placeholders.

- `metis-contract/` contains transport-agnostic DTOs and shared error/version contracts.
- `src-tauri/src/storage` contains repository interfaces, DB model stubs, and migration bootstrap.
- `src-tauri/migrations` defines the SQLx schema baseline and metadata seed pass.
- `src-tauri/src/runtime` defines event loop boundary placeholders with Tokio channels.
- `src-tauri/src/agents` contains a Rig integration boundary with a stub client that returns `"hello"`.

Real query implementations, state transitions, and replay behavior should be added in a future implementation phase.
