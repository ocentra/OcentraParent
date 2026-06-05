# AI-22 Live Vimeo/Generic Video Proof

Live proof command:

```powershell
cmd /c node scripts/test/vimeo-generic-live-metadata-proof.mjs
```

Live source surfaces:

- Vimeo public page host: `vimeo.com`
- Vimeo public player host: `player.vimeo.com`
- Generic VideoObject page host: `www.ted.com`

Proof artifacts:

- `test-results/vimeo-generic-live-metadata-proof/proof.json`
- `output/browser-plan-proof/ai-22-vimeo-generic-video-parser/11-live-vimeo-generic-metadata-proof.json`

What is proved:

- Live Vimeo page and player requests return HTTP success.
- The Vimeo player page exposes live JSON-LD `VideoObject` metadata.
- A real generic web page exposes live schema.org `VideoObject` metadata.
- The built activity-domain URL parser classifies both Vimeo page/player URLs as
  exact managed `vimeo` video rows for public video id `76979871`.
- The built activity-domain metadata adapter emits available metadata evidence
  for Vimeo and generic VideoObject inputs.
- Negative checks reject generic OpenGraph-only metadata and unmanaged Vimeo
  rows.

Redaction:

- Persisted proof stores hashes, lengths, hosts, statuses, content types,
  platform ids, and no-claim booleans.
- Persisted proof does not store raw HTML, raw page title, raw description, raw
  transcript, page body, cookies, tokens, local storage, or browser profile
  data.

Non-claims:

- No production network fetcher is implemented.
- No transcript parser is implemented.
- No hidden managed page loader is implemented.
- No AI execution, policy evaluator, UI, browser mutation, or enforcement is
  claimed.
