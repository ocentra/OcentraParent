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
   household/member/device/session authorization composer for the Data
   ParentOwner and step-up handoff; remote capability/lease reservations are
   outside these Data routes.
2. Preserve the reviewed WP05 base packet and close its dependency-owned
   composition gaps in this order: Account WP05 current authority/fence, Data
   WP09 provider-neutral byte custody, WP10 producer handoffs, WP11
   composition/mount, then a real caller. The base already supplies the schema
   contracts, pure `storage-custody-core` decisions, parent-runtime durable
   scheduler/job and restore/migration ledgers, restart reconciliation,
   rollback mount, and Eventing/outbox composition. It remains unmounted: no
   production caller implements its Account/provider/key/producer ports, and
   the sealed import-custody port needs an owner-side adapter design. Do not add
   another WP05-only seam or make the private port public. WP09/WP10 consume the
   WP05 base independently and must not duplicate these ledgers or depend on
   WP11. Account WP05A is the separate durable multi-owner coordinator for
   `ExportDeleteData`/`ImportRestoreData`-style ParentOwner and step-up-bound
   actions plus the typed Data handoff; remote-view/remote-control capability
   and controller-lease reservations are outside this Data route. It consumes
   the existing WP08 Account repository/read/CAS seam.
   After Account WP05A's durable opaque-effect coordinator/recovery owner is
   reviewed,
   finish WP08's trusted confirmation receipt plus reachable `Applied`/`Partial`
   decision path. Do not stage or consume a confirmation through a caller-made
   receipt while that Account handoff is missing.
   WP06's sealed Rust proof boundary and Rust-generated TypeScript edge are
   source-present for all seven states, exact request/row/citation/generation
   binding, and source/proof page limits. Its Account capability is an
   issuance-time snapshot, not race-safe repository currentness. No
   report/query runtime consumer or owner adapter is routed to it; do not
   invent an agent-service/report caller or substitute proof scripts or
   synthetic DTO callers for missing product behavior.
3. After the repository-wide source wave is complete, write WP01's missing
   Rust invariant-test family, migrate the stale moved-store tests, and write
   the full expected-test matrix for WP02-WP07. WP06 must migrate its Rust
   harnesses to the sealed wrapper, cover raw-DTO rejection, all-seven-state
   completeness, snapshot expiry/binding, and source/proof page-size
   negatives, and add the unwritten TypeScript scope/row/generation/array/
   pagination contract coverage. Only then run focused crate/domain tests and
   repair failures.
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

The legal source route is now `WP05 base -> Account WP05A owner coordinator -> (WP09, WP10) -> WP11 composition/mount`.
WP05 remains the direct base dependency for every Data runtime workpack;
WP05A is an additional direct reviewed-implementation dependency for the
Account/Device Trust/step-up/Protected Custody data-action outcome and typed
handoff; remote capability/lease is outside these Data workpacks. It does not
replace WP05's ledgers.
Reviewed-implementation gates let WP09/WP10 consume source-accepted Data
foundations without waiting for their tests or DONE, while the missing Account
WP05A owner-coordinator source remains a real blocker. WP11 is blocked on
Account WP05A's true durable multi-owner coordinator/recovery for the Data
actions plus Account WP05's base authority transaction/CAS and recovery
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
- WP06's Account-issued authority-snapshot Rust request/row/sealed-proof boundary and generated TypeScript scope/citation/generation/page edge are source-accepted for all seven states. No downstream report/query consumer or owner adapter is routed; the expected Rust tests are stale and the TypeScript contract test is unwritten.
- WP07 has a real internal child custody command/effect/tombstone lifecycle and startup recovery; trusted authority composition, external calling, test migration, and aggregate route acceptance remain open.
- WP08 source is incomplete: no trusted confirmation receipt/confirmed input exists, so `Applied` and `Partial` remain unreachable; its positive/negative expected tests and current proof are also open. The confirmation staging/consume path is explicitly blocked on Account WP05's durable opaque-effect CAS/recovery owner and typed handoff.
- State remains open until the remaining production source, complete expected tests, provider/AI/runtime handoffs, rollout refresh, and carried proof blockers are closed.
- Keep this file and `PLAN_STATE.md` synchronized before any DONE/PR_READY claim.

## 2026-08-18 source-map refresh

Data WP05's rollback dispatch validation is now explicitly mapped in the
engineering graph. A fresh caller/test audit confirms the base source is real
and fail-closed but unmounted: no production caller implements the external
ports, all five expected runtime test roots are absent, and a stale blocked-
restore test disagrees with the source's false/false local-authority/tombstone
flags. The next source work remains the dependency-ordered Account WP05, WP09,
WP10, WP11, and caller chain; tests and proof are intentionally later gates.

Data WP06's canonical `9462ce44e` and `d3c4b64ca` source is now explicitly
mapped through the sealed validated proof snapshot, all-seven-state gate,
source/proof page bound, and Rust/generated parity rules. This mapping does not
add its missing consumer/owner adapter, migrate or run tests, refresh proof, or
raise validation/DONE/PR readiness.
