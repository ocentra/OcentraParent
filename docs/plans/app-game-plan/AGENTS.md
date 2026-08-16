<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# App + Game Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects
`docs/plans/app-game-plan`.

## High-density execution contract

Task: work only the assignment slice for this plan.
Context: `PLAN_STATE.md` is current state; `CODE_AUDIT.md` is current
code/test ownership and Phase 1 status; `WORKPACK_INDEX.md` chooses one
workpack; `TEST_PROOF_EXPECTATIONS.md` defines required local tests/proof.
Scope rule: one plan, one workpack, exact checklist rows. Sibling plans, full checklists, source inventories, and checkpoints are closed unless named by the selected route.
Implementation rule: code may move only after route, workpack, expected tests, and proof location are identified.
Test rule: expected tests are obligations, not suggestions. If the test crate/folder does not exist yet, record the missing location and keep the row open.
Proof rule: proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
Authoring rule: this plan describes outcomes, boundaries, expected tests, proof, and failure conditions; it must not prescribe implementation code except for minimal public contract or artifact-shape examples.
Failure condition: no DONE/PR_READY when tests are happy-path only, proof is missing, product status moved without evidence, or validation scope is not listed.

## Ownership, Import, And Boundary Contract

App/game is the native app and native game evidence spine. It observes and normalizes local app/game facts, writes evidence, builds read models, and emits typed handoffs. It does not own AI classification, policy authority, enforcement authority, notification delivery, portal rendering, or platform custody beyond the assigned adapter surface.

Module roles:

```text
agent-protocol and agent-core: canonical app/game contracts, Windows source
observation, journal/SQLite projection, sessionization, and evidence/read-model
boundaries.
app-game-core: Rust-owned source-freshness, policy-preview, timer-handoff,
notification-intent, and runtime-decision models.
agent-service, parent-runtime-core, and apps/portal: service composition,
parent bridge, and rendered projections/actions when selected.
platforms/android/agent: Android UsageEvents, Accessibility, delivery, receipt,
and notification runtime sources; focused App/Game Java tests are required when
the selected workpack owns executable Android behavior.
schema-domain: generated validation/decoder edge only. Removed
activity-domain, parent-domain, agent-protocol-domain, text-domain, and
app-game-domain paths are not current owners and must not be recreated.
AI plan: classifier/digest consumer only. AI consumes stored evidence or structured digests and does not scan apps, games, launchers, windows, processes, or devices directly.
platform adapters: source observers for assigned OS/platform proof only. They produce typed observations and capability states; they do not decide policy or enforcement.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public helper surfaces:

```text
Rust-owned canonical app/game/evidence/policy-reference/protocol/capability/logging shapes plus generated DTOs or temporary edge decoders
neutral event/evidence/logging/protocol primitives
approved Rust runtime/parity crates when the selected workpack names Rust proof
generated schema-domain decoders only at TypeScript edges
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports:

```text
sibling feature owner runtime behavior from AI, policy, enforcement, notification, portal, child-runtime, LAN, remote, setup, payment, or data-custody plans
private source files from another plan's owning package/crate
peer feature contracts when the shared shape should live in crates/schema or another neutral Rust-owned boundary
portal, policy, AI, or notification code that scans app/game source state instead of consuming app/game evidence/read models
policy or enforcement internals that execute app/game actions without typed app/game authority, source freshness, and adapter-readiness proof
```

If app/game needs AI, policy, enforcement, notification, portal, child-runtime, LAN, or remote behavior, it must use typed evidence, commands, events, requests, read models, and proof handoffs. If a shape is used by multiple feature owners, place or consume it through `crates/schema` or another neutral Rust-owned boundary. Use `schema-domain` only as a temporary generated-validation or edge-decoder surface while migration is still incomplete. Do not solve cross-plan behavior by importing another feature's runtime internals.

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read list.
2. [CODE_AUDIT.md](CODE_AUDIT.md) - current code/test ownership and Phase 1 gap.
3. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
4. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
5. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) - classify the assigned workpack family only when owner path is unclear.
6. Assigned workpack under `workpacks/`, if any.
7. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) - exact checklist section/row lookup only.
8. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) - local test/proof decision tree after the workpack is known.
9. [PROOF_INDEX.md](PROOF_INDEX.md) - only when validating proof or PR-ready claims.

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

[app-game-control.md](../../features/app-game-control.md), [app-install-purchase-approval.md](../../features/app-install-purchase-approval.md), [enforcement-integrity-tamper.md](../../features/enforcement-integrity-tamper.md), [app-game-evidence.md](../../expectations/app-game-evidence.md), [enforcement.md](../../expectations/enforcement.md)

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

Read `PLAN_HEALTH.md` if you are making a broad completion/staleness claim. Update the assigned workpack, relevant checklist rows, proof references, and
feature/product docs as needed. Then follow `../../agent/PR_DONE_FLOW.md` and
`../../agent/VALIDATION_FLOW.md`.
