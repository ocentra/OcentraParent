# WP50 - Runtime Policy Consumption

## Scope

Consume the WP49 category/risk policy-routing output in the existing
service-backed policy-dispatch runtime read model.

This workpack proves the local service command can expose an app/game
category-risk policy consumption row through the V0.8 enforcement
policy-dispatch read-model path. The row stays dry-run-only, cites evidence,
and does not dispatch an adapter.

It does not add portal category/risk UI, live classifier/provider execution,
notification delivery, child request UX, adapter execution, broad app blocking,
or platform enforcement support.

## Implementation

- Extend the parent-domain enforcement policy-dispatch read model with an
  app/game category-risk dry-run policy preview row.
- Mirror the new row through Rust protocol constants and the agent-service
  policy-dispatch read model.
- Keep the row on `windows-policy-dry-run-preview` with `dryRun: true`,
  `outcomeState: dry-run-only`, `capabilityState: dry-run`, and
  `dispatchedAt: null`.
- Keep policy target and evidence refs parent-visible without promoting
  category/risk routes into source truth or enforcement authority.

## Proof

- `cmd /c npm run build --workspace @ocentra-parent/parent-domain`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- enforcement-policy-dispatch`
- `cargo test -p ocentra-parent-agent-service policy_dispatch`
- `node scripts/test/app-game-runtime-policy-consumption-proof.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/50-runtime-policy-consumption
```

The parallel app-plan proof pack lives in:

```text
output/app-plan-proof/50-runtime-policy-consumption
```

## No-Claim Boundaries

- Category and risk routes remain policy inputs, not final source truth.
- The runtime service row is dry-run-only and leaves `dispatchedAt` unset.
- No timer refs, process termination, package blocking, hide/suspend/shield
  behavior, or platform adapter state is created.
- Broad installed-app blocking and cross-platform enforcement remain
  manual-required or unclaimed until separate platform proof exists.
- Portal rendering, notification delivery, child UX, and live provider
  execution remain separate gaps.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because the
current coordination instructions route central checklist deltas through the
hub. WP50 removes the specific "runtime service policy consumption" gap for the
category/risk dry-run read-model path, but the broader app/game feature remains
in progress while portal polish, live provider quality, notifications/child UX,
adapter execution, broad blocking, and platform proof remain open.
