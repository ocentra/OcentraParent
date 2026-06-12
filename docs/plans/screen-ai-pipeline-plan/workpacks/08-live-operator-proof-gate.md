# 08 - Live Operator Proof Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `08 - Live Operator Proof Gate`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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

Current artifact gate:

```powershell
node scripts/test/screen-ai-live-operator-artifact-gate.mjs
```

The artifact gate rechecks the retained operator-run outputs without rerunning
capture. It requires all nine live rows, redacted live URL/title/text readiness
for browser rows, local VLM runtime evidence, policy dry-run handoff, parent
explanation screenshots, protected-surface non-claims, and raw image
deletion/no-retention custody.

## Checklist

- [x] Harness manifest template covers the required live operator scenarios.
- [x] Harness readiness proof records that live proof is not claimed without a
      manifest.
- [x] YouTube ordinary video.
- [x] YouTube or Vimeo education video.
- [x] Vimeo ordinary video.
- [x] Facebook/social surface.
- [x] Browser game/cloud-game surface.
- [x] Shopping page.
- [x] School/productivity page/app.
- [x] Native app.
- [x] Protected/unsupported state.

## Proof

- Harness readiness artifact.
- Partial live run artifact:
  `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`.
- Artifact gate summary:
  `output/screen-ai-pipeline-proof/live-operator-artifact-gate/proof-summary.json`.
- YouTube ordinary video source evidence:
  `output/screen-ai-pipeline-proof/live-operator/youtube-ordinary-video/01-redacted-source-evidence.json`.
- YouTube education video source evidence:
  `output/screen-ai-pipeline-proof/live-operator/youtube-education-video/01-redacted-source-evidence.json`.
- Vimeo ordinary video source evidence:
  `output/screen-ai-pipeline-proof/live-operator/vimeo-video/01-redacted-source-evidence.json`.
- Facebook/social source evidence:
  `output/screen-ai-pipeline-proof/live-operator/facebook-social-surface/01-redacted-source-evidence.json`.
- Browser game source evidence:
  `output/screen-ai-pipeline-proof/live-operator/browser-game/01-redacted-source-evidence.json`.
- Shopping page source evidence:
  `output/screen-ai-pipeline-proof/live-operator/shopping-page/01-redacted-source-evidence.json`.
- School/productivity source evidence:
  `output/screen-ai-pipeline-proof/live-operator/school-productivity/01-redacted-source-evidence.json`.
- Native app source evidence:
  `output/screen-ai-pipeline-proof/live-operator/native-app/01-redacted-source-evidence.json`.
- Protected/unsupported source evidence:
  `output/screen-ai-pipeline-proof/live-operator/protected-unsupported-state/01-redacted-source-evidence.json`.
- Operator scenario notes.
- Redacted URL/app evidence.
- Capture/analyze/policy artifacts.
- Portal screenshots.
- The YouTube live row records page readiness evidence before capture:
  expected hostname match, final redacted URL, title hash/length, visible text
  hash/length, and blank-page rejection. It passed with real local VLM category
  `video`, policy action `warn`, and raw image deletion proof.
- The education video row validates `school` and `allow`; the Vimeo row
  validates `video` and `warn`; the Facebook/social row validates `chat` and
  `warn`; the shopping row validates `shopping` and `ask-parent`; and the
  school/productivity row validates `school` and `allow`.
- The browser-game live row records page readiness evidence before capture,
  captures the selected browser window, validates local VLM category `game`,
  policy action `time-limit`, and raw image deletion proof.
- The native-app live row opens a real Notepad window with operator-supplied
  text, captures the active local window, validates local VLM category
  `productivity`, policy action `allow`, and raw image deletion proof.
- The protected/unsupported row consumes the protected-surface proof artifact,
  validates `protectedSurface`, and records no raw image, no AI analysis, and
  no policy decision claim.
- The artifact gate validates the live operator proof files can still support a
  PR-ready evidence claim after later main merges, while preserving the
  non-claim that it does not rerun the operator capture session.

## Remaining Non-Claims

- The harness does not own browser-plan managed URL trigger integration.
- Authenticated-account social surfaces remain separate; this proof uses
  operator-supplied public/live surfaces.
- The current live proof covers all required manifest rows and keeps raw images
  local and deleted after analysis. Product-complete claims still require the
  non-operator managed-browser trigger integration and remaining production
  adapters named in the implementation checklist.
