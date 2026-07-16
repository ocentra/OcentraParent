# Checklist Index

Status: WP11 type-safety hardening locally proved; WP12 rollout-proof route
complete; WP13 regression audit complete; plan remains open on WP10.

This checklist tracks the current execution slice only. Historical closure notes
remain in the plan docs, but the list below is the live tracker for the recent
local proof closures and the remaining open workpacks.

- [x] Read the plan and route docs.
- [x] Write or update the code.
- [x] Write or update the tests.
- [x] Compile and validate the touched code.
- [x] Run the tests.
- [ ] Run full crate or package validation.
- [x] Collect proof artifacts in the designated local artifact path.
- [x] Record the proof location outside the plan folder.
- [ ] Prepare handoff notes.

## Route-proof reconciliation

- [x] Restore the local WP12 rollout-proof bundle under
      `docs/proof/eventing-plan/`,
      `output/eventing-plan-proof/rollout-proof/`, and
      `test-results/eventing-rollout-proof/`.
- [x] Re-run `node scripts/test/eventing-rollout-proof.mjs` and confirm the
      route docs keep WP10 open without any `PR_READY` claim.
- [x] Record the restored WP12 route-proof state in `PLAN_STATE.md`,
      `NEXT_ACTIONS.md`, `PROOF_INDEX.md`, and
      `TEST_PROOF_EXPECTATIONS.md`.
- [x] Re-run the scoped WP11 package validation:
      `npm run type-check --workspace @ocentra-parent/agent-protocol-domain`,
      focused policy-control/contract tests, and touched-file
      `npm run lint:architecture -- --files ...`.

## Fresh regression audit

- [x] Audit the migrated state: deleted `src/tests.rs`, empty `src/tests/`
      scaffold, `src/lib.rs` source-side test entrypoint, and the external
      harness files already living under `crates/ocentra-eventing/tests/`.
- [x] Remove the remaining source-side test module entrypoint from
      `crates/ocentra-eventing/src/lib.rs`.
- [x] Remove the remaining empty `crates/ocentra-eventing/src/tests/`
      scaffold from `crates/ocentra-eventing/src/`.
- [x] Re-run the focused eventing test suites and architecture lint for the moved files.
- [x] Record the fresh proof artifacts or blockers under the regression-audit
      proof root and the WP12 route docs.
