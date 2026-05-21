# V0.4 Windows Network And Domain Observation Expectations

This is the milestone-specific expectation file for V0.4 in `docs/product-roadmap.md`.

Supporting expectation files: [capture](capture.md), [network flow evidence](network-flow-evidence.md), [evidence storage](evidence-storage.md), [contracts](contracts.md), [portal](portal.md), and [platforms](platforms.md).

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
