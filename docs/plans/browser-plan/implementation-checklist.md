# Browser Plan Implementation Checklist

This is the fill-in checklist for browser-plan implementation work. Future AI
workers must update this file and the matching workpack checklist before
reporting `DONE` or PR-ready.

This checklist tracks browser-plan execution only. It does not replace
`docs/product-capability-checklist.md`, and workers must not edit that product
checklist unless a feature row status, proof, or gap actually changes and the
worker holds the correct hub lock.

## Fill Rules

- Keep unchecked items unchecked until code, docs, tests, and proof are present.
- Record the lane, branch, PR, commit, or proof path in the notes column when an
  item moves.
- If an item is intentionally deferred, leave it unchecked and write the
  manual-required reason.
- Do not use this file to claim production readiness without proof artifacts.
- Fill the matching `## AI Worker Checklist` inside the workpack file before
  reporting `DONE`.
- Report product-doc updates, or explicitly state why no product-doc update was
  needed.

## Required Proof Pack

Every implementation workpack needs a proof pack before the main workpack row
can be marked complete. Use this root unless the assignment names a stricter
location:

```text
output/browser-plan-proof/<workpack-id>/
```

The proof pack must contain or explicitly mark N/A for each applicable item:

- [ ] `00-source-snapshot.md`: git branch, commit, `git status --short`,
      existing source paths inspected, existing behavior, and the before-state gap.
- [ ] `01-contract-proof.log`: TypeScript contract tests, decode failures, and
      schema-boundary/source-shape checks for new or changed contracts.
- [ ] `02-rust-protocol-proof.log`: Rust protocol parity, serialization, and
      invalid-state tests when protocol/service shapes change.
- [ ] `03-runtime-evidence.json`: managed launch/session/bridge evidence,
      unmanaged process evidence, or platform adapter evidence for the workpack.
- [ ] `04-journal-sqlite-proof.json`: journal entry refs, replay result, and
      SQLite/read-model rows when evidence persistence changes.
- [ ] `05-policy-action-proof.json`: policy input, compiled target, decision,
      action result, evidence refs, and degraded/manual-required labels when policy
      or enforcement changes.
- [ ] `06-ui-snapshots/`: parent portal and child-facing screenshots for every
      UI-visible state touched by the workpack.
- [ ] `07-playwright-ui-proof.log`: Playwright/browser test output for changed
      portal or child UI, including malicious text escaping and responsive state
      where applicable.
- [ ] `08-security-negative-proof.log`: negative tests proving no default
      profile attachment, no unowned bridge use, no unmanaged exact URL claim, no
      raw debugger URL exposure, and no AI-direct-enforcement path where applicable.
- [ ] `09-manual-platform-proof.md`: OS/browser/device versions, exact command
      steps, screenshots/logs, and manual-required labels for real platform claims.
- [ ] `10-validation-commands.log`: focused validation plus any requested
      `npm run validate`/`ci:local`/manual command output.

## UI Snapshot Gates

When a workpack touches portal, child-facing UI, managed browser block/warn
pages, policy authoring, dashboards, read models, or status surfaces, workers
must capture screenshots before marking the workpack complete.

- [ ] Parent portal snapshot for the normal/supported state.
- [ ] Parent portal snapshot for stale/degraded/manual-required state.
- [ ] Parent portal snapshot for unmanaged/bypass state when that surface is in
      scope.
- [ ] Policy authoring/preview snapshot when rules, catalog, forest, or policy
      compiler UI is in scope.
- [ ] Child checking/warning/block/approval snapshot when child UX is in scope.
- [ ] Responsive/narrow viewport snapshot when the touched UI is expected to be
      usable on small screens.
- [ ] Malicious/long text snapshot when evidence titles, URLs, game names,
      social account names, or AI labels are rendered.
- [ ] Explicit `ui-not-applicable.md` when the workpack has no UI surface.

## Evidence Quality Gates

- [ ] Raw fixture/evidence is stored with redacted sensitive values, not just a
      prose summary.
- [ ] Every action proof includes evidence refs and policy decision refs.
- [ ] Every stale/degraded/manual-required state is represented in contracts,
      runtime/read model, and UI where applicable.
- [ ] Every unsupported platform claim is represented as unsupported,
      manual-required, or not-claimed until real platform proof exists.
- [ ] Every enhancement claim links back to the URL/video AI, social gating, or
      browser-game plan section that owns the scope.
- [ ] Every failed, skipped, manual, or deferred test has a reason and follow-up
      owner recorded.

## Main Execution Gates

- [ ] Source docs read: folder README, source index, current snapshot, full
      scope plan, test blueprint, UI/UX guide, and the assigned workpack.
- [ ] Enhancement docs checked for overlap: URL/video AI intelligence, social
      platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers the workpack file and exact implementation/docs paths.
- [ ] Existing source layout inspected before editing; no parallel browser truth
      created.
- [ ] TypeScript Effect Schema contracts land before Rust/service/portal
      consumers.
- [ ] Rust protocol parity exists for new protocol-facing contracts.
- [ ] Journal/read-model/storage behavior exists before portal or policy claims
      depend on it.
- [ ] Portal UI renders capability, degraded, stale, unsupported, and
      manual-required states honestly.
- [ ] Managed browser exact URL claims are limited to Ocentra-launched managed
      sessions.
- [ ] Unmanaged browser behavior is reported as bypass/process evidence only.
- [ ] AI classification is evidence, not authority.
- [ ] Parent policy is the enforcement authority.
- [ ] Required proof pack exists with logs, JSON, screenshots, or explicit N/A
      reasons for every applicable gate.
- [ ] Feature docs, expectation docs, module READMEs, and product capability
      checklist decisions are recorded.
- [ ] `DONE` report includes workpack, touched paths, validation, proof, known
      gaps, and documentation changes.

## Base Workpack Checklist

Use `[ ]` for not started, `[~]` for in progress, and `[x]` only after the
required proof pack exists. The `Evidence Or Proof` cell must name concrete
artifact paths, command logs, screenshots, PR checks, or an explicit
manual-required/N/A file.

| Step | Workpack                                                                                                        | Status | Owner/Lane | Branch/PR/Commit | Evidence Or Proof | Doc/Checklist Decision |
| ---- | --------------------------------------------------------------------------------------------------------------- | ------ | ---------- | ---------------- | ----------------- | ---------------------- |
| 01   | [Contract boundary and Effect schemas](workpacks/01-contract-boundary-and-effect-schemas.md)                    | [ ]    |            |                  |                   |                        |
| 02   | [Source index and doc reconciliation](workpacks/02-source-index-and-doc-reconciliation.md)                      | [ ]    |            |                  |                   |                        |
| 03   | [Browser inventory model](workpacks/03-browser-inventory-model.md)                                              | [ ]    |            |                  |                   |                        |
| 04   | [Windows browser inventory adapter](workpacks/04-windows-browser-inventory-adapter.md)                          | [ ]    |            |                  |                   |                        |
| 05   | [Cross-platform inventory matrix](workpacks/05-cross-platform-inventory-matrix.md)                              | [ ]    |            |                  |                   |                        |
| 06   | [Managed profile store](workpacks/06-managed-profile-store.md)                                                  | [ ]    |            |                  |                   |                        |
| 07   | [Managed Chromium launcher](workpacks/07-managed-chromium-launcher.md)                                          | [ ]    |            |                  |                   |                        |
| 08   | [Bridge custody and security](workpacks/08-bridge-custody-and-security.md)                                      | [ ]    |            |                  |                   |                        |
| 09   | [CDP version and target adapter](workpacks/09-cdp-version-and-target-adapter.md)                                | [ ]    |            |                  |                   |                        |
| 10   | [Tab evidence mapper](workpacks/10-tab-evidence-mapper.md)                                                      | [ ]    |            |                  |                   |                        |
| 11   | [Active-tab proof model](workpacks/11-active-tab-proof-model.md)                                                | [ ]    |            |                  |                   |                        |
| 12   | [Journal and SQLite browser ingest](workpacks/12-journal-and-sqlite-browser-ingest.md)                          | [ ]    |            |                  |                   |                        |
| 13   | [Browser read models and service events](workpacks/13-browser-read-models-and-service-events.md)                | [ ]    |            |                  |                   |                        |
| 14   | [Portal browser status surfaces](workpacks/14-portal-browser-status-surfaces.md)                                | [ ]    |            |                  |                   |                        |
| 15   | [Browser policy authoring manifest and policy writer inputs](workpacks/15-browser-policy-authoring-manifest.md) | [ ]    |            |                  |                   |                        |
| 16   | [Policy target compiler](workpacks/16-policy-target-compiler.md)                                                | [ ]    |            |                  |                   |                        |
| 17   | [Managed intervention and block page](workpacks/17-managed-intervention-and-block-page.md)                      | [ ]    |            |                  |                   |                        |
| 18   | [Unmanaged browser detection](workpacks/18-unmanaged-browser-detection.md)                                      | [ ]    |            |                  |                   |                        |
| 19   | [Unmanaged fallback UX and actions](workpacks/19-unmanaged-fallback-ux-and-actions.md)                          | [ ]    |            |                  |                   |                        |
| 20   | [Windows AppLocker and App Control proof](workpacks/20-windows-applocker-app-control-proof.md)                  | [ ]    |            |                  |                   |                        |
| 21   | [Extension and native host boundary](workpacks/21-extension-and-native-host-boundary.md)                        | [ ]    |            |                  |                   |                        |
| 22   | [Performance and service health](workpacks/22-performance-and-service-health.md)                                | [ ]    |            |                  |                   |                        |
| 23   | [E2E and manual proof artifacts](workpacks/23-e2e-and-manual-proof-artifacts.md)                                | [ ]    |            |                  |                   |                        |
| 24   | [Rollout, checklist, and PR gate](workpacks/24-rollout-checklist-and-pr-gate.md)                                | [ ]    |            |                  |                   |                        |

## URL And Video AI Intelligence Checklist

Source:
[V0.5 Browser URL And Video AI Intelligence Plan](v0-5-browser-url-video-ai-intelligence-plan.md).

Use `[x]` only when the item has contract/runtime/UI/proof evidence or an
explicit manual-required artifact. URL/video AI evidence must include the input
evidence refs, model/provider route, confidence/degraded state, policy handoff,
and no-direct-enforcement proof where applicable.

| Step  | Enhancement Item                          | Status | Owner/Lane | Evidence Or Proof | Notes |
| ----- | ----------------------------------------- | ------ | ---------- | ----------------- | ----- |
| AI-01 | Browser AI intelligence plan links        | [ ]    |            |                   |       |
| AI-02 | URL shape classification contracts        | [ ]    |            |                   |       |
| AI-03 | Platform/video URL parser library         | [ ]    |            |                   |       |
| AI-04 | Browser intelligence memory contracts     | [ ]    |            |                   |       |
| AI-05 | Metadata extraction contracts             | [ ]    |            |                   |       |
| AI-06 | Hidden managed analysis profile design    | [ ]    |            |                   |       |
| AI-07 | Hidden analysis loader adapter            | [ ]    |            |                   |       |
| AI-08 | AI analysis input/output contracts        | [ ]    |            |                   |       |
| AI-09 | Local AI provider routing                 | [ ]    |            |                   |       |
| AI-10 | Family AI hub routing                     | [ ]    |            |                   |       |
| AI-11 | Parent-approved remote AI boundary        | [ ]    |            |                   |       |
| AI-12 | Prompt/template versioning                | [ ]    |            |                   |       |
| AI-13 | Structured category/risk/benefit model    | [ ]    |            |                   |       |
| AI-14 | URL/video analysis queue                  | [ ]    |            |                   |       |
| AI-15 | Memory/cache store                        | [ ]    |            |                   |       |
| AI-16 | Knowledge graph references                | [ ]    |            |                   |       |
| AI-17 | Policy evaluator integration              | [ ]    |            |                   |       |
| AI-18 | Post-analysis action model                | [ ]    |            |                   |       |
| AI-19 | Child-facing checking/warning UX          | [ ]    |            |                   |       |
| AI-20 | Parent explanation/audit UX               | [ ]    |            |                   |       |
| AI-21 | YouTube parser and metadata adapter       | [ ]    |            |                   |       |
| AI-22 | Vimeo/generic video parser                | [ ]    |            |                   |       |
| AI-23 | Dynamic feed/social URL handling          | [ ]    |            |                   |       |
| AI-24 | Provider degraded/fallback behavior       | [ ]    |            |                   |       |
| AI-25 | Proof gates, fixtures, tests, and rollout | [ ]    |            |                   |       |

## Social Platform Account Feed Checklist

Source:
[V0.5 Social Platform Account Feed And Gating Plan](v0-5-social-platform-account-feed-gating-plan.md).

Use `[x]` only when the item has evidence for the exact platform/surface being
claimed. Social account, feed, reel, short, livestream, messaging, connector,
and native-app claims need route/source evidence, permission/privacy boundary,
policy decision refs, UI snapshots, and manual-required labels where adapter
proof is missing.

| Step      | Enhancement Item                                  | Status | Owner/Lane | Evidence Or Proof | Notes |
| --------- | ------------------------------------------------- | ------ | ---------- | ----------------- | ----- |
| SOCIAL-01 | Social/video gating plan folder and README        | [ ]    |            |                   |       |
| SOCIAL-02 | Platform and route contract schemas               | [ ]    |            |                   |       |
| SOCIAL-03 | Social URL pattern library                        | [ ]    |            |                   |       |
| SOCIAL-04 | Signup/login/account-switch evidence contracts    | [ ]    |            |                   |       |
| SOCIAL-05 | Managed DOM/form-shape detector                   | [ ]    |            |                   |       |
| SOCIAL-06 | Social account identity registry                  | [ ]    |            |                   |       |
| SOCIAL-07 | Parent approval request/decision contracts        | [ ]    |            |                   |       |
| SOCIAL-08 | Feed/reels/shorts route classification            | [ ]    |            |                   |       |
| SOCIAL-09 | Video/social metadata extractor                   | [ ]    |            |                   |       |
| SOCIAL-10 | Social AI analysis contracts                      | [ ]    |            |                   |       |
| SOCIAL-11 | Social risk/benefit signal model                  | [ ]    |            |                   |       |
| SOCIAL-12 | Parent policy compiler for social targets         | [ ]    |            |                   |       |
| SOCIAL-13 | Managed browser account creation gate             | [ ]    |            |                   |       |
| SOCIAL-14 | Managed browser feed/short/video route gate       | [ ]    |            |                   |       |
| SOCIAL-15 | Unmanaged social bypass detector                  | [ ]    |            |                   |       |
| SOCIAL-16 | Android native-app capability matrix              | [ ]    |            |                   |       |
| SOCIAL-17 | iOS Screen Time/ManagedSettings capability matrix | [ ]    |            |                   |       |
| SOCIAL-18 | Platform connector authorization boundary         | [ ]    |            |                   |       |
| SOCIAL-19 | Memory/cache for account/video/channel decisions  | [ ]    |            |                   |       |
| SOCIAL-20 | Parent social dashboard UX                        | [ ]    |            |                   |       |
| SOCIAL-21 | Child approval/block UX                           | [ ]    |            |                   |       |
| SOCIAL-22 | Audit and explanation read model                  | [ ]    |            |                   |       |
| SOCIAL-23 | Tests, fixtures, Playwright, manual proof         | [ ]    |            |                   |       |
| SOCIAL-24 | Rollout and manual-required status labels         | [ ]    |            |                   |       |

## Browser Games Cloud Gaming Checklist

Source:
[V0.5 Browser Games Cloud Gaming And Game Portal Gating Plan](v0-5-browser-games-cloud-gaming-gating-plan.md).

Use `[x]` only when game/platform evidence is specific. Browser-game, cloud
gaming, WebGL/canvas, UGC, account, purchase, educational, and time-budget
claims need URL/runtime/metadata/AI/policy/action evidence, UI snapshots, and
explicit degraded states for ambiguous canvas, iframe, or cloud surfaces.

| Step    | Enhancement Item                          | Status | Owner/Lane | Evidence Or Proof | Notes |
| ------- | ----------------------------------------- | ------ | ---------- | ----------------- | ----- |
| GAME-01 | Browser game plan folder and README       | [ ]    |            |                   |       |
| GAME-02 | Browser game platform/route contracts     | [ ]    |            |                   |       |
| GAME-03 | Known browser game portal pattern library | [ ]    |            |                   |       |
| GAME-04 | Cloud gaming pattern library              | [ ]    |            |                   |       |
| GAME-05 | Game URL shape parser                     | [ ]    |            |                   |       |
| GAME-06 | Game runtime signal detector              | [ ]    |            |                   |       |
| GAME-07 | Game metadata extractor                   | [ ]    |            |                   |       |
| GAME-08 | Hidden analysis profile safety for games  | [ ]    |            |                   |       |
| GAME-09 | Educational game classifier contract      | [ ]    |            |                   |       |
| GAME-10 | Browser game AI analysis contract         | [ ]    |            |                   |       |
| GAME-11 | Game risk/benefit signal model            | [ ]    |            |                   |       |
| GAME-12 | Browser game memory/cache                 | [ ]    |            |                   |       |
| GAME-13 | Game account/signup/purchase gating       | [ ]    |            |                   |       |
| GAME-14 | Cloud gaming gating                       | [ ]    |            |                   |       |
| GAME-15 | Unblocked game site detection             | [ ]    |            |                   |       |
| GAME-16 | UGC/multiplayer/chat risk model           | [ ]    |            |                   |       |
| GAME-17 | Parent game policy compiler               | [ ]    |            |                   |       |
| GAME-18 | Managed browser game hold/block adapter   | [ ]    |            |                   |       |
| GAME-19 | Child game checking/block UX              | [ ]    |            |                   |       |
| GAME-20 | Parent browser-game dashboard UX          | [ ]    |            |                   |       |
| GAME-21 | Journal/SQLite read model                 | [ ]    |            |                   |       |
| GAME-22 | Tests, fixtures, Playwright, manual proof | [ ]    |            |                   |       |
| GAME-23 | Android/iOS capability matrix             | [ ]    |            |                   |       |
| GAME-24 | Rollout and manual-required labels        | [ ]    |            |                   |       |

## Worker Report Template

Use this shape in the hub report or PR-ready note:

```text
DONE browser workpack <number/name>
Owner/lane:
Branch/commit/PR:
Touched paths:
Checklist updates:
Source snapshot:
Validation commands and logs:
Proof pack root:
Raw evidence artifacts:
UI snapshots:
Security negative proof:
Manual/platform proof:
Feature docs updated:
Expectation docs updated:
Product capability checklist:
Known gaps/manual-required:
No-claim boundaries preserved:
```
