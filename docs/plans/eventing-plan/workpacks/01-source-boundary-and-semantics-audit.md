# WP01 Source Boundary And Semantics Audit

Scope: establish what the reusable Rust eventing crate is and is not allowed to own before implementation.

Source rows: `05-implementation-workpacks.md` rows 1-5.

Read next:

- `../01-rust-eventing-full-scope-plan.md`
- `../source-index.md`
- `../05-implementation-workpacks.md` only for rows 1-5
- `../../agent/SOURCE_BOUNDARY_FLOW.md` only after the crate/package boundary is known

Expected outcome:

- Source audit records the Ocentra Games eventing semantics that must be preserved or intentionally rejected.
- Decision record names the reusable crate boundary, workspace location, dependency constraints, and consumer-plan boundary.
- Parent/controller, child-agent, UI, network, LAN, AI, policy, enforcement, and portal responsibilities are separated from generic local eventing responsibilities.
- UI/Vite surfaces are explicitly prohibited from publishing business events directly.

Expected tests/proof:

- `eventing.source-boundary.audit` proof note links exact source docs and rejected assumptions.
- `eventing.workspace-boundary.decision` proof names crate/package location or records that it is missing.
- `eventing.consumer-boundary.matrix` lists consumer plans and the eventing obligations each can rely on.

Failure conditions:

- Do not implement eventing while source semantics are undocumented.
- Do not claim cross-device transport, policy authority, AI authority, enforcement behavior, or portal rendering from this generic crate.
- Do not copy implementation code from historical plans; preserve behavior as requirements and tests.
