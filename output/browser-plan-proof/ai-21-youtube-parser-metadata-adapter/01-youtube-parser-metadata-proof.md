# AI-21 YouTube Parser Metadata Proof

AI-21 adds deterministic YouTube support for:

- watch URLs with `v`;
- Shorts URLs;
- embed URLs;
- live URLs;
- channel handles;
- playlist URLs.

The metadata adapter accepts only exact managed YouTube video, short, channel, or
playlist classifications. It emits `BrowserUrlMetadataEvidence` with supported
metadata refs and keeps `pageBodyCaptured`, `transcriptTextCaptured`,
`contentSemanticsClaimed`, `aiDecisionClaimed`, `policyDecisionClaimed`, and
`policyAuthorityClaimed` false.

Live proof now runs through `scripts/test/youtube-live-metadata-proof.mjs`. The
script fetches the real public YouTube watch page and YouTube oEmbed metadata
endpoint, requires live HTTP success plus watch-page metadata markers, runs the
built activity-domain URL parser and YouTube metadata adapter, and writes
machine-readable proof to:

- `test-results/youtube-live-metadata-proof/proof.json`
- `output/browser-plan-proof/ai-21-youtube-parser-metadata-adapter/11-live-youtube-metadata-proof.json`

The persisted proof stores live status, marker booleans, public platform id,
hashes, lengths, and no-claim flags. It does not persist raw watch-page HTML,
raw page body, transcript text, cookies, tokens, local storage, or raw title and
description strings.
