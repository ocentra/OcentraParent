<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Agent Route
> Kind: plan route and local agent contract.
> Read when: The Plan Index or a hub assignment selects this plan.
> Stop rule: Work one workpack at a time. Do not widen into Account, Device Trust,
> Data Custody, or platform implementation unless the selected workpack names an
> owned handoff.
> Proves: only the route, boundary, and status recorded in this plan.
> Does not prove: broker implementation, authenticated IPC, platform custody,
> tests, proof, runtime reachability, PR readiness, or product completion.

<!-- /agent-capsule -->

# Protected Capability Custody Foundation Agent Route

This plan owns the neutral protected-capability custody boundary. It is the
owner of the isolated broker/client contract and the handoff into the existing
`ocentra-protected-capability-custody-core` substrate. It does not become an
Account, Device Trust, Data Custody, policy, or provider authority.

## Execution contract

- `PLAN_STATE.md` is the current truth and `WORKPACK_INDEX.md` selects the one
  workpack.
- The existing core is real source, but it is an inert, fail-closed substrate:
  its admission and platform authority are crate-private, SQLite is a checked
  replica, and no external broker, client, production caller, or expected test
  family is present.
- The next source packet may add a real isolated Windows broker process and an
  authenticated client boundary. It must not turn the current process into a
  broker or use same-process DPAPI, mutex/file-lock custody, caller attestation,
  mocks, fake success, or no-op adapters.
- The neutral protocol package is the single wire-contract owner. The graph
  records the broker/client manifests and required targets as implementation
  obligations; active workspace membership is verified from `cargo metadata
  --no-deps`, never from comments, file presence, or opaque metadata.
- The core may expose only a narrow broker-entry/facade seam. Keep
  `CustodyAdmission`, authority/platform owner traits, and platform guards
  sealed/core-private; a cross-crate caller must not implement or mint them.
- Non-Windows and unavailable platform paths remain typed manual-required or
  unavailable. A plan document or graph map never changes that state.
- Tests are written after the complete source packet. Proof, pre-commit, CI, PR,
  and merge are later gates and are not part of this docs route.

## Required reading for a selected workpack

1. `PLAN_STATE.md`
2. `NEXT_ACTIONS.md`
3. `WORKPACK_INDEX.md`
4. the selected workpack under `workpacks/`
5. `TEST_PROOF_EXPECTATIONS.md` and `PROOF_INDEX.md` before any later DONE or
   PR_READY claim

## Failure conditions

Keep the workpack open if the broker is in-process, IPC is unauthenticated,
SQLite becomes the authority rather than a checked replica, an external caller
can mint capability/authority, restart/recovery is unproved, or a test only
blesses a disconnected helper. No READY/DONE claim follows from this route.
