# Remote Access WP04 Pairing/Standing-Grant Lifecycle — Validation Manifest

Date: 2026-08-10
Plan: `remote-access-plan`
Workpack: `04-session-pairing-grants`
Owner boundary: `crates/remote-access-core/src/remote_access_grant/`

## Result

The narrow Rust-owned pairing and standing-access lifecycle boundary is
implemented and locally validated. It models parent confirmation, child
disclosure, paired/active/paused/stopped/reconnect states, route binding,
current parent-authority rechecks on access-starting transitions, explicitly
parent-approved support access, authorized household-actor revoke/remove
transitions, terminal reconnect denial, and a redacted `ocentra-eventing`
audit milestone for accepted and denied transitions. Lifecycle fields are
private and deserialization validates state invariants, including early
terminal snapshots. The graph state is `validation`, not `done`.

## Commands and results

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | passed |
| `cargo test -p ocentra-remote-access-core --test unit` | 44 passed, 0 failed |
| `cargo test -p ocentra-schema --test contract remote_capability_fabric` | 7 passed, 0 failed |
| `cargo clippy -p ocentra-remote-access-core --all-targets -- -D warnings` | passed |
| `npm run lint:architecture -- --files crates/remote-access-core crates/schema` | passed |
| `npm run graph:validate --silent` | passed; 703 nodes, 705 edges |
| `npm run hub:guard --silent` | passed; no findings, blockers, conflicts, or merge risks |
| `npm run precommit --silent` | passed; formatting, Enforcer gates, Rust check, and focused crate tests |

## Negative and boundary coverage

The focused tests reject wrong actor, wrong household, wrong child device,
wrong route, undisclosed child pairing, stale parent authority at pair time,
stale authority at activation/reconnect, cross-actor non-terminal transitions,
unauthorized revoke/remove, support/admin hidden standing access, reconnect
after revoke, reconnect after device removal, activation bypass from pending
reconnect, reactivation of removed grants, and deserialization with impossible
lifecycle evidence. They accept explicitly parent-approved support access,
canonical parent-granted support live view, revocation by another authorized
household actor, and emit accepted/denied redacted audit milestones.
Serialization preserves both early
and late terminal states. Attempt references make cycle audit ids distinct and
replay the original accepted or denied report across retries and restore.
Typed device-trust handoff gates pairing and access-starting transitions.
Explicit Denied and Failed states preserve terminal pairing outcomes.
The replay window retains all recorded identities and rejects new attempts once
full; accepted milestones reject transition/result-state mismatches; replay
denials include length-framed invalid-context identity; and system-failure
stops require an explicit recovery proof before reconnect.

## Review repair checkpoint (2026-08-10)

PR #645's initial full CI run passed all product, build, security, and
portal-to-Rust E2E jobs, but the mergeability gate correctly held eight review
threads. This repair addresses those findings in code, tests, and evidence:
accepted transition milestones now validate their resulting state; the
bounded replay history fails closed instead of evicting identities; replay
denials and audit idempotency keys use unambiguous length-framed components;
the supersession test is mapped; the unit count is current at 44; reconnect
after a system-failure stop requires typed recovery proof; and the checked-in
engineering graph is regenerated from plan sources. The earlier lifecycle,
authority, schema-v2 migration, terminal-state, and device-trust findings
remain covered by the focused suite. The follow-up CI run and review-thread
resolution remain open until the updated commit is checked by GitHub.

## No-claim boundary

This manifest does not claim persistence adapters, relay/session transport,
device-trust enrollment, child/portal rendered disclosure, durable audit
storage, custody/retention, abuse controls, generated proof output, successful
follow-up CI, resolved review, or merge to `main`. Remote input/control remains
deferred in WP03.
