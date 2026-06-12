<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.4 Windows Network And Domain Observation Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.4 Windows Network And Domain Observation Expectations

This is the milestone-specific expectation file for V0.4 in `docs/product-roadmap.md`.

Supporting expectation files: [capture](../expectations/capture.md), [network flow evidence](../expectations/network-flow-evidence.md), [evidence storage](../expectations/evidence-storage.md), [contracts](../expectations/contracts.md), [portal](../expectations/portal.md), and [platforms](../expectations/platforms.md).

## Outcome

- Windows network/domain observation records typed flow or destination metadata without decrypting HTTPS or claiming page semantics.
- Process attribution, DNS/domain attribution, bytes/counts, VPN/proxy/tunnel indicators, and unknown attribution are explicit where available.
- Network evidence remains separate from browser URL/tab evidence unless a deliberate join contract links evidence ids.

## Acceptance

- Flow/domain observations are journaled, ingested, and queryable through the local evidence store.
- Unknown process, IP-only, encrypted-content-unavailable, and adapter-degraded states are visible.
- Portal and policy inputs do not infer exact browser URLs, search terms, chat text, or page content from network metadata.

## Validation

- Run `npm run validate`.
- Include parser/mapping tests, Rust parity tests, read-model tests, and portal evidence for the typed network summary.
