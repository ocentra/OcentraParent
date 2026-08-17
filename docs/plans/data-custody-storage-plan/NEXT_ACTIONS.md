<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `NEXT_ACTIONS.md`
> Kind: short resume/action list.
> Read when: When starting or resuming this plan after PLAN_STATE.md.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Update when current next product slice changes.

<!-- /agent-capsule -->

# Data Custody Storage Plan Next Actions

1. Finish the remaining production-source gaps before writing tests: provide a
   real Account/family-owned trusted custody-authority adapter to the existing
   opaque child-runtime handle, then mount an external upstream caller without
   accepting authority selectors from request/JSON input. Preserve the dynamic
   Device Trust gate and current manual-required default. This is routed through
   Account WP08's sealed current authority, Account WP04's correlated
   export/delete handoff, and Account WP05's current
   household/member/device/session/capability/lease authorization composer.
2. Finish WP05 backup cadence/manual-backup plus migration execution/rollback
   source, the WP06 thin TypeScript adapter/rules edge, and WP08's trusted
   confirmation receipt plus reachable `Applied`/`Partial` decision path. Do
   not substitute proof scripts or synthetic DTO callers for missing product
   behavior.
3. After the repository-wide source wave is complete, write WP01's missing
   Rust invariant-test family, migrate the stale moved-
   store tests and write the full expected-test matrix for WP02-WP07. Only then
   run focused crate/domain tests and repair failures.
4. After focused tests and per-domain Enforcer gates, resolve the aggregate-
   proof publication model so fresh checkouts can audit durable artifacts;
   follow with repo-wide Enforcer, proof, precommit, one PR/CI cycle, and merge.
5. Confirm product source docs in [DOC_INDEX.md](DOC_INDEX.md) for the next selected workpack.
6. Select required proof intents from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
7. Record any adjacent-plan handoff in the selected workpack before opening that adjacent plan.
8. Keep overall plan status open until the remaining source, expected tests, focused gates, and proof roots are complete.

## Production-code audit boundary

WP02/WP03/WP04/WP05/WP06 now carry the accepted source-wave deltas recorded in
their workpacks. WP07 has a real internal child-service ingress/command/dispatch
path through durable effect and tombstone ownership, but shipped composition
still installs a manual-required custody authority and has no external upstream
caller. This checkpoint changes production source only; no tests, builds,
proof, precommit, CI, or PR were run.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.

## State

- WP04 production source is accepted at the generic/child-runtime boundary; moved-store expected tests and current validation/proof are open.
- WP01 production contract source is complete with a Rust-owned 28-row matrix and generated TS edge; the full Rust invariant-test family and current proof acceptance remain open. Deleted handwritten TS adapters/tests/proof runners must not be restored without a real consumer.
- WP02 decrypt-scope authority source is accepted; expected tests and current validation/proof are open.
- WP03 manifest-custody source is accepted; provider runtime, expected tests, and current validation/proof are open.
- WP05 import-integrity source is accepted, while backup cadence/manual backup and migration execution/rollback source remain open.
- WP06 Rust request/row validation source is accepted, while its thin TypeScript edge and expected tests remain open.
- WP07 has a real internal child custody command/effect/tombstone lifecycle and startup recovery; trusted authority composition, external calling, test migration, and aggregate route acceptance remain open.
- WP08 source is incomplete: no trusted confirmation receipt/confirmed input exists, so `Applied` and `Partial` remain unreachable; its positive/negative expected tests and current proof are also open.
- State remains open until the remaining production source, complete expected tests, provider/AI/runtime handoffs, rollout refresh, and carried proof blockers are closed.
- Keep this file and `PLAN_STATE.md` synchronized before any DONE/PR_READY claim.
