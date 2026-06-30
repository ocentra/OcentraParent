<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# LAN Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects
`docs/plans/lan-plan`.

## High-density execution contract

Task: work only the assignment slice for this plan.
Context: `PLAN_STATE.md` is current state; `WORKPACK_INDEX.md` chooses one
workpack; `TEST_PROOF_EXPECTATIONS.md` defines required local tests/proof.
Scope rule: one plan, one workpack, exact checklist rows. Sibling plans, full
checklists, source inventories, and checkpoints are closed unless named by the
selected route.
Implementation rule: code may move only after route, workpack, expected tests,
and proof location are identified.
Test rule: expected tests are obligations, not suggestions. Use visible
crate/package test folders that match the real risk surface. Placeholder
directories, `.gitkeep`, and inline source-owned tests do not count as closure
when a real test surface is practical; if the expected test crate/folder does
not exist yet, record the missing location and keep the row open.
Proof rule: proof must contain command log, negative case, artifact path,
updated row, and skipped-risk note when applicable.
Authoring rule: this plan describes outcomes, boundaries, expected tests,
proof, and failure conditions; it must not prescribe implementation code except
for minimal public contract or artifact-shape examples.
Failure condition: no DONE/PR_READY when tests are happy-path only, proof is
missing, product status moved without evidence, or validation scope is not
listed.

## Ownership, Import, And Boundary Contract

This plan owns LAN discovery, LAN pairing route contracts, household LAN
evidence/readiness state, weak/strong LAN source classification, LAN read
models, and LAN proof gates. It does not own every setup, portal, eventing,
remote, device-trust, package, policy, or child-runtime behavior that consumes
LAN state.

Module roles:

```text
Rust shared schema/protocol crates: canonical LAN pairing, discovery,
source-matrix, route-snapshot, signed hello, heartbeat, assignment,
revocation, audit, and read-model shapes when those shapes cross package,
crate, app, or plan boundaries. Rust is authoritative for contracts and
runtime truth.

lan-core, agent-protocol, agent-service, and parent-runtime-core: own LAN
business logic, protocol/service behavior, read models, route snapshots, and
proof when the selected workpack names those surfaces.

apps/portal and thin TS bridge/presentation code: projection and UI only. TS
consumes Rust-backed snapshots, generated DTOs, and host-bridge events and
must not invent LAN truth or own contracts.

eventing-plan: local event bus semantics only. Eventing does not own LAN
transport, discovery, route authority, or physical topology proof.

device-trust-bootstrap-plan and account-identity-family-plan:
trusted-device, household, actor, role, and authority owners.

remote-access-plan: relay and remote-access transport owner.

parent/child runtime distribution plans: package, installer, signed child
package, and child-agent distribution owners.

cloudflare-control-plane-plan: cloud/backend runtime owner when relay/backend
storage is selected.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit
public helper surfaces:

```text
Rust-owned schema/protocol/read-model shapes or generated bridge DTOs derived
from them
public Rust-backed command/event/route-snapshot surfaces named by the
selected workpack
portal presentation helpers only when the selected workpack names UI proof
neutral event/evidence/logging/protocol primitives
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports and claims:

```text
portal, setup, account, device-trust, remote, eventing, Cloudflare, policy,
enforcement, package distribution, or child-runtime internals imported to
bypass typed LAN handoffs
legacy TypeScript package proofs or contract catalogs upgraded into
authoritative LAN ownership
unit tests upgraded into integration, e2e, security, physical, release, or
load proof
single-machine proof upgraded into real household LAN proof
schema/contract proof upgraded into packet/runtime proof
source-matrix proof upgraded into physical discovery proof
portal rendering proof upgraded into LAN truth proof
B1/B2 proof upgraded into signed hello, heartbeat, service/runtime, portal,
physical household, router/firewall, or relay proof
active workpacks 21-25 skipped, auto-closed, or treated as out of LAN scope
```

If LAN work needs eventing, device trust, account authority, remote relay,
portal UI, package distribution, setup, cloud, policy, enforcement, child
runtime, or data custody behavior, it must use typed evidence refs, commands,
events, requests, read models, proof roots, and explicit handoffs. If a shape
is used by multiple feature owners, place or consume it through the current
Rust shared-schema owner or another neutral Rust boundary. TS edge decoders may
exist at untrusted or presentation edges, but they must not become business
truth. Do not solve cross-plan behavior by importing another feature owner's
runtime internals.

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read
   list.
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack
   owner/proof family is unclear.
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
- If the assignment is docs/status only, use `DOC_INDEX.md` and the docs/status
  rows in `TEST_PROOF_EXPECTATIONS.md`; do not inspect source or sibling plans
  unless the row names them.
- If the assignment touches source, contracts, runtime, UI, AI, platform,
  security, persistence, or observability, read
  `../../agent/SOURCE_BOUNDARY_FLOW.md` only after the local workpack is known.
- If the assignment is PR_READY or DONE, read `TEST_PROOF_EXPECTATIONS.md`,
  `PROOF_INDEX.md`, `PLAN_HEALTH.md` only for broad claims, then
  `../../agent/PR_DONE_FLOW.md`.
- If `TEST_PROOF_EXPECTATIONS.md` says a required test/proof is missing, keep
  the row open and report the missing test/proof instead of claiming
  completion.

## Local work loop

1. Read only the route files above and the assigned workpack/checklist row.
2. Identify the intended owning Rust crate plus any thin TS presentation
   consumer if the workpack includes UI proof.
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

[family-setup-device-roles.md](../../features/family-setup-device-roles.md),
[remote-lan-mobile-platforms.md](../../features/remote-lan-mobile-platforms.md),
[lan-pairing.md](../../expectations/lan-pairing.md),
[family-setup.md](../../expectations/family-setup.md)

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
- `source-index.md` or pasted-content audits unless source ownership is
  unclear.
- sibling plan folders.
- global checkpoints unless `PROOF_INDEX.md` names them for your proof.

## Before DONE / PR_READY

Read `PLAN_HEALTH.md` if you are making a broad completion/staleness claim.
Update the assigned workpack, relevant checklist rows, proof references, and
feature/product docs as needed. Then follow `../../agent/PR_DONE_FLOW.md` and
`../../agent/VALIDATION_FLOW.md`.
