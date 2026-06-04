# 08 - Live Operator Proof Gate

## Target State

Before product-complete claim, the user or worker can run real URLs/apps and see
the product behavior, not just fixtures.

## Executable Gate

Use the manifest-driven live operator harness:

```powershell
node scripts/test/screen-ai-live-operator-proof.mjs --print-template
node scripts/test/screen-ai-live-operator-proof.mjs --verify-harness
node scripts/test/screen-ai-live-operator-proof.mjs --manifest <operator-manifest.json>
```

The harness:

- opens operator-supplied real URLs/apps;
- captures the focused local window using the real screen-capture adapter;
- runs the local VLM path and validates screen-analysis/local-AI/policy
  contracts;
- deletes raw image material after analysis;
- writes redacted source evidence, AI result, policy decision, deletion proof,
  and parent explanation artifacts under
  `output/screen-ai-pipeline-proof/live-operator/`;
- refuses to claim live proof when no manifest is supplied.

Current harness readiness proof:

```text
output/screen-ai-pipeline-proof/live-operator/harness-readiness/proof-summary.json
```

## Checklist

- [x] Harness manifest template covers the required live operator scenarios.
- [x] Harness readiness proof records that live proof is not claimed without a
      manifest.
- [ ] YouTube ordinary video.
- [ ] YouTube or Vimeo education video.
- [ ] Vimeo ordinary video.
- [ ] Facebook/social surface.
- [ ] Browser game/cloud-game surface.
- [ ] Shopping page.
- [ ] School/productivity page/app.
- [ ] Native app.
- [ ] Protected/unsupported state.

## Proof

- Harness readiness artifact.
- Operator scenario notes.
- Redacted URL/app evidence.
- Capture/analyze/policy artifacts.
- Portal screenshots.

## Non-Claims Until Manifest Run

- The harness is not proof that any live website or account was classified.
- The harness does not own browser-plan managed URL trigger integration.
- A product-complete live proof claim requires all required scenario rows to
  produce manifest-run artifacts and pass the expected category/action checks.
