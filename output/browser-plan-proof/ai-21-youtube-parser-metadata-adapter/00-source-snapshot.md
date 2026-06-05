# AI-21 Source Snapshot

- YouTube parser and metadata adapter:
  - `packages/activity-domain/src/browser-url-intelligence.ts`
  - `packages/activity-domain/src/browser-youtube-metadata.ts`
  - `packages/activity-domain/tests/browser-youtube-metadata.test.ts`
  - `packages/activity-domain/package.json` focused subpath export:
    `@ocentra-parent/activity-domain/browser-youtube-metadata`
- Live proof harness:
  - `scripts/test/youtube-live-metadata-proof.mjs`
  - `test-results/youtube-live-metadata-proof/proof.json`
  - `output/browser-plan-proof/ai-21-youtube-parser-metadata-adapter/11-live-youtube-metadata-proof.json`
- Browser-plan documentation:
  - `docs/plans/browser-plan/implementation-checklist.md`
  - `docs/plans/browser-plan/v0-5-browser-url-video-ai-intelligence-plan.md`
  - `docs/features/browser-web-control.md`
  - `docs/expectations/browser-evidence.md`

Coordination note: AI-21 uses the focused package subpath export and does not
expand the already-large browser barrel.
