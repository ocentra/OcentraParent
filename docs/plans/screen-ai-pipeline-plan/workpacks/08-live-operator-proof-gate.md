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

## Remaining Non-Claims

- The harness does not own browser-plan managed URL trigger integration.
- Authenticated-account social surfaces remain separate; this proof uses
  operator-supplied public/live surfaces.
- The current live proof covers all required manifest rows and keeps raw images
  local and deleted after analysis. Product-complete claims still require the
  non-operator managed-browser trigger integration and remaining production
  adapters named in the implementation checklist.
