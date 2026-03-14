## Purpose
Define adapter boundaries so transport handlers remain thin while shared application use-cases own behavior and orchestration.

## Requirements

### Requirement: Transport command handlers SHALL be thin adapters
Transport-specific command handlers SHALL only perform request/response mapping and delegate execution behavior to shared application use-cases.

#### Scenario: Tauri command delegates to shared use-case
- **WHEN** a Tauri command endpoint receives a valid request
- **THEN** the handler delegates execution to a transport-agnostic application use-case and returns mapped response output

### Requirement: Shared use-cases SHALL be transport-agnostic
Application use-cases that implement orchestration behavior SHALL NOT depend on Tauri-specific request, response, or emitter types.

#### Scenario: Same use-case can be called by non-Tauri adapter
- **WHEN** a non-Tauri adapter invokes the same operation
- **THEN** the adapter can call the shared use-case without importing Tauri-only types

### Requirement: Existing desktop command contract SHALL remain stable
The refactor SHALL preserve existing desktop command names and `CommandResponse` envelope semantics for compatibility.

#### Scenario: Existing frontend invoke command names continue to work
- **WHEN** the frontend invokes existing desktop command identifiers
- **THEN** the commands resolve and return the same `status: ok` or `status: err` response envelope shape
