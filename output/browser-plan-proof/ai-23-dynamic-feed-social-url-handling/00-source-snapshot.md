# AI-23 Source Snapshot

Base:

- `origin/main` `1d2a625f0dfa88457eca1842b3443a8c9ecff50b`
- worker branch `codex/browser-dynamic-social-live-proof`

Scope:

- `packages/activity-domain/src/browser-url-intelligence-schemas.ts`
- `packages/activity-domain/src/browser-url-intelligence.ts`
- `packages/activity-domain/tests/browser-url-intelligence.test.ts`
- `scripts/test/dynamic-social-live-url-proof.mjs`

The URL shape contract now includes social post, messaging, upload/post, and
livestream route targets alongside social feed. The deterministic parser
recognizes visible social route shapes for managed exact URL evidence from
Instagram, TikTok, Facebook, Twitch, X/Twitter, Reddit, and Discord.

Live proof:

- `cmd /c node scripts/test/dynamic-social-live-url-proof.mjs`
- `test-results/dynamic-social-live-url-proof/proof.json`
- `output/browser-plan-proof/ai-23-dynamic-feed-social-url-handling/11-live-dynamic-social-url-proof.json`

The live proof fetches real public route surfaces for Instagram, TikTok,
Facebook, Twitch, X/Twitter, Reddit, and Discord, then exercises the built
activity-domain parser against those URLs. Persisted evidence is redacted to
host names, response status, content type, byte length, route/path/query/body
hashes, title hashes/lengths, redirect host/path hash, typed route
classification, and no-claim flags. No raw page HTML, raw title, raw
description, cookie, account, message, feed, upload, livestream, or credential
data is persisted.

Not claimed:

- account identity proof
- feed recommendation analysis
- messaging/contact analysis
- upload monitoring
- livestream content analysis
- UI rendering
- enforcement
- expanded browser barrel export
