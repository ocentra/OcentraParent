# 25 Platform Extension Checklist And Proof Routing

## Target State

macOS, iOS, Android, and Linux app/game work routes to explicit extension
checklists without bloating the base MVP or overclaiming support.

## Scope

- Platform-specific evidence sources.
- Authority tier and setup requirements.
- Manual proof artifacts.
- Store/signing/entitlement constraints.
- App/game-specific differences where platform APIs expose different meaning.

## Tests And Proof

- [x] Extension row names platform and authority tier.
- [x] Manual proof is attached before status moves.
- [x] No platform row says simply unsupported when a more precise status exists.
- [x] Cross-plan handoffs are linked.

## Current Proof - 2026-06-03

`packages/parent-domain/src/app-game-platform-extension-routing.ts`,
`packages/parent-domain/src/app-game-platform-extension-routing-rules.ts`, and
split platform row data under
`packages/parent-domain/src/app-game-platform-extension-routing-*-data.ts` now
define the platform-extension routing matrix for `MAC-01` through `MAC-12`,
`IOS-01` through `IOS-12`, `ANDROID-01` through `ANDROID-14`, and `LINUX-01`
through `LINUX-14`.

The focused proof harness
`scripts/test/app-game-platform-extension-routing-proof.mjs` builds the
parent-domain package, runs
`packages/parent-domain/tests/app-game-platform-extension-routing.test.ts`, and
records proof under:

```text
output/app-game-plan-proof/25-platform-extension-checklist-and-proof-routing/
output/app-plan-proof/24-platform-extension-checklist-and-proof-routing/
```

The matrix keeps all extension rows manual-required or not-claimed until a
future promoted row has authority-tier, permission/setup, rollback,
manual-platform, validation, and cross-plan proof artifacts attached.

## Done Signal

Platform extension work can start independently without weakening base app/game
claim boundaries.

## Product Doc Decision

Feature, plan checklist, source-index, snapshot, and platform deep-dive docs were
updated. `docs/product-capability-checklist.md` was not changed because this
slice adds routing/proof guards only and does not move any product capability
status or live platform support claim.

Use the standard checklist in [workpacks README](README.md).
