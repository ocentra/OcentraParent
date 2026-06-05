# AI-24 Provider Degraded/Fallback Proof

Contract proof:

- The fallback planner derives the decision from real validated route proofs
  rather than caller-owned object assembly.
- Local provider fallback decisions select the local runtime only when the
  selected local route and runtime ref match.
- Family AI hub fallback decisions select the household hub only after the local
  provider route is not selected.
- Parent-approved remote fallback decisions select the remote runtime only when
  the remote route has explicit parent approval and local safety fallback.
- Metadata-only fallback decisions keep the runtime ref null and use
  `metadata-only-review`.
- No-AI fallback decisions keep the runtime ref null and use an explicit parent
  or manual fallback action.
- Every accepted decision keeps parent fallback visibility, child fallback
  visibility, local safety preservation, and audit evidence.

Proof artifacts:

- `scripts/test/browser-ai-provider-fallback-proof.mjs`
- `test-results/browser-ai-provider-fallback-proof/proof.json`
- `output/browser-plan-proof/ai-24-provider-degraded-fallback-behavior/11-provider-fallback-proof.json`

The proof output records five route-derived decisions:

- child-device local AI selected;
- family AI hub selected after local model missing;
- parent-approved remote AI selected after local model missing and explicit
  parent approval/local safety fallback;
- metadata-only fallback with null runtime ref;
- no-AI parent-review fallback with null runtime ref.

Validation:

- `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-ai-provider-fallback.test.ts`
- `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-ai-provider-fallback-planner.test.ts`
- `cmd /c npm run build:contracts`
- `cmd /c node --check scripts/test/browser-ai-provider-fallback-proof.mjs`
- `cmd /c node scripts/test/browser-ai-provider-fallback-proof.mjs`
