<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Mobile Controller Observer Runtime Proof - 2026-05-29
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Mobile Controller Observer Runtime Proof - 2026-05-29

Branch: `codex/v0-9-mobile-controller-observer-runtime-proof`

## Scope

This checkpoint adds a typed parent-domain contract and proof harness for the
non-visual V0.9 parent mobile controller/observer runtime boundary. It does not
touch C-owned UI, vendor UI paths, visual catalogs, browser automation, or portal
E2E flows.

The proof records:

- Android parent mobile as observer read-only;
- iOS parent mobile as controller-candidate with controller takeover
  manual-required;
- read-only observer operations for status, policy preview, and capability
  refresh;
- rejected observer write operations for policy writes, approvals, pairing, and
  revocation;
- controller takeover request as manual-required until real mobile package and
  device authority proof exists;
- controller release as backend local-service proof, not mobile write authority;
- LAN AI job submission as degraded-provider state;
- package readiness gaps for signing, stores, notifications, background
  behavior, and controller authority;
- proof harness inputs from the parent mobile shell, production mobile
  controller, and discovery runtime proof artifacts.

## Proof Command

```powershell
node scripts/test/v0-9-mobile-controller-observer-runtime-proof.mjs
```

The command writes:

```text
test-results/v0-9-mobile-controller-observer-runtime-proof/proof.json
```

It is registered in:

```text
docs/expectations/pre-ai-proof-matrix.json
```

The typed contract is exported as:

```text
@ocentra-parent/lan-domain/v0-9-mobile-controller-observer-runtime
```

It consumes:

```text
test-results/parent-mobile-shell-runtime-proof/proof.json
test-results/v0-9-production-lan-mobile-controller-proof/proof.json
test-results/v0-9-mobile-controller-discovery-runtime-proof/proof.json
```

## Honest Boundaries

- Parent mobile observer runtime remains read-only unless a real mobile package
  and device authority proof exists.
- Controller takeover remains manual-required.
- Backend controller release proof remains local-service-owned and does not
  upgrade parent mobile write authority.
- LAN AI job submission remains degraded when provider or mobile package bridge
  proof is unavailable.
- Cloud relay remains not implemented.
- Physical household LAN remains manual-required until two real devices,
  router/firewall behavior, origin checks, stale/offline behavior, and provider
  artifacts exist.
- Android child-agent device-owner behavior and iOS Family Controls behavior are
  not claimed by this parent mobile proof.

## Manual Upgrade Requirements

Before upgrading this proof to product-ready mobile controller behavior, record
real artifacts for:

1. Android and iOS parent mobile package install and launch,
2. mobile package controller takeover and release behavior,
3. mobile foreground/background LAN reachability,
4. notification permission and delivery behavior,
5. signing, TestFlight, app store, and Google Play release paths,
6. physical household LAN controller/observer behavior across two real devices,
7. authenticated cloud relay behavior if cloud relay becomes part of the product
   path.
