<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# V0.8 Enforcement Control Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects
`docs/plans/v0-8-enforcement-control-plan`.

## High-density execution contract

Task: work only the assignment slice for this plan.
Context: `PLAN_STATE.md` is current state; `WORKPACK_INDEX.md` chooses one
workpack; `WORKPACK_FAMILIES.md` resolves owner/handoff boundaries when unclear;
`TEST_PROOF_EXPECTATIONS.md` defines required local tests/proof.
Scope rule: one plan, one workpack, exact checklist rows. Sibling plans, full
checklists, source inventories, and checkpoints are closed unless named by the
selected route.
Implementation rule: code or docs may move only after route, workpack, expected
tests, proof location, and no-claim boundary are identified.
Test rule: expected tests are obligations, not suggestions. If the test
crate/folder does not exist yet, record the missing location and keep the row
open.
Proof rule: proof must contain command log, negative case, artifact path,
updated row, and skipped-risk note when applicable.
Authoring rule: this plan describes outcomes, boundaries, expected tests,
proof, and failure conditions; it must not prescribe implementation code except
for minimal public contract or artifact-shape examples.
Failure condition: no DONE/PR_READY when tests are happy-path only, proof is
missing, product status moved without evidence, or validation scope is not
listed.

## Hard ownership doctrine

This plan is the enforcement control plane. It is the policy-to-action
execution boundary, not a generic feature folder.

- `schema-domain` owns canonical shared enforcement contracts whenever action,
  audit, capability, reason, or read-model shapes cross package, crate,
  protocol, or plan boundaries.
- `policy-control-plane-plan` owns policy source truth, schedule/budget rules,
  ask-parent and override authority, and parent authorization before an
  enforcement handoff exists.
- `v0-8-enforcement-control-plan` owns the transition from deterministic policy
  decision refs to adapter capability, execution states, rollback/recovery,
  audit, and parent/child visible control state.
- `enforcement-domain` is a helper, proof, and read-model consumer surface. It
  must not silently replace `schema-domain` as the canonical public schema
  owner.
- `agent-protocol` and `agent-protocol-domain` own protocol parity and
  transport/read-model seams only.
- `app-game`, `browser`, `network`, `screen`, `tracking`, and AI/evidence plans
  own their source evidence and platform facts. `portal` owns rendered
  presentation and typed user intent surfaces only.

## Hard no-claim gate

No AI result, portal click, screen result, browser observation, app/game
session, network/domain observation, or tracking signal can become enforcement
unless it passes through:

```text
policy decision refs
-> actor / device / household authority
-> target and evidence refs
-> adapter capability and platform state
-> observe-only | dry-run | report-only | manual-required | dispatch-ready | rejected
-> execution result | no-op | mismatch | unavailable
-> rollback / recovery / expiry / override
-> audit / journal
-> parent-visible and child-visible state
```

If any step is missing, the state remains manual-required, dry-run, report-only,
or rejected. It is not enforcement.

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read
   list.
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner or handoff
   boundaries are unclear.
5. Assigned workpack under `workpacks/`, if any.
6. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) - exact checklist section/row lookup
   only.
7. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) - local test/proof
   decision tree after the workpack is known.
8. [PROOF_INDEX.md](PROOF_INDEX.md) - only when validating proof or PR-ready
   claims.

## Local decision tree

- If the hub assignment names a workpack, open only that workpack, then use
  `TEST_PROOF_EXPECTATIONS.md` to choose expected tests/proof for that work
  type.
- If the assignment names a checklist row but no workpack, use
  `CHECKLIST_INDEX.md` to locate the row, then use
  `TEST_PROOF_EXPECTATIONS.md` for required tests/proof.
- If the assignment is docs/status only, use `ROUTE_INDEX.md`, then
  `DOC_INDEX.md` and the docs/status rows in
  `TEST_PROOF_EXPECTATIONS.md`; do not inspect source or sibling plans unless
  the row names them.
- If ownership, protocol, or cross-plan handoff is unclear, use
  `WORKPACK_FAMILIES.md` before opening source or sibling plans.
- If the assignment touches source, contracts, runtime, UI, AI, platform,
  security, persistence, or observability, read
  `../../agent/SOURCE_BOUNDARY_FLOW.md` only after the local workpack is known.
- If the assignment is PR_READY or DONE, read
  `TEST_PROOF_EXPECTATIONS.md`, `PROOF_INDEX.md`, `PLAN_HEALTH.md` only for
  broad claims, then `../../agent/PR_DONE_FLOW.md`.
- If `TEST_PROOF_EXPECTATIONS.md` says a required test/proof is missing, keep
  the row open and report the missing test/proof instead of claiming completion.

## Local work loop

1. Read only the route files above and the assigned workpack/checklist row.
2. Identify the intended implementation crate/package or current owning
   package/crate if the per-plan implementation crate is not created yet.
3. Make the narrow code/doc change.
4. Run the lightest relevant compile/lint/type/schema check for the touched
   area before expanding scope.
5. Add or update the tests named by `TEST_PROOF_EXPECTATIONS.md`; if the
   expected test folder/crate does not exist yet, record the missing location
   and keep the row open.
6. Run the focused tests/proof commands, then run broader validation only when
   `VALIDATION_FLOW.md` or PR_READY scope requires it.
7. Update workpack/checklist/proof docs with exact test names, command logs,
   proof artifacts, skipped checks, and remaining gaps.

## Product docs for this plan

[enforcement-integrity-tamper.md](../../features/enforcement-integrity-tamper.md),
[browser-web-control.md](../../features/browser-web-control.md),
[app-game-control.md](../../features/app-game-control.md),
[policy-schedules-approvals.md](../../features/policy-schedules-approvals.md),
[enforcement.md](../../expectations/enforcement.md)

## Validation and proof choice

After the assigned workpack is known, use
[TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) first, then
[TEST_PROOF_DECISION_MATRIX.md](../../agent/TEST_PROOF_DECISION_MATRIX.md) only
for global risk escalation. Record the selected rows in DONE/PR_READY. Do not
read unrelated proof docs, and do not close checklist rows with happy-path-only
proof when auth, protocol, persistence, UI, AI, platform, security,
performance, or observability risk is touched.

## Do not read by default

- `implementation-checklist.md` as a whole.
- all `workpacks/*.md`.
- `README_FULL_ORIGINAL.md`.
- `source-index.md` or pasted-content audits unless source ownership is unclear.
- sibling plan folders.
- global checkpoints unless `PROOF_INDEX.md` names them for your proof.

## Before DONE / PR_READY

Read `PLAN_HEALTH.md` if you are making a broad completion/staleness claim.
Update the assigned workpack, relevant checklist rows, proof references, and
feature/product docs as needed. Then follow `../../agent/PR_DONE_FLOW.md` and
`../../agent/VALIDATION_FLOW.md`.
