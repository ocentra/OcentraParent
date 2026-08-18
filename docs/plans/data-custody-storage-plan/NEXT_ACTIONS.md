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
2. Review and integrate the bounded WP05 base source packet in its reviewed
   ownership order: schema-owned durable backup cadence/schedule/job and
   migration/rollback contracts; pure `storage-custody-core`
   backup/restore/migration/preflight decisions and compensation; and the
   `parent-runtime-core` durable scheduler/job ledger, restore/migration
   ledger, restart reconciliation, executor/rollback mount, and real
   Eventing/outbox composition. Runtime composition/custody mounting is now
   isolated in blocked WP11. Mount only opaque Account/family authority,
   key/decrypt capability, provider-neutral adapter, producer artifact, and
   producer-result ports there; leave unavailable external owners
   manual-required or blocked. WP09/WP10 consume the WP05 base independently
   and must not duplicate these ledgers or depend on WP11.
   After Account WP05's durable opaque-effect CAS/recovery owner is reviewed,
   finish WP08's trusted confirmation receipt plus reachable `Applied`/`Partial`
   decision path. Do not stage or consume a confirmation through a caller-made
   receipt while that Account handoff is missing.
   WP06's generated TypeScript edge is source-present, but no report/query
   runtime consumer is routed to it; do not invent an agent-service/report
   caller or substitute proof scripts or synthetic DTO callers for missing
   product behavior.
3. After the repository-wide source wave is complete, write WP01's missing
   Rust invariant-test family, migrate the stale moved-store tests, and write
   the full expected-test matrix for WP02-WP07, including WP06's stale Rust
   authority/pagination cases and unwritten TypeScript contract coverage. Only
   then run focused crate/domain tests and repair failures.
4. After focused tests and per-domain Enforcer gates, resolve the aggregate-
   proof publication model so fresh checkouts can audit durable artifacts;
   follow with repo-wide Enforcer, proof, precommit, one PR/CI cycle, and merge.
5. Confirm product source docs in [DOC_INDEX.md](DOC_INDEX.md) for the next selected workpack.
6. Select required proof intents from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
7. Record any adjacent-plan handoff in the selected workpack before opening that adjacent plan.
8. Keep overall plan status open until the remaining source, expected tests, focused gates, and proof roots are complete.

## 2026-08-17 routing correction

The WP05 owner route above is now source-present before the expected-test wave.
WP09 supplies only downstream pure provider-neutral byte custody/adapter-port
composition, and WP10 supplies only downstream pure producer-handoff
orchestration after their graph dependencies are legal. Neither may duplicate
the WP05 parent-runtime scheduler/job or restore/migration ledger. Account
WP02's target-aware authority correction remains a separate prerequisite for
action-bound custody composition; missing Account/key/provider/producer owners
must remain opaque blockers rather than being fabricated.

## 2026-08-18 base/composition routing correction

The legal source route is now `WP05 base -> Account WP05 participant/CAS source -> (WP09, WP10) -> WP11 composition/mount`.
Reviewed-implementation gates let WP09/WP10 consume source-accepted Data
foundations without waiting for their tests or DONE, while the missing Account
WP05 participant/CAS source remains a real blocker. WP11 is blocked on Account
WP05's true authority transaction/CAS and recovery
owner, key/import custody, producer artifact custody, WP09 provider operation
capability, and WP10 owner-derived outcomes. Its only planned roots are
`crates/parent-runtime-core/src/data_custody_runtime_composition.rs`,
`crates/parent-runtime-core/src/data_custody_runtime_composition_mount.rs`,
and `crates/parent-runtime-core/tests/integration/data_custody_runtime_composition.rs`.
This is routing-only: no production source, tests, proof, public private
traits, runtime completeness, or PR readiness is implied.

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
- WP05's bounded backup cadence/manual backup and migration execution/rollback
  source packet is present, including the independently accepted rollback
  authority-binding validator; external Account/key/provider/producer
  composition, production reachability, expected tests, focused validation,
  and proof remain open, with unavailable provider-backed paths
  manual-required.
- WP06's current Account-authority-derived opaque Rust request/row boundary and generated TypeScript page/cursor edge are source-accepted; no downstream report/query consumer is routed, and the expected Rust tests are stale while the TypeScript contract test is unwritten.
- WP07 has a real internal child custody command/effect/tombstone lifecycle and startup recovery; trusted authority composition, external calling, test migration, and aggregate route acceptance remain open.
- WP08 source is incomplete: no trusted confirmation receipt/confirmed input exists, so `Applied` and `Partial` remain unreachable; its positive/negative expected tests and current proof are also open. The confirmation staging/consume path is explicitly blocked on Account WP05's durable opaque-effect CAS/recovery owner and typed handoff.
- State remains open until the remaining production source, complete expected tests, provider/AI/runtime handoffs, rollout refresh, and carried proof blockers are closed.
- Keep this file and `PLAN_STATE.md` synchronized before any DONE/PR_READY claim.

## 2026-08-18 source-map refresh

Data WP05's rollback dispatch validation is now explicitly mapped in the
engineering graph. The next source work remains the real Account/family,
key/import, provider-neutral, and producer owners plus a reachable caller;
tests and proof are intentionally still later gates.
