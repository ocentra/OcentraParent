<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Plan Health`
> Kind: consistency and readiness check.
> Read when: before claiming the plan is complete, stale, blocked, or PR-ready.
> Stop rule: do not use this as implementation instructions; use assigned workpacks.
> Proves: plan consistency only.
> Does not prove: source implementation or validation completion.

<!-- /agent-capsule -->

# Data Custody Storage Plan Health

## Current health

```text
route docs: present
architecture/decision docs: present
workpack index: upgraded
checklist index: upgraded
proof index: upgraded
execution blueprint: upgraded
workpacks: present, not fully rewritten in this pass
implementation: not started by this plan route
source proof: partial contract proof exists for parent-owned sync/export; runtime proof remains open
PR-ready: false
```

## Consistency checks

Before reporting broad progress, verify:

```text
AGENTS.md routes to PLAN_STATE.md, NEXT_ACTIONS.md, WORKPACK_INDEX.md.
WORKPACK_INDEX.md lists every executable workpack and proof root.
WORKPACK_FAMILIES.md is used only when owner/proof family is unclear.
CHECKLIST_INDEX.md has rows for every workpack.
PROOF_INDEX.md has proof roots and required artifacts for every workpack.
TEST_PROOF_EXPECTATIONS.md has focused command/proof expectations for every workpack.
Each selected workpack has exact proof artifacts and no-claim boundaries before DONE.
Active proof roots use output/data-custody-storage-plan-proof/; legacy docs/proof routes do not raise status.
```

## Known healthy boundaries

This plan intentionally separates:

```text
data class/source-of-truth
key custody
parent-owned cloud sync
retention/delete/tombstone
export/import/restore
report/query custody
assistant citation custody
parent storage settings/apply flow
rollout proof gate
```

Do not collapse those boundaries.

## Known incomplete areas

The plan is not implementation-complete until these are done:

```text
WP01 custody source-of-truth proof
WP02 key custody proof
WP03 parent-owned sync proof
WP04 retention/delete/tombstone proof
WP05 export/import/restore proof
WP06 report/query custody proof
WP08 parent storage settings/apply proof
WP07 rollout proof and route gate
```

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist/proof row.
- Update PLAN_STATE.md and NEXT_ACTIONS.md if current state changes.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not claim READY from contract/schema proof alone.
- Do not claim READY from sync manifest without provider, encryption, key, and tombstone proof when the claim requires those tiers.
- Do not claim READY from export proof as restore/apply proof.
- Do not claim READY from delete proof without tombstone, idempotency, replay, and restore-blocking proof when the claim requires them.
- Do not claim READY from report/query proof without source/ref/citation/redaction proof.
- Do not claim READY from parent settings UI without apply/confirm/proof.
- Do not claim READY from provider status rows as OAuth, upload, delete, or readable-payload runtime.
- Do not claim READY from automatic Ocentra-hosted fallback storage without explicit product decision and proof.
- Do not claim feature completeness until the relevant E2E tier in TEST_PROOF_EXPECTATIONS.md is explicitly proven or blocked.

## Rejection conditions

The plan is unhealthy if:

```text
storage/sync/export/delete/report claims are made without proof roots
Ocentra-hosted fallback storage is implied without explicit decision and proof
private payloads appear in report/query/assistant outputs without allowed references
restore can revive deleted/tombstoned state
parent storage settings apply changes without confirmation and proof
eventing internals are edited while eventing-plan owns active lane work
proof/checklist changed before source/tests for implementation work
production-domain legacy path is treated as current parent-owned sync/export source of truth
```

## Agent route walkthrough

- Landing decision: root plan routing selects this plan for custody guarantees, encrypted storage policy, retention/delete/tombstone, export/import/restore, parent-owned sync, report/query/assistant custody, and parent storage settings/apply flow.
- Scope split: eventing, portal UI, account authority, device trust, Cloudflare runtime, payment semantics, setup journey, remote transport, notification delivery, report rendering, and AI runtime stay in sibling plans unless the selected workpack names a typed handoff.
- Minimum read set: AGENTS.md, PLAN_STATE.md, NEXT_ACTIONS.md, WORKPACK_INDEX.md, WORKPACK_FAMILIES.md only when owner/proof family is unclear, one workpack, TEST_PROOF_EXPECTATIONS.md, and PROOF_INDEX.md when validating proof.
- Test/proof decision: require custody source-of-truth, key custody, sync/provider, retention/delete/tombstone, export/import/restore, report/query, assistant citation, parent settings/apply, and rollout tiers only where the selected workpack claims them.
- DONE blocker: no custody claim may move unless proof distinguishes source, custody, key, provider, retention, tombstone, restore, report/query, assistant, settings/apply, and no-claim boundaries.

## PR-ready rule

The whole plan is PR-ready only when WP07 consumes or blocks every earlier proof root and updates PLAN_STATE.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, and remaining open workpacks listed.
