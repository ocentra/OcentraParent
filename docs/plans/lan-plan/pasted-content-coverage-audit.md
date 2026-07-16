# Pasted Content Coverage Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `Pasted Content Coverage Audit`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This audit records the final read-through of the LAN-plan pasted content. The
source attachments were consolidated into repo-owned plan docs instead of
copied as unmanaged notes.

## Attachment Map

| Source Content                 | Source Theme                                                                                                                        | Covered By                                                                                                                                                                                                                | Coverage Notes                                                                                                                                                                 |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Pasted LAN 20-step plan        | LAN discovery, household inventory, canonical device records, signed child-agent confirmation, parent assignment, and rollout proof | [README](README.md), [20-step plan](v0-9-lan-discovery-20-step-plan.md), [implementation checklist](implementation-checklist.md), [workpacks](workpacks/), [current snapshot](current-lan-snapshot.md)                    | Covered. The original roughly 20-step idea is normalized to 20 base workpacks plus active follow-on rows `21-25`, with the current authoritative model tracked as `01-25`.     |
| Pasted LAN testing blueprint   | Fixture layout, property tests, parser tests, service integration, Playwright proof, manual proof, performance, and rollout gates   | [test blueprint](v0-9-lan-discovery-test-blueprint.md), [implementation checklist](implementation-checklist.md), [workpack 20](workpacks/20-proof-gates-fixtures-rollout.md), [current snapshot](current-lan-snapshot.md) | Covered. The implementation checklist now requires proof packs, command logs, raw evidence JSON, UI screenshots, security-negative proof, and manual physical LAN proof.       |
| Pasted LAN UI/UX requirements  | Parent-visible LAN device states, evidence-first cards, activity/network diagnostics, target binding, and setup/pairing UX          | [UI/UX guide](ui-ux-requirements-guide.md), [implementation checklist](implementation-checklist.md), [current snapshot](current-lan-snapshot.md), [source index](source-index.md)                                         | Covered. UI guidance is treated as requirement/acceptance input, not as a claim that every screen is already implemented.                                                      |
| User-provided live screenshots | Current Devices/LAN, Activity, Activity/Network, and Network policy surfaces                                                        | [current snapshot](current-lan-snapshot.md), [implementation checklist](implementation-checklist.md), proof screenshots under `output/playwright/lan-source-matrix-plan-completion/`                                      | Covered. Screenshots anchor where we are today and identify Activity/Network as the primary parent-visible diagnostic surface for LAN evidence and policy target verification. |
| Repo product docs              | Feature, expectation, platform, family setup, and product capability boundaries                                                     | [source index](source-index.md), [current snapshot](current-lan-snapshot.md), [README](README.md), owning feature docs and expectations linked from the source index                                                      | Covered. ChatGPT-originated LAN guidance is treated as a guide, then reconciled against current Ocentra Parent code, proof, docs, and no-claim boundaries.                     |

## Coverage Checklist

- [ ] The original 20-step plan is represented as 20 base workpack files plus
      active follow-on workpacks `21-25`.
- [ ] The implementation checklist tracks all 25 authoritative workpack rows in
      one table.
- [ ] The source matrix carries all 25 workpack ids through the Rust-owned
      source-matrix contract, Rust protocol/service/runtime state, and portal
      diagnostics.
- [ ] The current snapshot records where we are, where we want to be, current
      proof, current UI screenshots, and current gaps.
- [ ] The source index names owning feature docs, expectation docs, adjacent
      feature boundaries, TypeScript paths, Rust paths, portal paths, proof
      scripts, and test files.
- [ ] The UI/UX guide is treated as product requirement guidance, not as an
      already-complete implementation claim.
- [ ] Exact fixture/proof expectations are carried by the test blueprint,
      workpack 20, and the proof-pack checklist.
- [ ] Property-based test expectations for merge, evidence, parser robustness,
      events, and presence state are carried by the test blueprint and proof
      gates.
- [ ] Proof matrix coverage is represented by source-matrix rows and proof JSON,
      not only by prose acceptance.
- [ ] Playwright UI proof is required for changed service-backed surfaces, with
      contract-fixture proof before backend proof where needed.
- [ ] Scan cadence, network-change triggers, selected-interface state, and
      stale/offline transitions remain visible as open work where not yet
      implemented.
- [ ] Modular Rust ownership is recorded in the source index instead of being
      collapsed into one LAN service file.
- [ ] Android/iOS child-agent limits remain platform-specific/manual-required
      until real devices, entitlements, signing, stores, and external transport
      proof exist.
- [ ] Coverage targets for core model/security/protocol parsers are required by
      the proof pack and validation gates.
- [ ] UI states keep discovered, assigned, confirmed, trusted, ignored, revoked,
      stale, offline, LAN-seen, and agent-connected concepts separate.
- [ ] Evidence-first device cards and details must not show guessed owner or
      child identity.
- [ ] Activity/Network diagnostics are the parent-visible place for route,
      source, evidence, signed proof, parent decision, manual proof, and audit
      state.
- [ ] Policy Network target binding must come from service-backed target rows,
      not unsupported/passive/router rows.
- [ ] Weak-source fence is explicit: weak sources cannot confirm child-agent
      identity, assign a child profile, or enable control.
- [ ] Manual-required physical proof remains explicit for the second child
      device, signed hello/heartbeat, router/firewall reachability, and proof
      artifacts.

## Pasted Details Preserved As Requirements

| Requirement Detail                                | Current Location                                           | Status                                                                                                                          |
| ------------------------------------------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| One physical device becomes one canonical row     | Workpack 13, source index, current snapshot                | Partial. Local child-agent plus passive LAN merge proof exists; all-source dedupe remains open.                                 |
| Routers and unsupported devices remain visible    | UI/UX guide, implementation checklist, current snapshot    | Partial. Portal shows visible-only unsupported/router/passive rows; broader adapter proof remains open.                         |
| LAN scan discovers, child agent confirms          | README, source index, implementation checklist             | Covered as governing rule.                                                                                                      |
| Parent assignment is durable and auditable        | Workpack 19, implementation checklist, feature docs        | Partial. Command surfaces and audit rows exist; full restart/recovery proof remains open.                                       |
| Signed hello/heartbeat are strong identity        | Workpack 18, source matrix, current snapshot               | Partial/manual-required. Rows exist and remain artifact-gated; real signed artifacts are missing.                               |
| Packet-mode adapters require manual proof         | Workpacks 05-09, test blueprint, implementation checklist  | Open/manual-required or not implemented.                                                                                        |
| mDNS/SSDP are discovery hints, not identity proof | Workpacks 08-09, source matrix, coverage checklist         | Covered. They remain weak/manual-required until child-agent signing proves identity.                                            |
| Activity/Network should expose diagnostics        | UI/UX guide, current snapshot, screenshot proof            | Partial. Current source-matrix diagnostics render; broader activity/event pipeline proof remains open.                          |
| Network policy page should bind real LAN targets  | UI/UX guide, current snapshot, screenshot proof            | Partial. Current target binding screenshot exists; unsupported/passive/router rows still must stay out of controllable targets. |
| Tests and UI snapshots are both required          | Test blueprint, implementation checklist, workpack 20      | Covered. Code validation alone is not enough for UI-visible work.                                                               |
| Physical proof is separate from CI proof          | Test blueprint, implementation checklist, current snapshot | Covered. CI/local proof cannot claim production household LAN readiness.                                                        |

## Current Proof Coverage

The current generated source-matrix proof currently shows:

- 13 implemented workpacks;
- 11 partial workpacks;
- 1 manual-required workpack;
- 14 implemented source rows;
- 15 partial source rows;
- 2 manual-required source rows;
- 4 not-implemented source rows;
- weak discovery, name-only, and presence-only sources remain fenced from child
  confirmation and profile assignment.

That is useful progress, but it is not the full LAN plan complete.

## Missing Before This Pass

Before this docs pass, the LAN folder had the source plan, test blueprint,
UI/UX guide, README, and workpacks, but it did not have browser-plan-level
execution docs:

- a fill-in implementation checklist;
- a source index tied to real feature/source ownership;
- a current snapshot with proof/gap state;
- a pasted-content coverage audit.

Those files now exist and should be used as the operating checklist for the
remaining LAN work.

## Consolidation Decisions

- The LAN base split remains 20 workpacks, with follow-on workpacks `21-25`
  active in the current authoritative `01-25` execution model.
- Source rows and workpack rows are the canonical progress ledger for LAN
  implementation status.
- UI/UX pasted guidance stays in the LAN UI/UX guide and checklist gates rather
  than being copied into every workpack.
- Product status stays in feature docs and `docs/product-capability-checklist.md`;
  this folder links to those docs and updates them only when implementation
  status or proof changes.
- Optional relay/cache, parent-owned storage, Android/iOS parity, signing, and
  stores remain no-claim/manual-required boundaries until separate proof exists.

## No Duplicate Truth Rule

Do not solve remaining LAN work by adding a second code path that disagrees with
the existing read model. The superior source of truth is:

```text
domain contracts
-> Rust protocol parity
-> Rust service/read-model state
-> portal derived UI
-> proof scripts and screenshots
```

If a UI surface needs data not present in the service read model, add the typed
contract and service read-model field first. Do not hardcode a portal-only LAN
state to make a screenshot pass.
