<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: LAN implementation, packet/runtime behavior, physical household proof, portal proof, relay proof, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# LAN Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns LAN discovery/readiness proof and local-network handoff boundaries. It does not own account authority, device trust, eventing, portal UI, remote relay, package distribution, setup, policy, enforcement, or child-agent runtime behavior.

Active direction: Rust owns LAN contracts, shapes, business logic, read models, runtime truth, and proof truth. TypeScript is presentation only.

## Contract and schema boundary family

```text
Workpacks:
01 Contract Boundary And Effect Schemas (historical title; execution is Rust-owned
schema/bridge boundary work, not TS schema authority)

Owners:
Rust schema crate for canonical LAN contract/read-model/proof shapes
legacy TS proof-consumer residue only when explicitly selected; it does not own LAN truth

Rule:
Schema/contract proof is not packet/runtime proof, physical LAN proof, portal proof, or service delivery proof.
Generated UI edge types are allowed, but TS stays presentation-only and does not
become the contract authority.
```

## Evidence model and device record family

```text
Workpacks:
02 Evidence Model And Device Record
13 Merge And De-Duplication Engine
14 Explainable Classification

Owners:
Rust schema crate for canonical evidence/device/merge/classification shapes
Rust LAN crates for current runtime/read-model truth
account/device-trust handoffs only when authority/trust is selected

Rule:
Evidence/read-model proof must keep weak/manual sources visible. It cannot confirm child-agent identity, household membership, or trusted-device state without the owning trust/authority proof.
```

## Interface and neighbor discovery family

```text
Workpacks:
03 Interface Detection
04 Neighbor Table Ingestion

Owners:
selected OS/platform adapter surface when implemented
Rust schema crate for normalized LAN source/read-model shapes
legacy TS proof-consumer residue only when explicitly selected

Rule:
Interface and neighbor proof is platform-specific. Windows proof is not Linux/mac proof, and source presence is not physical household discovery.
```

## Active ARP and sweep family

```text
Workpacks:
05 Targeted ARP Checks
06 Bounded ARP Sweep

Owners:
LAN packet/runtime implementation when selected
Rust schema crate for ARP evidence/readiness shapes

Rule:
ARP proof must show bounded scope, rate limits, malformed packet handling, stale/offline behavior, and manual-required states. ARP evidence is weak until merged with stronger source/trust proof.
```

## Passive listener family

```text
Workpacks:
07 Passive Discovery Listeners

Owners:
LAN packet/runtime implementation when selected
Rust schema crate for passive-source evidence shapes

Rule:
Passive listener proof must handle malformed/oversized packets, privacy redaction, source confidence, and explicit weak-source fencing. Passive evidence is not device trust proof.
```

## mDNS, DNS-SD, SSDP, and UPnP family

```text
Workpacks:
08 mDNS And DNS-SD Discovery
09 SSDP And UPnP Discovery
17 Parent And Child mDNS Advertisements

Owners:
LAN packet/runtime implementation when selected
child runtime/distribution owners when signed child-agent advertisements are selected
Rust schema crate for advertisement/discovery shapes

Rule:
Discovery advertisements prove only visible LAN metadata unless signed child-agent proof and trust handoff exist. mDNS/SSDP proof is not household assignment or physical child-agent confirmation by itself.
```

## NetBIOS, LLMNR, reverse DNS, service probe, and vendor family

```text
Workpacks:
10 NetBIOS, LLMNR, And Reverse DNS
11 Light Service Probing
12 OUI And Vendor Lookup

Owners:
LAN source adapters when implemented
Rust schema crate for weak-source/vendor/evidence confidence shapes

Rule:
Names, service hints, and vendor/OUI evidence are weak/manual sources. They must not confirm child identity, household membership, or device trust.
```

## Household device store family

```text
Workpacks:
15 Household Device Store
19 Assignment, Revocation, And Audit

Owners:
account-identity-family-plan for household/actor authority
device-trust-bootstrap-plan for trust state
lan-plan for LAN read-model/store projection and assignment/revocation proof boundary

Rule:
Household device store proof must distinguish registry/read-model persistence from actor authority and device trust. Assignment/revocation proof needs wrong-household/device and stale/replay negatives.
```

## Read models and LAN event family

```text
Workpacks:
16 Read Models And LAN Events

Owners:
Rust agent-protocol/agent-service when service-backed read models are selected
portal plans for presentation only
Rust schema crate for canonical read-model/event shapes
eventing-plan for local event bus semantics only

Rule:
Read-model field presence is not replayable event-stream proof. Portal consumption is not LAN truth proof. Eventing proof is not LAN transport proof.
```

## Signed child hello and heartbeat family

```text
Workpacks:
18 Signed Child Hello And Heartbeat

Owners:
child-agent-runtime-distribution-plan for child package/runtime surfaces
device-trust-bootstrap-plan for trust material
lan-plan for LAN signed hello/heartbeat proof boundary
Rust schema crate for signed hello/heartbeat shapes

Rule:
Signed hello/heartbeat proof must show real signed artifacts, family/route binding, expiry, replay rejection, revoked-state rejection, and manual-required physical gaps. Contract rows are not signed artifact proof.
```

## Proof gate and rollout family

```text
Workpacks:
20 Proof Gates Fixtures And Rollout

Owners:
selected proof roots under output/lan-plan-proof/<slice-or-workpack>/
PLAN_STATE, WORKPACK_INDEX, CHECKLIST_INDEX, PROOF_INDEX, TEST_PROOF_EXPECTATIONS, and feature docs when status changes

Rule:
Rollout proof may aggregate only accepted roots or exact carried blockers. B1/B2 local proof cannot become portal, service, signed child hello, physical household, router/firewall, or relay proof.
```

## Active follow-on family

```text
Workpacks:
21-25

Owners:
selected owning Rust crate/plan per row; TS remains presentation only

Rule:
Rows 21-25 are active LAN follow-on scope. Use current row truth instead of
treating the whole family as uniformly open: 21, 22, and 24 are locally
complete with their own proof; 23 and 25 remain partial/manual. Do not skip,
auto-close, or use any row for PR_READY without current Rust-first proof,
organized tests where applicable, and row truth.
```
