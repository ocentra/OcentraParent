# WP37 - Service Foreground Capture Bridge

## Scope

Cross-record the shared app/game WP37 service foreground capture bridge for the
native app plan.

The native app plan gains service-backed optional foreground row capture through
the shared app/game read-model path, without claiming product-complete native app
foreground control.

## Expected Outcome

- Native app plan records the shared app/game foreground capture proof without owning a duplicate source.
- Foreground evidence remains optional and platform-bounded.
- Evidence proves active window/foreground state only, not content inspection, app category, policy effect, or adapter execution.
- Unsupported platform and missing permission states remain explicit.

## Proof

Proof artifacts live in:

```text
output/app-plan-proof/37-service-foreground-capture-bridge
```

The authoritative implementation proof is the shared app/game workpack:

```text
docs/plans/app-game-plan/workpacks/37-service-foreground-capture-bridge.md
output/app-game-plan-proof/37-service-foreground-capture-bridge
```

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. Native app
product status remains blocked on portal, policy, adapter, and platform proof.

## Execution Detail

Minimum context:

- `docs/plans/app-game-plan/workpacks/37-service-foreground-capture-bridge.md`
- `docs/plans/screen-plan/AGENTS.md` only if capture/custody boundaries are touched.

Expected tests/proof names:

- `app-plan.wp37.foreground-row-captured`
- `app-plan.wp37.no-content-claim`
- `app-plan.wp37.unsupported-platform-state`
- `app-plan.wp37.no-policy-or-adapter-claim`

Failure conditions:

- Foreground evidence is used as content knowledge, category classification, or enforcement authority.
