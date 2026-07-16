# Network Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Route Index`
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

- Network metadata evidence, DNS/domain/flow classification, network-specific correlation signals, evidence cascade routing, policy handoff shape, adapter proof gates, read-model proof, and network no-claim boundaries.
- Route/proof separation for passive capture, parsing, classification, platform gates, AI audit, performance/security, rollout, and control-catalog reference material.

## Boundary split

```text
crates/schema or the owning Rust crate owns canonical shared network shapes and generated DTOs. `packages/schema-domain` is temporary generated-validation or edge-decoder surface only where migration is still incomplete.
network-domain is package metadata/proof-consumer unless a selected public export exists.
ocentra-network-evidence owns Rust network evidence/proof helper logic.
agent-protocol, agent-core, and agent-service own protocol/runtime/service proof only when selected.
portal-domain/apps/portal own projection/UI only.
eventing-plan owns reusable local bus semantics only.
browser-plan owns exact browser URL/page evidence.
screen-plan owns screen fallback/summary evidence.
ai-plan owns AI runtime/provider execution.
policy-control-plane-plan owns policy decisions.
v0-8-enforcement-control-plan owns enforcement authority and action execution.
lan-plan owns LAN/family-hub delivery.
data-custody-storage-plan owns retention, deletion, export, and storage custody.
```

## Does Not Own

- Exact URL, exact video, private message, search text, or decrypted/private payload truth from network-only evidence.
- AI runtime/provider behavior.
- Policy authority or enforcement execution.
- Portal UI ownership beyond selected projection proof.
- LAN delivery, data custody, device trust, notification delivery, or product rollout outside selected proof roots.
- Control catalog or settings inventory as implementation proof.

## Handoff Rule

Open an adjacent plan only after the selected workpack records the exact handoff reason, owner path, expected proof, and no-claim boundary.

## No-claim Rule

Do not claim PR_READY from checklist count, shim-cleanup skeleton proof, metadata package proof, schema/unit tests, replay fixtures, catalog existence, or lab-only platform proof. Every product claim needs the selected proof root or an explicit blocker.
