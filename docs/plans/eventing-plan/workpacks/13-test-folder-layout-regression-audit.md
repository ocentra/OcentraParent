# WP13 Test Folder Layout Regression Audit

Scope: reopen the eventing plan for a fresh audit that removes the remaining
source-side test entrypoint/scaffold from `crates/ocentra-eventing/src/` and
re-proves the relocated suite already living under
`crates/ocentra-eventing/tests/`.

Source rows: fresh regression audit.

Read next:

- `../AGENTS.md`
- `../PLAN_STATE.md`
- `../NEXT_ACTIONS.md`
- `../WORKPACK_INDEX.md`
- `../CHECKLIST_INDEX.md`
- `../TEST_PROOF_EXPECTATIONS.md`
- `../../agent/SOURCE_BOUNDARY_FLOW.md` only after the crate/test boundary is known

Expected outcome:

- No eventing test modules remain under `crates/ocentra-eventing/src/`.
- The existing external harnesses under `crates/ocentra-eventing/tests/` own the eventing test suites and subfolders.
- Empty placeholder test folders are either removed or justified by an actual test file in that folder.
- Fresh proof is recorded under
  `output/eventing-plan-proof/13-test-folder-layout-regression-audit/` plus
  `test-results/eventing-test-folder-layout-regression-audit/proof.json`.
- The fresh run can explain the migration and validate the moved tests without relying on the old closure state.

Expected tests/proof:

- `cargo test -p ocentra-eventing --test unit`
- `cargo test -p ocentra-eventing --test journal_replay`
- `cargo test -p ocentra-eventing --test integration`
- `cargo test -p ocentra-eventing --test version_skew`
- `cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests`
- proof notes or blocker notes under `output/eventing-plan-proof/13-test-folder-layout-regression-audit/`

Proof artifacts:

- `output/eventing-plan-proof/13-test-folder-layout-regression-audit/00-source-snapshot.md`
- `output/eventing-plan-proof/13-test-folder-layout-regression-audit/10-validation-commands.log`
- `output/eventing-plan-proof/13-test-folder-layout-regression-audit/proof-summary.json`
- `test-results/eventing-test-folder-layout-regression-audit/proof.json`

Failure conditions:

- Do not leave any source-side eventing test module entrypoint in `src/`.
- Do not claim the fresh audit is complete without focused crate validation and
  a fresh proof root.
- Do not describe `src/tests.rs` as the remaining blocker when that file is
  already deleted in this checkout.
- Do not mark the plan stable again until the moved test layout has been re-validated and the new proof root exists.
