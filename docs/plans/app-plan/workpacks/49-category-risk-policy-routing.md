# WP49 - Category/Risk Policy Routing

## Scope

Cross-record the shared app/game WP49 category/risk policy-routing proof for the
native app plan.

This workpack proves native app category and risk candidates can route into
parent-domain policy compiler inputs only with active category proof, supporting
evidence refs, source/confidence disclosure, and no adapter dispatch.

It does not add live classifier/provider execution, service runtime policy
evaluation, portal UI, notifications, child request delivery, broad app
blocking, platform support, or platform adapter execution.

## Implementation

- Reuse the shared parent-domain category/risk policy-routing contract.
- Route native app categories to app-category policy targets.
- Route risk candidates to risk-app policy targets without hard adapter action
  claims.
- Require local AI candidate routes to cite AI digest refs.
- Keep manual-review candidates manual-required.
- Preserve the no-adapter boundary with `adapterDispatchState:
not-dispatched`.

## Proof

- `cmd /c npm run build --workspace @ocentra-parent/parent-domain`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-category-risk-policy-routing`
- `node scripts/test/app-game-category-risk-policy-routing-proof.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-plan-proof/49-category-risk-policy-routing
```

## No-Claim Boundaries

- Category and risk candidates remain policy inputs, not final decisions.
- Risk candidates cannot request block-launch or other hard adapter actions.
- Local AI category routes require digest refs and cannot dispatch adapters.
- Stale category proof is rejected before compile-ready routing.
- Runtime service consumption, portal rendering, child UX, notifications,
  platform adapters, and broad app blocking remain gaps.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because
primary owns central checklist edits during the merge wave. WP49 moves native
app category quality and risk-app policy-routing proof forward at the
parent-domain contract layer, but product status should not move until runtime
service policy consumption, portal category/risk UI, live classifier/provider
execution, notification/child request UX, broad app blocking, and platform
proof are complete.
