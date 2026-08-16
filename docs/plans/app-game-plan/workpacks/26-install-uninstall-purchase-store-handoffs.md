# 26 Install, Uninstall, Purchase, And Store Handoffs

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `26 Install, Uninstall, Purchase, And Store Handoffs`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Install, uninstall, purchase, and store signals are routed as app/game evidence
and approval handoffs without pretending the full install/purchase product is
complete.

## Scope

- New app/game detected.
- Installer/updater process.
- Store package install signal.
- Game purchase capability or purchase event where available.
- Uninstall/tamper handoff.
- App-install/purchase approval feature handoff.

## Tests And Proof

- Store/purchase signal is context, not automatic decision.
- Install approval refs include evidence.
- Uninstall/tamper routes to owning feature docs.
- Manual-required states are visible where platform support is missing.

## Done Signal

App/game plan can hand off install, uninstall, purchase, and store work without
silently claiming another feature.

Use the standard checklist in [workpacks README](README.md).

## Completion Note - 2026-06-03

- Read source docs: app-game feature, app-install/purchase feature,
  enforcement-integrity/tamper feature, app/game evidence expectation,
  app-install/purchase expectation, tamper/uninstall expectation, app/app-game
  plan READMEs, source indexes, current snapshots, platform deep dives, test
  blueprints, UI guides, main checklists, and this workpack.
- Locked implementation and docs under codex-c for
  `app-game-install-store-handoff` without editing E-C-owned
  `app-install-purchase-approval*` files or E-B-owned parent-domain export map.
- Added parent-domain schema/rule/proof/test coverage for six handoff rows:
  new app/game inventory, installer/updater process, store package install,
  game purchase signal, uninstall delta, and tamper/uninstall candidate.
- Proof output:
  `output/app-game-plan-proof/26-install-uninstall-purchase-store-handoffs/`
  and `test-results/app-game-install-store-handoff-proof/proof.json`.
- UI not applicable: no parent portal, child UI, policy authoring, approval, or
  evidence drawer source changed.
- Product checklist unchanged: this is contract/handoff proof only and does not
  move live store, approval UI, platform adapter, billing entitlement, uninstall
  blocking, or anti-tamper support up.
