# App + Game Plan

This folder is the shared native app and native game control plan. It exists
because apps and games share the low-level evidence spine, but they do not share
product meaning.

Browser games, cloud gaming web pages, browser videos, and browser social pages
remain in [browser-plan](../browser-plan/README.md). Native games, game
launchers, installed apps, packaged apps, portable apps, process/session
evidence, app/game policy targets, and app/game enforcement proof route here.

The rule is:

```text
One shared evidence spine.
Two product slices.
Zero fake claims.
```

## Source Inputs

- [App and game control feature](../../features/app-game-control.md)
- [App/game evidence expectation](../../expectations/app-game-evidence.md)
- [App/game evidence sessions architecture](../../architecture/app-game-evidence-sessions.md)
- [Native apps plan](../app-plan/README.md)
- [App control capability guide](../../plans/app-game-plan/workpacks/app-control-capability-guide.md)
- [App control settings inventory](../../plans/app-game-plan/workpacks/app-control-settings-inventory.md)
- [Game control settings inventory](../../plans/app-game-plan/workpacks/game-control-settings-inventory.md)
- [Browser games/cloud gaming plan](../browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md)

## Product Split

Apps include productivity, school, social, video, music, VPN/proxy, remote
desktop, torrent/download, AI/chatbot, developer, creative, office, email,
system, store/installer, unknown, and risk apps.

Games include native games, launchers, launcher-game candidates, game sessions,
game budgets, ratings, multiplayer, UGC, purchases, store packages, native cloud
game clients, and unknown game-like executables.

Both use the same evidence chain:

```text
inventory/source observation
  -> runtime or foreground observation
  -> journal entry
  -> SQLite replay/read model
  -> app/game identity merge
  -> session and duration summary
  -> optional classifier digest
  -> parent policy decision
  -> capability/authority check
  -> adapter action or manual-required state
  -> audit proof and parent UI
```

## Non-Negotiable Claim Rules

- Inventory is not use.
- Runtime is not foreground.
- Foreground is not content.
- Launcher is not game.
- AI is evidence, not authority.
- Parent policy decides action.
- Enforcement requires adapter proof.
- Manual-required and not-claimed states must never execute adapters.
- Every visible claim needs evidence refs, freshness, and capability state.

Never claim:

- Steam running means a Steam game is running.
- A launcher installed means a child played a game.
- A process name that looks game-like is a known game.
- A foreground app/game means Ocentra knows app or game content.
- A store rating means the game is safe.
- AI classification can directly block, terminate, hide, suspend, or shield.

## Plan Files

- [Source index](source-index.md)
- [Current app/game snapshot](current-app-game-snapshot.md)
- [Shared evidence spine plan](v0-5-app-game-shared-evidence-spine-plan.md)
- [Native apps product slice](v0-5-native-apps-product-slice-plan.md)
- [Native games product slice](v0-5-native-games-product-slice-plan.md)
- [Platform deep dive](v0-5-app-game-platform-deep-dive.md)
- [Test blueprint](v0-5-app-game-test-blueprint.md)
- [UI/UX requirements guide](ui-ux-requirements-guide.md)
- [Implementation checklist](implementation-checklist.md)
- [Pasted content coverage audit](pasted-content-coverage-audit.md)
- [Workpacks](workpacks/README.md)

## Work Order

1. Reconcile docs and source truth before implementation.
2. Add or extend TypeScript Effect Schema contracts first.
3. Add Rust protocol parity only after TypeScript contracts exist.
4. Add journal/SQLite/read-model behavior before portal or policy claims.
5. Add portal UI only from service-backed read models or explicit fixtures.
6. Add policy compiler and adapter behavior only with evidence refs and
   capability status.
7. Add real platform proof only where authority tier, setup, rollback, and
   manual proof are attached.

## Proof Root

Every workpack stores proof under:

```text
output/app-game-plan-proof/<workpack-id>/
```

Required proof files are defined in
[implementation checklist](implementation-checklist.md). A workpack can mark a
proof file not applicable only with a written reason.

## Current State

This plan starts from existing merged app/game contracts and scoped
owned-process time-limit proof. It does not downgrade or duplicate
`docs/plans/app-plan`; it adds the missing game-specific product slice and uses a
shared implementation route so app and game evidence do not fork into parallel
systems.
