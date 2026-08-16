<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `AGENTS.md`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX or FEATURE_ROUTE_INDEX selects the plan.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_STATE.md, ROUTE_INDEX.md, WORKPACK_INDEX.md, PLAN_INDEX.md, and FEATURE_ROUTE_INDEX.md to stay aligned.

<!-- /agent-capsule -->

# Data Custody Storage Plan Agent Route

Use this plan for data custody guarantees, encrypted storage, evidence retention, export/import/restore, sync, deletion/tombstones, no-stolen-data boundaries, cloud/relay custody, report/query custody, and parent storage settings/apply flow.

## High-Density Execution Contract

Task: work only the assigned slice for this plan.
Context: [PLAN_STATE.md](PLAN_STATE.md) is current state; [NEXT_ACTIONS.md](NEXT_ACTIONS.md) is the resume queue; [WORKPACK_INDEX.md](WORKPACK_INDEX.md) chooses one workpack; [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) defines required proof.
Scope rule: one plan, one workpack, exact proof rows. Adjacent plans are closed until the selected workpack names a handoff.
Execution rule: architecture decisions live in [DECISIONS.md](DECISIONS.md), [DATA_CLASSIFICATION.md](DATA_CLASSIFICATION.md), [KEY_CUSTODY_MODEL.md](KEY_CUSTODY_MODEL.md), [PARENT_STORAGE_PROVIDER_MATRIX.md](PARENT_STORAGE_PROVIDER_MATRIX.md), [BUNDLE_PROTOCOL.md](BUNDLE_PROTOCOL.md), [EVENT_MODEL.md](EVENT_MODEL.md), [UI_EXPECTATIONS.md](UI_EXPECTATIONS.md), [PLATFORM_KEY_CUSTODY_MATRIX.md](PLATFORM_KEY_CUSTODY_MATRIX.md), [PARENT_SAVE_RETRIEVE_APPLY_FLOW.md](PARENT_SAVE_RETRIEVE_APPLY_FLOW.md), and [RESEARCH_AND_UI_GUIDANCE.md](RESEARCH_AND_UI_GUIDANCE.md). Workpacks must reference those decisions instead of re-litigating them.
Implementation rule: docs define outcome, boundary, shape, validation, and proof. They do not prescribe implementation code.
Proof rule: proof must include command/log evidence, negative cases, artifact paths, updated rows, and skipped-risk notes when applicable.
Failure condition: no DONE/PR_READY when expected proof is missing, only happy-path evidence exists, or this plan is used to claim adjacent implementation completion.

## Ownership, Import, And Boundary Contract

This plan owns custody policy, custody proof, retention/export/sync/restore rules, and parent-visible custody state. It does not own every runtime that stores, syncs, renders, or transports data.

Module roles:

```text
crates/schema: canonical shared custody, parent-owned sync/export, bundle, restore, report/query, assistant-citation, provider-state, retention, tombstone, and parent-storage-setting shapes when those shapes cross package, crate, app, or plan boundaries.
storage-custody-core: Rust generic custody/delete/export decision logic and custody action-plan events.
ocentra-evidence: evidence references, evidence identity, and evidence custody ref semantics.
ocentra-eventing: event journal/replay/idempotency spine. This plan consumes eventing contracts; it does not re-own the bus implementation.
production-domain: legacy package identity unless a selected public export is named. Current parent-owned sync/export contract proof is routed through the Rust owner and generated TS edge surfaces.
portal-domain and apps/portal: parent-visible custody projection, settings, preview, confirmation, and status UI only.
account-identity-family-plan: actor, household, role, guardian/admin/support authority.
device-trust-bootstrap-plan: device trust material and trusted-device key state.
cloudflare-control-plane-plan, payment-subscription-plan, setup-install-provisioning-plan, remote-access-plan, LAN, notification, AI, and report producers: sibling producers or consumers that must use custody handoffs rather than re-owning custody truth.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public helper surfaces:

```text
canonical crates/schema custody/export/sync/restore/report/query/assistant-citation shapes
storage-custody-core public custody decision/event helpers when Rust custody proof is selected
ocentra-evidence public evidence reference types
ocentra-eventing public event/journal/idempotency primitives when event proof is selected
selected public package exports from producer/consumer domains when the workpack names that handoff
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports and claims:

```text
portal, report, AI, payment, Cloudflare, remote, LAN, setup, account, or device-trust runtime internals imported to bypass custody handoffs
private source files from another plan's owning package/crate used as custody source of truth
contract/schema proof upgraded into runtime custody proof
sync manifest proof upgraded into provider OAuth/upload/delete runtime
export proof upgraded into restore/apply proof
provider status proof upgraded into readable payload or key-access proof
delete proof upgraded into tombstone propagation, idempotency, or offline replay proof without selected proof
report/query proof upgraded into assistant-safe citation proof without source/ref/redaction proof
parent storage settings UI upgraded into applied custody state without confirmation and proof
automatic Ocentra-hosted fallback storage implied without explicit product decision and proof
```

If custody work needs eventing, account, device trust, portal, Cloudflare, payment, setup, remote, LAN, notification, report, or AI behavior, it must use typed evidence refs, commands, events, requests, read models, artifact manifests, proof roots, and explicit handoffs. If a shape is used by multiple feature owners, place or consume it through `crates/schema` or another neutral Rust shared boundary. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Local Decision Tree

- If the assignment names a workpack, open only that workpack.
- If the assignment names a checklist row but no workpack, use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md), then choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
- If the assignment changes product status, read [DOC_INDEX.md](DOC_INDEX.md), [PLAN_STATE.md](PLAN_STATE.md), and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows.
- If the assignment touches adjacent implementation ownership, open only the adjacent plan named by the selected workpack.
- If the selected workpack owner/proof family is unclear, read [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only for classification.
- If the assignment is DONE/PR_READY, read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md), [PROOF_INDEX.md](PROOF_INDEX.md), [PLAN_HEALTH.md](PLAN_HEALTH.md), then [../../agent/PR_DONE_FLOW.md](../../agent/PR_DONE_FLOW.md).

## Required Read Order

1. [PLAN_STATE.md](PLAN_STATE.md)
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner/proof family is unclear
5. One assigned workpack under workpacks/
6. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)
7. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows/artifacts

## Product Sources

- Architecture docs: DECISIONS.md, DATA_CLASSIFICATION.md, KEY_CUSTODY_MODEL.md, PARENT_STORAGE_PROVIDER_MATRIX.md, BUNDLE_PROTOCOL.md, EVENT_MODEL.md, UI_EXPECTATIONS.md, PLATFORM_KEY_CUSTODY_MATRIX.md, PARENT_SAVE_RETRIEVE_APPLY_FLOW.md, RESEARCH_AND_UI_GUIDANCE.md
- Feature docs: evidence-store-query.md, reports-notifications-sync.md
- Expectation docs: data-custody.md, evidence-storage.md, sync-export.md, cloud.md
- Adjacent plans: eventing-plan, portal-ux-household-surfaces-plan, parent-client-runtime-distribution-plan, account-identity-family-plan, device-trust-bootstrap-plan, payment-subscription-plan
