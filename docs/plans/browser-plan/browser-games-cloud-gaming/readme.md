# Browser Games Cloud Gaming Workpacks

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Games Cloud Gaming Workpacks`
> Kind: short plan entry point.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This folder owns the focused browser-plan workpacks for managed-browser game
portals, HTML5/WebGL/canvas games, cloud-gaming web sessions, game account
flows, purchases, UGC/multiplayer risk, educational game classification, and
unmanaged browser-game bypass evidence. It turns the
[V0.5 Browser Games Cloud Gaming And Game Portal Gating Plan](../v0-5-browser-games-cloud-gaming-gating-plan.md)
into small implementation rows with proof roots.

## Source Boundaries

Browser-plan-owned:

- managed-browser exact URL route evidence for game portals and cloud-gaming
  surfaces;
- managed-browser page/runtime signals after a later adapter proves them;
- browser-game platform, route, runtime, metadata, AI, risk, memory, account,
  purchase, policy, and action candidate contracts;
- unmanaged browser game use as bypass/process evidence only;
- browser-plan proof packs for contract, parser, UI-not-applicable, manual, and
  rollout rows.

Adjacent, not owned here:

- native games, launchers, stores, install records, process/session duration,
  and owned-process game timers belong to app/game evidence;
- cloud-streamed frame analysis belongs to screen/vision proof before any
  scene/content claim;
- network/domain evidence may support platform hints but cannot identify exact
  game pages or titles alone;
- parent policy decisions belong to typed policy contracts and cannot be
  replaced by route evidence or AI output;
- child/parent visual surfaces need explicit UI proof before delivery claims.

## Workpack Map

| Row     | Boundary                                  | First Proof Root                                                             |
| ------- | ----------------------------------------- | ---------------------------------------------------------------------------- |
| GAME-01 | Plan folder and README                    | `output/browser-plan-proof/game-01-browser-game-plan-folder-readme/`         |
| GAME-02 | Browser game platform/route contracts     | `output/browser-plan-proof/game-02-browser-game-platform-route-contracts/`   |
| GAME-03 | Known browser game portal pattern library | `output/browser-plan-proof/game-03-known-game-portal-pattern-library/`       |
| GAME-04 | Cloud gaming pattern library              | `output/browser-plan-proof/game-04-cloud-gaming-pattern-library/`            |
| GAME-05 | Game URL shape parser                     | `output/browser-plan-proof/game-05-game-url-shape-parser/`                   |
| GAME-06 | Game runtime signal detector              | `output/browser-plan-proof/game-06-game-runtime-signal-detector/`            |
| GAME-07 | Game metadata extractor                   | `output/browser-plan-proof/game-07-game-metadata-extractor/`                 |
| GAME-08 | Hidden analysis profile safety for games  | `output/browser-plan-proof/game-08-hidden-analysis-profile-safety/`          |
| GAME-09 | Educational game classifier contract      | `output/browser-plan-proof/game-09-educational-game-classifier-contract/`    |
| GAME-10 | Browser game AI analysis contract         | `output/browser-plan-proof/game-10-browser-game-ai-analysis-contract/`       |
| GAME-11 | Game risk/benefit signal model            | `output/browser-plan-proof/game-11-game-risk-benefit-signal-model/`          |
| GAME-12 | Browser game memory/cache                 | `output/browser-plan-proof/game-12-browser-game-memory-cache/`               |
| GAME-13 | Game account/signup/purchase gating       | `output/browser-plan-proof/game-13-game-account-signup-purchase-gating/`     |
| GAME-14 | Cloud gaming gating                       | `output/browser-plan-proof/game-14-cloud-gaming-gating/`                     |
| GAME-15 | Unblocked game site detection             | `output/browser-plan-proof/game-15-unblocked-game-site-detection/`           |
| GAME-16 | UGC/multiplayer/chat risk model           | `output/browser-plan-proof/game-16-ugc-multiplayer-chat-risk-model/`         |
| GAME-17 | Parent game policy compiler               | `output/browser-plan-proof/game-17-parent-game-policy-compiler/`             |
| GAME-18 | Managed browser game hold/block adapter   | `output/browser-plan-proof/game-18-managed-browser-game-hold-block-adapter/` |
| GAME-19 | Child game checking/block UX              | `output/browser-plan-proof/game-19-child-game-checking-block-ux/`            |
| GAME-20 | Parent browser-game dashboard UX          | `output/browser-plan-proof/game-20-parent-browser-game-dashboard-ux/`        |
| GAME-21 | Journal/SQLite read model                 | `output/browser-plan-proof/game-21-journal-sqlite-read-model/`               |
| GAME-22 | Tests, fixtures, Playwright, manual proof | `output/browser-plan-proof/game-22-tests-fixtures-playwright-manual-proof/`  |
| GAME-23 | Android/iOS capability matrix             | `output/browser-plan-proof/game-23-android-ios-capability-matrix/`           |
| GAME-24 | Rollout and manual-required labels        | `output/browser-plan-proof/game-24-rollout-manual-required-labels/`          |

## Proof Rules

- Start every row from managed exact browser evidence or an explicit
  manual-required/unavailable state.
- Keep URL shape, runtime signal, metadata, AI analysis, risk signal, memory,
  policy decision, action, audit, and UI delivery as separate proof layers.
- Use `ui-not-applicable.md` for contract-only rows.
- Use screenshots only when a parent or child visual surface changes.
- Preserve no-claim language for native games, launchers, process/session time,
  platform stores, cloud-streamed frame analysis, unmanaged exact URL evidence,
  and game chat/content reading.

## Current State

GAME-01 only creates this workpack home and proof-root map. It does not add
schemas, parsers, runtime adapters, policy decisions, UI delivery, native
app/game support, cloud-streamed frame analysis, or enforcement.

GAME-02 now adds parent-domain browser-game platform/route contracts in
`packages/parent-domain/src/browser-game-platform-route-contracts.ts`. Rows
model browser-game portals, educational game sites, UGC game platforms,
cloud-gaming/cloud PC platforms, classic game archives, school game platforms,
and unknown/manual-required platforms through route surface kinds, route source
kinds, custody labels, pattern refs, evidence refs, confidence, and status. They
reject raw domains, raw URLs, raw paths, raw page bodies, live URL parser
claims, runtime detection claims, AI classification claims, policy decisions,
native game control, cloud-frame analysis, and enforcement. The live route proof
fetches real public CrazyGames, Poki, Coolmath Games, Xbox Cloud Gaming, itch.io
HTML5 catalog, and Chess.com play surfaces, stores only response metadata plus
hashed origin/path/body refs, parses six route contracts plus a reviewed
catalog, and rejects raw-data/runtime/parser/AI/policy/native/cloud-frame/
enforcement overclaims. Package subpath exports are now present.

GAME-03 now adds parent-domain browser-game portal pattern library contracts in
`packages/parent-domain/src/browser-game-portal-pattern-library.ts`. Entries
model known-game, educational, UGC, indie, classic archive, school, and unknown
portal families through route kinds, signal kinds, pattern fingerprints,
evidence refs, confidence, and review states. They reject raw domains, raw URLs,
raw page titles, raw page bodies, runtime detection claims, AI classification
claims, policy decisions, cloud-gaming ownership, and enforcement. Package
subpath exports are now present. GAME-03 also now includes live public-surface
proof in
`output/browser-plan-proof/game-03-known-game-portal-pattern-library/05-live-pattern-library-proof.json`
and
`test-results/browser-game-portal-pattern-library-live-evidence-proof/proof.json`.
That proof fetches real CrazyGames, Poki, Coolmath Games, itch.io HTML5,
Internet Archive MS-DOS games, and Chess.com play surfaces; stores only response
metadata plus hashed origin/path/body refs; parses six reviewed portal pattern
rows plus a reviewed library; and rejects 17 overclaims. It does not claim a
runtime portal detector, URL parser, AI classifier, UI, final policy, product
checklist update, release readiness, cloud-frame analysis, native game control,
or enforcement.

GAME-04 now adds parent-domain cloud-gaming pattern library contracts in
`packages/parent-domain/src/browser-game-cloud-pattern-library.ts`. Entries
model cloud-gaming platforms, cloud PC platforms, mobile cloud-game portals,
browser-embedded cloud-game surfaces, native launcher prompt bridges, and
unknown/manual-required cloud surfaces through route kinds, signal kinds,
pattern fingerprints, evidence refs, confidence, and review states. They reject
raw cloud domains, raw cloud URLs, raw cloud titles, raw stream frames, runtime
detection claims, cloud-streamed frame analysis, per-game cloud-title certainty,
native launcher/game control, final policy decisions, and enforcement. Package
subpath exports are now present. GAME-04 also now includes live public-surface
proof in
`output/browser-plan-proof/game-04-cloud-gaming-pattern-library/06-live-cloud-pattern-proof.json`
and
`test-results/browser-game-cloud-pattern-library-live-evidence-proof/proof.json`.
That proof fetches real Xbox Cloud Gaming, NVIDIA GeForce Now, Amazon Luna,
Boosteroid, PlayStation Plus games catalog, Shadow cloud PC, and now.gg
surfaces; stores only response metadata plus hashed origin/path/body refs;
parses seven reviewed cloud pattern rows plus a reviewed library; and rejects 20
overclaims. It does not claim a runtime detector, cloud-frame analyzer, native
launcher controller, UI, final policy, product checklist update, release
readiness, native game control, or enforcement.

GAME-05 now adds a parent-domain redacted URL-shape parser in
`packages/parent-domain/src/browser-game-url-shape-parser.ts`. It accepts
unknown input, uses the platform URL parser only transiently, and emits
protocol/host/path-depth shape, route surface kind, route hint booleans,
query/fragment shape booleans, reason codes, confidence, and a route-shape
fingerprint. It rejects raw URL, domain, path, and query storage; browser
navigation, runtime detection, AI classification, final policy decisions,
cloud-frame analysis, native game control, and enforcement. The live proof
fetches real public CrazyGames, Poki, Coolmath Games, Chess.com play, Xbox
Cloud Gaming play/cloud, and NVIDIA GeForce Now route surfaces; stores only
response metadata plus hashed origin/path/body refs; parses seven URL-shape
rows; and rejects 16 raw-storage/runtime/AI/policy/native/cloud-frame and
enforcement overclaims. Package subpath exports are now present.

GAME-06 now adds parent-domain runtime signal detector contracts in
`packages/parent-domain/src/browser-game-runtime-signal-detector.ts`. Signal
rows model canvas, WebGL, Gamepad API, fullscreen, pointer-lock, audio context,
animation-loop, iframe game surface, cloud-streaming, and unknown/manual-required
runtime shapes through fingerprints, evidence refs, source kinds, confidence,
status, and reason codes. They reject raw DOM, canvas frame, stream frame,
audio, and gamepad input storage; browser instrumentation, runtime detection
execution, AI classification, final policy decisions, cloud-frame analysis,
native game control, and enforcement. The live Playwright proof opens real
public Poki, Coolmath Games, Chess.com play, and Xbox Cloud Gaming pages in
Chromium; stores only response metadata, hashed origin/path refs, shape booleans,
and fingerprints; parses 12 runtime signal rows plus a detection bundle; and
rejects 27 raw-runtime-data/instrumentation, AI, policy, native, cloud-frame, and
enforcement overclaims. Package subpath exports are now present.

GAME-07 now adds parent-domain metadata extractor contracts in
`packages/parent-domain/src/browser-game-metadata-extractor.ts`. Field rows
model title, description, genre, age-rating, publisher, thumbnail, educational
subject, cloud platform title, and unknown/manual-required metadata shapes
through fingerprints, evidence refs, source kinds, confidence, status, and
reason codes. They reject raw title, description, page body, image, and
structured-data storage; runtime DOM extraction, platform API calls, AI
classification, final policy decisions, cloud-frame analysis, native game
control, and enforcement. The live proof fetches real public Poki, Coolmath
Games, Chess.com play, PlayStation Plus games catalog, and Xbox Cloud Gaming
pages; stores only response metadata, hashed origin/path/body refs, metadata
shape booleans, length buckets, and value hashes; parses 15 metadata field rows
plus an extraction bundle; and rejects 27 raw-metadata, DOM/API, AI, policy,
native, cloud-frame, and enforcement overclaims. Package subpath exports are now present.

GAME-08 now adds parent-domain hidden analysis profile safety contracts in
`packages/parent-domain/src/browser-game-hidden-analysis-profile-safety.ts` and
a live public safety-shape proof in
`scripts/test/browser-game-hidden-analysis-profile-safety-live-evidence-proof.mjs`.
Profile and loader rows model Ocentra-owned isolated profiles, bounded
retention, proof-backed metadata-only/analysis-ready states, disabled-policy,
proof-missing, manual-required, and unavailable states through evidence refs,
profile fingerprints, reason codes, confidence, and loader proof refs. The live
proof fetches real public Poki, Coolmath Games, Chess.com play, PlayStation Plus
games catalog, and Xbox Cloud Gaming pages; stores only response metadata,
hashed origin/path/body refs, profile fingerprints, loader proof refs, and
no-capture safety flags; validates 10 profile rows plus 10 planned/proof-backed
loader results; and rejects 40 child profile reuse, raw capture,
hidden/browser/native, AI, policy, UI, cloud-frame, and enforcement overclaims.
Package subpath exports are now present.

GAME-09 now adds parent-domain browser-game educational classifier contracts in
`packages/parent-domain/src/browser-game-educational-classifier.ts` and a live
public candidate-shape proof in
`scripts/test/browser-game-educational-classifier-live-evidence-proof.mjs`. The
classifier models evidence rows for domain reputation, school-provided URLs,
teacher/parent allowlists, page/subject metadata, AI classification refs, past
parent approval, homework context, school platforms, platform self-labels, and
manual-required states. The live proof fetches real public Code.org Minecraft,
Chess.com play, Coolmath Run 3, Poki Subway Surfers, and Xbox Cloud Gaming
pages; stores only response metadata, hashed origin/path/body refs, evidence
refs, classifier candidate rows, and no-authority flags; validates 9 evidence
rows plus 5 educational, entertainment, misleading-claim, and manual-required
candidate results; and rejects 23 raw content, model text, platform-authority,
account/purchase, policy, runtime, UI, native, cloud-frame, and enforcement
overclaims. Package subpath exports are now present.

GAME-10 now adds parent-domain browser-game AI analysis contracts in
`packages/parent-domain/src/browser-game-ai-analysis.ts` and a live public
AI-analysis shape proof in
`scripts/test/browser-game-ai-analysis-live-evidence-proof.mjs`. Inputs consume
typed evidence refs only for browser evidence, URL shape, runtime signals,
metadata, screen summaries, parent rules, recent activity, memory, task, and
custody labels. Results model game classification, educational check, risk
classification, cloud-gaming detection, UGC risk, purchase risk, and
policy-support outputs as candidate-only benefit/risk signals, surface kind,
modifiers, recommended policy input, confidence, uncertainty, summary refs,
model runtime refs, prompt template version, expiry, and degraded/manual states.
The live proof fetches real public Poki Subway Surfers, Code.org Minecraft,
Chess.com play, Xbox Cloud Gaming, and Roblox discover pages; stores only
response metadata, hashed origin/path/body refs, typed evidence refs, candidate
policy inputs, and no-authority flags; validates 5 inputs plus 5 candidate-only
results; and rejects 28 raw URL/page/game/frame/model text, account/purchase,
native, cloud-frame, policy, runtime, UI, and enforcement overclaims. Package
subpath exports are now present.

GAME-11 now adds parent-domain browser-game risk/benefit signal contracts in
`packages/parent-domain/src/browser-game-riskbenefit-signal.ts` and a live
public risk/benefit signal shape proof. The signal set models evidence-backed
risk rows for violence, horror, adult themes,
addictive loops, multiplayer/contact, chat, purchase, loot box/random item, UGC,
privacy, unblocked-bypass, and unknown risk; and benefit rows for educational
value, homework relevance, skill building, creativity, problem solving,
parent-approved game, neutral, and unknown benefit. It returns candidate
recommended policy inputs only and rejects raw game payloads, chat content, page
body, raw model text, account/purchase execution, cloud-frame analysis, native
game control, final policy decisions, runtime gate execution, and enforcement.
The live proof fetches real public Poki Subway Surfers, Code.org Minecraft,
Chess.com play, Xbox Cloud Gaming, and Roblox Discover pages; stores only
response metadata, hashed origin/path/body refs, typed evidence refs, candidate
risk/benefit signal rows, and no-authority flags; validates 5 signal sets with
7 risk signals and 8 benefit signals; and rejects 30 overclaims. Package
subpath exports are now present.

GAME-12 now adds parent-domain browser-game memory/cache contracts in
`packages/parent-domain/src/browser-game-memory-cache.ts` and a live public
memory/cache shape proof. The cache uses schema-backed ref/hash key kinds for
canonical URL refs, platform game refs,
domain path hashes, cloud game title refs, parent decision refs, game category
refs, policy versions, child profiles, parent rule sets, and evidence refs.
Entries model bounded fresh-hit, stale-hit, miss, and manual-required states
with short dynamic, cloud launcher, UGC, stable approved game, and
parent-approved account TTL classes. Fresh hits may be reused for policy input
only when decision refs, evidence refs, required subject keys, and bounded TTL
are present; stale/miss/manual rows cannot drive policy input. The contracts
reject raw canonical URLs, raw platform game IDs, raw cloud game titles, raw
game payloads, raw model text, runtime cache-store claims, AI cache claims, UI
delivery, native game control, cloud-frame analysis, final policy decisions, and
enforcement. The live proof fetches real public Poki Subway Surfers, Code.org
Minecraft, Chess.com play, Xbox Cloud Gaming, and Roblox Discover pages; stores
only response metadata, hashed origin/path/body refs, cache-key refs, evidence
refs, snapshots, and no-authority flags; validates 5 snapshots with 15 bounded
entries; and rejects 20 overclaims. Package subpath exports are now present.

GAME-13 now adds parent-domain browser-game account/signup/purchase gate
contracts in
`packages/parent-domain/src/browser-game-account-purchase-gate.ts`. The
contracts cover account creation, login, secondary account, purchase,
subscription, loot box/random item, virtual currency, download/install,
wallet/gambling-like payment, cloud-gaming start, and unknown-game start
approval states. They store evidence refs and candidate decisions only, and
reject raw URLs/titles/account identifiers, credentials, form submission,
account creation, purchase/payment execution, launcher downloads,
notifications, rendered UI, child notification, final policy decisions, runtime
gate execution, native game control, cloud-frame analysis, and enforcement.
The live proof fetches real public Roblox login, Roblox subscription, Steam app
purchase, Xbox Cloud Gaming, Code.org sign-in, and PlayStation store pages;
stores only response metadata, hashed origin/path/body refs, evidence refs,
request and decision refs, and no-authority flags; validates 6 approval
requests plus 6 candidate decisions; and rejects 38 overclaims. Package subpath
exports are now present.

GAME-14 now adds parent-domain browser-game cloud-gaming gate contracts in
`packages/parent-domain/src/browser-game-cloud-gaming-gate.ts` plus a live
public cloud-gaming route proof in
`scripts/test/browser-game-cloud-gaming-gate-live-evidence-proof.mjs`. The
contracts cover known cloud domains, streaming session routes,
gamepad/fullscreen/high-bandwidth/low-latency signal refs, optional platform
title/rating metadata refs, unknown cloud-game approval, mature cloud-game
block candidates, school-night blocks, time-budget candidates, manual-required
content-frame gaps, and unavailable platform proof states. The proof fetches
real Xbox Cloud Gaming, GeForce Now, Amazon Luna, Boosteroid, PlayStation Plus,
Shadow cloud PC, and now.gg surfaces, stores only response hashes, evidence
refs, request/decision refs, and no-authority flags, and rejects raw
URLs/titles/stream frames, cloud-streamed frame analysis, per-game cloud title
claims, native game/launcher control, game chat content, account/purchase
flows, notifications, rendered UI, child notification, final policy decisions,
runtime gate execution, and enforcement. Package subpath exports are present.

GAME-15 now adds parent-domain browser-game unblocked-site detection contracts
in `packages/parent-domain/src/browser-game-unblocked-site-detection.ts`. The
detection models managed browser routes/pages, search intent, portal indexes,
iframe embeds, proxy/mirror routes, hidden game origins, school bypass language,
unmanaged browser process-only bypass evidence, manual-required states, and
unavailable states. Candidate actions remain block-during-school, parent-review,
allow-specific-game, block-unknown-iframe, bypass-evidence-only, manual-review,
or unknown inputs. The contracts reject raw URLs, raw page body, raw search
queries, captured iframe content, exact unmanaged URL claims, native game
control, cloud-frame analysis, account/purchase flows, rendered UI, final
policy decisions, runtime gate execution, and enforcement. Package subpath exports are now present.

GAME-16 now adds parent-domain browser-game UGC/multiplayer/chat risk contracts
in `packages/parent-domain/src/browser-game-ugc-multiplayer-chat-risk.ts` and a
live public risk-shape proof in
`output/browser-plan-proof/game-16-ugc-multiplayer-chat-risk-model/02-live-ugc-multiplayer-chat-risk-shape-proof.json`.
The assessment models UGC game pages, experience pages, multiplayer lobbies,
profile/friends/message routes, launch prompts, web-to-app launch surfaces,
manual-required states, and unavailable states. Risk rows cover unknown player
contact, chat/voice contact, UGC worlds, unsafe user-created experiences,
off-platform contact, virtual currency, in-game purchase, missing age rating,
web-to-app launch risk, manual-required, and unknown risk. Recommended controls
remain candidates only: approved-experience-only, parent-review, block-chat,
time-limit, purchase approval, block unknown UGC, manual review, or unknown.
The live proof fetches real Roblox Discover, Scratch Games Explore, Minecraft
Marketplace, Chess.com online play, Steam Community chat, Rec Room, and Xbox
Cloud Gaming surfaces while persisting only response hashes, evidence refs, risk
row refs, and no-authority flags. The contracts and proof reject raw chat
content, profile content, experience identifiers, account identifiers, raw game
payloads, web-to-app launch execution, purchase execution, native game control,
final policy decisions, runtime gate execution, UI rendering, and enforcement.
Package subpath exports are now present.

GAME-17 now adds parent-domain browser-game policy compiler contracts in
`packages/parent-domain/src/browser-game-policy-compiler.ts` and a live public
compiler-shape proof in
`output/browser-plan-proof/game-17-parent-game-policy-compiler/02-live-policy-compiler-shape-proof.json`.
The compiler accepts parent-owned refs for browser-game evidence, analysis,
mobile capability, parent rule, and schedule context rows, then returns
candidate-only allow, warn, parent-review, block, time-limit, manual-review, or
unknown decisions. The live proof fetches real Code.org Minecraft, Poki Subway
Surfers, Roblox Discover, Coolmath Run, Hooda unblocked games, Rec Room, and
Internet Archive MS-DOS game surfaces while persisting only response hashes,
evidence refs, compiler input refs, candidate decision refs, and no-authority
flags. It rejects raw game payloads, raw model text, activity-domain object
transfer, final policy decision claims, runtime gate claims, UI claims, native
game control, cloud-frame analysis, and enforcement. Package subpath exports are
now present.

GAME-18 now adds parent-domain managed browser-game hold/block adapter
contracts in `packages/parent-domain/src/browser-game-hold-block-adapter.ts`
and a live public hold/block adapter proof in
`output/browser-plan-proof/game-18-managed-browser-game-hold-block-adapter/02-live-hold-block-adapter-shape-proof.json`.
The proof fetches real Scratch games, Roblox Discover, Hooda unblocked games,
Poki Subway Surfers, Code.org Minecraft, Coolmath Run, Xbox Cloud Gaming, and
Steam Store surfaces, persists only response hashes, evidence refs, adapter plan
refs, and no-authority flags, and parses managed hold, approval, block,
warning, candidate-only allow/time-limit, manual-required cloud, and unavailable
native rows. It rejects raw URL/page/game payloads, child cookie/session reuse,
unmanaged exact URL claims, browser mutation, rendered child pages,
notification delivery, final policy decisions, applied time limits,
cloud-frame analysis, native game control, and enforcement. Package subpath exports are now present.

GAME-19 now adds parent-domain browser-game child checking/block UX contracts in
`packages/parent-domain/src/browser-game-child-checking-block-ux.ts` and a live
public child UX surface proof in
`output/browser-plan-proof/game-19-child-game-checking-block-ux/02-live-child-checking-block-ux-shape-proof.json`.
The proof fetches real Scratch games, Roblox Discover, Hooda unblocked games,
Code.org Minecraft, Coolmath Run, Xbox Cloud Gaming, and Steam Store surfaces,
persists only response hashes, evidence refs, child UX surface refs, and
no-authority flags, and parses checking, approval, blocked, educational
allowed, time-limit, cloud-gaming manual-required, and native unavailable rows.
It rejects raw child copy, rendered child UI, notification delivery, browser
navigation block, block-page rendering, applied time limits, final policy
decisions, cloud-frame analysis, native game control, and enforcement. Package subpath exports are now present.

GAME-20 now adds parent-domain parent browser-game dashboard UX contracts in
`packages/parent-domain/src/browser-game-dashboard-ux.ts` and a live public
dashboard panel proof in
`output/browser-plan-proof/game-20-parent-browser-game-dashboard-ux/02-live-dashboard-ux-shape-proof.json`.
The proof fetches real Scratch games, Roblox Discover, Xbox Cloud Gaming,
Code.org Minecraft, Coolmath Run, Steam Store, and Rec Room surfaces, persists
only response hashes, evidence refs, dashboard panel refs, and no-authority
flags, and parses detected-game, approval queue, cloud approval, educational
allowlist, time-budget, mobile/native gap, and manual-required gap panels. It
rejects rendered portal UI, notification delivery, runtime data fetch, final
policy decisions, cloud-frame analysis, native game control, and enforcement. Package subpath exports are now present.

GAME-21 now adds parent-domain browser-game journal/SQLite read-model contracts
in `packages/parent-domain/src/browser-game-journal-sqlite-read-model.ts` plus
a live public evidence-backed read-model shape proof. The snapshot indexes
managed browser evidence journal replay, app-game session
report proof, managed browser-game adapter audit refs, manual-required
cloud-gaming rows, and unavailable native/unmanaged rows. Proof-backed rows
require journal entry refs, SQLite row refs, source read-model refs, proof refs,
positive event/row counts, and matching reason codes. They reject raw
URL/page/game/title/account/purchase storage, child cookie/session reuse, cloud
title certainty, browser mutation, rendered UI, final policy decisions, and
enforcement. The live proof persists only response hashes, origin/path hashes,
source refs, and no-claim flags; it does not claim a runtime SQLite query,
rendered browser-game UI, Playwright screenshot, product checklist upgrade, or
release readiness. Package subpath exports are now present.

GAME-22 now adds `scripts/test/browser-game-proof-artifacts.mjs` and
`scripts/test/browser-game-child-intervention-page-proof.mjs`. The artifact gate
validates GAME-01 through GAME-23 checklist ownership, proof directory
references, required proof files, README references, feature coverage, and
expectation boundary text, then writes
`test-results/browser-game-proof-artifacts/proof.json` and
`output/browser-plan-proof/game-22-tests-fixtures-playwright-manual-proof/01-browser-game-proof-artifact-manifest.md`.
The rendered proof opens real public Roblox, Coolmath Games, Scratch, Xbox Cloud
Gaming, and Steam Store surfaces through Playwright, captures live backdrops,
renders the shared BrowserChildInterventionPage, serves it through the Rust
child-agent `/api/browser/intervention/page` endpoint, and stores screenshots
plus hash-only proof JSON in
`test-results/browser-game-child-intervention-page-proof/proof.json`,
`output/browser-plan-proof/game-22-tests-fixtures-playwright-manual-proof/02-rendered-browser-game-child-intervention-proof.json`,
and
`output/browser-plan-proof/game-22-tests-fixtures-playwright-manual-proof/06-ui-snapshots/`.

GAME-23 now adds parent-domain Android/iOS browser-game capability matrix
contracts in
`packages/parent-domain/src/browser-game-android-ios-capability-matrix.ts` plus
a real Android host/emulator proof. The proof builds the Android agent APK,
boots or reuses an Android 15 emulator, installs and launches the package,
observes the running agent status through UIAutomator tree evidence, hashes
UI/package/device evidence, and queries known browser package targets without storing a raw
package list. The matrix records owned-browser-shell, WebView, Custom Tabs,
installed-browser, cloud-gaming, device-owner, Family Controls, Safari
web-domain token, application-token, managed-browser, and Web Clip/PWA surfaces
as manual-required, token-limited, entitlement-required, app-level, or
domain-level only. It rejects exact game content, cloud-streamed frame analysis,
native game/launcher control, game chat, per-game cloud title detection,
runtime signals, app-store or purchase control, UI delivery, and enforcement.
Package subpath exports are now present.

GAME-24 now adds the browser-game rollout/manual-required gate in
`scripts/test/browser-game-rollout-gate.mjs`. GAME rollout state:
partial/manual-required. The gate labels GAME-01 as scaffold-proof-present,
GAME-02 as live-route-proof-present, GAME-03 as
live-portal-pattern-proof-present, GAME-04 as live-cloud-pattern-proof-present,
GAME-05 as live-url-shape-proof-present, GAME-06 as
live-runtime-signal-shape-proof-present, GAME-07 as
live-metadata-shape-proof-present, GAME-08 as
live-hidden-analysis-profile-safety-proof-present, GAME-09 as
live-educational-classifier-proof-present, GAME-10 as
live-ai-analysis-proof-present, GAME-11 as
live-riskbenefit-signal-proof-present, GAME-12 as
live-memory-cache-proof-present, GAME-13 as
live-account-purchase-gate-proof-present, GAME-14 as
live-cloud-gaming-gate-proof-present, GAME-15 as
live-unblocked-site-detection-proof-present, GAME-16 as
live-ugc-multiplayer-chat-risk-proof-present, GAME-17 as
live-policy-compiler-proof-present, GAME-18 as
live-hold-block-adapter-proof-present, GAME-19 as
live-child-checking-block-ux-proof-present, GAME-20 as
live-parent-dashboard-ux-proof-present, GAME-21 as
live-journal-sqlite-read-model-proof-present, GAME-22 as
live-rendered-child-intervention-proof-present, GAME-23 as
live-android-ios-host-proof-present, GAME-24 as
rollout-label-proof-present, and no browser-game rows as open/manual-required.
Product checklist upgrade is not claimed; final policy decisions, notification
or approval delivery, parent dashboard runtime UI, cloud-streamed frame
analysis, native game control, and enforcement remain open or manual-required
until separate proof exists.
