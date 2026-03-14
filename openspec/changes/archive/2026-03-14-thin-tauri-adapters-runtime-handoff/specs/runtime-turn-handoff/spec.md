## ADDED Requirements

### Requirement: History append SHALL trigger runtime turn handoff
When a user message is appended successfully to channel history through the shared application use-case, the system SHALL enqueue a runtime turn-processing command containing at minimum channel identity, trigger history identity, and correlation identity.

#### Scenario: Successful user message append queues runtime turn
- **WHEN** a user-role history message append succeeds
- **THEN** the system enqueues a runtime turn-processing command derived from the persisted event

#### Scenario: Failed history append does not queue runtime turn
- **WHEN** a history append fails validation or persistence
- **THEN** the system does not enqueue a runtime turn-processing command

### Requirement: Runtime handoff SHALL be independent of task records
The runtime handoff path SHALL NOT require creating or updating task records as an orchestration trigger.

#### Scenario: Turn handoff occurs without task orchestration dependency
- **WHEN** a user message append triggers runtime handoff
- **THEN** the trigger condition is satisfied by history persistence success rather than task state transitions

### Requirement: Runtime handoff SHALL be best-effort after persistence
After successful history persistence, runtime handoff enqueue SHALL be treated as best-effort and SHALL NOT invalidate the original append response.

#### Scenario: Runtime enqueue failure after persistence still returns append success
- **WHEN** history persistence succeeds but runtime enqueue fails
- **THEN** the append operation remains successful and failure is handled as non-blocking runtime trigger loss
