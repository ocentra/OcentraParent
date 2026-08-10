# Remote Access WP04 Pairing/Standing-Grant Lifecycle — Validation Manifest

Date: 2026-08-10
Plan: `remote-access-plan`
Workpack: `04-session-pairing-grants`
Owner boundary: `crates/remote-access-core/src/remote_access_grant/`

## Result

The narrow Rust-owned pairing and standing-access lifecycle boundary is
implemented and locally validated. It models parent confirmation, child
disclosure, paired/active/paused/stopped/reconnect states, current parent
authority rechecks, explicitly parent-approved support access, authorized
household-actor revoke/remove transitions, terminal reconnect denial, and a
redacted `ocentra-eventing` audit milestone for accepted and denied
transitions. Lifecycle fields are private and deserialization validates state
invariants. The graph state is `validation`, not `done`.

## Commands and results

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | passed |
| `cargo test -p ocentra-remote-access-core --test unit` | 24 passed, 0 failed |
| `cargo clippy -p ocentra-remote-access-core --all-targets -- -D warnings` | passed |
| `npm run lint:architecture -- --files crates/remote-access-core` | passed |
| `npm run graph:validate --silent` | passed; 703 nodes, 705 edges |
| `npm run hub:guard --silent` | passed; no findings, blockers, conflicts, or merge risks |
| `npm run precommit --silent` | passed; formatting, Enforcer gates, Rust check, and focused crate tests |

## Negative and boundary coverage

The focused tests reject wrong actor, wrong household, wrong child device,
undisclosed child pairing, stale parent authority at pair time, unauthorized
revoke/remove, support/admin hidden standing access, reconnect after revoke,
reconnect after device removal, reactivation of removed grants, and
deserialization with impossible lifecycle evidence. They accept explicitly
parent-approved support access, revocation by another authorized household
actor, and emit accepted/denied redacted audit milestones. Serialization
preserves terminal state.

## Review repair checkpoint (2026-08-10)

PR #645's initial full CI run passed all product, build, security, and
portal-to-Rust E2E jobs, but the mergeability gate correctly held six review
threads. The implementation now addresses those findings in code and tests:
pairing rechecks current parent authority; grant lifecycle fields are private
with validated deserialization; terminal transitions accept a different
currently-authorized household actor; support access requires an explicit
parent-grant flag; the canonical schema `RemoteActorRole` is consumed; and
transition reports expose a redacted eventing milestone for both outcomes.
The follow-up CI run and review-thread resolution remain open until the
updated commit is checked by GitHub.

## No-claim boundary

This manifest does not claim persistence adapters, relay/session transport,
device-trust enrollment, child/portal rendered disclosure, durable audit
storage, custody/retention, abuse controls, generated proof output, successful
follow-up CI, resolved review, or merge to `main`. Remote input/control remains
deferred in WP03.
