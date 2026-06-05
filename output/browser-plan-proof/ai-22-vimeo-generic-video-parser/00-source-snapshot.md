# AI-22 Source Snapshot

Scope:

- `packages/activity-domain/src/browser-url-intelligence.ts`
- `packages/activity-domain/src/browser-video-metadata.ts`
- `packages/activity-domain/tests/browser-video-metadata.test.ts`
- `scripts/test/vimeo-generic-live-metadata-proof.mjs`

The parser recognizes managed exact Vimeo page and player URLs with numeric
video ids. The metadata adapter accepts exact managed Vimeo video rows or exact
managed generic web rows with schema.org VideoObject metadata, then emits
`BrowserUrlMetadataEvidence`.

Live proof branch:

- Branch: `codex/browser-vimeo-generic-live-metadata-proof`
- Base: `origin/main` at `1d2a625f0dfa88457eca1842b3443a8c9ecff50b`
- Before-state gap: AI-22 had contract/parser proof but no live Vimeo or live
  generic VideoObject capture evidence.

Live proof source surfaces:

- Public Vimeo page and Vimeo player page for the same numeric video id.
- Public TED page with live schema.org `VideoObject` JSON-LD.
- Persisted proof stores HTTP status, content type, byte length, host, path hash,
  metadata hashes/lengths, platform ids, and no-claim flags only.
- Persisted proof does not store raw HTML, raw page title, raw description,
  page body text, transcript text, cookies, tokens, local storage, or browser
  profile data.

Not claimed:

- network metadata fetcher
- transcript fetcher or parser
- hidden page load
- AI execution
- policy evaluation
- enforcement
- expanded browser barrel export
