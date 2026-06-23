<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Portal UX Household Surfaces Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects `docs/plans/portal-ux-household-surfaces-plan`.

## High-density execution contract

Task: work only the assignment slice for this plan.
Context: `PLAN_STATE.md` is current state; `WORKPACK_INDEX.md` chooses one workpack; `TEST_PROOF_EXPECTATIONS.md` defines required local tests/proof.
Scope rule: one plan, one workpack, exact checklist rows. Sibling plans, full checklists, source inventories, and checkpoints are closed unless named by the selected route.
Implementation rule: code may move only after route, workpack, expected tests, and proof location are identified.
Test rule: expected tests are obligations, not suggestions. If the test crate/folder does not exist yet, record the missing location and keep the row open.
Proof rule: proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
Authoring rule: this plan describes outcomes, boundaries, expected tests, proof, and failure conditions; it must not prescribe implementation code except for minimal public contract or artifact-shape examples.
Failure condition: no DONE/PR_READY when tests are happy-path only, proof is missing, product status moved without evidence, or validation scope is not listed.
Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Ownership, Import, And Boundary Contract

This plan owns parent-facing portal UX presentation, route composition, read-model projection, degraded/empty/error/manual-required display, accessibility/responsive behavior, screenshot/manual-review proof, and no-fake-data boundaries. It does not own capture, policy source truth, enforcement authority, AI runtime, LAN transport, device trust, data custody, billing, setup/install, child runtime, or domain service truth.

Module roles:

```text
apps/portal: React/Tauri parent portal runtime, route composition, rendered UX, local interaction state, and focused UI/e2e proof.
portal-domain: public portal route, DOM, panel, presentation, read-model projection, proof-artifact, and UI contract package.
schema-domain and domain packages: canonical source shapes/read models consumed by portal projections; they own contracts, not portal presentation.
agent-protocol-domain and agent-protocol/service surfaces: service/read-model seams consumed by portal when selected.
policy-control-plane-plan: policy source truth, compiler, delivery, approval, and ask-parent semantics; portal renders selected policy states only.
setup-install-provisioning-plan, account-identity-family-plan, device-trust-bootstrap-plan, lan-plan, browser-plan, app-game-plan, network-plan, screen-plan, tracking-plan, ai-plan, payment-subscription-plan, data-custody-storage-plan, notification, and enforcement plans: sibling owners for domain truth and runtime behavior.
```

Direct imports are allowed only for explicit public helper surfaces:

```text
portal-domain public exports for route/panel/projection contracts
schema-domain and domain-package public read models for selected views
agent-protocol-domain public read models and commands when selected
apps/portal local view components for the selected route
neutral logging/evidence/protocol helpers that do not own sibling behavior
```

Forbidden direct imports and claims:

```text
sibling runtime internals imported to bypass typed read models or handoffs
portal-local replacement read models treated as product truth
fixture/demo data rendered as real service state
visual screenshot upgraded into runtime proof
portal route existence upgraded into domain readiness
happy-path UI test upgraded into product readiness
policy preview upgraded into applied policy
delivered/acknowledged upgraded into active enforcement
assistant output upgraded into parent-approved action
portal projection upgraded into capture, transport, billing, custody, or enforcement proof
```

If portal UX needs domain truth, it must consume typed read models, commands, events, evidence refs, custody labels, source labels, proof roots, and explicit no-claim boundaries from the owning plan. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read list.
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner/proof family is unclear.
5. Assigned workpack under `workpacks/`, if any.
6. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) - exact checklist section/row lookup only.
7. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) - local test/proof decision tree after the workpack is known.
8. [PROOF_INDEX.md](PROOF_INDEX.md) - only when validating proof or PR-ready claims.

## Local decision tree

- If the hub assignment names a workpack, open only that workpack, then use `TEST_PROOF_EXPECTATIONS.md` to choose expected tests/proof for that work type.
- If the assignment names a checklist row but no workpack, use `CHECKLIST_INDEX.md` to locate the row, then use `TEST_PROOF_EXPECTATIONS.md` for required tests/proof.
- If the assignment is docs/status only, use `DOC_INDEX.md` and the docs/status rows in `TEST_PROOF_EXPECTATIONS.md`; do not inspect source or sibling plans unless the row names them.
- If the assignment touches source, contracts, runtime, UI, AI, platform, security, persistence, or observability, read `../../agent/SOURCE_BOUNDARY_FLOW.md` only after the local workpack is known.
- If the assignment is PR_READY or DONE, read `TEST_PROOF_EXPECTATIONS.md`, `PROOF_INDEX.md`, `PLAN_HEALTH.md` only for broad claims, then `../../agent/PR_DONE_FLOW.md`.
- If `TEST_PROOF_EXPECTATIONS.md` says a required test/proof is missing, keep the row open and report the missing test/proof instead of claiming completion.

## Local work loop

1. Read only the route files above and the assigned workpack/checklist row.
2. Identify the intended implementation crate/package or current owning package/crate if the per-plan implementation crate is not created yet.
3. Make the narrow code/doc change.
4. Run the lightest relevant compile/lint/type/schema check for the touched area before expanding scope.
5. Add or update the tests named by `TEST_PROOF_EXPECTATIONS.md`; if the expected test folder/crate does not exist yet, record the missing location and keep the row open.
6. Run the focused tests/proof commands, then run broader validation only when `VALIDATION_FLOW.md` or PR_READY scope requires it.
7. Update workpack/checklist/proof docs with exact test names, command logs, proof artifacts, skipped checks, and remaining gaps.

## Product docs for this plan

[family-setup-device-roles.md](../../features/family-setup-device-roles.md), [policy-schedules-approvals.md](../../features/policy-schedules-approvals.md), [reports-notifications-sync.md](../../features/reports-notifications-sync.md), [parent-assistant-actions.md](../../features/parent-assistant-actions.md), [family-setup.md](../../expectations/family-setup.md), [policy.md](../../expectations/policy.md)

## Validation and proof choice

After the assigned workpack is known, use [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) first, then [TEST_PROOF_DECISION_MATRIX.md](../../agent/TEST_PROOF_DECISION_MATRIX.md) only for global risk escalation. Record the selected rows in DONE/PR_READY. Do not read unrelated proof docs, and do not close checklist rows with happy-path-only proof when auth, protocol, persistence, UI, AI, platform, security, performance, or observability risk is touched.

## Do not read by default

- `implementation-checklist.md` as a whole.
- all `workpacks/*.md`.
- `README_FULL_ORIGINAL.md`.
- `source-index.md` or pasted-content audits unless source ownership is unclear.
- sibling plan folders.
- global checkpoints unless `PROOF_INDEX.md` names them for your proof.

## Before DONE / PR_READY

Read `PLAN_HEALTH.md` if you are making a broad completion/staleness claim. Update the assigned workpack, relevant checklist rows, proof references, and feature/product docs as needed. Then follow `../../agent/PR_DONE_FLOW.md` and `../../agent/VALIDATION_FLOW.md`.
