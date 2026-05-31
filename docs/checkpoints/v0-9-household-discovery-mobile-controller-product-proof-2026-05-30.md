# V0.9 Household Discovery Mobile Controller Product Proof

Date: 2026-05-30

Branch: `codex/v0.9-household-discovery-mobile-controller-product-proof`

## Scope

This checkpoint records the non-visual V0.9 aggregate proof that composes the
existing local-service household discovery proof with the parent-mobile
controller/observer runtime proofs.

The proof covers:

- production discovery state labels: discovered, pending, paired, revoked,
  stale, offline, and unavailable;
- paired route acceptance plus failed-unpaired, wrong-origin, wrong-device,
  replay, revoked, stale, offline, and unavailable rejection states;
- Android parent-mobile observer route state and iOS manual controller takeover
  state;
- observer read-only operation coverage and rejected write, pair, revoke, and
  controller-takeover operations;
- backend controller release and controller transition labels already proven by
  the local-service route proof;
- manual-required physical household proof and mobile write-authority proof;
- not-implemented cloud relay with a manual decision gate.

## Non-Claims

This proof does not claim:

- two physical household devices on the same router or firewall path;
- production household router discovery;
- real Android or iOS parent-mobile write authority;
- mobile background LAN behavior;
- cloud relay routing, authentication, storage, or remote control;
- C-owned UI or vendor rendering.

## Validation

Primary harness:

```powershell
node scripts/test/v0-9-household-discovery-mobile-controller-product-proof.mjs
```

The harness builds contracts, runs the source proof harnesses, parses the
aggregate parent-domain read model, checks the proof-matrix registration, and
emits:

```text
test-results/v0-9-household-discovery-mobile-controller-product-proof/proof.json
```

Before PR-ready handoff, also run:

```powershell
npm run test --workspace @ocentra-parent/parent-domain -- v0-9-household-discovery-mobile-controller-product-proof
npm run test:pre-ai-proof
npm run --silent lint:schema-boundaries
npm run validate
```
