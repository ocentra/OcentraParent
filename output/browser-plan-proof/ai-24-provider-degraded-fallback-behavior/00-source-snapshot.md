# AI-24 Source Snapshot

Base:

- `origin/main` `1d2a625f0dfa88457eca1842b3443a8c9ecff50b`
- worker branch `codex/browser-ai-provider-fallback-proof`

Scope:

- `packages/activity-domain/src/browser-ai-provider-fallback-schemas.ts`
- `packages/activity-domain/tests/browser-ai-provider-fallback.test.ts`
- `packages/activity-domain/tests/browser-ai-provider-fallback-planner.test.ts`
- `scripts/test/browser-ai-provider-fallback-proof.mjs`

The provider fallback contract now records one visible/auditable fallback
decision across the existing local provider, family AI hub, and parent-approved
remote AI route proofs. The implementation now includes
`planBrowserAiProviderFallbackDecision`, which derives the selected provider,
runtime ref, fallback action, fallback reasons, and audit evidence from the
validated local-provider, family-hub, and remote route proofs instead of
requiring callers to hand-assemble the decision shape.

Proof:

- `cmd /c node scripts/test/browser-ai-provider-fallback-proof.mjs`
- `test-results/browser-ai-provider-fallback-proof/proof.json`
- `output/browser-plan-proof/ai-24-provider-degraded-fallback-behavior/11-provider-fallback-proof.json`

The proof harness imports built activity-domain contracts and planners, creates
five route-derived fallback decisions, and persists only decision summaries,
negative-check booleans, and no-claim flags.

Not claimed:

- model execution
- AI analysis result production
- policy decision authority
- runtime delivery
- UI rendering
- enforcement
- real provider calls
