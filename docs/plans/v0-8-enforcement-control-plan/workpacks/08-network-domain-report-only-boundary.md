# 08 Network/Domain Report-Only Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `08 Network/Domain Report-Only Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md),
[folder README](../README.md),
[network-domain-control feature](../../features/network-domain-control.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Keep network/domain states visibility-only until a real blocking adapter is
proved, while still preserving useful policy-preview and parent-visible signal
states.

## Central schema boundary

```text
schema-domain owns public network/domain action-state, reason, and audit schemas when they cross package/crate/protocol boundaries.
network-plan owns network/domain evidence, metadata, and future adapter truth when selected.
browser-plan owns browser URL evidence and exact-page context separately.
v0-8-enforcement-control-plan owns the report-only/manual-required action boundary and no-claim posture.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/network-domain-control.md`
- `../../expectations/enforcement.md`

## Target State

Network/domain states can support visibility and policy preview without implying
decrypted content, exact URL, or proved block capability.

## Required proof fields

```text
canonical_schema_owner_state
network_evidence_state
process_attribution_state
domain_known_state
ip_only_state
tunnel_indicator_state
block_capability_state
report_only_state
manual_required_state
browser_url_separation_state
no_decrypted_content_claim
no_block_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/08-network-domain-report-only-boundary/`

Focused validation should record:

- `npm run test --workspace @ocentra-parent/enforcement-domain -- enforcement`
- selected network evidence or read-model proof for this slice
- selected portal tests only when parent-visible network/domain state changes
- selected architecture gate for touched network/enforcement/portal surfaces

## AI Worker Checklist

- [ ] Keep report-only/manual-required states for block actions until adapter proof exists.
- [ ] Preserve process-attribution, domain-known, IP-only, and unknown states.
- [ ] Add proof checks that prevent claim upgrades.
- [ ] Show VPN/proxy/tunnel indicators only where evidence supports them.
- [ ] Keep network flow evidence separate from browser URL evidence.

## Where We Are

Network/domain observation exists as metadata evidence. Real network/domain
blocking adapter proof is still manual-required.

## Negative Cases

- IP-only or unknown-domain signals must not claim exact domain control
- inferred VPN/proxy/tunnel state must stay evidence-backed and explicit
- browser URL or exact-page state must not leak into network/domain proof
- report-only/manual-required rows must not be upgraded into block-ready state
- missing blocking adapter proof must block enforcement claims

## Manual-Required Gaps

- Real network/domain blocking remains manual-required until a named adapter and
  proof artifact exist.
- Decrypted content and exact URL claims remain out of scope.
- Platform-specific packet/filter driver behavior remains unproved.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/08-network-domain-report-only-boundary/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
