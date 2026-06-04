# WP49 - Category/Risk Policy Routing

## Scope

Route category, risk, and game-context candidates into the existing app/game
policy target compiler boundary without treating those candidates as source
truth or enforcement authority.

This workpack proves parent-domain category/risk routing can carry active
category proof, confidence, candidate source, target kind, policy action, and
supporting evidence refs into soft/manual compiler inputs.

It does not add live classifier/provider execution, service runtime policy
evaluation, portal UI, notifications, child request delivery, adapter execution,
broad blocking, or platform support.

## Implementation

- Add a parent-domain category/risk policy-routing contract.
- Map native app categories, risk candidates, native game categories, and game
  context signals to matching app/game policy target kinds.
- Require active category proof and supporting evidence refs before a route is
  compile-ready.
- Require local AI category routes to cite an AI digest ref.
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
output/app-game-plan-proof/49-category-risk-policy-routing
```

## No-Claim Boundaries

- Category and risk candidates remain policy inputs, not final decisions.
- Risk candidates cannot request hard adapter actions.
- Local AI category routes require digest refs and cannot dispatch adapters.
- Stale category proof is rejected before compile-ready routing.
- Runtime service consumption, portal rendering, child UX, notifications,
  platform adapters, and broad blocking remain gaps.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because
primary owns central checklist edits during the merge wave. WP49 moves category
quality and policy-routing proof forward at the parent-domain contract layer,
but product status should not move until runtime service policy consumption,
portal category/risk UI, live classifier/provider execution, notification/child
request UX, adapter execution, and platform proof are complete.
