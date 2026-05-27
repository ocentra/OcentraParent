# V0.9 LAN Pairing And Multi-Device Local Control Expectations

This is the milestone-specific expectation file for V0.9 in `docs/product-roadmap.md`.

Supporting expectation files: [LAN pairing](lan-pairing.md), [contracts](contracts.md), [portal](portal.md), [platforms](platforms.md), [platform deliverables](platform-deliverables.md), and [static analysis and security](static-analysis-security.md).

## Outcome

- A parent device on the same LAN can discover, pair with, select, query, and configure a child-device agent through explicit trusted flow.
- Execution remains on the child-device agent.
- Anonymous, wrong-origin, wrong-device, stale, malformed, or replayed control attempts are rejected and audited.
- Windows, macOS, Linux, Android, and iOS LAN behavior is validated per
  platform because firewall, service, background, and mobile network behavior
  are not interchangeable.

## Acceptance

- Pairing proof, trusted device registry, selected device, route id, intent id, rejection reason, and audit event shapes are typed.
- Loopback remains default, LAN mode is explicit, and origin/route checks remain active after pairing.
- Portal can distinguish multiple local child agents and show offline/stale state.
- CI covers shared contracts and route behavior; real LAN proof covers
  cross-device discovery, firewall, origin, and stale-device behavior.
- Local multi-service proof must keep production discovery state honest by
  recording `discovered`, `pending`, `paired`, `revoked`, `stale`, `offline`,
  or `unavailable` instead of implying household discovery happened.
- If physical devices are not available, the proof record must include a manual
  two-device checklist with exact commands and required artifacts.

## Validation

- Run `npm run validate`.
- Include TypeScript contracts, Rust parity, service route tests, and portal Playwright coverage for accepted and rejected routes.
- Run `node scripts/test/v0-9-production-lan-multidevice-hardening.mjs` for the
  current local multi-service LAN proof bundle.
