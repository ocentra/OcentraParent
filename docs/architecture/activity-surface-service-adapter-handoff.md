# Activity Surface Service Adapter Handoff

The C-owned Activity UI can consume service-backed data without owning product data.

Use `@ocentra-parent/agent-protocol-domain/activity-surface-adapter` to create Activity report, save/history, and tab read-model commands. Parse returned events through the same helper before rendering. The helper returns typed payloads or explicit adapter failure reasons; the UI should render those failure states instead of falling back to UI-check data.

Runtime source of truth remains:

1. Activity UI sends typed agent protocol command.
2. Rust service reads the local Activity query store or saved report store.
3. Rust service reports typed unavailable, empty, offline, or ready states.
4. Portal renders the typed result; Vite does not invent product data.
