<!-- agent-capsule -->

> Agent Capsule
> Doc: Policy UI Fix Plan
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Policy UI Fix Plan

Status: first-pass Manage UI wiring is implemented on `codex/parent-portal-manage-ia`; keep this note as the product/UI reference for the next backend wiring pass.

## Goal

Policy is the main product surface. It should not be a pile of side-panel subpages named Browser, Rules, Schedules, Approvals, and Enforcement. Those concepts belong inside the policy editor for each policy area.

The policy surface should let parents define what happens for:

- browser and web use
- native apps
- native games
- screen analysis
- network activity

Each area needs the same parent workflow:

- choose Family defaults or Per device overrides
- choose or inspect the target device when Per device is active
- configure rules
- configure schedule or time budget
- configure ask-parent approvals
- preview the decision
- choose observe-only, dry-run, or enforcement-eligible behavior where supported
- review audit and capability state

The portal does not evaluate policy, run timers, kill apps, capture screens, or enforce anything. It sends typed parent intents and renders typed read models. The child-device agent validates, evaluates, executes, and journals.

## Current Findings

- `packages/portal-domain/src/parent-portal-nav.ts` currently exposes separate Policy side-panel routes:
  - `#/browser-settings`
  - `#/rule-management`
  - `#/schedules`
  - `#/approvals`
  - `#/enforcement`
- `ParentPortalSvgSurface.tsx` currently has generic specs for browser, rules, schedules, approvals, enforcement, screen, app use, and network. That explains why the UI feels convoluted.
- `docs/expectations/policy.md` says parent surfaces author rules and approvals, while child-device agents validate and execute local decisions.
- `docs/expectations/enforcement.md` says enforcement only happens after typed policy decisions and must be auditable.
- `docs/expectations/browser-evidence.md` says exact URL/tab evidence comes from an Ocentra-managed browser boundary; unmanaged browser use is bypass evidence, not successful URL capture.
- `docs/expectations/app-game-evidence.md` says app/game time and classification come from local stored evidence, not portal state or AI guesses.
- `docs/expectations/screen-evidence.md` says screen analysis is explicit, parent-controlled, local-only, encrypted while queued, summarized, and then raw images are deleted by default.
- `docs/expectations/network-flow-evidence.md` says network policy consumes metadata and flow summaries, not decrypted payloads or exact URLs.

## IA Decision

Recommended side-panel cleanup:

- Collapse the current Policy subitems into one main `Policy` manage item.
- Keep Browser, Apps, Games, Screen, and Network as tabs inside Policy.
- Move Rules, Schedules, Approvals, and Enforcement into the body of the selected policy tab.

Rationale:

- Browser rules are different from game time budgets.
- Screen analysis settings are privacy-sensitive and do not belong in a generic schedule page.
- Network rules need metadata and adapter capability state, not browser controls.
- Approvals are not a standalone product area; they are the ask-parent workflow for a specific rule target.
- Enforcement is not a page where parents randomly toggle power. It is a per-target capability and mode.

## Shared Layout Pattern

Policy should reuse the Activity-style page structure because Policy needs Family vs Per device selection.

Top region:

- Header: icon, `Policy`, info badge, optional right action.
- Scope toggle: Family, Per Device.
- If Family is active:
  - show compact family-default status
  - disable or hide the device grid
  - rules apply to family defaults unless a per-device override exists
- If Per Device is active:
  - show the same device selector/grid style used by Activity
  - require a selected connected/known portal device before editing device overrides
  - offline devices can show read-only last-known policy state

Bottom region:

- Primary tabs: Browser, Apps, Games, Screen, Network.
- Inside each primary tab, use a consistent editor layout:
  - Summary
  - Rules
  - Schedule
  - Approvals
  - Enforcement
  - Preview/Audit

Use the existing multi-choice SVG toggle for compact decisions:

- Allow, Warn, Ask, Limit, Block
- Observe, Dry Run, Enforce
- Family Default, Override
- Always, School, Homework, Bedtime, Weekend, Custom
- Monitor, Warn, Ask, Relaunch Managed, Block

If the policy matrix becomes too dense for the existing toggle, design a new SVG control before implementing instead of falling back to ugly badges.

## Product Truth

Policy may show:

- parent-authored rules
- family defaults
- per-device overrides
- schedules and time budgets
- permission-request state
- dry-run preview results
- decision reason codes
- evidence references
- local AI contribution when applicable
- enforcement capability and result state
- unsupported/degraded/unavailable adapter states

Policy must not show:

- fake production rules
- raw child screenshots
- raw browser bodies, chat text, forms, cookies, or tokens
- decrypted network payloads
- portal-only UI state as a policy input
- billing provider state as a policy input
- hidden product-level blocks without a matching parent rule

Policy actions must stay parent-controlled. Ocentra provides typed targets, previews, capability state, explanation, and safe boundaries. Parents decide the household rulebook.

## Shared Policy Editor Model

Every policy tab should use the same editor concepts, but each tab fills them with domain-specific targets.

Summary:

- Current scope: Family or selected device.
- Current mode: Family default or device override.
- Capability state: ready, degraded, unavailable, unsupported, dry-run only, observe-only.
- Last preview decision.
- Last audit event.

Rules:

- Target type and target value.
- Action: allow, warn, time-limit, ask-parent, block.
- Priority.
- Reason code.
- Enabled state.
- Conflict explanation when more than one rule applies.

Schedule:

- Always, School, Homework, Bedtime, Weekend, Custom.
- Time windows.
- Time budget if applicable.
- Reset behavior.
- Grace period.

Approvals:

- Ask-parent trigger.
- Child reason required or optional.
- Approval duration.
- Remember answer mode.
- Expiry.
- Parent notification route.

Enforcement:

- Observe-only.
- Dry-run preview.
- Enforcement eligible.
- Adapter capability.
- Rollback/unavailable state.
- Child-facing explanation.

Preview/Audit:

- Ask the child device for a dry-run preview.
- Show policy decision, evidence refs, AI result ref if used, rule refs, schedule state, and enforcement handoff state.
- Show rule edits and parent approvals.
- Never run evaluation in the portal.

## Browser Policy

Purpose: manage web browsing and browser bypass behavior.

Browser is the most complex policy surface because exact URL evidence only exists inside the Ocentra-managed browser boundary. Unmanaged Chrome, Edge, Brave, Firefox, Opera, portable browsers, and browser-like processes are possible bypass states unless a supported adapter proves otherwise.

Expected tabs inside Browser:

- Overview
- Managed Browser
- Web Rules
- Bypass Handling
- Preview/Audit

Overview:

- Supported browsers detected.
- Running browsers.
- Managed browser session status.
- Active tab evidence status.
- Unmanaged browser detections.
- Last browser policy decision.

Managed Browser:

- Supported first: Edge Stable and Chrome or Chrome for Testing.
- Later candidates: Brave after proof, Firefox after separate adapter proof, Opera/portable/unknown as unsupported or unmanaged.
- Managed profile path state, bridge state, localhost-only bridge state, profile ready state.
- Action to launch managed browser.
- Action to set Ocentra-managed browser path as the normal child path where the platform allows it.

Web Rules:

- Target types: site, domain, URL, category, video/channel, browser process, browser session.
- Actions: allow, warn, ask-parent, time-limit, block.
- Examples for UI fixture:
  - School sites allowed during school schedule.
  - Video sites time-limited after homework.
  - Unknown risky category asks parent.
  - Adult content blocked.

Bypass Handling:

- What to do when an unmanaged browser is detected.
- Parent choices:
  - Monitor only.
  - Warn child and notify parent.
  - Ask parent before continuing.
  - Close/terminate unmanaged browser when adapter supports it.
  - Close unmanaged browser and relaunch managed browser.
  - Block browser-like process until managed browser is used.
- This is a parent choice and capability-gated. It must not be the hidden default.

Preview/Audit:

- Preview web rule against recent managed browser evidence.
- Show unmanaged-browser events as bypass evidence, not exact URL evidence.
- Show intervention mechanism: managed bridge, managed extension, OS app control, monitor-only, none.
- Show intervention outcome: applied, allowed, blocked, failed, unsupported, monitor-only.

Possible controls:

- `ScopeToggle`: Monitor, Warn, Ask, Relaunch, Block.
- `ScopeToggle`: Allow, Warn, Ask, Limit, Block.
- `ScopeToggle`: Observe, Dry Run, Enforce.

Fake UI-check data:

- `POLICY_UI_CHECK_BROWSER`
- Example states: Edge managed ready, Chrome unmanaged detected, Firefox unsupported, active managed tab evidence ready.
- Do not use real browsing history. Use synthetic invalid domain fixtures from `POLICY_UI_CHECK_BROWSER_FIXTURE_DOMAINS` (for example `policy-ui-check-school.ocentra.invalid`) for UI-check only.

Open browser decisions:

- Should the MVP require managed browser for exact URL policy?
- Should unsupported browsers be monitor-only by default?
- When is it acceptable to terminate unmanaged browsers?
- Should the product set the managed browser as default browser during setup?
- Do we need a child-facing managed browser launcher UI?

## App Policy

Purpose: manage native applications that are not specifically games.

Expected tabs inside Apps:

- Overview
- Inventory
- App Rules
- New Apps
- Preview/Audit

Overview:

- Installed apps detected.
- Running apps.
- Foreground app.
- Unknown/suspicious apps.
- App control capability state.
- Last app policy decision.

Inventory:

- App name, process name, path/signature/hash where available.
- Category candidate: school, productivity, chat, media, utility, unknown.
- Publisher/signature trust state where available.
- Capability: observe, time-limit, terminate/block, unsupported.

App Rules:

- Target types: app, process, window, category, activity type.
- Actions: allow, warn, ask-parent, time-limit, block.
- Time budget support for categories like entertainment/chat.
- School/homework allowlist support.

New Apps:

- Parent choices:
  - Allow new apps by default.
  - Warn and record.
  - Ask parent first.
  - Block until reviewed.
- Unknown app classification should stay unknown until evidence supports stronger classification.

Preview/Audit:

- Preview using stored app/process evidence.
- Show evidence refs and session refs.
- Show whether local AI classified an unknown app.

Possible controls:

- `ScopeToggle`: Allow, Warn, Ask, Limit, Block.
- `ScopeToggle`: Known Apps, Unknown Apps, New Apps.
- `ScopeToggle`: Observe, Dry Run, Enforce.

Fake UI-check data:

- `POLICY_UI_CHECK_APPS`
- Example rows: school app allowed, chat app ask-parent after bedtime, unknown app asks parent.
- Do not hardcode personal app paths.

## Game Policy

Purpose: manage native games, launchers, and game time budgets.

Expected tabs inside Games:

- Overview
- Game Library
- Time Budgets
- Game Rules
- Preview/Audit

Overview:

- Detected game sessions.
- Foreground game state.
- Launcher state: Steam, Epic, Xbox, Riot, Battle.net, EA, Ubisoft, GOG, Roblox, Minecraft, unknown.
- Session duration and foreground duration.
- Catalog match state: known game, possible game, unknown.

Game Library:

- Installed games and launchers.
- Known-game catalog match.
- Unknown/ambiguous entries.
- Launcher-only vs actual game session distinction.

Time Budgets:

- Daily or weekly game budget.
- School-day vs weekend budget.
- Per-game budget and category budget.
- Reset behavior and grace period.

Game Rules:

- Target types: game title, launcher, process, category, unknown game.
- Actions: allow, warn, ask-parent, time-limit, block.
- Child-facing explanation when a game is stopped.

Preview/Audit:

- Preview time-limit decision using stored session summary.
- Show running time, foreground time, evidence refs, rule refs, and enforcement handoff state.
- Show whether a game was terminated, already exited, unavailable, or left running in observe-only mode.

Possible controls:

- `ScopeToggle`: Allow, Warn, Ask, Limit, Block.
- `ScopeToggle`: School Day, Weekend, Temporary.
- `ScopeToggle`: Observe, Dry Run, Enforce.

Fake UI-check data:

- `POLICY_UI_CHECK_GAMES`
- Example rows: known game over budget, launcher running without game, unknown possible game asks parent.
- Use generic names like `Known Game A` or fixture ids, not personal names.

## Screen Analysis Policy

Purpose: configure whether local screen analysis can contribute to policy decisions.

This is high-sensitivity. It must be explicit, parent-controlled, local-only by default, and honest about deletion/custody state.

Expected tabs inside Screen:

- Overview
- Capture Settings
- Policy Use
- Retention
- Preview/Audit

Overview:

- Screen capability state: ready, permission required, protected surface, screen locked, model unavailable, queue unavailable, unsupported.
- Current analysis mode: observe-only, policy dry-run, enforcement eligible.
- Latest summary state.
- Latest image deletion state.

Capture Settings:

- Enable/disable screen analysis.
- Cadence capture enabled.
- Cadence seconds.
- Strict mode.
- Trigger capture enabled.
- Triggers: foreground app change, managed URL change, app/game foreground start, unusual network change, policy ambiguity, manual parent test.
- Scope: active window, managed browser window, active display, full screen, unsupported.

Policy Use:

- Whether screen-derived categories can affect rules.
- Visible categories: school, video, chat, game, adult content, violence, bypass tool, shopping, productivity, unknown.
- Risk signals: possible bypass tool, credential prompt, unsafe visible content, self-harm signal, explicit content signal, unknown.
- Confidence threshold.
- Low-confidence fallback: no-op, warn, ask-parent.

Retention:

- Temporary images encrypted while queued.
- Delete after success.
- Delete after expiry.
- Retain raw image false by default.
- OCR snippets enabled/disabled.
- Redaction mode.
- Deletion failures visible.

Preview/Audit:

- Preview with a stored summary, not a raw screenshot.
- Show source evidence refs, category, confidence, deletion state, and policy decision.
- Show parent setting version and who changed it.

Possible controls:

- `ScopeToggle`: Off, Summary, Dry Run, Enforce.
- `ScopeToggle`: 5 Min, 1 Min Strict, Triggered.
- `ScopeToggle`: No OCR, Redacted OCR, Summary Only.

Fake UI-check data:

- `POLICY_UI_CHECK_SCREEN`
- Example states: screen analysis disabled by parent, trigger capture ready, delete-after-success required.
- Never include fake screenshot thumbnails.

Open screen decisions:

- What is the default cadence if enabled?
- Should strict mode be separate from enforcement?
- Which visible categories are MVP?
- Should OCR snippets ever be shown in the portal, or only summarized?

## Network Policy

Purpose: manage network metadata, destination rules, unusual traffic, and bypass indicators.

Network policy is not browser policy. It cannot claim exact URLs or page content. It can use process, domain/IP, protocol, counters, and VPN/proxy/tunnel indicators where available.

Expected tabs inside Network:

- Overview
- Destinations
- Network Rules
- Bypass Indicators
- Preview/Audit

Overview:

- Network adapter capability.
- Recent top processes.
- Recent top destinations.
- Domain-known vs IP-only state.
- Process attribution state.
- VPN/proxy/tunnel indicators.
- Last network policy decision.

Destinations:

- Domain, IP, port, protocol.
- Process name where attributed.
- Connection count and byte counters where supported.
- Destination category if known.
- Unknown/encrypted-content-unavailable states.

Network Rules:

- Target types: process, domain, IP, protocol, destination category, VPN/proxy/tunnel indicator, unusual traffic digest.
- Actions: allow, warn, ask-parent, time-limit or temporary block where meaningful, block.
- School allowlist.
- VPN/proxy rules.
- Unknown high-volume process rules.

Bypass Indicators:

- VPN/proxy/tunnel detected.
- New destination.
- High-volume flow.
- Repeated failure.
- Unusual unknown process.
- Adapter unavailable.
- Encrypted content unavailable.

Preview/Audit:

- Preview using stored network flow summaries.
- Show evidence refs, not raw packets.
- Show whether adapter can enforce network/domain block or only observe.

Possible controls:

- `ScopeToggle`: Metadata, Warn, Ask, Block.
- `ScopeToggle`: Domains, VPN/Proxy, High Volume, Unknown.
- `ScopeToggle`: Observe, Dry Run, Enforce.

Fake UI-check data:

- `POLICY_UI_CHECK_NETWORK`
- Example rows: school domain allowed, VPN/proxy ask-parent, unknown high-volume process warning.
- No packet payloads, URLs, credentials, or real domains.

## Requests And Audit

Permission requests and audit should be accessible from every policy tab, and may also deserve a compact cross-policy drawer later.

Permission request fields:

- request id
- child profile
- device
- target
- requested action
- evidence refs
- state: open, approved, denied, expired, cancelled
- expiry
- parent response

Audit fields:

- previous policy version
- new policy version
- actor
- source surface
- validation result
- decision id
- evidence refs
- enforcement result if any

UI behavior:

- If Family mode is active, show family-wide pending requests.
- If Per device mode is active, show selected-device requests first.
- Approve once, deny once, approve for schedule, or create rule from request should be typed intents.
- The child-device agent validates the final approval or rule before local execution.

## Fake UI Data Policy

Temporary fake policy data is allowed only to judge UI. It must be verbose and impossible to confuse with production state.

Rules:

- Gate fake data with `POLICY_UI_CHECK_FAKE_DATA_ENABLED`.
- Prefix fixtures with `POLICY_UI_CHECK_*`.
- Use fixture devices like `D001`, `D002`, not human names.
- Mark every fake row with `source: "ui-check-fake"`.
- Use `.invalid` domains for browser/network examples.
- Do not include real app paths, personal browser history, screenshots, URLs, phone numbers, credentials, or packet payloads.
- Remove fake data once real typed read models land.

Useful fake fixture set:

- Family scope enabled.
- Per-device override available for `D001`.
- Browser fixture with managed Edge ready and unmanaged Chrome detected.
- Apps fixture with school app allowed, chat app asks after bedtime, unknown app asks parent.
- Games fixture with known game over budget and launcher-only state.
- Screen fixture disabled by parent but capability ready.
- Network fixture with school domain allowed and VPN/proxy indicator ask-parent.
- One open approval request and two audit events.

## Future Function Intent

These names are provisional surface entries for UI planning; each entry must be replaced by test-backed domain contracts in the portal policy implementation layer before production rollout.

Read models:

- `getPolicyScopeReadModel()`
- `getPolicyDeviceSelectorReadModel()`
- `getFamilyPolicySetReadModel()`
- `getPolicySurfaceReadModel(surface, scope)`
- `getBrowserPolicyReadModel(scope)`
- `getAppPolicyReadModel(scope)`
- `getGamePolicyReadModel(scope)`
- `getScreenPolicyReadModel(scope)`
- `getNetworkPolicyReadModel(scope)`
- `getPolicyPreviewReadModel(input)`
- `getPermissionRequestReadModel(scope)`
- `getPolicyAuditReadModel(scope)`

Intents:

- `setPolicyScopeIntent(input)`
- `selectPolicyDeviceIntent(input)`
- `savePolicyRuleIntent(input)`
- `savePolicyScheduleIntent(input)`
- `savePolicyApprovalRuleIntent(input)`
- `setPolicyEnforcementModeIntent(input)`
- `requestPolicyPreviewIntent(input)`
- `approvePermissionRequestIntent(input)`
- `denyPermissionRequestIntent(input)`
- `createRuleFromPermissionRequestIntent(input)`
- `resetDevicePolicyOverrideIntent(input)`
- `launchManagedBrowserIntent(input)`
- `setManagedBrowserPreferenceIntent(input)`
- `refreshPolicyCapabilityIntent(input)`

Expected typed states:

- `family-default`
- `device-override`
- `ready`
- `stale`
- `unsupported`
- `unavailable`
- `degraded`
- `observe-only`
- `dry-run`
- `enforcement-eligible`
- `permission-required`
- `pending-parent`
- `failed`

## Implementation Phases

Do not start these until the plan is accepted.

1. Keep this as docs only until reviewed.
2. Decide IA: collapse Policy side-panel subitems or keep them but route to one Policy surface with selected tab.
3. Define missing policy read-model contracts in domain packages.
4. Define policy surface labels/actions in domain constants, not naked UI strings.
5. Build a shared Policy shell using the Activity-style Family/Per Device selector.
6. Build primary tabs: Browser, Apps, Games, Screen, Network.
7. Build shared inner panels: Summary, Rules, Schedule, Approvals, Enforcement, Preview/Audit.
8. Add explicit UI-check fake data behind the loud policy flag.
9. Validate wide, narrow, and mobile-like layouts.
10. Wire real read models and typed intents after backend/Rust contracts exist.
11. Remove fake data.

## Open Decisions

- Should the side panel show one Policy item or keep current subitems as deep links into Policy tabs?
- Should Browser be the first/default Policy tab?
- Should exact URL policy require Ocentra-managed browser in MVP?
- Should unmanaged browser handling default to monitor-only, warn, ask-parent, or require managed browser?
- When, if ever, should unmanaged browser processes be terminated automatically?
- Which browsers are MVP supported: Edge and Chrome only?
- Should apps and games be separate tabs, or Games as a filtered app policy view?
- Which game launcher integrations are MVP?
- What should default screen analysis be: off, summary-only, or dry-run after explicit parent enablement?
- Which network indicators are actionable in MVP: VPN/proxy only, high-volume only, new destinations, or all?
- Should enforcement mode be global per surface, per rule, or both?
- Should child-facing explanation copy be configured per rule or generated from reason codes?

## Non-Goals For This Slice

- No UI implementation.
- No production policy evaluator changes.
- No enforcement adapter changes.
- No browser capture changes.
- No screen capture changes.
- No network adapter changes.
- No fake production rule data.
- No cloud policy storage.
- No rule evaluation in the portal.
