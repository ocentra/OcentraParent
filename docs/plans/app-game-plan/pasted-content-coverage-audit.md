# Pasted Content Coverage Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `Pasted Content Coverage Audit`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This audit records consolidation of the 2026-06-02 pasted app + game guidance.
The paste is kept as source context, not copied verbatim as unmanaged notes.

Attachment:

```text
C:\Users\sujan\.codex\attachments\1a64e280-5cfb-4c39-b7d8-9811f60191db\pasted-text.txt
```

## Coverage Map

| Pasted requirement                                  | Covered by                                                                                                                                                                 | Notes                                                                                        |
| --------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| One shared app/game low-level evidence spine        | [README](README.md), [shared evidence spine plan](v0-5-app-game-shared-evidence-spine-plan.md), [workpacks](workpacks/README.md)                                           | Implement app and game evidence once; separate product meaning in product slices.            |
| Apps and games have separate product slices         | [native apps slice](v0-5-native-apps-product-slice-plan.md), [native games slice](v0-5-native-games-product-slice-plan.md)                                                 | Native games now have a first-class product slice.                                           |
| Browser games remain in browser-plan                | [README](README.md), [source index](source-index.md), [games slice](v0-5-native-games-product-slice-plan.md)                                                               | Avoid duplicate browser-game work in this plan.                                              |
| Inventory/runtime/foreground/session no-claim rules | [README](README.md), [shared evidence spine plan](v0-5-app-game-shared-evidence-spine-plan.md), [test blueprint](v0-5-app-game-test-blueprint.md)                          | Merge blockers include inventory-as-use, runtime-as-foreground, foreground-as-content.       |
| Launcher is not game                                | [shared evidence spine plan](v0-5-app-game-shared-evidence-spine-plan.md), [games slice](v0-5-native-games-product-slice-plan.md), [UI guide](ui-ux-requirements-guide.md) | Adds launcher-only and launcher-game candidate states.                                       |
| AppGameIdentity and shared contract families        | [shared evidence spine plan](v0-5-app-game-shared-evidence-spine-plan.md), [workpack 01](workpacks/01-contract-boundary-and-effect-schemas.md)                             | Concrete implementation remains in domain packages.                                          |
| Native app categories, targets, actions             | [native apps slice](v0-5-native-apps-product-slice-plan.md), [implementation checklist](implementation-checklist.md)                                                       | App risks and approval states remain source/confidence-backed.                               |
| Native game categories, targets, actions            | [native games slice](v0-5-native-games-product-slice-plan.md), [implementation checklist](implementation-checklist.md)                                                     | Adds game budgets, ratings, UGC, multiplayer, purchases, and launcher states.                |
| Platform authority matrix                           | [platform deep dive](v0-5-app-game-platform-deep-dive.md), [implementation checklist](implementation-checklist.md)                                                         | Platform-specific rows remain proof-gated and manual-required until proved.                  |
| Test blueprint and proof pack                       | [test blueprint](v0-5-app-game-test-blueprint.md), [implementation checklist](implementation-checklist.md)                                                                 | Proof root is `output/app-game-plan-proof/<workpack-id>/`.                                   |
| Required fixtures and UI states                     | [test blueprint](v0-5-app-game-test-blueprint.md), [UI guide](ui-ux-requirements-guide.md)                                                                                 | Includes app/game dashboard, launcher UI, approvals, evidence drawer, and capability matrix. |

## Attachment Section Coverage

| Pasted section                                                                               | Consolidated plan target                                                                                                                            | Coverage decision                                                                                         |
| -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Doc 1 sections 1-3, product rule/source docs/spine                                           | [README](README.md), [source index](source-index.md), [shared evidence spine plan](v0-5-app-game-shared-evidence-spine-plan.md)                     | Keep as shared route and no-claim rules.                                                                  |
| Doc 1 section 4, inventory/runtime/launcher evidence                                         | [shared evidence spine plan](v0-5-app-game-shared-evidence-spine-plan.md), workpacks 05-10                                                          | Implement through typed evidence contracts, then Rust and adapter proof.                                  |
| Doc 1 section 5, shared contracts                                                            | workpacks 01, 04, 05, 08, 10, 13-15, 24                                                                                                             | WP01 starts this with evidence claim, runtime state, identity-strength, AI digest, and authority schemas. |
| Doc 1 sections 6-7, native apps/native games                                                 | [native apps slice](v0-5-native-apps-product-slice-plan.md), [native games slice](v0-5-native-games-product-slice-plan.md)                          | Keep separate product meaning while sharing low-level evidence.                                           |
| Doc 1 sections 8-9, platform tiers/reality matrix                                            | [platform deep dive](v0-5-app-game-platform-deep-dive.md), workpacks 11, 22, 23, 25                                                                 | Manual-required until platform-specific adapter proof exists.                                             |
| Doc 1 sections 10-12, read models/parent UX/child UX                                         | workpacks 15-18, 21, [UI guide](ui-ux-requirements-guide.md)                                                                                        | Portal and child UX must consume stored read models, not run scanners.                                    |
| Doc 1 section 13, workpack split                                                             | [workpacks README](workpacks/README.md), [implementation checklist](implementation-checklist.md)                                                    | Consolidated into 28 workpacks with app/game combined execution and separate product slices.              |
| Doc 1 sections 14-15, must-not-claim/minimum MVP                                             | [README](README.md), [test blueprint](v0-5-app-game-test-blueprint.md), [implementation checklist](implementation-checklist.md)                     | Merge blockers keep inventory/use, runtime/foreground, launcher/game, and AI/authority separate.          |
| Doc 2 sections 1-4, proof rule/test layers/invariants                                        | [test blueprint](v0-5-app-game-test-blueprint.md), [workpacks README](workpacks/README.md)                                                          | Proof packs are required per workpack.                                                                    |
| Doc 2 sections 5-8, unit/integration/contract/security-negative tests                        | [test blueprint](v0-5-app-game-test-blueprint.md), workpack test sections                                                                           | WP01 covers first security-negative contract tests; runtime and storage tests remain open.                |
| Doc 2 sections 9-12, platform/E2E/Playwright/fixtures                                        | [platform deep dive](v0-5-app-game-platform-deep-dive.md), [UI guide](ui-ux-requirements-guide.md), workpacks 16, 21-23, 25, 28                     | Platform and UI proof cannot be claimed before service-backed evidence exists.                            |
| Doc 2 sections 13-18, performance/CI/merge blockers/proof pack/worker instructions/final bar | [test blueprint](v0-5-app-game-test-blueprint.md), [implementation checklist](implementation-checklist.md), [workpacks README](workpacks/README.md) | Each workpack must produce focused proof and report gaps before DONE.                                     |

## Bridge Gaps

Reconciled in WP02/WP03:

- `docs/plans/app-plan`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/architecture/app-game-evidence-sessions.md`

Remaining bridge gaps:

- `docs/app-control-capability-guide.md` remains a source guide for app-control
  capabilities and should not be rewritten as app/game implementation proof.
- Product capability checklist rows should change only when runtime proof changes
  product status; no checklist update is needed for WP02/WP03 doc routing.
- Browser-plan keeps browser-game/cloud-gaming work. This app/game plan keeps
  native app/game and launcher scope.
