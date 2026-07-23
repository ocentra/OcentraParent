<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Next Actions`
> Kind: resume queue and highest-open work.
> Read when: starting or resuming after PLAN_STATE.md.
> Stop rule: Pick one workpack; do not broaden into unrelated plans.
> Proves: next-action routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: Update this file only when queue state changes.

<!-- /agent-capsule -->

# Logging Domain Parity Next Actions

## How to use

1. Confirm the active branch matches the assigned branch. For this audit-hardening pass the branch is `codex/plan-harness-update`.
2. Confirm claimed proof roots and test-result roots exist before trusting any done claim.
3. Pick one honest next slice from `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
5. Open that workpack only.
6. Fill the workpack pre-edit note.
7. Implement or reconcile, test, run, proof, then update docs.

## Highest-priority queue

### 1. Remaining Proof-Inventory Restoration / Claim Reduction

Current status:

```text
WP04 is complete-proven at 12/12 against source commit `d21288ff68b1aa93aa839b2c49cd62f3009905ed`: bounded NDJSON tail and operation-state recovery, atomic repairable intent/commit markers, exact committed-record custody verification, complete artifact replay custody validation including createdAt, hard-link and crash-safe copy fallback, Windows publication durability, extended UNC preservation, real subprocess conflict custody, persisted adversarial redaction, and both normal and all-features gates are current. The registered agent-service dev-log target runs 3 tests, and checked-in CI now requires the logging-core all-features gate. All five ignored proof artifacts were regenerated against that source tree. WP06 has live agent-query/MCP proof-inventory detection, and WP08 has its bounded partial-proof evidence; the remaining proof-inventory restoration queue is WP01/WP02/WP05/WP09
```

Expected result:

```text
the remaining proof-missing workpacks either get real canonical proof roots or drop back from any overstated status
the remaining proof-missing workpacks keep explicit no-claim language until proof is restored
proof-inventory wrappers report only real blocking gaps
```

### 2. WP03 Parent Architecture and Routing Truthful Closeout

Current status:

```text
the portal dev-log consumer slice is proved locally: bridge-first routing, compatibility fallback, parent scope definitions, snapshot-language separation, focused portal logging tests, and the canonical WP03 proof root are present in this checkout
```

Expected result:

```text
WP03 stays partial until the remaining Rust-side agent-service mapping row is closed by its owning slice or reduced to a narrower no-claim boundary
```

### 3. Root Dev-Log-Routing Handoff For Full WP06 Closeout

Current status:

```text
root logging validation still has one route-check failure owned outside this delegated logging-only slice
```

Expected result:

```text
the owning slice either resolves or narrows the route-check expectation
WP06 can then move from partial-proof to a true focused-validation pass without widening this thread
```

## PR readiness guard

A PR-ready slice should close a named workpack or explicitly list remaining rows.

Do not create a PR that only updates checklist text, adds proof prose, renames docs, or adds placeholder comments unless the assigned workpack is explicitly proof-routing-only.

## Actioned completion tracker

- [x] Re-check this plan route from `README.md`, `AGENTS.md`, and `PLAN_STATE.md`.
- [x] Audit the current source/test/proof state against the plan claims.
- [x] Reconcile plan-state and workpack docs with the restored WP03/WP06/WP07/WP10 proof truth.
- [ ] Rebuild the remaining missing proof roots or remove the claims that say they already exist.
- [x] Write the canonical WP03 portal-dev-log consumer proof root and truth-sync the workpack/checklist for that bounded slice.
- [x] Write the canonical WP06 validation/enforcement proof root and truth-sync the bounded workpack/checklist state.
- [x] Write the canonical WP08 logger instrumentation proof root and truth-sync the bounded partial-proof state.
- [x] Fix the standalone proof-trace smoke claim with a self-seeding clean-workspace harness and canonical proof roots.
- [x] Close WP04 with current-head subprocess artifact custody, persisted adversarial redaction, scoped gates, and regenerated proof custody.
