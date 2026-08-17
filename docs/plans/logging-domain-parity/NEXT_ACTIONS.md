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

1. Confirm the active branch matches the assigned branch. For this source-wave reconciliation the branch is `codex/logging-source-wave-repair` at accepted source head `735df89de`.
2. Confirm claimed proof roots and test-result roots exist before trusting any done claim.
3. Pick one honest next slice from `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
5. Open that workpack only.
6. Fill the workpack pre-edit note.
7. Implement or reconcile, test, run, proof, then update docs.

## Highest-priority queue

### 0. Accepted Source Delta / Deferred Expected-Test Wave

Current status:

```text
integration source through 3fec0793a contains reviewed canonical redaction, Vite/writer routing, logger sanitization, portal-fallback sanitization, and query realpath/symlink hardening for WP02/WP03/WP07/WP08
no test source changed in that packet
```

Rust owns the exact 18-key sensitive-key policy and generates the checked-in
TypeScript artifact. The TypeScript sanitizer, Logger serialization path,
canonical dev writer, and portal compatibility fallback all consume that one
policy; no alternate local regex or fail-open policy is accepted.

Expected result after the remaining production-source wave closes:

```text
write the complete redaction/export/writer/query/logger expected-test matrix
then run only focused logging-domain and portal boundary tests and fix failures
do not regenerate proof or invoke broad validation before that writing phase is complete
```

The source mapping is recorded in the affected workpacks and graph. Focused
validation, proof, and external composition remain deferred until the complete
expected-test matrix is written.

### 1. Remaining Proof-Inventory Restoration / Claim Reduction

Current status:

```text
WP06 has a canonical proof root plus live agent-query/MCP proof-inventory detection, and WP08 has its canonical partial-proof root; the remaining proof-inventory restoration queue is the still-missing WP01/WP02/WP04/WP05/WP09 roots
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
the portal dev-log consumer slice is proved locally: bridge-first routing, compatibility fallback, parent scope definitions, snapshot-language separation, focused portal logging tests, and the canonical WP03 proof root are present in this checkout. Source inspection also confirms the live Rust callers: app::health, service_runtime::run_agent_service, and activity_capture route through agent-service::dev_log into logging-core::DevLogger, with core redaction and locked/synced NDJSON append.
```

Expected result:

```text
WP03 stays partial because its focused validation/proof rows remain deferred and the separate root dev-log-routing check is outside this slice; there is no missing production agent-service-to-logging-core mapping in the current source
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
