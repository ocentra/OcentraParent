# AI-24 Provider Fallback Proof

Proof mode: real Effect Schema contract and parser tests.

Primary command:

```text
cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-ai-provider-fallback.test.ts
```

Result:

```text
Test Files  1 passed (1)
Tests       7 passed (7)
```

Behavior proved:

- Child-device local AI can be selected only when the local route is selected
  and the selected runtime ref matches the route runtime ref.
- Family AI hub can be selected only after the local provider route is not
  selected, with a selected family-hub route and matching runtime ref.
- Parent-approved remote AI can be selected only after local and family-hub
  routes are not selected, with explicit parent approval, local safety fallback,
  a selected remote route, and a matching runtime ref.
- Metadata-only and no-AI fallbacks keep selected runtime refs null and expose
  parent/child-visible fallback actions and reasons.
- Hidden fallback, claimed AI analysis result authority, claimed policy
  decision authority, disabled local safety, remote default blocking, and remote
  outage disabling local safety are rejected.
- Route mismatch states are rejected, including remote fallback while local or
  family-hub fallback is already selected.

Implementation evidence:

- `packages/activity-domain/src/browser-ai-provider-fallback-schemas.ts`
  requires remote fallback to have no selected local route and no selected
  family-hub route.
- `packages/activity-domain/tests/browser-ai-provider-fallback.test.ts` adds the
  remote-order negative test.
- `packages/activity-domain/tests/browser-ai-provider-fallback.fixtures.ts`
  holds reusable real contract fixtures so the main test file stays focused.

No-claim boundary:

- This proof does not execute a model.
- This proof does not call a remote provider.
- This proof does not run a policy evaluator.
- This proof does not render child or parent UI.
- This proof does not deliver warnings, holds, blocks, alerts, or enforcement.
