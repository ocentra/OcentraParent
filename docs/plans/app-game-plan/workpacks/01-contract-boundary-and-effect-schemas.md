# 01 Contract Boundary And Effect Schemas

## Target State

Shared app/game identity, inventory, runtime, foreground, launcher, session,
category, risk, approval, policy, authority, capability, action, AI digest, and
enforcement-result contracts exist in TypeScript before Rust, service, portal,
or adapter code consumes them.

## Scope

- Extend existing `packages/activity-domain/src/app-game*.ts` for evidence and
  session contracts.
- Extend `packages/parent-domain` for app/game policy, approval, authority, and
  capability contracts.
- Keep native app and native game product fields separate inside shared
  contracts.
- Encode stale, degraded, permission-limited, manual-required, unavailable, and
  not-claimed states.

## Tests And Proof

- Effect Schema accepts valid app and game rows.
- Display-name-only identity stays weak.
- Inventory cannot set running or foreground.
- Launcher evidence cannot become known game without child-game proof.
- AI output cannot contain block/terminate/hide/suspend/shield authority.
- Manual-required/unavailable states cannot mark actions executed.

## Done Signal

TypeScript contracts and decode tests exist for the shared app/game spine, and
Rust/service/portal changes are either absent or follow those contracts.

Use the standard checklist in [workpacks README](README.md).
