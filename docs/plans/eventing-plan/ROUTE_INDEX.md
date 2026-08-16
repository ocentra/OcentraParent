# Reusable Rust Eventing Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Reusable Rust Eventing Plan Route Index`
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

- Reusable local Rust event bus semantics.
- Typed event envelopes, typed ids, event contracts, version/compatibility checks, idempotency, aggregate ordering, TTL, retry, dead-letter, request/response, journal/replay, topology, contract registry, and local runtime lifecycle proof.
- Route/proof gates for consumer handoffs where the selected eventing workpack names the handoff.

## Boundary split

```text
LAN and remote-access plans own transport, mesh, relay, pairing, cross-device delivery, and remote access behavior.
network, browser, app-game, screen, tracking, AI, policy, enforcement, portal, data-custody, setup, payment, and account plans own their consumer behavior.
agent-protocol and agent-service own protocol/service delivery proof for their surfaces.
data-custody-storage-plan owns retention, deletion, export, custody, and storage policy.
schema-domain owns neutral shared event/contract shapes when shapes cross package or plan boundaries.
event-domain is package-boundary metadata only unless a selected workpack names an explicit public surface.
```

## Does Not Own

- Cross-device transport or remote relay delivery.
- Product UI rendering.
- AI classification, policy decisions, enforcement actions, adapter side effects, account authority, payment semantics, setup journey, data retention/deletion policy, or portal behavior.
- Production durability, fsync policy, SQLite projections, remote replication, retention/delete execution, or export/import policy.
- Consumer runtime completion without the owning consumer plan proof.

## Handoff Rule

Open an adjacent plan only after the selected workpack records the exact handoff reason, owner path, expected proof, and no-claim boundary.

## No-claim Rule

Do not claim product or transport readiness from local eventing proof. Local bus proof is not LAN/remote delivery proof, protocol shape proof is not service delivery proof, and journal/replay proof is not production custody/retention proof.
