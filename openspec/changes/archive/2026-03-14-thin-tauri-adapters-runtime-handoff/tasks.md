## 1. App and Runtime Boundaries

- [x] 1.1 Create an app-layer use-case module for history append + runtime trigger behavior, independent of Tauri types.
- [x] 1.2 Define internal runtime inbox command(s) for turn processing based on persisted history context.
- [x] 1.3 Wire runtime inbox sender into shared application state during bootstrap.

## 2. Runtime Turn Handoff

- [x] 2.1 Move post-append orchestration trigger logic from command handlers into app use-cases.
- [x] 2.2 Trigger runtime handoff from successful user-message history append without relying on task state changes.
- [x] 2.3 Ensure runtime handoff failures are best-effort and do not fail successful history append responses.

## 3. Event Fanout and Sink Separation

- [x] 3.1 Introduce a domain-event fanout publisher abstraction with multi-sink support.
- [x] 3.2 Implement a Tauri sink adapter that maps internal domain events to current Tauri event names/payloads.
- [x] 3.3 Add at least one non-Tauri sink boundary (stub or concrete) to validate fanout composition without changing use-cases.
- [x] 3.4 Isolate sink failures so one failing sink does not block others or invalidate state mutation success.

## 4. Thin Transport Adapters

- [x] 4.1 Refactor `src-tauri/src/commands/history.rs` to delegate through app use-cases and remove direct orchestration/event logic.
- [x] 4.2 Apply the same thin-adapter delegation pattern across other command modules as needed for consistency.
- [x] 4.3 Preserve existing desktop command identifiers and `CommandResponse` envelope compatibility.

## 5. Validation and Regression Coverage

- [x] 5.1 Add/adjust unit tests for thin-adapter delegation and stable response mapping.
- [x] 5.2 Add tests for runtime handoff behavior (append success triggers enqueue; append failure does not).
- [x] 5.3 Add tests for multi-sink fanout behavior and sink-failure isolation.
- [x] 5.4 Run backend test suite to verify no regressions in existing command contract behavior.
