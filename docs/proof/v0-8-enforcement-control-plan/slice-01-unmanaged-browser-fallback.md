# WP07 Unmanaged Browser Fallback

- checkedAt: `2026-06-17T02:03:51.932Z`
- branch: `codex/tracking-plan-full-continuation-a`
- commit: `1f192e52b931d3b2b8080f3e9479d37a94172958`
- result: `pass`

## Commands

- `npm run test --workspace @ocentra-parent/enforcement-domain -- v0-8-browser-enforcement-timer-recovery-proof`
- `node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`

## Artifacts

- `test-results/windows-managed-unmanaged-browser-enforcement-proof/2026-06-17T02-03-51-932Z.json`
- `output/v0-8-enforcement-control-plan-proof/07-unmanaged-browser-fallback/`
- `packages/enforcement-domain/tests/unit/v0-8-browser-enforcement-timer-recovery-proof.test.ts`

## Covered states

- process identity required rejection
- report-only, warn-child, parent-review, terminate-process
- relaunch-managed manual-required
- degraded and unavailable
- exact URL, active tab, title, and content remain not-claimed

## Negative cases

- contract parsing rejects exact URL, active tab, title, content, notification-delivery, and broad-browser claim upgrades
- runtime proof rejects missing process id and mismatched pid/name attempts before any terminate action
- managed-browser exact URL service command stays manual-required

## Remaining gaps

- warning delivery artifacts
- managed relaunch custody proof
- managed exact URL enforcement
- broad browser, app, and network/domain blocking
