# WP06 Manual-Required Gap Register

| Area | State | Blocker class | Evidence | Exact next action |
| --- | --- | --- | --- | --- |
| WP01 source-of-truth closeout proof | Present | Cleared in current slice | `01-*.md` plus owner test and architecture logs in `16-validation-commands.log` | keep the route synced; do not reopen unless source-truth contract changes |
| WP02 parent authoring/preview closeout proof | Open | Real dependency blocker | `docs/features/policy-schedules-approvals.md` still marks authoring, conflict, approval, and audit UX incomplete | finish the dependency-owned portal/household surfaces before claiming WP02 |
| WP03 compiler closeout proof | Open | Avoidable local execution gap | current checkout still has deleted `03-*.md` proof artifacts and no refreshed replacement bundle | review the current compiler/core diff, then rebuild the WP03 proof bundle from the active compiler tests and source/code audit |
| WP04 delivery/ack/audit closeout proof | Missing | Avoidable local execution gap | no WP04 artifact files under `docs/proof/policy-control-plane-plan/` | assemble delivery and audit proof from owner tests and seam reads |
| WP05 ask-parent/override closeout proof | Open | Real dependency blocker | `docs/features/parent-assistant-actions.md` still marks parent confirmation, child-agent validation, and portal chat/audit incomplete | finish dependency-owned assistant and portal handoffs before claiming WP05 |
| WP07 schedule/time-budget/conflict closeout proof | Present | Cleared in current slice | `07-*.md` plus owner test and architecture logs in `16-validation-commands.log` | keep the route synced; do not reopen unless schedule/conflict contract changes |
| WP08 event-model closeout proof | Present | Cleared in current slice | `08-*.md` plus owner test logs in `16-validation-commands.log` | keep the route synced; do not reopen unless event-model contract changes |
| Architecture gate | Open outside current core slice | Avoidable local execution gap | `npm run lint:architecture -- --files packages/policy-domain` and `cargo lint-architecture crates/policy-control-core` passed, but the broader plan slice still fails in `packages/agent-protocol-domain` | clear the agent-protocol re-export debt before claiming the broader plan validation route green |
| iOS/macOS proof | Not run on this host | External platform constraint | Windows host only | schedule real iOS/macOS proof from an appropriate host only if a selected workpack requires it |
