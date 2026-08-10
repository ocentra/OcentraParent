# Remote Access Next Actions

## Scope and ownership

- Plan owner: `remote-access-plan/AGENTS.md` with security/product coordination and platform runtime dependencies.
- Ownership boundary: remote live-view capability grants, standing access, pairing, relay sessions, revoke/remove-device lifecycle, abuse controls, and proof route.
- Scope boundary: define runtime proof and threat-model gates before launching implementation.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
- Remote input/control remains deferred in WP03 and must not be claimed from current live-view work.

## Decision routes and failure conditions

- Decision path:
  - If remote capability grant model is unresolved -> stay in execution lane.
  - If screen/view versus deferred control separation is undefined -> split lanes and delay control claims.
  - If abuse/threat proof matrix is incomplete -> block rollout claims.
  - If pairing/standing access/revoke/remove-device lifecycle is incomplete -> block live-view readiness.
  - If custody/retention for remote artifacts is unresolved -> block privacy safety claims.
- Failure modes:
  - Undefined abuse/safe-off states for deferred remote-control attempts.
  - Missing pairing, disclosure, or revoke/remove-device contract for parent/child participants.
  - No explicit degraded/unavailable behavior under relay/session failure.
  - Support/admin access path without parent-visible grant and audit.
  - Relay diagnostics retaining raw screen/input/child-private payloads by default.

## Ordered current-pass queue

```text
WP01 capability fabric -> WP04 pairing/standing grants -> WP02 live-screen relay -> WP05 relay security/abuse -> WP06 rollout gate
```

`WP03 remote input/control` stays deferred unless a future assignment explicitly opens the control slice.

## Actioned completion tracker

- [x] Define the Rust-owned pairing/standing-grant lifecycle boundary and focused negative tests; persistence and runtime integration remain open.
- [ ] Separate live screen view from deferred remote input/control.
- [ ] Define relay fallback, unavailable, and degraded states.
- [ ] Define pairing/disclosure and child-visible state expectations.
- [ ] Define revoke/remove-device/reconnect/crash-recovery semantics.
- [ ] Define retention/export/delete boundary for remote artifacts and diagnostics.
- [ ] Define abuse/security proof matrix and route sync.

## Latest code/test checkpoint (2026-08-09)

WP01's narrow Rust contract is locally validated and recorded in
`docs/proof/remote-access-plan/slice-01-capability-fabric.md`. Continue with
the runtime-owned gaps in the ordered queue: pairing/standing grants (WP04),
live relay/session behavior (WP02), relay abuse controls (WP05), then rollout
proof (WP06). Keep remote input/control deferred.

## Latest code/test checkpoint (2026-08-10)

WP04's narrow lifecycle slice is locally validated in
`crates/remote-access-core/src/remote_access_grant/` with its focused unit
tests. The follow-up repair also covers current authority at pair,
activation, and reconnect time; explicit support parent-grant visibility;
canonical actor and route typing; immutable lifecycle fields with validated
deserialization; early-terminal round trips; authorized household-actor
terminal transitions; per-attempt audit idempotency; and redacted
accepted/denied audit milestones. Continue with fresh follow-up CI/review,
persistence/adapter ownership, relay/session integration, device-trust
handoff, child/portal disclosure, audit custody, and generated proof before
treating the workpack as complete.
