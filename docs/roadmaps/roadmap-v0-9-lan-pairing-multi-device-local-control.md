<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 LAN Pairing And Multi-Device Local Control Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.9 LAN Pairing And Multi-Device Local Control Expectations

This is the milestone-specific expectation file for V0.9 in `docs/product-roadmap.md`.

Supporting expectation files: [LAN pairing](../expectations/lan-pairing.md), [contracts](../expectations/contracts.md), [portal](../expectations/portal.md), [platforms](../expectations/platforms.md), [platform deliverables](../expectations/platform-deliverables.md), and [static analysis and security](../expectations/static-analysis-security.md).

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
