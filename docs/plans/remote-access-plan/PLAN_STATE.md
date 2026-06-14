# Remote Access Plan State

Status: execution-grade live-view-first plan. Standing paired access is the current model; remote control is deferred to a later slice.

Research status: current access model defined. This plan still needs focused proof work against screen capture, LAN transport, portal remote routes, local service capabilities, and RustDesk comparison docs before implementation claims, but repeated permission prompts are not part of the model.

Current truth:

- `screen-plan` can own capture primitives, but not the remote session product.
- `lan-plan` can own local transport, but not relay-backed remote access.
- Initial pairing creates standing parent access until revoke or device removal.
- Remote access requires account/household/device authority before pairing is opened.
- Remote control is deferred; the current pass only proves live view and standing access.

Open gaps:

- No remote pairing/access lifecycle with standing authority, revoke, removal, and audit.
- No relay availability/fallback state machine.
- No proof matrix for live view and standing access.
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
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice. No PR-ready claim may imply control in this pass.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
