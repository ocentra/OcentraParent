# AI-22 Vimeo/Generic Video Proof

Contract proof:

- Vimeo page URLs such as `https://vimeo.com/123456` classify as managed exact
  video rows with platform `vimeo` and a numeric platform video id.
- Vimeo player URLs such as `https://player.vimeo.com/video/789012` classify as
  managed exact video rows with platform `vimeo` and a numeric platform video id.
- `buildVideoMetadataEvidence` accepts exact managed Vimeo video
  classifications and emits `BrowserUrlMetadataEvidence` with schema.org
  `video-object` metadata fields.
- `buildVideoMetadataEvidence` accepts exact managed generic web
  classifications only when the metadata source is `schema-org-video-object` and
  at least one video metadata field is present.
- Partial Vimeo metadata remains usable as a partial metadata state with
  degraded reasons.

Live proof:

- `cmd /c node scripts/test/vimeo-generic-live-metadata-proof.mjs`
- Fetches a real public Vimeo page, real public Vimeo player page, and a real
  public generic web page with schema.org `VideoObject` JSON-LD.
- Exercises the built `parseBrowserUrlShape` and `buildVideoMetadataEvidence`
  implementations from `packages/activity-domain/dist`.
- Writes redacted machine proof to
  `test-results/vimeo-generic-live-metadata-proof/proof.json` and
  `output/browser-plan-proof/ai-22-vimeo-generic-video-parser/11-live-vimeo-generic-metadata-proof.json`.
- Proves Vimeo page and player URLs classify as exact managed `vimeo` video
  rows for public video id `76979871`.
- Proves the live generic VideoObject page builds metadata evidence with
  `metadataState=available` without raw page body, raw title, raw description,
  transcript text, AI decision, policy decision, or enforcement claims.

Validation:

- `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-video-metadata.test.ts`
- `cmd /c npm run build:contracts`
- `cmd /c node --check scripts/test/vimeo-generic-live-metadata-proof.mjs`
- `cmd /c node scripts/test/vimeo-generic-live-metadata-proof.mjs`
