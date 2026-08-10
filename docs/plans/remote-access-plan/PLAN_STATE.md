# Remote Access Plan State

Status: execution-grade live-view-first plan. Standing paired access is the current model; remote control is deferred to a later slice.

Research status: current access model defined. This plan still needs focused proof work against screen capture, LAN transport, portal remote routes, local service capabilities, and RustDesk comparison docs before implementation claims, but repeated permission prompts are not part of the model.

## Current ownership interpretation

```text
remote-access-plan:
  Remote live-view capability authority, standing grant semantics, pairing/revocation/remove-device lifecycle, relay session semantics, abuse controls, and proof route.

screen-plan:
  Capture primitives, protected-surface behavior, screenshot custody, local screen retention settings, and screen-specific disclosure.

lan-plan:
  Local pairing, LAN transport, local peer discovery, and LAN-only proof.

account-identity-family-plan:
  Account, household, role, session, parent actor, selected-device, and authority proof.

device-trust-bootstrap-plan:
  Parent presence proof, trusted-device bootstrap, and step-up gating for remote grants.

data-custody-storage-plan:
  Retention, export, deletion, privacy, and custody for remote artifacts or diagnostics.

portal-ux-household-surfaces-plan:
  Rendered remote state, parent/child visible status, and UI proof once remote read models exist.

eventing-plan:
  Reusable idempotency, replay, journal, request/response, and audit mechanics.
```

## Current truth

- `screen-plan` can own capture primitives, but not the remote session product.
- `lan-plan` can own local transport, but not relay-backed remote access.
- Initial pairing creates standing parent access until revoke or device removal.
- Remote access requires account/household/device authority before pairing is opened.
- Remote control is deferred; the current pass only proves live view and standing access.
- Relay availability is not permission to retain raw screen/input/child-private data.
- Support/admin remote access requires parent-visible grant and audit; no hidden support tunnel is in scope.

## Current coupling risks

```text
- Local screen proof is not remote access proof.
- LAN pairing proof is not relay-backed remote access proof.
- Relay route existence is not remote readiness.
- UI-only proof is not remote product proof.
- Live-view proof is not remote input/control proof.
- Standing access without revoke/remove-device proof is unsafe.
- Reconnect cannot resurrect revoked or removed grants.
- Relay diagnostics must not retain raw screen/input/private payloads by default.
```

## Current proof interpretation

```text
output/remote-access-plan-proof/<workpack>/ is the deterministic proof root.
Remote control WP03 is deferred and must not be consumed by current live-view readiness claims.
Runtime rows remain open until selected code, tests, negative cases, redacted diagnostics, custody notes, rollback/teardown notes, validation logs, and proof bundles exist.
```

Open gaps:

- No remote pairing/access lifecycle with standing authority, revoke, removal, and audit.
- No relay availability/fallback state machine.
- No proof matrix for live view and standing access.
- No retention/delete/export boundary for remote artifacts.
- No child-visible disclosure state proof.
- No relay abuse/load/replay/cross-household proof.

## Latest selected slice (2026-08-09)

WP01's Rust-owned view-only capability/grant/session contract was replayed on
the consolidated E: branch. The focused contract target passed 3/3 tests,
format and scoped architecture passed, and Enforcer guard passed. The tracked
manifest is `docs/proof/remote-access-plan/slice-01-capability-fabric.md`.

This is a validation slice only. It does not close the plan or claim pairing,
standing access, relay/session runtime, device-trust integration,
revoke/remove behavior, custody, portal disclosure, abuse controls, remote
control, CI, review, or main merge.

## Latest selected slice (2026-08-10)

WP04 now has a Rust-owned pairing and standing-access lifecycle boundary in
`crates/remote-access-core/src/remote_access_grant/`. The focused tests cover
parent confirmation, child disclosure, paired/active/paused/stopped/reconnect
states, wrong actor/household/device rejection, support/admin hidden-access
rejection, parent-authorized revoke/remove, terminal reconnect denial, and
serialization round-trip of terminal state.

This is still `validation`, not `done`. Persistence adapters, relay/session
integration, device-trust handoff, child/portal rendered disclosure, durable
audit storage, generated proof output, CI, review, and main merge remain open.
The durable local record is
`docs/proof/remote-access-plan/slice-04-session-pairing-grants.md`.

## HID Execution Guard

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log or explicit known blocker from the assigned implementation boundary,
  - a proof manifest under `output/remote-access-plan-proof/<workpack>/`.
- Required proof must include commands, pass/fail, negative cases, manual-required notes, redaction/custody notes, and no-control no-claim for the current live-view pass.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, revocation/remove-device, relay degraded-state, custody, abuse, and rollback/teardown proofs are present for the assigned slice. No PR-ready claim may imply control in this pass.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
