# AI-23 Dynamic Feed/Social URL Proof

Contract proof:

- Dynamic social feeds classify as `social-feed` with `dynamic-feed` and
  `parsed-social-route` reason codes from managed exact URL evidence.
- Instagram reels and X/Twitter status URLs carry visible post ids without
  claiming content semantics.
- Instagram direct routes classify as `social-messaging`.
- Instagram create routes classify as `social-upload-post`.
- TikTok live routes classify as `social-livestream`.
- X/Twitter search routes carry the visible query string as URL-shape evidence.
- Unmanaged social URLs stay `unknown` and non-exact.
- Social post rows without a post id are rejected.
- Dynamic-feed TTL stale memory rows cannot drive policy input.

Validation:

- `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-url-intelligence.test.ts`
- `cmd /c npm run build:contracts`
- `cmd /c node --check scripts/test/dynamic-social-live-url-proof.mjs`
- `cmd /c node scripts/test/dynamic-social-live-url-proof.mjs`

Live proof artifacts:

- `test-results/dynamic-social-live-url-proof/proof.json`
- `output/browser-plan-proof/ai-23-dynamic-feed-social-url-handling/11-live-dynamic-social-url-proof.json`

The live proof fetches 16 real public route surfaces across Instagram, TikTok,
Facebook, Twitch, X/Twitter, Reddit, and Discord. The persisted evidence keeps
only response status, content type, byte length, route/path/query/body hashes,
title hashes/lengths, redirect host/path hash, parser classification, and
no-claim flags. It does not store raw HTML, title text, descriptions, cookies,
account identity, feed/message/upload/livestream content, or credentials.
