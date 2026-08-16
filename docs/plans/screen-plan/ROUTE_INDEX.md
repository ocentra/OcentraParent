# Screen Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Route Index`
> Kind: route map for this plan.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Choose the smallest local route for this plan.

| If the task says...          | Read                                                                               |
| ---------------------------- | ---------------------------------------------------------------------------------- |
| Start/resume this plan       | `PLAN_STATE.md` then `NEXT_ACTIONS.md` then `WORKPACK_INDEX.md`                    |
| Assigned a numbered workpack | `WORKPACK_INDEX.md` then that one workpack                                         |
| Owner/proof family unclear   | `WORKPACK_FAMILIES.md` only for selected-workpack classification                   |
| Need checklist status        | `CHECKLIST_INDEX.md`; open `implementation-checklist.md` only at named row/section |
| Need proof validation        | `PROOF_INDEX.md` and exact proof file                                              |
| Need source ownership        | `DOC_INDEX.md` then `source-index.md` if necessary                                 |
| Need original full narrative | `README_FULL_ORIGINAL.md` only after current state/indexes are insufficient        |

## Owns

```text
local screen capture evidence
capture scope and trigger contracts
platform adapter capability/degraded states
protected surface handling
temporary encrypted image queue
local deletion and retention proof
screen evidence schemas and read models
screen-specific portal/status proof
child disclosure and settings contracts
optional raw screenshot retention gates
optional live-view local/preflight gates
redacted-summary-only remote boundary
screen proof tiers and rollout status
```

## Boundary split

```text
screen-ai-pipeline-plan owns screen -> AI -> policy/action product-path proof.
ai-plan/schema-domain owns AI runtime/model/evidence-context contracts.
policy-control-plane-plan owns policy authority and parent-rule precedence.
v0-8-enforcement-control-plan owns enforcement adapter execution and rollback.
data-custody-storage-plan owns product retention/export/delete/privacy policy.
portal-ux-household-surfaces-plan owns broad portal UX readiness.
remote-access-plan owns relay-backed remote live-access/session authority.
browser/app-game/network/tracking plans own source-trigger/source-truth behavior.
agent-protocol/agent-service/agent-core own selected service/protocol/queue/read-model seams.
```

## Handoff rule

Open a sibling plan only when the selected workpack names the exact handoff, owner path, expected proof, and no-claim boundary.

## Proof-root rule

The deterministic proof root is:

```text
output/screen-plan-proof/<workpack-file-stem>/
```

Historical/current snapshot artifacts may use named subdirectories such as `output/screen-plan-proof/real-capture/...`. The selected workpack must name the accepted artifact path before a row is checked.

## No-claim rule

Do not claim screen readiness from stale checklist status, unrelated checked workpacks, mock screenshots, fixture-only proof, local capture alone, local live-view preflight alone, portal screenshots alone, or redacted summary export alone. Screen proof must preserve raw-image custody, deletion, redaction, platform/manual-required states, and explicit no-claims for AI, policy, enforcement, remote access, and product live view unless a selected proof root proves that tier.
