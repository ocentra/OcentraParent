# V0.7 Local AI Policy Evaluator Expectations

This is the milestone-specific expectation file for V0.7 in `docs/product-roadmap.md`.

Supporting expectation files: [AI](ai.md), [policy](policy.md), [evidence storage](evidence-storage.md), [portal](portal.md), [platforms](platforms.md), and [platform deliverables](platform-deliverables.md).

## Outcome

- Stored evidence, parent rules, local runtime status, and evidence-backed context produce deterministic dry-run policy decisions.
- Local AI output is evidence for policy, not hidden household authority.
- Enforcement remains disabled by default.
- Dry-run policy evaluation remains portable by consuming stored evidence refs
  and platform capability states rather than OS-specific adapter assumptions.

## Acceptance

- Decisions explain allow, warn, block, time-limit, ask-parent, unknown, degraded, and conflict outcomes with evidence and parent-rule refs.
- Explicit parent rules override ambiguous or contradictory local AI output.
- Portal can preview why a decision happened without claiming enforcement.
- The pre-AI platform checkpoint records which Windows, macOS, Linux, Android,
  and iOS evidence inputs are implemented, scaffold-only, unavailable, or
  degraded before more AI or enforcement work starts.

## Validation

- Run `npm run validate`.
- Include real stored-evidence integration tests, policy conflict tests, provider status tests, and portal preview coverage.
- Include test or manual-check notes proving dry-run decisions degrade honestly
  when a platform cannot produce a given evidence type.
