<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.5.3 Local Screen Evidence Analysis Queue Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.5.3 Local Screen Evidence Analysis Queue Expectations

This is the milestone-specific expectation file for V0.5.3 in `docs/product-roadmap.md`.

Supporting expectation files: [screen evidence](../expectations/screen-evidence.md), [capture](../expectations/capture.md), [evidence storage](../expectations/evidence-storage.md), [AI](../expectations/ai.md), [policy](../expectations/policy.md), [enforcement](../expectations/enforcement.md), [portal](../expectations/portal.md), [platforms](../expectations/platforms.md), and [platform deliverables](../expectations/platform-deliverables.md).

## Outcome

- Optional local screen evidence is disabled by default and parent controlled.
- Temporary images are encrypted in a local queue, summarized by local OCR/vision, then deleted according to TTL/deletion state.
- Policy consumes schema-valid summaries and evidence refs, not retained screenshots or raw AI text.
- Screen capture, foreground-window targeting, and OCR/vision runtime
  availability are platform capability states before policy or AI can use them.

## Acceptance

- Screen images do not leave the child PC for remote/API AI or Ocentra-hosted processing.
- Queue, summary, confidence, category, source evidence refs, image digest, deletion state, cadence, trigger, and retention settings are typed.
- Portal shows enablement, cadence, triggers, retention/deletion, capability status, and summary state clearly.
- Unsupported or permission-limited platforms report unavailable/degraded screen
  evidence rather than silently falling back to guessed activity.

## Validation

- Run `npm run validate`.
- Include queue encryption tests, schema validation tests, Rust read-model tests, and portal state checks.
