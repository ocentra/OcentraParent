# 11 Active-Tab Proof Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `11 Active-Tab Proof Model`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Current CDP `/json/list` evidence is correctly represented as active state
`unknown`.

## Where We Want To Be

The product has an explicit active-tab proof model so no worker can accidentally
promote target-list evidence into known-active tab evidence.

## Scope

- `known-active`, `known-inactive`, and `unknown` state semantics.
- Freshness/stale behavior for active evidence.
- Proof source labels: target-list-only, CDP focus/activation, extension event,
  foreground correlation, owned-shell event.
- UI copy for unknown active state.

## Touched Paths

- `packages/activity-domain/src/browser-schemas.ts`
- `crates/agent-protocol/src/browser_values.rs`
- `apps/portal/src/portal-browser-route-panels.ts`
- `docs/expectations/browser-evidence.md` if acceptance changes.

## Tests And Proof

- Unit tests preventing target-list-only known-active mapping.
- Bridge/read-model tests preserving target-list-only active proof source.
- Portal parser and route scaffold tests for active unknown display.
- Proof pack:
  `output/browser-plan-proof/11-active-tab-proof-model/`.

## Implementation Notes

- Browser tab evidence now carries an explicit `activeProofSource` alongside
  `activeState`.
- Target-list-only evidence is contractually limited to `unknown`; it cannot be
  promoted to `known-active` or `known-inactive`.
- Stronger proof labels are reserved for later CDP focus/activation, managed
  extension, foreground correlation, or owned-shell evidence.
- Rust protocol constants, browser read models, journal replay, service browser
  evidence payloads, and portal parser state now preserve the proof-source
  boundary without claiming a known active tab.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/11-active-tab-proof-model/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: bridge/read-model/service proof records active proof source; no real-browser focus source is claimed.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots are not applicable; `06-ui-snapshots/ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: target-list-only cannot claim known-active or known-inactive.
- [ ] Manual platform proof is not applicable; `09-manual-platform-proof.md` records why.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Known-active remains not claimed until a separate focus/activation proof source
is implemented and validated.
