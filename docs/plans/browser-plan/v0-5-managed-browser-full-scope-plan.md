# V0.5 Managed Browser Full Scope Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `V0.5 Managed Browser Full Scope Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This plan turns the existing browser source docs and pasted scope notes into a
concrete Ocentra Parent browser subsystem plan. It keeps browser evidence
separate from app/process capture, network/domain observation, screen analysis,
LAN pairing, and broad OS enforcement.

This is a plan document only. It does not claim production-complete browser
enforcement, unmanaged exact URL evidence, mobile browser parity, AppLocker/App
Control deployment, extension security, or active-tab proof until matching
artifacts exist.

Companion requirement doc:
[V0.5 Managed Browser Test Blueprint](v0-5-managed-browser-test-blueprint.md).
Companion intelligence doc:
[V0.5 Browser URL And Video AI Intelligence Plan](v0-5-browser-url-video-ai-intelligence-plan.md).
Companion social gating doc:
[V0.5 Social Platform Account Feed And Gating Plan](v0-5-social-platform-account-feed-gating-plan.md).
Companion browser-game doc:
[V0.5 Browser Games Cloud Gaming And Game Portal Gating Plan](v0-5-browser-games-cloud-gaming-gating-plan.md).

## Product Boundary

- Owning feature: Browser and web control.
- Primary milestone: V0.5.1 browser URL/tab evidence.
- Secondary milestone: V0.8 enforcement for managed browser intervention and
  unmanaged browser fallback.
- Product goal: prove exact URL/title/tab evidence only inside an Ocentra-owned
  managed browser boundary and show every unsupported/degraded/manual-required
  state honestly in the parent UI.
- Intelligence goal: classify URL/page/video meaning only from typed evidence,
  model/runtime proof, and explicit degraded states; feed parent policy, not
  direct enforcement.
- Social gating goal: make managed-browser social account creation, login,
  account switch, feed, reel, short, livestream, messaging route, and upload/post
  attempts first-class policy targets when the evidence supports them.
- Browser-game goal: make managed-browser game portals, WebGL/canvas games,
  cloud gaming, unblocked game sites, game account/purchase flows, educational
  games, and unknown game starts first-class policy targets when evidence
  supports them.
- Non-goals: attaching to default personal profiles, reading browser secrets,
  capturing page body, decrypting HTTPS payloads, guessing URLs from process
  windows or network domains, hiding unsupported browsers, claiming video
  semantics from URL alone, letting AI enforce directly, or claiming broad OS
  blocking before adapter proof.

## Code-To-UI Layout

Use this layout before assigning work. The goal is to enhance the current
implementation, not rewrite it.

| Layer                            | Existing destination                                                                        | Browser responsibility                                                                                                                                                         |
| -------------------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Evidence contracts               | `packages/activity-domain/src/browser*.ts`                                                  | Browser tab evidence, managed session status, read models, intervention rows, custody/source labels, stale/degraded states.                                                    |
| Parent policy/catalog            | `packages/parent-domain/src/browser-control-*.ts`                                           | Authoring manifest, policy value/update shape, effective policy compile inputs, catalog/forest values, capability states.                                                      |
| Protocol adapter                 | `packages/agent-protocol-domain/src/browser-policy-adapter.ts`                              | Portal/service command/event adapter for browser policy updates and reports.                                                                                                   |
| Rust protocol                    | `crates/agent-protocol/src/browser*.rs`                                                     | Rust mirrors of evidence/read-model/session/policy/intervention shapes.                                                                                                        |
| Runtime core                     | `crates/agent-core/src/browser_*.rs`                                                        | Managed discovery, launch plans, bridge polling, tab observations, journal/event helpers, intervention event helpers.                                                          |
| Service shell                    | `crates/agent-service/src/browser_*.rs`                                                     | Browser runtime status, policy API/runtime/store/compiler, payloads, activity API reports, service-backed read models.                                                         |
| Portal app                       | `apps/portal/src/*browser*.ts`                                                              | Managed status, browser evidence, browser intervention, product status summaries, redacted diagnostics.                                                                        |
| Rich UI source                   | `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/BrowserRulesQuestionnaire.*`           | Browser rules questionnaire visuals and controls, rendered from typed policy question data.                                                                                    |
| URL/video intelligence           | Existing browser evidence, parent policy, local AI, memory graph, and service paths         | URL shape, metadata, local AI result, memory/cache, provider route, policy decision, degraded state, and explanation refs without a parallel decision system.                  |
| Social/account/feed gating       | Existing browser evidence, parent policy/approval, social/video, local AI, and portal paths | Social route shape, signup/login/account-switch flow, approved account identity, feed/short-video route, approval request, child hold screen, parent decision, and audit refs. |
| Browser-game/cloud-gaming gating | Existing browser evidence, app/game, parent policy/approval, local AI, and portal paths     | Game URL shape, runtime signals, metadata, cloud platform, game account/purchase gate, educational/UGC/cloud risk, time budget, approval request, and audit refs.              |
| Proof scripts                    | `scripts/test/*browser*.mjs`                                                                | Real browser matrix, service proof, intervention proof, V0.8 browser/domain adapter proof, Windows managed/unmanaged proof.                                                    |

## Policy, Catalog, And Forest Routing

The browser policy catalog and questionnaire forest are design inputs for the
authoring manifest. Workers should not copy catalog prose directly into runtime
or portal code.

- Raw 1,057-setting inventory stays as raw review input.
- Questionnaire forest stays as product question-reduction input.
- `browser-control-schema-proposal.md` stays a design proposal, not runtime
  source.
- `browser-control-coverage-matrix.md` records what has been covered by typed
  manifests/contracts.
- Runtime policy values belong in `parent-domain`.
- Portal renders typed manifest fields and sends typed commands.
- The child agent validates, persists, compiles, and reports policy state.
- Enforcement/adapters execute only after typed decisions and capability proof.

## Activity And Evidence Routing

Browser evidence must follow the shared activity path:

```text
browser adapter observation
-> activity-domain browser evidence
-> encrypted journal
-> SQLite query store
-> agent-service read model
-> portal / policy / local AI evidence refs
```

Do not route browser evidence directly from CDP to the portal. Do not let
policy, AI, or reports read browser profiles, raw DevTools payloads, journal
files, or SQLite files directly.

## Browser Intelligence Routing

Browser URL/video intelligence starts after browser evidence is typed and
journaled. It does not replace evidence capture, social/video source docs, local
AI expectations, policy contracts, or enforcement adapter proof.

```text
managed browser evidence
-> URL shape classification
-> memory lookup
-> metadata extraction
-> optional hidden managed analysis load
-> typed local AI result or degraded state
-> deterministic parent policy decision
-> capability-gated action
-> audit/report/UI explanation
```

Rules:

- URL shape can classify platform and stable ids, not actual content.
- Metadata is evidence, not authority.
- Hidden managed analysis must use an isolated Ocentra profile and cannot borrow
  child cookies or session tokens.
- Local AI returns recommendation evidence, confidence, reason codes, model
  refs, prompt version, and degraded state.
- Parent policy decides allow, warn, ask-parent, time-limit, block, or unknown.
- Enforcement executes only when the adapter and source evidence prove the
  action.

## Social Platform Gating Routing

Social platform work starts in this plan when the source is managed browser
evidence or unmanaged browser bypass. It points to social/video, policy,
local-AI, screen, app/native, and mobile feature docs when the evidence source
or product action leaves the browser boundary.

```text
managed browser URL evidence
-> social URL/platform route shape
-> managed metadata and optional DOM/form-shape evidence
-> account/feed/video/messaging target classification
-> optional local AI analysis
-> parent policy and approval request
-> managed browser hold/warn/block or manual-required state
-> audit/report/UI explanation
```

Rules:

- Social account creation is a policy target, not a generic browser block.
- New, unknown, or secondary social account flows require parent approval unless
  parent policy explicitly allows them.
- Signup/login/account-switch gates need strong or medium evidence such as known
  signup URL, DOM/form-shape proof, metadata, or approved account identity.
- Messaging/contact risk is privacy-sensitive and must stay at the allowed
  evidence level.
- Native app route-level controls stay manual-required until app, screen,
  Accessibility, Screen Time, Device Owner, or platform proof exists.
- Platform connectors are optional and parent-authorized; they are not core
  gating dependencies.

## Browser Game Routing

Browser game work starts in this plan when the source is a managed browser game
URL, game portal, cloud gaming web session, browser runtime signal, unmanaged
browser bypass, or browser policy gate. Native games and launchers remain in
app/game control.

```text
managed browser URL evidence
-> game URL/platform route shape
-> managed runtime signals such as canvas/WebGL/gamepad/fullscreen
-> metadata and optional hidden analysis
-> optional local AI game classification
-> parent policy, time budget, or approval request
-> managed browser hold/warn/block or manual-required state
-> audit/report/UI explanation
```

Rules:

- Browser games are policy targets, not generic website blocks.
- Canvas/WebGL is supporting evidence only.
- Cloud gaming may hide exact game title/content; unknown cloud game must remain
  explicit when the title/rating is not captured.
- Educational game claims need domain, metadata, allowlist, AI, past approval,
  or school/homework context evidence.
- Native game app/session timing stays under app/game-control evidence.
- Game chat and cloud-streamed visual content need separate screen/native/app
  proof before stronger claims.

## Proof Routing

Use the existing focused scripts where they already cover the claim:

- managed profile/browser matrix: `npm run test:managed-browser-matrix`;
- service-backed evidence path: `npm run test:managed-browser-service-proof`;
- managed intervention/block page: `npm run test:managed-browser-intervention`;
- V0.8 browser/domain adapter no-claim state:
  `node scripts/test/v0-8-browser-domain-adapter-proof.mjs`;
- Windows managed/unmanaged process proof:
  `node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`.

If a workpack needs a new proof script, name the artifact path in the workpack
and update the test blueprint before implementing it.

## 24-Step Plan

1. Establish the contract boundary.
   Keep browser evidence in `activity-domain`, browser policy/product meaning in
   `parent-domain`, WebSocket command/event routing in
   `agent-protocol-domain`, Rust protocol mirrors in `crates/agent-protocol`,
   runtime adapter/storage behavior in `agent-core`, service orchestration in
   `agent-service`, and UI display in portal/core-ui.

2. Reconcile existing docs.
   Link the feature doc, expectation docs, architecture doc, managed/unmanaged
   guide, policy catalog, schema proposal, coverage matrix, raw 1057 inventory,
   and questionnaire forest. Do not create a second browser-control truth.

3. Build the browser inventory model.
   Represent installed, running, managed, unmanaged, supported, unsupported,
   candidate, packaged, portable, and block-only browsers with reason codes and
   capability flags.

4. Add Windows browser inventory.
   Detect Edge, Chrome, Chrome for Testing, Brave, Firefox, Opera, Vivaldi,
   Tor, DuckDuckGo, AppX/MSIX packages, and portable/running candidates from
   registry, known paths, Start Menu/package data, process snapshots, and
   signature/hash refs where available.

5. Add cross-platform inventory states.
   Represent macOS app bundles, Linux desktop/package entries, Android browser
   capability states, iOS/Safari platform-specific states, and Firefox BiDi or
   extension requirements without treating them as Chromium CDP.

6. Add managed profile store.
   Create, load, repair, delete, and redact Ocentra-owned browser profiles per
   child/device/browser. Reject default profile paths and unowned roots.

7. Add managed Chromium launcher.
   Launch Edge/Chrome/Chrome for Testing with `--user-data-dir`,
   `--profile-directory`, `--remote-debugging-address=127.0.0.1`, a reserved
   local port, managed session id, profile id, policy revision, and process id.

8. Prove bridge custody.
   Connect only to Ocentra-launched loopback bridge endpoints for the current
   managed session. Reject wrong port, non-loopback host, wrong profile, wrong
   process, stale session, default profile, and raw debugger URL leakage.

9. Build CDP adapter.
   Read `/json/version` and `/json/list`, parse browser metadata and page
   targets, skip non-page targets, reject malformed payloads, and map failures
   into typed degraded status.

10. Map tab evidence.
    Convert page targets to typed evidence with evidence id, source id, adapter
    id, managed session id, profile id, URL, origin, domain, title, observedAt,
    freshUntil, staleAt, custody, and capability status.

11. Model active-tab proof.
    Keep `/json/list` evidence as target-list proof with active state
    `unknown`. Mark `known-active` only after a CDP focus/activation event,
    managed extension event, foreground correlation, or owned-shell event is
    proved and documented.

12. Journal and ingest browser evidence.
    Write browser evidence to the encrypted journal first, ingest into SQLite
    second, and serve portal/policy/AI through typed read models only.

13. Add browser service read models.
    Serve managed session status, recent tab evidence, active-state certainty,
    stale/degraded bridge state, unmanaged browser detections, and intervention
    rows through protocol events.

14. Build portal browser status surfaces.
    Show installed/supported browsers, managed profile readiness, running
    sessions, tab evidence, stale/degraded status, unmanaged browser use,
    custody labels, and redacted diagnostics.

15. Build browser policy authoring manifest.
    Render browser policy questions from typed manifests, not UI-invented
    strings. Persist parent intent through validated child-agent policy update
    commands.

16. Compile policy targets.
    Compile exact URL, origin, domain, domain suffix, category, search query,
    video URL/channel, download, browser family, managed session, and unmanaged
    browser rules only against proved capability. URL/video intelligence can
    supply category, risk, benefit, memory, and metadata evidence to policy, but
    it cannot become the policy decision.
    Social account creation, unknown account, secondary account, route kind,
    feed, short-video feed, messaging, upload/post, livestream, and unknown
    social site targets must compile through parent-domain policy and approval
    contracts.
    Browser game targets such as all browser games, game portal, specific game
    URL, educational games, cloud gaming, WebGL/canvas games, multiplayer/UGC,
    game chat, purchases, loot boxes, unknown games, and unblocked game sites
    must compile through policy and approval contracts.

17. Add managed intervention proof.
    Prove observe, dry-run, warn, redirect, and block behavior in a managed
    browser session with evidence refs, policy decision refs, audit refs, and a
    child-visible block/warning page before product claims.

18. Detect unmanaged browsers.
    Detect browser-like processes outside managed sessions. Record process id,
    process name, executable path ref, signature/hash ref, browser-family guess,
    confidence, reason, and timestamp. Never record exact URL.

19. Build unmanaged fallback UX/actions.
    Represent report-only, warn, ask-parent, terminate, relaunch-managed,
    OS-block-configured, OS-block-unavailable, and manual-required states.

20. Prove Windows app-control boundaries.
    Keep AppLocker/App Control as platform-specific real proof for preventing
    unmanaged browsers. Process termination proof does not equal broad app
    control.

21. Keep extension/native host optional.
    If CDP active-tab proof is insufficient, design a managed-profile-only
    extension/native-host adapter with permission, origin, heartbeat, and
    schema validation proof. Do not use unmanaged personal profiles.

22. Add performance/service-health gates.
    Keep inventory scanning, bridge polling, journal writes, SQLite replay,
    process scanning, and portal rendering bounded and nonblocking.

23. Capture real proof artifacts.
    Store JSON, journal snippets, SQLite read-model output, portal screenshots,
    child block-page screenshots, and manual validation notes under
    `output/browser-proof/` or `test-results/` for each real browser claim.

24. Add rollout and PR gates.
    Browser work is PR-ready only when contracts, Rust parity, service behavior,
    journal/read-model path, portal UI, tests, proof artifacts, manual gaps, and
    docs/checklist updates match the claim being made.

## Implementation Order

The first concrete implementation sequence should be:

1. Source-index reconciliation and plan docs.
2. Browser inventory/support matrix contracts.
3. Managed profile store contracts and tests.
4. Managed launcher command/state tests.
5. CDP fake-server adapter tests.
6. Tab evidence mapper and active-state proof boundary.
7. Journal/SQLite replay path.
8. URL shape and metadata intelligence contracts.
9. Social platform route, signup/login/account-switch, and approval contracts.
10. Browser game route, runtime signal, cloud-gaming, and purchase/account gate
    contracts.
11. Local AI result, memory, and provider-route contracts.
12. Portal fixtures and service-backed UI coverage.
13. Browser policy compile/dry-run tests.
14. Managed intervention, account-approval hold, and game-checking proof.
15. Unmanaged fallback and process-control proof.
16. Windows AppLocker/App Control manual proof.

## Validation Expectations

- The companion test blueprint is required for every implementation slice.
- Contract tests must exist before Rust protocol/service paths claim support.
- Rust tests must prove protocol parity and runtime guardrails.
- Browser intelligence tests must prove AI output cannot enforce directly and
  cannot run remote/API models by default.
- Social platform tests must prove account/feed gates require evidence,
  approval, adapter proof, and audit refs.
- Browser-game tests must prove game evidence, game approval/time-budget rules,
  and cloud/UGC/manual-required states do not overclaim exact content.
- Portal changes need service-backed Playwright proof.
- Real installed-browser harnesses are platform/manual proof unless explicitly
  included in a focused local validation run.
- Product docs and checklist rows change only when proof status changes.

## Open Product Questions

- Which browser inventory fields should be parent-visible versus diagnostic-only
  for privacy and support?
- What is the minimum active-tab proof acceptable for a product claim: CDP
  focus/activation, managed extension event, OS foreground correlation, or owned
  shell event?
- Which managed-browser block path ships first: CDP Fetch, browser policy,
  managed extension declarative rules, or owned browser shell?
- What AppLocker/App Control posture is acceptable for consumer Windows devices:
  observe-only, guided setup, admin/manual-required, or managed installer?
- Should Firefox enter the first supported list through WebDriver BiDi or stay
  later-adapter until extension/native-host proof?
- How should Android/iOS browser controls be represented in family setup before
  real device-owner/FamilyControls proof exists?
