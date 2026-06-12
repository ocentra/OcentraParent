# V0.5 Browser Games Cloud Gaming And Game Portal Gating Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `V0.5 Browser Games Cloud Gaming And Game Portal Gating Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This document defines Ocentra's browser-game control plan. It belongs in the
browser plan when the source is a managed browser URL/page/runtime signal,
browser game portal, cloud-gaming web session, unmanaged browser bypass, or
browser policy gate. Native games, launchers, process/session duration, broad
app blocking, and platform stores remain app/game-control boundaries and must be
linked rather than duplicated.

This is a plan document only. It does not claim exact game content safety,
cloud-streamed frame analysis, native-app scene control, game chat reading,
per-game cloud title detection, or broad game blocking until evidence, policy,
adapter, custody, and audit proof exist.

## External Game And Cloud Context

The pasted enhancement included current background context. Treat it as
competitive/product pressure, not as Ocentra source truth:

- [ESRB ratings guidance](https://www.esrb.org/ratings/ratings_guide.aspx)
  includes interactive elements such as users interacting, digital purchases,
  random items, shares information/location, and unrestricted internet. Those
  concepts map well to Ocentra game risk signals.
- [Xbox Cloud Gaming](https://www.xbox.com/en-US/cloud-gaming) and
  [NVIDIA GeForce NOW system requirements](https://www.nvidia.com/en-gb/geforce-now/system-reqs/)
  show that cloud gaming is browser-accessible on supported platforms and
  browsers, making it a real browser-control surface.
- [Network Anatomy and Real-Time Measurement of Nvidia GeForce NOW Cloud
  Gaming](https://arxiv.org/abs/2401.06366) frames cloud gaming as rendered in
  the cloud and streamed back as real-time video, which supports special network
  and runtime detection.
- [User-Generated Content and Editors in Games: A Comprehensive
  Survey](https://arxiv.org/abs/2412.13743) supports treating UGC games as their
  own evidence-sensitive category.
- [Los Angeles County's 2026 Roblox lawsuit announcement](https://lacounty.gov/2026/02/19/la-county-sues-roblox-for-unfair-and-deceptive-business-practices-that-endanger-and-exploit-children/)
  is public risk context for UGC/multiplayer platforms. The allegations are not
  Ocentra proof; the product response is to model UGC multiplayer risk carefully
  with evidence, policy, and audit.

Refresh these links before using them in marketing, PR copy, or product claims.

## Covered Surfaces

This plan covers:

```text
browser game portals
HTML5 games
WebGL games
canvas games
iframe/embedded games
unblocked game sites
school game sites
educational games
Roblox web launch flows
Minecraft web/account launch flows
cloud gaming
Xbox Cloud Gaming
GeForce NOW
Amazon Luna
Boosteroid
Steam web/remote play surfaces
game streaming sites
itch.io web games
CrazyGames
Poki
Miniclip
Coolmath Games
Kongregate-style portals
new/unknown game portals
game account creation
game login
microtransactions
loot boxes
multiplayer chat
user-generated games
violent/adult/unsafe game content
AI classification
parent approval
time budgets
evidence
tests and proof
```

Browser games are not just websites. They can be entertainment, educational
tools, multiplayer social spaces, UGC platforms, cloud-streamed mature games,
microtransaction systems, chat/contact systems, unblocked school bypasses, and
downloads/install launchers.

## 1. Core Product Rule

```text
Browser URL evidence proves what game page or game platform was opened.
Game intelligence classifies what type of game or gaming surface it appears to be.
Parent policy decides whether to allow, warn, ask, limit, or block.
Enforcement applies only where the browser/app adapter is proved.
```

Never claim:

```text
This exact game content is safe
```

from only:

```text
domain
URL
title
canvas/WebGL detection
network traffic
```

Say instead:

```text
Likely browser game / game portal / cloud gaming session.
Evidence: managed browser URL, page title, canvas/WebGL signals, platform metadata.
```

## 2. Why Browser Games Need Separate Handling

Browser games have risks that differ from normal web/video/social:

```text
time sink / addictive loops
violent or adult game themes
user-generated game worlds
chat and contact with strangers
in-game purchases and loot boxes
account creation
download/install prompts
cloud gaming mature titles
school bypass/unblocked-game portals
fake educational labels
embedded iframes hiding the real game origin
gamepad/fullscreen/pointer-lock behavior
```

A simple `block games` rule is too crude. A child may use browser games for math
practice, typing practice, coding games, chess, educational simulation, casual
gaming, violent shooters, casino-like games, cloud-streamed mature games, or
social UGC worlds. Ocentra must distinguish those surfaces.

## 3. Product Differentiator

Most parental controls can block game sites, limit app/game time, use store
ratings, or rely on platform parental controls. Ocentra should add:

```text
managed-browser exact game URL evidence
browser-game shape detection
game portal detection
canvas/WebGL/game-engine signals
cloud gaming detection
educational-vs-entertainment classification
UGC/multiplayer/chat risk signals
microtransaction/purchase risk signals
AI-assisted game classification
memory of known game URLs/titles/platforms
parent approval for new game accounts
parent approval for cloud gaming
time budget by game type/platform/risk
explainable evidence and audit
```

## 4. Browser Game Types

Ocentra should classify browser-game surfaces into these types:

```text
educational_game
casual_game
puzzle_game
strategy_game
arcade_game
shooter_game
fighting_game
horror_game
casino_like_game
sports_game
racing_game
simulation_game
coding_game
typing_game
chess_or_board_game
multiplayer_game
user_generated_game
cloud_gaming_session
game_portal
game_search_page
game_download_or_launcher
game_account_signup
game_store_or_purchase
unknown_game
```

Modifiers:

```text
webgl
canvas
iframe_embedded
fullscreen
pointer_lock
gamepad
keyboard_controlled
cloud_streamed
multiplayer
chat_available
voice_available
user_generated_content
in_game_purchases
loot_boxes_or_random_items
unrestricted_internet
account_required
login_required
unknown_rating
school_educational_claim
unblocked_game_site
```

## 5. Evidence Layers

### Layer 1: URL And Domain Shape

Detect:

```text
known game portal domain
known cloud gaming domain
known educational game domain
game path patterns
play route
game id
iframe embed route
account signup route
store/purchase route
download/launcher route
```

Example platforms/domains:

```text
roblox.com
now.gg
xbox.com/play
play.geforcenow.com
luna.amazon.com
boosteroid.com
itch.io
poki.com
crazygames.com
coolmathgames.com
miniclip.com
kongregate.com
armorgames.com
newgrounds.com
chess.com
lichess.org
code.org games
typing.com games
```

Contract family:

```text
BrowserGameUrlShape
BrowserGamePlatform
BrowserGameRouteKind
BrowserGamePlatformIds
```

Route kinds:

```text
play_game
game_detail
game_portal
cloud_game_session
game_search
account_signup
login
purchase
download_launcher
leaderboard
multiplayer_lobby
chat
unknown
```

### Layer 2: Browser Runtime Signals

Inside a managed browser, detect game-like behavior:

```text
canvas element
WebGL context
WebGPU context
fullscreen request
pointer lock request
gamepad API usage
high animation frame activity
keyboard capture pattern
audio context
large asset bundles
wasm module loaded
iframe game embed
cloud video stream + gamepad/pointer input
```

Contract family:

```text
BrowserGameRuntimeSignals
BrowserGameRuntimeReasonCode
```

Expected fields:

```text
evidence id
source browser evidence id
has canvas
has WebGL
has WebGPU
has wasm
requested fullscreen
requested pointer lock
gamepad API used
keyboard control pattern
audio context used
iframe embedded game
cloud streaming pattern
confidence
reason codes
observed at
```

Important:

```text
Canvas/WebGL means likely interactive/game/media, not guaranteed game.
```

Use runtime signals as supporting evidence, not sole proof.

### Layer 3: Metadata Extraction

Extract:

```text
page title
game title
description
OpenGraph metadata
schema.org game/app metadata
creator/publisher
platform category
rating if available
tags
thumbnail hash
multiplayer/chat indicators
purchase indicators
```

Contract family:

```text
BrowserGameMetadataEvidence
BrowserGameRatingSource
BrowserGameInteractiveElement
```

Interactive elements should include:

```text
users_interact
in_game_purchases
random_items
shares_info
shares_location
unrestricted_internet
```

ESRB-style interactive elements map well to Ocentra because they distinguish
users interacting, purchases, random items/loot boxes, sharing info/location,
and unrestricted internet access.

### Layer 4: Hidden Analysis Load

For a new or unknown browser game, Ocentra may load the game page in a hidden
managed analysis profile.

Purpose:

```text
extract metadata
detect canvas/WebGL/game runtime signals
detect account/purchase/chat prompts
classify educational vs entertainment vs risky
avoid interrupting child immediately unless strict policy says hold
```

Safety rules:

```text
use Ocentra analysis profile
do not use child cookies/session
no payment info
no login automation
no account creation
no game play automation beyond safe metadata load
no chat interaction
no purchases
strict timeout
block downloads by default
no permanent raw screenshot unless separate screen-evidence setting allows
```

### Layer 5: Screen/OCR/Vision Summary

Optional future. Useful for:

```text
canvas-only games with no metadata
violent/horror/adult visual content
UGC game thumbnails
cloud gaming streams
Roblox-like UGC worlds
```

This must be separately permissioned and proved.

### Layer 6: Network/Traffic Signals

Useful for:

```text
cloud gaming bandwidth
game CDN endpoints
multiplayer websocket connections
launcher/download endpoints
voice/chat servers
```

Not enough for:

```text
exact game content
exact game title
age rating
current scene
chat content
```

Cloud gaming can look like video streaming plus input. Network research on
GeForce NOW supports modeling cloud gaming as graphics rendered in the cloud and
streamed back as real-time video; the pasted scope also calls out high-bandwidth
behavior, around 10-20 Mbps in some cloud-gaming contexts.

## 6. Browser Game AI Analysis

AI consumes typed evidence only.

Input contract family:

```text
BrowserGameAiAnalysisInput
BrowserGameAiTask
```

Input fields:

```text
request id
child profile ref
device ref
source evidence refs
browser evidence ref
URL shape ref
runtime signal ref
metadata evidence refs
screen summary refs
parent rule refs
recent activity ref
memory refs
task
custody label
```

Tasks:

```text
game_classification
educational_game_check
risk_classification
cloud_gaming_detection
ugc_game_risk
purchase_risk
policy_support
```

Output contract family:

```text
BrowserGameAiAnalysisResult
BrowserGameBenefitSignals
BrowserGameRiskSignals
```

Output must include:

```text
analysis id
request id
source evidence refs
parent rule refs
is game
game surface kind
modifiers
benefit signals
risk signals
recommended policy input
confidence
uncertainty reason codes
parent summary
child-safe summary where appropriate
model runtime ref
prompt template version
analyzed at
expires at
degraded state
```

Benefit signals:

```text
educationalValue
homeworkRelevance
skillBuilding
creativity
problemSolving
```

Risk signals:

```text
violence
horror
adult
gambling
addictionLoop
multiplayerContact
chatRisk
purchaseRisk
lootBoxRisk
userGeneratedContentRisk
privacyRisk
unknownRisk
```

AI cannot enforce directly.

## 7. Parent Policy Targets

Policy target kinds:

```text
all_browser_games
game_platform
game_portal
specific_game_url
game_id
educational_games
cloud_gaming
webgl_games
canvas_games
multiplayer_games
ugc_games
game_chat
game_purchases
loot_box_or_random_items
violent_games
casino_like_games
unknown_games
unblocked_game_sites
```

Actions:

```text
allow
observe
warn
time_limit
ask_parent
block
hold_until_classified
hold_until_parent_approval
block_download
block_purchase
block_account_creation
require_managed_browser
manual_required
```

## 8. Game Account And Purchase Gating

Browser games often push account creation and purchases.

Gate:

```text
game account signup
login to unknown game account
secondary game account
purchase page
subscription page
loot box/random item purchase
virtual currency purchase
download launcher
install prompt
connect wallet / crypto / gambling-like payment
```

Parent policy:

```text
Require parent approval for new game accounts.
Require parent approval for game purchases.
Block loot boxes/random items.
Block game downloads/installers.
Allow educational game accounts only after parent approval.
```

Contract family:

```text
BrowserGameApprovalRequest
BrowserGameApprovalDecision
BrowserGameApprovalRequestKind
```

Request kinds:

```text
game_account_creation
game_purchase
loot_box_purchase
game_download
cloud_gaming_start
unknown_game_start
```

Approval request fields:

```text
request id
child profile ref
device ref
request kind
platform
game title
URL
evidence refs
AI analysis ref
confidence
reason codes
requested at
expires at
```

## 9. Cloud Gaming

Cloud gaming deserves special handling because the browser URL may be only the
launcher/stream while the actual game is streamed.

Examples:

```text
Xbox Cloud Gaming
GeForce NOW
Amazon Luna
Boosteroid
PlayStation cloud surfaces if web-accessible
Shadow/cloud PC gaming
now.gg app streaming
```

Detect:

```text
known cloud gaming domain
streaming session route
gamepad API usage
fullscreen/pointer lock
high bandwidth stream
low-latency websocket/webRTC pattern
game title metadata if platform exposes it
```

Policy:

```text
allow cloud gaming only during free time
ask parent before starting unknown cloud game
block mature cloud games unless approved
limit cloud gaming to X minutes
block cloud gaming on school nights
```

Limitation:

```text
Cloud gaming stream may hide exact game content from browser metadata.
If game title/rating is not captured, mark unknown_cloud_game and apply parent fallback.
```

## 10. Educational Game Handling

Educational games should not be blocked just because they are games.

Classification evidence:

```text
domain reputation
school-provided URL
teacher/parent allowlist
page title/description
subject metadata
AI classification
past parent approval
session context: homework/school mode
```

Educational categories:

```text
math
science
coding
typing
language
history
art
chess/logic
problem solving
school platform
```

Policy examples:

```text
Allow educational games during homework.
Allow coding games.
Allow chess up to 30 minutes.
Ask parent for unknown educational claims.
Block game portals pretending to be educational until reviewed.
```

## 11. Game Portal And Unblocked Sites

Unblocked games sites are a special bypass category.

Detect:

```text
domain contains unblocked/game/proxy/play
page lists many games
iframe embeds external game
URL is mirror/proxy
game portal with hidden origin
search terms: unblocked games
```

Policy:

```text
block unblocked game portals during school/homework
ask parent for unknown game portals
allow specific games only
block if iframe origin is unknown
```

Do not overclaim:

```text
Game portal detection does not classify every embedded game.
```

## 12. Roblox And UGC Game Platforms

Roblox is both native app and web platform.

Browser web can detect:

```text
roblox.com game page
experience/game id
account/login/signup
launch prompt
web-to-app launch
profile/friends/messages where visible
```

Native Roblox app needs app/game evidence.

Risks:

```text
UGC worlds
unknown players
chat/contact
virtual currency
in-game purchases
unsafe user-created experiences
off-platform grooming
```

Recent public reporting and lawsuits around Roblox focus on
grooming/exploitation concerns and parental-control gaps. Whether allegations
are legally proven, disputed, or resolved later, Ocentra should treat UGC
multiplayer game platforms as high-risk, evidence-sensitive surfaces.

Policy examples:

```text
Allow approved Roblox experiences only.
Ask parent for new Roblox experience.
Block Roblox chat/messaging where adapter supports it.
Limit Roblox to X minutes.
Block unknown UGC games.
Require parent approval for Robux/purchases.
```

## 13. Runtime Modes

Mode A: Observe Only.

```text
Record browser game use.
Classify.
Report to parent.
No blocking.
```

Mode B: Allow Known, Review Unknown.

```text
Known educational games allowed.
Known blocked games blocked.
Unknown games queued for AI/parent review.
```

Mode C: Hold Unknown Games.

```text
Unknown browser games show checking screen.
AI/memory/policy decides.
Timeout fallback follows parent setting.
```

Mode D: Strict Game Mode.

```text
Block unknown games.
Allow only approved educational/game list.
Ask parent for new games.
Block cloud gaming by default.
Block game portals by default.
```

Mode E: Time Budget Mode.

```text
Allow games but count time.
Apply daily/weekly budget.
Warn near limit.
Stop at limit.
```

## 14. Memory And Cache

Cache classifications by:

```text
canonical URL
game portal + game id
platform + game id
domain + path hash
title + metadata hash
cloud platform + game title
parent approval decision
AI analysis id
policy version
```

Contract family:

```text
BrowserGameMemoryHit
BrowserGameMemoryKeyType
BrowserGameMemoryStaleReason
```

Key types:

```text
canonical_url
platform_game_id
domain_path_hash
cloud_game_title
parent_decision
game_category
```

Short TTL:

```text
game portals
dynamic pages
cloud gaming launcher pages
UGC game pages
```

Longer TTL:

```text
specific known educational game URL
specific approved chess/coding/math game
parent-approved game account page
```

## 15. Child UX

Child messages:

```text
Ocentra is checking whether this game matches your family rules.
```

```text
This game needs parent approval before you can play.
```

```text
Game time is limited right now.
```

```text
Educational games are allowed during homework. This game is still unknown.
```

Avoid:

```text
You are addicted.
This game is evil.
AI caught you.
```

## 16. Parent UX

Parent card:

```text
Browser game detected

Child: Aarav
Device: Aarav's laptop
Platform: CrazyGames
Game: Unknown / page title
Type: likely casual browser game
Signals: WebGL, fullscreen, keyboard controls
Risk: low/unknown
Policy: Unknown games ask parent during homework
Decision: Parent approval required
Actions: Allow once, Allow this game, Limit, Block, View evidence
```

Cloud gaming card:

```text
Cloud gaming session detected

Platform: GeForce NOW
Game title: Unknown
Evidence: play.geforcenow.com, gamepad API, streaming pattern
Policy: Cloud gaming requires approval
Decision: Ask parent
```

Educational game card:

```text
Educational game allowed

Game: Fraction Practice
Subject: Math
Evidence: title, metadata, parent allowlist
Policy: Educational games allowed during homework
Decision: Allowed
```

## 17. Enforcement Matrix

| Surface                       | Detect game                      | Classify game | Time-limit               | Block/hold               | Notes                      |
| ----------------------------- | -------------------------------- | ------------- | ------------------------ | ------------------------ | -------------------------- |
| Managed Chrome/Edge game page | high                             | medium/high   | high                     | high if adapter exists   | Best desktop path.         |
| Managed browser cloud gaming  | high platform, medium exact game | medium        | high                     | high by platform/session | Exact game may be unknown. |
| Unmanaged browser             | process/domain only              | low           | browser-level only       | browser-level only       | No exact URL claim.        |
| Native game app               | high with process evidence       | medium/high   | high for owned process   | scoped proof only        | App/game path owns this.   |
| Android browser shell         | high if owned                    | medium/high   | high                     | high                     | Future owned-shell path.   |
| Android native app            | medium                           | medium        | app-level                | app-level                | Permission-dependent.      |
| iOS native/web                | limited                          | limited       | Screen Time/app/category | Screen Time/app/category | No desktop-style adapter.  |
| Network/DNS                   | domain/platform only             | low           | network-level only       | domain/platform only     | No exact game claim.       |

## 18. Tests And Validation

Unit tests:

```text
game platform domain mapping
browser game URL shape parsing
cloud gaming route parsing
game portal detection
unblocked game site detection
canvas/WebGL signal schema
game metadata schema
game AI input/output schema
risk/benefit signal validation
policy target/action schema
memory key generation
```

Integration tests:

```text
managed browser detects known game portal
managed browser detects WebGL/canvas signals
managed browser detects cloud gaming URL
managed browser detects game account signup
managed browser detects purchase/loot box route
hidden analysis profile extracts metadata safely
AI classifies educational game from stored metadata
AI classifies unknown game as parent-review candidate
policy allows educational game during homework
policy blocks unknown game during homework
policy time-limits browser games
unmanaged browser game produces bypass only
```

Contract tests:

```text
BrowserGameUrlShape
BrowserGameRuntimeSignals
BrowserGameMetadataEvidence
BrowserGameAiAnalysisInput
BrowserGameAiAnalysisResult
BrowserGamePolicyTarget
BrowserGamePolicyDecision
BrowserGameApprovalRequest
BrowserGameMemoryHit
```

E2E tests:

```text
Child opens Coolmath-style educational game -> allowed.
Child opens unknown game portal during homework -> ask parent.
Child opens YouTube game stream -> handled by video/social docs, not browser-game doc.
Child opens GeForce NOW/Xbox Cloud Gaming -> cloud gaming approval required.
Child opens WebGL game -> game runtime evidence recorded.
Child opens game purchase page -> parent approval required.
Child opens unblocked games site -> blocked/ask depending policy.
Child opens game in unmanaged browser -> bypass evidence only.
```

Playwright tests:

```text
parent browser-game dashboard renders detected games
game evidence drawer shows URL/runtime/metadata/AI/policy
parent approves unknown game once
parent blocks game portal
parent sets educational game allow rule
parent sets cloud gaming approval rule
child checking screen appears
child block screen appears
unmanaged browser game shows no exact URL
malicious game title escaped
large game portal list does not freeze UI
```

Security tests:

```text
fake educational label does not auto-allow without evidence confidence
canvas-only page does not automatically become game
game portal iframe origin mismatch creates degraded/unknown state
hidden analysis cannot use child cookies
hidden analysis cannot trigger downloads
hidden analysis cannot click purchase/signup
AI cannot enforce directly
memory without evidence refs cannot allow/block
remote AI cannot run by default
```

## 19. Manual Proof Artifacts

Before claiming support for a platform/surface, capture:

```text
managed browser evidence
URL shape result
runtime signal result
metadata result
AI analysis result
policy decision
action result
journal entry
SQLite read model
parent portal screenshot
child-facing screenshot if action occurs
manual-required/degraded states
```

Artifact folders:

```text
output/browser-game-proof/educational-game-allow/
output/browser-game-proof/unknown-game-parent-review/
output/browser-game-proof/webgl-game-detected/
output/browser-game-proof/cloud-gaming-detected/
output/browser-game-proof/game-purchase-gated/
output/browser-game-proof/unblocked-game-blocked/
output/browser-game-proof/unmanaged-browser-game-bypass/
```

## 20. Workpack Split

```text
01. Browser game plan folder and README
02. Browser game platform/route contracts
03. Known browser game portal pattern library
04. Cloud gaming pattern library
05. Game URL shape parser
06. Game runtime signal detector
07. Game metadata extractor
08. Hidden analysis profile safety for games
09. Educational game classifier contract
10. Browser game AI analysis contract
11. Game risk/benefit signal model
12. Browser game memory/cache
13. Game account/signup/purchase gating
14. Cloud gaming gating
15. Unblocked game site detection
16. UGC/multiplayer/chat risk model
17. Parent game policy compiler
18. Managed browser game hold/block adapter
19. Child game checking/block UX
20. Parent browser-game dashboard UX
21. Journal/SQLite read model
22. Tests, fixtures, Playwright, manual proof
23. Android/iOS capability matrix
24. Rollout and manual-required labels
```

If these become implementation assignments, create focused workpack files or
worker messages before code changes. Do not mix all 24 into one PR.

## Implementation Checkpoint - 2026-06-03

- GAME-01 now creates the
  [browser games/cloud gaming workpack README](browser-games-cloud-gaming/readme.md)
  and proof-root map. This is scaffold/proof-map work only: no game route
  contracts, runtime signals, metadata extraction, AI analysis, policy
  decisions, UI delivery, native app/game control, cloud-streamed frame
  analysis, or enforcement is claimed.
- GAME-02 now adds parent-domain browser-game platform/route contracts. Rows
  model browser-game portals, educational game sites, UGC game platforms,
  cloud-gaming/cloud PC platforms, classic game archives, school game
  platforms, and unknown/manual-required platforms through route surface kinds,
  route source kinds, custody labels, pattern refs, evidence refs, confidence,
  and status. They reject raw domains, raw URLs, raw paths, raw page bodies,
  live URL parser claims, runtime detection claims, AI classification claims,
  policy decisions, native game control, cloud-frame analysis, and enforcement.
  The live route proof fetches real public CrazyGames, Poki, Coolmath Games,
  Xbox Cloud Gaming, itch.io HTML5 catalog, and Chess.com play surfaces, stores
  only response metadata plus hashed origin/path/body refs, parses six route
  contracts plus a reviewed catalog, and rejects raw-data/runtime/parser/AI/
  policy/native/cloud-frame/enforcement overclaims. Package subpath exports are
  present; no live URL parser, runtime detector, AI classifier, UI, product
  checklist update, release claim, cloud-frame analysis, native game control,
  final policy, or enforcement is made.
- GAME-03 now adds parent-domain browser-game portal pattern library contracts.
  Entries model known-game, educational, UGC, indie, classic archive, school,
  and unknown portal families through route kinds, signal kinds, pattern
  fingerprints, evidence refs, confidence, and review states. They reject raw
  domains, raw URLs, raw page titles, raw page bodies, runtime detection
  claims, AI classification claims, policy authority, cloud-gaming ownership,
  and enforcement. GAME-03 also now includes live public-surface proof in
  `output/browser-plan-proof/game-03-known-game-portal-pattern-library/05-live-pattern-library-proof.json`
  and
  `test-results/browser-game-portal-pattern-library-live-evidence-proof/proof.json`.
  That proof fetches real CrazyGames, Poki, Coolmath Games, itch.io HTML5,
  Internet Archive MS-DOS games, and Chess.com play surfaces; stores only
  response metadata plus hashed origin/path/body refs; parses six reviewed
  portal pattern rows plus a reviewed library; and rejects 17 overclaims. No
  runtime portal detector, URL parser, AI classifier, final policy, UI, product
  checklist update, release claim, cloud-frame analysis, native game control,
  or enforcement is made.
- GAME-04 now adds parent-domain cloud-gaming pattern library contracts.
  Entries model cloud-gaming platforms, cloud PC platforms, mobile cloud-game
  portals, browser-embedded cloud-game surfaces, native launcher prompt
  bridges, and unknown/manual-required cloud surfaces through route kinds,
  signal kinds, pattern fingerprints, evidence refs, confidence, and review
  states. They reject raw cloud domains, raw cloud URLs, raw cloud titles, raw
  stream frames, runtime detection claims, cloud-streamed frame analysis,
  per-game cloud-title certainty, native launcher/game control, final policy
  decisions, and enforcement. GAME-04 also now includes live public-surface
  proof in
  `output/browser-plan-proof/game-04-cloud-gaming-pattern-library/06-live-cloud-pattern-proof.json`
  and
  `test-results/browser-game-cloud-pattern-library-live-evidence-proof/proof.json`.
  That proof fetches real Xbox Cloud Gaming, NVIDIA GeForce Now, Amazon Luna,
  Boosteroid, PlayStation Plus games catalog, Shadow cloud PC, and now.gg
  surfaces; stores only response metadata plus hashed origin/path/body refs;
  parses seven reviewed cloud pattern rows plus a reviewed library; and rejects
  20 overclaims. No runtime detector, cloud-frame analyzer, native launcher
  controller, final policy, UI, product checklist update, release claim, native
  game control, or enforcement is made.
- GAME-05 now adds a parent-domain redacted URL-shape parser. It accepts
  unknown input, uses the platform URL parser only transiently, and emits
  protocol/host/path-depth shape, route surface kind, route hint booleans,
  query/fragment shape booleans, reason codes, confidence, and a route-shape
  fingerprint. It rejects raw URL, domain, path, and query storage; browser
  navigation, runtime detection, AI classification, final policy decisions,
  cloud-frame analysis, native game control, and enforcement. The live proof
  fetches real public CrazyGames, Poki, Coolmath Games, Chess.com play, Xbox
  Cloud Gaming play/cloud, and NVIDIA GeForce Now route surfaces; stores only
  response metadata plus hashed origin/path/body refs; parses seven
  URL-shape rows; and rejects 16 raw-storage/runtime/AI/policy/native/cloud-frame
  and enforcement overclaims.
- GAME-06 now adds parent-domain runtime signal detector contracts. Signal rows
  model canvas, WebGL, Gamepad API, fullscreen, pointer-lock, audio context,
  animation-loop, iframe game surface, cloud-streaming, and
  unknown/manual-required runtime shapes through fingerprints, evidence refs,
  source kinds, confidence, status, and reason codes. They reject raw DOM,
  canvas frame, stream frame, audio, and gamepad input storage; browser
  instrumentation, runtime detection execution, AI classification, final policy
  decisions, cloud-frame analysis, native game control, and enforcement. The
  live Playwright proof opens real public Poki, Coolmath Games, Chess.com play,
  and Xbox Cloud Gaming pages in Chromium; stores only response metadata, hashed
  origin/path refs, shape booleans, and fingerprints; parses 12 runtime signal
  rows plus a detection bundle; and rejects 27 raw-runtime-data/instrumentation,
  AI, policy, native, cloud-frame, and enforcement overclaims.
- GAME-07 now adds parent-domain metadata extractor contracts. Field rows model
  title, description, genre, age-rating, publisher, thumbnail, educational
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
  native, cloud-frame, and enforcement overclaims.
- GAME-08 now adds parent-domain hidden analysis profile safety contracts and a
  live public safety-shape proof. Profile and loader rows model Ocentra-owned
  isolated profiles, bounded retention, proof-backed metadata-only/analysis-ready
  states, disabled-policy, proof-missing, manual-required, and unavailable states
  through evidence refs, profile fingerprints, reason codes, confidence, and
  loader proof refs. The live proof fetches real public Poki, Coolmath Games,
  Chess.com play, PlayStation Plus games catalog, and Xbox Cloud Gaming pages;
  stores only response metadata, hashed origin/path/body refs, profile
  fingerprints, loader proof refs, and no-capture safety flags; validates 10
  profile rows plus 10 planned/proof-backed loader results; and rejects 40 child
  profile reuse, raw capture, hidden/browser/native, AI, policy, UI, cloud-frame,
  and enforcement overclaims.
- GAME-09 now adds parent-domain browser-game educational classifier contracts
  and a live public candidate-shape proof. The classifier models evidence rows
  for domain reputation, school-provided URLs, teacher/parent allowlists,
  page/subject metadata, AI classification refs, past parent approval, homework
  context, school platforms, platform self-labels, and manual-required states.
  The live proof fetches real public Code.org Minecraft, Chess.com play,
  Coolmath Run 3, Poki Subway Surfers, and Xbox Cloud Gaming pages; stores only
  response metadata, hashed origin/path/body refs, evidence refs, classifier
  candidate rows, and no-authority flags; validates 9 evidence rows plus 5
  educational, entertainment, misleading-claim, and manual-required candidate
  results; and rejects 23 raw content, model text, platform-authority,
  account/purchase, policy, runtime, UI, native, cloud-frame, and enforcement
  overclaims.
- GAME-10 now adds parent-domain browser-game AI analysis input/result contracts
  and a live public AI-analysis shape proof. Inputs consume typed evidence refs
  only for browser evidence, URL shape, runtime signals, metadata, screen
  summaries, parent rules, recent activity, memory, task, and custody labels.
  Results model game classification, educational check, risk classification,
  cloud-gaming detection, UGC risk, purchase risk, and policy-support outputs as
  candidate-only benefit/risk signals, surface kind, modifiers, recommended
  policy input, confidence, uncertainty, summary refs, model runtime refs,
  prompt template version, expiry, and degraded/manual states. The live proof
  fetches real public Poki Subway Surfers, Code.org Minecraft, Chess.com play,
  Xbox Cloud Gaming, and Roblox discover pages; stores only response metadata,
  hashed origin/path/body refs, typed evidence refs, candidate policy inputs, and
  no-authority flags; validates 5 inputs plus 5 candidate-only results; and
  rejects 28 raw URL/page/game/frame/model text, account/purchase, native,
  cloud-frame, policy, runtime, UI, and enforcement overclaims.
- GAME-11 now adds parent-domain browser-game risk/benefit signal contracts
  and a live public risk/benefit signal shape proof.
  The signal set models evidence-backed risk rows for violence, horror, adult
  themes, addictive loops, multiplayer/contact, chat, purchase, loot box/random
  item, UGC, privacy, unblocked-bypass, and unknown risk; and benefit rows for
  educational value, homework relevance, skill building, creativity, problem
  solving, parent-approved game, neutral, and unknown benefit. It returns
  candidate recommended policy inputs only and rejects raw game payloads, chat
  content, page body, raw model text, account/purchase execution,
  cloud-frame analysis, native game control, final policy authority, runtime
  gate execution, and enforcement. The live proof fetches real public Poki
  Subway Surfers, Code.org Minecraft, Chess.com play, Xbox Cloud Gaming, and
  Roblox Discover pages; stores only response metadata, hashed origin/path/body
  refs, typed evidence refs, candidate risk/benefit signal rows, and
  no-authority flags; validates 5 signal sets with 7 risk signals and 8 benefit
  signals; and rejects 30 raw content, model text, platform-authority,
  account/purchase, policy, runtime, UI, native, cloud-frame, and enforcement
  overclaims.
- GAME-12 now adds parent-domain browser-game memory/cache contracts and a live
  public memory/cache shape proof. The cache uses schema-backed ref/hash key
  kinds for canonical URL refs, platform game
  refs, domain path hashes, cloud game title refs, parent decision refs, game
  category refs, policy versions, child profiles, parent rule sets, and
  evidence refs. Entries model bounded fresh-hit, stale-hit, miss, and
  manual-required states with short dynamic, cloud launcher, UGC, stable
  approved game, and parent-approved account TTL classes. Fresh hits may be
  reused for policy input only when decision refs, evidence refs, required
  subject keys, and bounded TTL are present; stale/miss/manual rows cannot
  drive policy input. The contracts reject raw canonical URLs, raw platform
  game IDs, raw cloud game titles, raw game payloads, raw model text, runtime
  cache-store claims, AI cache claims, UI delivery, native game control,
  cloud-frame analysis, final policy authority, and enforcement. The live proof
  fetches real public Poki Subway Surfers, Code.org Minecraft, Chess.com play,
  Xbox Cloud Gaming, and Roblox Discover pages; stores only response metadata,
  hashed origin/path/body refs, cache-key refs, evidence refs, snapshots, and
  no-authority flags; validates 5 snapshots with 15 bounded entries; and
  rejects 20 raw URL/title/game/model, runtime cache, AI-cache, policy, UI,
  native, cloud-frame, and enforcement overclaims.
- GAME-13 now adds parent-domain browser-game account/signup/purchase gate
  contracts and a live public account/purchase route proof. The contracts model
  account creation, login, secondary account, purchase, subscription, loot
  box/random item, virtual currency, download/install, wallet/gambling-like
  payment, cloud-gaming start, and
  unknown-game start approval states as evidence-backed candidate requests and
  decisions only. They reject raw URLs/titles/account identifiers, credentials,
  form submission, account creation, purchase/payment execution, downloads,
  notifications, UI, runtime gate execution, native game control,
  cloud-frame analysis, final policy authority, and enforcement. The live proof
  fetches real public Roblox login, Roblox subscription, Steam app purchase,
  Xbox Cloud Gaming, Code.org sign-in, and PlayStation store pages; stores only
  response metadata, hashed origin/path/body refs, evidence refs, request and
  decision refs, and no-authority flags; validates 6 approval requests plus 6
  candidate decisions; and rejects 38 raw URL/title/account/credential,
  submission, purchase/payment, notification/UI, policy/runtime, native,
  cloud-frame, and enforcement overclaims.
- GAME-14 now adds parent-domain browser-game cloud-gaming gate contracts
  plus a live public cloud-gaming route proof. The contracts model known cloud
  domains, streaming session routes, gamepad/fullscreen/high-bandwidth/
  low-latency signal refs, optional platform title/rating metadata refs,
  unknown cloud-game approval, mature cloud-game block candidates,
  school-night blocks, time-budget candidates, manual-required content-frame
  gaps, and unavailable platform proof states. The proof fetches real Xbox
  Cloud Gaming, GeForce Now, Amazon Luna, Boosteroid, PlayStation Plus,
  Shadow cloud PC, and now.gg surfaces while persisting only response hashes,
  evidence refs, request/decision refs, and no-authority flags. It rejects raw
  URLs/titles/stream frames, cloud-streamed frame analysis, per-game cloud
  title claims, native game/launcher control, game chat content,
  account/purchase flows, notifications, UI, final policy authority, runtime
  gate execution, and enforcement.
- GAME-15 now adds parent-domain browser-game unblocked-site detection
  contracts. The detection models managed browser routes/pages, search intent,
  portal indexes, iframe embeds, proxy/mirror routes, hidden game origins,
  school bypass language, unmanaged browser process-only bypass evidence,
  manual-required states, and unavailable states. Candidate actions remain
  block-during-school, parent-review, allow-specific-game, block-unknown-iframe,
  bypass-evidence-only, manual-review, or unknown inputs. It rejects raw URLs,
  raw page body, raw search queries, captured iframe content, exact unmanaged
  URL claims, native game control, cloud-frame analysis, account/purchase
  flows, rendered UI, final policy authority, runtime gate execution, and
  enforcement.
- GAME-16 now adds parent-domain browser-game UGC/multiplayer/chat risk
  contracts and a live public risk-shape proof. The assessment models UGC game
  pages, experience pages, multiplayer lobbies, profile/friends/message routes,
  launch prompts, web-to-app launch surfaces, manual-required states, and
  unavailable states. Risk rows cover unknown player contact, chat/voice
  contact, UGC worlds, unsafe user-created experiences, off-platform contact,
  virtual currency, in-game purchase, missing age rating, web-to-app launch
  risk, manual-required, and unknown risk. Recommended controls remain
  candidates only: approved-experience-only, parent-review, block-chat,
  time-limit, purchase approval, block unknown UGC, manual review, or unknown.
  The live proof fetches real Roblox Discover, Scratch Games Explore, Minecraft
  Marketplace, Chess.com online play, Steam Community chat, Rec Room, and Xbox
  Cloud Gaming surfaces while persisting only response hashes, evidence refs,
  risk row refs, and no-authority flags. It rejects raw chat/profile/account/
  experience identifiers, raw game payloads, web-to-app launch execution,
  purchase execution, native game control, final policy decisions, runtime gate
  execution, UI rendering, and enforcement.
- GAME-17 now adds parent-domain browser-game policy compiler contracts and a
  live public compiler-shape proof. The compiler consumes parent-owned evidence,
  analysis, mobile capability, rule, and schedule refs, and returns
  candidate-only decisions without claiming final policy authority, runtime gate
  execution, UI delivery, native game control, cloud-frame analysis, or
  enforcement. The live proof fetches real Code.org Minecraft, Poki Subway
  Surfers, Roblox Discover, Coolmath Run, Hooda unblocked games, Rec Room, and
  Internet Archive MS-DOS game surfaces while persisting only response hashes,
  evidence refs, compiler input refs, candidate decision refs, and
  no-authority flags.
- GAME-18 now adds parent-domain managed browser-game hold/block adapter
  contracts and a live public hold/block adapter proof. The proof fetches real
  Scratch games, Roblox Discover, Hooda unblocked games, Poki Subway Surfers,
  Code.org Minecraft, Coolmath Run, Xbox Cloud Gaming, and Steam Store
  surfaces, persists only response hashes, evidence refs, adapter plan refs, and
  no-authority flags, and parses managed hold, approval, block, warning,
  candidate-only allow/time-limit, manual-required cloud, and unavailable native
  rows. It rejects raw URL/page/game payloads, child cookie/session reuse,
  unmanaged exact URL claims, browser mutation, rendered child pages,
  notification delivery, final policy decisions, applied time limits,
  cloud-frame analysis, native game control, and enforcement.
- GAME-19 now adds parent-domain browser-game child checking/block UX contracts
  and a live public child UX surface proof. The proof fetches real Scratch
  games, Roblox Discover, Hooda unblocked games, Code.org Minecraft, Coolmath
  Run, Xbox Cloud Gaming, and Steam Store surfaces, persists only response
  hashes, evidence refs, child UX surface refs, and no-authority flags, and
  parses checking, approval, blocked, educational allowed, time-limit,
  cloud-gaming manual-required, and native unavailable rows. It rejects raw child
  copy, rendered child UI, notification delivery, runtime browser blocking,
  block-page rendering, applied time limits, final policy decisions, cloud-frame
  analysis, native game control, and enforcement claims.
- GAME-20 now adds parent-domain parent browser-game dashboard UX contracts and
  a live public dashboard panel proof. The proof fetches real Scratch games,
  Roblox Discover, Xbox Cloud Gaming, Code.org Minecraft, Coolmath Run, Steam
  Store, and Rec Room surfaces, persists only response hashes, evidence refs,
  dashboard panel refs, and no-authority flags, and parses detected-game,
  approval queue, cloud approval, educational allowlist, time-budget,
  mobile/native gap, and manual-required gap panels. It rejects rendered portal
  UI, notification delivery, runtime data fetch, final policy decisions,
  cloud-frame analysis, native game control, and enforcement claims.
- GAME-21 now adds parent-domain browser-game journal/SQLite read-model
  contracts plus a live public evidence-backed read-model shape proof. The
  snapshot indexes managed browser evidence journal replay,
  app-game session report proof, managed browser-game adapter audit refs,
  manual-required cloud rows, and unavailable native/unmanaged rows. Proof rows
  require journal entry refs, SQLite row refs, source read-model refs, proof
  refs, positive event/row counts, and matching reason codes while rejecting raw
  URL/page/game/title/account/purchase storage, child session reuse, cloud title
  certainty, browser mutation, rendered UI, final policy decisions, and
  enforcement. The live proof persists only response hashes, origin/path hashes,
  source refs, and no-claim flags; it does not claim a runtime SQLite query,
  rendered browser-game UI, Playwright screenshot, product checklist upgrade, or
  release readiness.
- GAME-22 now adds `scripts/test/browser-game-proof-artifacts.mjs` and
  `scripts/test/browser-game-child-intervention-page-proof.mjs`. The artifact
  gate verifies GAME-01 through GAME-23 checklist ownership, proof directory
  references, required proof files, browser-game workpack README references,
  feature coverage, and expectation boundary text. The rendered proof opens real
  public Roblox, Coolmath Games, Scratch, Xbox Cloud Gaming, and Steam Store
  surfaces through Playwright, captures live backdrops, renders the shared
  `BrowserChildInterventionPage`, serves it through the Rust child-agent
  `/api/browser/intervention/page` endpoint, and stores screenshots plus
  hash-only proof JSON at
  `test-results/browser-game-child-intervention-page-proof/proof.json` and
  `output/browser-plan-proof/game-22-tests-fixtures-playwright-manual-proof/02-rendered-browser-game-child-intervention-proof.json`.
  The manifest records 23 complete/proof rows and zero partial/manual-required
  rows. No final policy decision, product runtime browser-game detection,
  notification or approval delivery, cloud-streamed frame analysis, native game
  control, enforcement, or product checklist claim is made.
- GAME-23 now adds parent-domain Android/iOS browser-game capability matrix
  contracts plus a real Android host/emulator proof. The proof builds the
  Android agent APK, boots or reuses an Android 15 emulator, installs and
  launches the package, observes the running agent status through UIAutomator
  tree evidence, hashes UI/package/device evidence, and queries known browser package targets
  without storing a raw package list. The matrix keeps Android browser shells,
  WebView, Custom Tabs, installed-browser, cloud-gaming, device-owner, iOS
  Family Controls, Safari web-domain token, application-token, managed-browser,
  cloud-gaming, and Web Clip/PWA surfaces manual-required, token-limited,
  entitlement-required, app-level, or domain-level until owned-browser-shell,
  iOS entitlement, and device-owner proof exists.
- GAME-24 now adds the browser-game rollout/manual-required gate. GAME rollout
  state: partial/manual-required. The gate labels GAME-01 as
  scaffold-proof-present, GAME-02 as live-route-proof-present, GAME-03 as
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
  Product checklist upgrade is not claimed, and final policy decisions,
  notification or approval delivery, parent dashboard runtime UI,
  cloud-streamed frame analysis, native game control, and enforcement remain
  open or manual-required until separate proof exists.

## 21. Must-Not-Claim List

Do not claim:

```text
Canvas/WebGL always means game.
Cloud gaming URL always reveals exact game title.
A game is educational because the site says so.
A game is safe because ESRB/platform label says so.
We can inspect cloud-streamed game frames without screen/vision proof.
We can block individual native-app game scenes.
We can read game chat without explicit permission/proof.
We can block all browser games without side effects.
We can classify UGC game safety permanently.
AI can enforce directly.
Unmanaged browser game gives exact URL evidence.
```

## 22. Minimum MVP

First strong MVP:

```text
managed browser detects browser-game URLs
known game portals are classified
known cloud gaming domains are classified
canvas/WebGL/runtime signals are recorded
educational vs entertainment unknown classification exists
unknown browser games can ask parent
approved educational games can be allowed
browser games can have time budget
cloud gaming can require approval
game account/signup/purchase routes can require approval
unmanaged browser game use is bypass evidence only
parent sees evidence, AI classification, rule, action, and audit
```

## 23. Done Signal

This feature is credible when:

```text
A child opening a managed-browser game produces typed browser-game evidence.
Ocentra can distinguish educational, casual, cloud, UGC, multiplayer, purchase, and unknown game states where evidence supports it.
Unknown games can be held or sent to parent approval.
Educational games can be allowed without opening all games.
Cloud gaming can be gated separately.
Game account/purchase flows can be gated.
Unmanaged browser games remain bypass evidence only.
Every decision cites browser evidence, game evidence, AI analysis where used, parent policy, and audit refs.
```

Final rule:

```text
Do not block all games blindly.
Do not allow all games blindly.
Classify the game surface, prove the evidence, apply parent policy, and show the audit.
```
