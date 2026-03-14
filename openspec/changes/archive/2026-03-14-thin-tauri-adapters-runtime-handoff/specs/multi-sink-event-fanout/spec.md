## ADDED Requirements

### Requirement: Domain events SHALL support multi-sink publication
The system SHALL publish internal domain events through a fanout publisher that can target multiple sinks in one publish action.

#### Scenario: One event publishes to all registered sinks
- **WHEN** a domain event is published
- **THEN** each registered sink receives the event in the same publish cycle

### Requirement: Tauri and non-Tauri channels SHALL be supported as sinks
The fanout publisher SHALL support a Tauri event sink and SHALL allow additional non-Tauri sink implementations without changing use-case logic.

#### Scenario: Adding new sink does not change use-case code path
- **WHEN** a new sink implementation is registered
- **THEN** existing use-cases publish once and the new sink receives events without use-case modifications

### Requirement: Sink failures SHALL be isolated from command success
Failure in one sink SHALL NOT prevent successful publication attempts to other sinks and SHALL NOT invalidate already successful state mutations.

#### Scenario: Tauri sink failure does not block other sinks
- **WHEN** Tauri sink publish fails for an emitted domain event
- **THEN** non-failing sinks still receive the event and upstream state mutation remains successful
