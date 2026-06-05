# AI-23 Dynamic Feed/Social Live Proof

Command:

- `cmd /c node scripts/test/dynamic-social-live-url-proof.mjs`

Artifacts:

- `scripts/test/dynamic-social-live-url-proof.mjs`
- `test-results/dynamic-social-live-url-proof/proof.json`
- `output/browser-plan-proof/ai-23-dynamic-feed-social-url-handling/11-live-dynamic-social-url-proof.json`

Live route surfaces:

- Instagram explore, reel, direct inbox, and create-post routes.
- TikTok For You and live routes.
- Facebook Watch and live routes.
- Twitch following route.
- X/Twitter home, search, and status routes.
- Reddit root, community, and comment routes.
- Discord channel route.

What this proves:

- Real public route URLs are reachable with live non-5xx HTTP responses.
- The built activity-domain parser classifies those exact managed URLs as
  social feed, short-video, social messaging, upload/post, social livestream,
  search, social post, or forum route shapes as applicable.
- Exact public route ids visible in the URL, such as post ids, channel ids, and
  search query text, are carried as URL-shape evidence only.
- Dynamic feed route rows carry `dynamic-feed` and `parsed-social-route`
  reason codes without claiming feed recommendations or page content semantics.

Privacy and no-claim boundary:

- Proof output persists redacted status, content type, byte length, hashes,
  redirect host/path hash, typed classification, route ids already visible in
  the URL, and no-claim flags.
- Proof output does not persist raw HTML, raw title, raw description, cookies,
  account identity, messages, feed content, upload data, livestream media, or
  credentials.
- This is not an AI decision, policy decision, connector/native app proof, UI
  proof, or enforcement proof.
