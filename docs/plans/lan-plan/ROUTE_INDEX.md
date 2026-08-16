# LAN Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Route Index`
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

- LAN local discovery, weak/strong source classification, packet/source proof boundaries, household LAN read models, selected-device route state, signed child hello/heartbeat proof boundaries, assignment/revocation/audit proof boundaries, and LAN proof gates.
- LAN no-claim boundaries for single-machine proof, physical household proof, portal projection, service/runtime proof, router/firewall proof, signed child-agent artifacts, Android/mobile proof, and relay proof.

## Boundary split

```text
Rust schema crate owns canonical LAN shapes.
Rust LAN and agent crates own business logic, read models, runtime truth, and proof truth when selected.
Historical TS package residue may still appear only as migration scaffolding when explicitly selected; it does not own LAN truth.
agent-protocol and agent-service own wire/service/read-model proof only when selected.
eventing-plan owns local event bus semantics only; it does not own LAN transport or topology proof.
account-identity-family-plan owns household, actor, role, and assignment authority.
device-trust-bootstrap-plan owns trusted-device and key/trust material.
remote-access-plan owns relay and remote transport.
apps/portal renders service-backed LAN state only; TS is presentation only.
parent/child runtime distribution plans own package/install surfaces and child-agent artifacts.
cloudflare-control-plane-plan owns backend/relay runtime when selected.
policy/enforcement plans own policy decisions and actions.
```

## Does Not Own

- Account/session authority, trusted-device bootstrap, setup journey, remote relay behavior, event bus implementation, portal shell UX, policy/enforcement outcomes, package distribution, child-agent package runtime, Cloudflare backend behavior, or data-custody policy.
- automatic closure or skipping of active workpacks `21-25` without their own
  selected route, row truth, and proof
- Broad product readiness outside the selected proof root.

## Handoff Rule

Open an adjacent plan only after the selected workpack records the exact handoff reason, owner path, expected proof, and no-claim boundary.

## No-claim Rule

Do not claim real household LAN readiness from schema proof, unit tests,
source-matrix proof, single-machine proof, B1/B2 local proof, portal
rendering, or adjacent follow-on docs. Active workpacks `21-25` still need
their own selected proof roots or explicit blockers before completion claims.
