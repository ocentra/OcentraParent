<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.5.1 Browser URL And Tab Evidence Capture Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.5.1 Browser URL And Tab Evidence Capture Expectations

This is the milestone-specific expectation file for V0.5.1 in `docs/product-roadmap.md`.

Supporting expectation files: [browser evidence](../expectations/browser-evidence.md), [capture](../expectations/capture.md), [evidence storage](../expectations/evidence-storage.md), [contracts](../expectations/contracts.md), [portal](../expectations/portal.md), and [platforms](../expectations/platforms.md).

## Outcome

- Supported managed-browser sessions can produce typed active URL/tab evidence with URL, title, normalized domain, active state, evidence id, source id, adapter id, and capability status.
- Browser URL/tab evidence is distinct from process/window and network/domain evidence.
- Unmanaged browser use is reported as unmanaged or possible bypass, not as successful URL capture.

## Acceptance

- Supported, unsupported, unmanaged, missing bridge, missing permission, stale evidence, and adapter-error states are represented.
- Browser evidence is journaled and ingested before portal or local AI use.
- The milestone does not capture page body text, screenshots, keystrokes, form values, browser secrets, or decrypted HTTPS payloads.

## Validation

- Run `npm run validate`.
- Include managed-browser contract tests, Rust protocol parity, real service read-model tests, and portal visibility checks.
