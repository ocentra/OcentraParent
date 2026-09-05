# Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

Status: WP26 is open for real child/runtime ingress, W15/W18/W19 authority
composition, durable custody, and private Eventing WP10 handoff. Code, tests,
and proof remain open; no portal authority or fake transport is accepted.

This checklist tracks execution only. Proof documents intended for source
control are collected under the designated `docs/proof/lan-plan/` workpack
root. Generated rerun output stays ignored under `output/` or in CI artifacts.

- [ ] Read the plan and route docs.
- [ ] Write or update the code.
- [ ] Write or update the tests.
- [ ] Compile and validate the touched code.
- [ ] Run the tests.
- [ ] Run full crate or package validation.
- [ ] Collect tracked proof documents in the designated `docs/proof/lan-plan/`
      workpack root and keep generated output untracked.
- [ ] Record the exact tracked proof location outside the plan folder.
- [ ] Prepare PR-ready notes.

## WP26 Signed Child Beacon Ingress And Household Mesh Authority Handoff

- [ ] Implement the real shipped child/runtime peer ingress; a fixture, manual
      observation command, fake socket, or synthetic receiver does not count.
- [ ] Compose W15 canonical household custody and restart state, W18 signed
      hello/heartbeat trust and transport authority, and W19 route, lease,
      assignment, revocation, and audit authority.
- [ ] Persist accepted/rejected message identity, nonce/replay state, event or
      message reference, route/device reference, and idempotency state atomically.
- [ ] Add real organized Rust tests for ingress, restart recovery, duplicate and
      replay rejection, stale/offline transitions, revoked state, wrong family,
      wrong target, wrong device/household, and provider-policy denial.
- [ ] Prove the private typed Eventing WP10 authorization handoff occurs only
      after LAN trust, route, custody, revocation, target, and provider-policy
      checks pass; portal/UI remains projection-only.
- [ ] Record command logs and negative-case proof under
      `output/lan-plan-proof/26-signed-child-beacon-ingress-and-household-mesh-authority-handoff/`.
