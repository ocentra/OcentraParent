# Pasted Content Coverage Audit

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

## Bridge Gaps

The current lock state prevents editing these existing source-truth files in
this docs-only pass:

- `docs/plans/app-plan`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/architecture/app-game-evidence-sessions.md`
- `docs/app-control-capability-guide.md`

Once those locks are released, bridge edits should:

- link the app-game plan from the app/game feature and expectation docs;
- update the existing app-plan README to point native game work to this shared
  app-game plan;
- clarify that native games are not deferred anymore, but are worked through the
  shared app/game evidence spine;
- preserve browser-game routing to browser-plan;
- update product checklist rows only when proof/status changes, not for this
  docs-only plan creation.
