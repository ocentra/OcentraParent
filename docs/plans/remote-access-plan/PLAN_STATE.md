# Remote Access Plan State

Status: first-pass plan created because remote desktop/control and remote live view were scattered across screen, LAN, architecture, and roadmap docs.

Research status: incomplete. This plan requires a full follow-up research pass against existing screen capture, LAN transport, portal remote routes, local service capabilities, RustDesk comparison docs, and Sujan's privacy/control decisions before implementation claims.

Current truth:

- `screen-plan` can own capture primitives, but not the remote session product.
- `lan-plan` can own local transport, but not relay-backed remote access.
- Remote input/control is higher risk than remote viewing and must have separate authority, proof, and failure states.
- Remote access requires account/household/device authority before any session is opened.

Open gaps:

- No remote capability grant model.
- No remote session lifecycle with consent, expiry, revocation, and audit.
- No relay availability/fallback state machine.
- No proof matrix for remote viewing versus remote control.
- No retention/delete/export boundary for remote artifacts.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/remote-access-plan/.
- Required proof manifest names:
  - docs/proof/remote-access-plan/slice-01-\*.md
  - docs/proof/remote-access-plan/slice-02-\*.md
  - docs/proof/remote-access-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
