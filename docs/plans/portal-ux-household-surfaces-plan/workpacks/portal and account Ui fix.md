<!-- agent-capsule -->

> Agent Capsule
> Doc: Portal And Account UI Fix Plan
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Portal And Account UI Fix Plan

Status: first-pass Manage UI wiring is implemented on `codex/parent-portal-manage-ia`; keep this note as the product/UI reference for the next backend wiring pass.

## Goal

Portal and Account manage surfaces should become portal-owned control-plane UI. They should not look like device discovery, child activity, or generic scaffold badge pages.

Device and Activity already established the desired frame rhythm:

- shared manage frame and header treatment
- consistent icon, title, info control, and right-side action placement
- clear top/bottom split where the top area is the main selector/status region and the bottom area is the detail body
- explicit temporary UI-check fake data only when requested

Portal and Account should inherit that same layout truth, but they do not need a LAN/Parent Portal selector, a family/per-device selector, or a device grid. They are about the parent portal, account state, subscriptions, entitlements, notification routing, and support diagnostics.

## Current Findings

- `packages/portal-domain/src/parent-portal-nav.ts` currently has a Portal section with Settings, Alerts, Channels.
- The same file has an Account section with Plan, Support, Access.
- `ParentPortalSvgSurface.tsx` already maps settings, notifications, channels, subscription, entitlements, support, diagnostics, family settings, and settings rules into the `portal` manage lane.
- The current full-frame/header behavior is still tied too closely to the visible section label being Portal. Account pages are portal-owned by behavior, but do not automatically inherit the same layout treatment.
- Existing portal/account control specs are too generic. Modes like Included, Limited, Locked or Trial, Paid, Grace are useful, but they currently render as generic status rows instead of a coherent product surface.
- Product docs are clear that Ocentra-hosted data is for non-activity control-plane data only: account, subscription, entitlement, notification route metadata, release/update metadata, and minimal provider delivery metadata.
- Portal UI should read typed state and send typed intents. It should not pretend to execute child work in Vite.

## IA Decision

Keep Portal and Account as separate side-panel groups if that helps navigation clarity, but treat both as the same portal-owned layout family.

Recommended behavior:

- Portal group: Settings, Alerts, Channels.
- Account group: Plan, Access, Support.
- Shared frame/layout predicate should be based on the normalized manage lane, such as `manageLane === "portal"`, not on `sectionLabel === "PORTAL"`.
- Account can remain visually distinct in the side panel, but page layout should inherit the Portal surface rules.

Possible later cleanup:

- Move any install/update/platform support concepts into Support or Portal Ops if they are really portal runtime concerns.
- Keep Devices for actual child/agent/LAN device management.
- Keep Activity for read-model activity views.

## Shared Surface Pattern

Portal and Account pages should use one shared shell:

- Header: icon, title, info badge, optional right action.
- Top area: portal/account status strip, mode selector if needed, and compact summary.
- Divider: same spacing and thickness as Device/Activity.
- Body tabs: proper reflective/beveled tabs matching the established style.
- Body panel: square bottom corners, consistent border, no random badge garden.
- No device selector grid unless the surface explicitly needs to target a child device, which these pages generally should not.

Use the existing multi-choice SVG toggle where a compact choice control is needed. Do not create plain badge clusters for real state. If the existing toggle cannot express the needed UI cleanly, pause and design a new SVG control.

Good uses for the existing multi-choice toggle:

- Subscription state preview: Trial, Active, Grace, Past Due, Unavailable.
- Entitlement filter: Included, Limited, Locked.
- Alert delivery route: In-app, Push, Email, SMS, WhatsApp.
- Quiet-hours mode: Off, Schedule, Critical Only.
- Support bundle mode: Summary, Status, Logs.
- Settings preset: Basic, Strict, Custom.

Avoid:

- hardcoded names
- vague colorful badges
- fake production state
- generic rows like "active role" or "setting source" unless they map to a real contract
- child activity data on portal/account pages

## Product Truth

Portal and Account may show:

- parent account state
- subscription and plan state
- entitlement snapshots
- device-seat limits
- account mismatch or provider failure
- notification preferences and provider delivery health
- local parent portal health
- local Rust agent connection status
- version/platform/update metadata
- support bundle contents and redaction state

Portal and Account should not show:

- screen captures
- browser URLs
- app usage evidence
- game usage evidence
- network activity evidence
- generated child reports
- raw logs that include child private content
- anything that belongs to Activity, Policy, Data, or Devices

Every stale or unavailable state must be visible as a typed state, not hidden behind a normal-looking UI.

## External Services Boundary

This product is local-first. External services are allowed only for the things that cannot sensibly be local-only for a global product launch.

Allowed external service areas:

- parent registration and sign-in
- subscription and plan state
- Stripe Checkout, Stripe Billing, and Stripe Customer Portal flows
- entitlement snapshots derived from plan/subscription state
- public download/release/update metadata
- minimal notification routing metadata
- customer support account metadata and safe support contact flows

Not allowed by default:

- child activity evidence
- screen captures
- app/browser/game/network activity history
- generated child reports
- raw local logs with private evidence
- parent-owned report storage unless the parent explicitly chooses a destination

Cloud failure should show `stale`, `unavailable`, `queued`, or `degraded` state in the UI. It should not silently disable local safety behavior.

## Ocentra Games Pattern To Reuse Carefully

Ocentra Games already has useful infrastructure patterns:

- `infra/cloudflare` as the edge/backend boundary.
- Wrangler-driven local Worker development.
- Vite `/api` proxy to the local Worker during development.
- Seed scripts for local Cloudflare KV state.
- Product catalog records with Stripe price ids.
- Use deterministic UI-check fixture price IDs during planning (`price_ui_check_basic`, `price_ui_check_pro`, `price_ui_check_max`) while the contract-backed billing path is implemented.
- A dedicated payment Durable Object style boundary for payment state.
- Explicit go-live gaps for real Stripe price ids and production KV namespaces.

For Ocentra Parent, reuse the shape, not the full game backend:

- Keep an `infra/cloudflare` style boundary for hosted account/subscription APIs.
- Use local dev seed data for plan/catalog/account UI checks.
- Keep Vite as the local UI/HMR surface only.
- Keep Tauri/Rust as the production parent portal runtime path.
- Keep Stripe secrets and webhook verification only in Cloudflare/backend code, never in Vite or the Tauri frontend.
- Keep child evidence out of Cloudflare by default.

Suggested hosted services:

- Cloudflare Worker or Pages Function for account, subscription, entitlement, release, and notification metadata APIs.
- Cloudflare KV for public-ish plan catalog, release metadata, and feature flag/catalog snapshots.
- Cloudflare D1 or Durable Object storage for parent account/subscription linkage if we need stronger consistency.
- Cloudflare Queues for asynchronous Stripe webhook follow-up if webhook processing grows beyond a direct update.
- Stripe Billing plus Checkout Sessions for subscriptions.
- Stripe Customer Portal for upgrades, downgrades, cancellation, and payment method updates.

Avoid building manual subscription renewal logic. Stripe Billing should own renewals, trials, retries, dunning, and subscription state transitions. The Ocentra backend should translate Stripe events into typed entitlement snapshots that the parent portal can read.

External references to re-check before backend implementation:

- Stripe Billing and Checkout Sessions for subscriptions.
- Stripe Customer Portal for customer-managed plan/payment changes.
- Web Push API and mobile push/portal socket routing for parent-device notifications.
- Telegram Bot API for bot-linked notification delivery.
- WhatsApp Business Platform or provider documentation for approved-template delivery.

## Cloud And Payment Setup Backlog

This is backend-later. UI can be built first with explicit UI-check fake data, but the plan should preserve the future integration shape.

Cloudflare backlog:

- Define account/subscription/entitlement endpoint contracts in the domain packages first.
- Add an `infra/cloudflare` worker package for Ocentra Parent.
- Add local Wrangler dev scripts similar to Ocentra Games.
- Add a local seed script for plans, features, entitlements, and fake account status.
- Add production namespace/storage setup notes before go-live.
- Add route allowlists, CORS rules, and localhost dev security rules.
- Add typed error states for offline, unavailable, stale, provider failure, account mismatch, and validation failure.
- Add deployment and preview rules that do not claim production behavior before Stripe and account auth are real.

Stripe backlog:

- Define product/price catalog: Basic, Plus, Pro, Family, or whatever final plan names become.
- Define seat model: parent portal plus child device seats, including trial/grace behavior.
- Define Stripe Products and Prices for recurring subscriptions.
- Use Checkout Sessions for new subscriptions.
- Use Customer Portal for self-service plan changes and payment method management.
- Add webhook handling for subscription created, updated, deleted, trial ending, invoice paid, invoice payment failed, checkout completed, customer updated.
- Store Stripe customer id, subscription id, price id, status, current period, trial/grace dates, and last sync metadata as hosted non-activity account metadata.
- Convert Stripe state into an `EntitlementSnapshot` contract for the local portal.
- Add stale/unavailable behavior when Stripe or Cloudflare cannot be reached.
- Keep all Stripe secret keys and webhook secrets out of app/runtime UI code.

Global launch backlog:

- Decide supported countries/currencies at launch.
- Decide tax handling and invoices/receipts.
- Decide refund/cancellation policy copy.
- Decide account deletion and data export flow.
- Decide public account/download domain ownership.
- Decide production monitoring for account/payment APIs.
- Decide how support handles account/subscription questions without seeing child private data.

## Surface Plans

### Portal Settings

Purpose: settings for this parent portal app and local runtime. This page should not duplicate Alerts, Channels, Plan, or Access.

Expected tabs:

- Portal
- Privacy
- Runtime
- Updates

Portal tab:

- Parent portal identity: signed-in parent account state, local portal name, current device role.
- Family profile summary: family display name, primary parent, timezone, locale.
- App behavior defaults: start on login, lock portal after idle, confirm destructive changes.
- Local AI posture: free because it uses the parent's own hardware; show availability and hardware state, not pricing.

Privacy tab:

- Data custody default: local parent portal, parent-owned storage, or ask every time.
- Support sharing default: never attach diagnostics, ask each time, attach safe summary.
- Hosted metadata boundary: account/subscription/entitlement/notification route metadata only.
- Child evidence boundary: never sent to hosted services by default.

Runtime tab:

- Local Rust agent URL and connection state.
- Tauri/portal app version, platform, update channel.
- Local notification capability: desktop notifications available, mobile portal paired, web push unavailable, etc.
- Cloud metadata state: ready, stale, unavailable.

Updates tab:

- Release channel: Stable, Preview, Local.
- Current version and available version.
- Last update check.
- Changelog/open release notes action.

Possible controls:

- `ScopeToggle`: Stable, Preview, Local.
- `ScopeToggle`: Never Attach, Ask Each Time, Safe Summary.
- Regular toggles: start on login, lock after idle, confirm destructive changes.

Actions:

- Save portal settings.
- Reset portal defaults.
- Check for updates.
- Refresh runtime state.

Fake UI-check data:

- `PORTAL_ACCOUNT_UI_CHECK_SETTINGS`
- Example rows: local agent ready, cloud metadata stale, local AI available, support sharing asks every time.
- No fake child activity evidence.

### Alerts

Purpose: decide what is important enough to notify the parent, when it should notify, and how noisy it should be. Alerts are event rules. Channels are delivery methods.

Expected tabs:

- Rules
- Schedule
- Escalation
- History

Rules tab:

- Alert reason rows: ask-parent request, policy violation, device offline, sync failure, provider failure, unknown/suspicious activity.
- Each row shows enabled state, severity, delivery mode, dedupe window, and last triggered state.
- Safety-critical rows should visibly resist being disabled unless the parent confirms.

Schedule tab:

- Quiet hours.
- Digest window.
- Critical bypass rules.
- School/sleep profile handoff if those product concepts land later.

Escalation tab:

- What happens if the parent does not acknowledge.
- Example modes: notify once, repeat, escalate to backup channel, emergency only.
- This should connect to Channels but not configure the channel itself.

History tab:

- Minimal delivery/audit history: reason code, time, channel used, acknowledged state.
- No raw child evidence or private activity content.

Possible controls:

- `ScopeToggle`: Immediate, Digest, Quiet.
- `ScopeToggle`: Off, Schedule, Critical Only.
- `ScopeToggle`: Notify Once, Repeat, Escalate.

Actions:

- Save alert rules.
- Test selected alert using configured channels.
- Pause non-critical alerts.
- Review delivery history.

Fake UI-check data:

- `PORTAL_ACCOUNT_UI_CHECK_ALERTS`
- Example reasons: ask-parent request critical, device offline normal, provider failure normal, policy violation critical.
- Example history uses reason codes only, not real URLs/apps/screens.

### Channels

Purpose: configure where notifications go and prove each route is verified. Channels should not decide which events matter.

Expected tabs:

- Routes
- Verify
- Templates
- Delivery

Routes tab:

- In-app portal notification: always local if portal is running.
- Desktop OS notification: available when OS permission is granted.
- Mobile portal push/socket route: available when the parent's phone has the portal app paired.
- Email route: needs verified email.
- SMS route: needs verified phone and a provider.
- WhatsApp route: needs provider setup, opt-in/verified phone, and approved templates.
- Telegram route: needs bot linking; parent must start/link the bot before messages can be sent.

Verify tab:

- Email verification flow.
- Phone verification flow.
- WhatsApp verification/setup state.
- Telegram link code or deep link.
- Mobile portal pairing state.

Templates tab:

- Minimal safe notification previews.
- Template types: ask parent, device offline, provider failure, billing action needed.
- Template preview should hide private child details.

Delivery tab:

- Provider status: ready, not configured, failed, rate limited, stale.
- Retry queue: pending, retried, dropped.
- Last delivery result per route.

Possible controls:

- `ScopeToggle`: In-app, Mobile, Email, SMS, WhatsApp, Telegram.
- `ScopeToggle`: Ready, Needs Setup, Failed.
- `ScopeToggle`: Minimal, Parent Summary.

Actions:

- Verify route.
- Send test message.
- Disable route.
- Retry failed delivery.
- Pair mobile portal.
- Link Telegram.

Provider notes:

- Mobile portal is the cleanest first-class parent notification path once the portal app exists on phone.
- Web push/browser push is viable for web surfaces but needs a push subscription and permission flow.
- Telegram is practical only after the parent links a bot chat.
- WhatsApp needs a proper Business Platform/provider setup and template approval flow; do not treat it as a simple raw message API.
- SMS/email are ordinary provider integrations but still require verification and abuse controls.

Fake UI-check data:

- `PORTAL_ACCOUNT_UI_CHECK_CHANNELS`
- Example routes: in-app ready, desktop permission granted, mobile portal not paired, email verified, SMS unverified, WhatsApp setup required, Telegram not linked.
- Use synthetic fixture contacts only (`portal-account-fixture-parent@ocentra.demo`, no real person identifiers) so the UI can display realistic channel examples.

### Plan

Purpose: the selling and subscription-management surface. This page should look like plan cards plus the current subscription strip, not a generic settings table.

Draft pricing is not final, but the UI fixture can use this proposal:

- Basic: about USD 5/month, 15-day trial, 3 child devices included.
- Pro: about USD 10/month, 7 child devices included.
- Max: about USD 20/month, 15 child devices included.
- Extra child device seat: about USD 1/month each when allowed.
- Local AI: free because it runs on parent hardware.
- External AI credits: separate paid credit packs; exact usage pricing later.

Expected tabs:

- Plans
- Seats
- Billing
- AI Credits

Plans tab:

- Three large plan cards: Basic, Pro, Max.
- Current plan card gets selected treatment.
- Trial state: show days remaining during 15-day Basic trial.
- Compare rows: included devices, extra seat availability, report retention/window, remote convenience features, external AI credit support, priority support if applicable.
- Buttons: Current Plan, Upgrade, Downgrade, Start Trial, Manage Billing.

Seats tab:

- Included seats, used seats, extra seats, pending seats.
- Add/remove seat controls.
- Projected monthly cost when extra seats are added.
- Seat warning when current paired device count exceeds lower plan.

Billing tab:

- Stripe subscription state: trialing, active, past due, canceled, incomplete, unavailable.
- Current period, renewal date, payment action needed, invoice state.
- Customer Portal entry point for payment method, invoices, cancellation, upgrades/downgrades.

AI Credits tab:

- External AI credit balance.
- Buy credit pack cards.
- Local AI reminder: local hardware AI costs no Ocentra credits.
- Usage estimate row, but no hard pricing until the AI usage model is decided.

Possible controls:

- `ScopeToggle`: Monthly, Yearly if yearly pricing lands.
- `ScopeToggle`: Basic, Pro, Max for compact comparison mode.
- `ScopeToggle`: Included Seats, Extra Seats, Pending.

Actions:

- Start trial.
- Upgrade plan.
- Downgrade plan.
- Add/remove extra seats.
- Open Stripe Customer Portal.
- Buy external AI credits.
- Refresh plan state.

Fake UI-check data:

- `PORTAL_ACCOUNT_UI_CHECK_PLAN`
- Example current state: Basic trial, 11 days remaining, 2 of 3 devices used, no extra seats, 250 external AI credits.
- Plan cards must be fake-data marked and easy to remove or replace.

### Access

Purpose: actual entitlement truth after plan/payment state is resolved. If this remains unclear later, merge it into Plan. For now it is useful as "what do I currently have access to and why?"

Expected tabs:

- Entitlements
- Feature Gates
- Seats
- Credits

Entitlements tab:

- Snapshot source, fetched time, valid-until time, stale/unavailable state.
- Plan-derived rights: Basic/Pro/Max, trial/grace, extra seats.
- Account mismatch or provider failure warning.

Feature Gates tab:

- Feature matrix: local safety basics, local AI, extra seats, external AI credits, remote relay, advanced summaries, longer report history, priority support.
- Each feature shows included, limited, locked, unavailable, or stale.
- Locked rows should link back to Plan.

Seats tab:

- Current seat allowance and actual paired/claimed seats.
- Extra seat count and billing source.
- Warning if a downgrade would orphan devices or require unpairing.

Credits tab:

- External AI credit balance and last refresh.
- Credit pack purchase history summary.
- Local AI free state.

Possible controls:

- `ScopeToggle`: Included, Limited, Locked.
- `ScopeToggle`: Fresh, Stale, Unavailable.
- `ScopeToggle`: Safety, Convenience, AI, Support.

Actions:

- Refresh entitlements.
- Open Plan.
- Buy AI credits.
- Export entitlement snapshot.
- Resolve account mismatch.

Important rule:

Safety-critical local behavior should never disappear silently because hosted billing state is unavailable. The UI should show degraded or stale state clearly and explain what continues locally.

Fake UI-check data:

- `PORTAL_ACCOUNT_UI_CHECK_ACCESS`
- Example state: Basic trial active, local safety included, local AI included, external AI credits limited by balance, extra seats locked unless added.

### Support

Purpose: a chat-like contact form for sending a parent-authored message to the Ocentra support team.

Expected surface:

- Single Contact form only.
- No Tickets, Attachments, Diagnostics, or Status dashboard in this first pass.
- Fields: category, reply email, subject, message.
- Categories: billing, account, device pairing, notifications, app problem, safety concern, other.
- Clear note that the support team replies by email or the configured support channel.
- If the app is offline, queue locally and show not sent yet.
- Explicitly excluded from this form: child screenshots, browser/app/network evidence, raw private logs, and diagnostic bundles.

Actions:

- Save draft.
- Send message.
- Retry queued message when a backend connector exists.

Fake UI-check data:

- `PORTAL_ACCOUNT_UI_CHECK_SUPPORT_DRAFT`
- Example state: one local draft, no attachment, no fake live support reply.
- Do not fake a live human response as production truth.

## Future Function Intent

The UI should make the future integration shape clear even while using temporary UI-check data.

Proposed read functions:

- `getPortalSettingsReadModel()`
- `getAlertPreferencesReadModel()`
- `getNotificationChannelReadModel()`
- `getPlanCatalogReadModel()`
- `getSubscriptionStatusReadModel()`
- `getSeatUsageReadModel()`
- `getExternalAiCreditReadModel()`
- `getEntitlementSnapshotReadModel()`
- `getSupportContactDraftReadModel()`

Proposed intent functions:

- `savePortalSettingsIntent(input)`
- `saveAlertPreferencesIntent(input)`
- `verifyNotificationChannelIntent(input)`
- `linkNotificationChannelIntent(input)`
- `refreshSubscriptionStatusIntent()`
- `startTrialIntent(input)`
- `openBillingPortalIntent()`
- `changePlanIntent(input)`
- `addDeviceSeatIntent(input)`
- `removeDeviceSeatIntent(input)`
- `buyExternalAiCreditsIntent(input)`
- `refreshEntitlementsIntent()`
- `saveSupportDraftIntent(input)`
- `submitSupportMessageIntent(input)`

These names are provisional UI-read intents; replace each with a typed domain contract from the portal module before enabling real account data paths.

Expected typed states:

- `ready`
- `stale`
- `unavailable`
- `degraded`
- `unknown`
- `queued`
- `failed`
- `needs-parent-action`

Expected custody/source labels:

- `local-parent-portal`
- `local-rust-agent`
- `parent-owned-storage`
- `ocentra-hosted-non-activity`
- `provider-metadata`
- `ui-check-fake`

## Fake UI Data Policy

Temporary fake data is allowed only to judge UI. It should be impossible to mistake for production state.

Rules:

- Gate fake data with an explicit flag.
- Use verbose names like `PORTAL_ACCOUNT_UI_CHECK_FAKE_DATA_ENABLED`.
- Prefix sample data with `PORTAL_ACCOUNT_UI_CHECK_*`.
- Mark every fake row with `source: "ui-check-fake"`.
- Add TODO comments pointing to the real read model or contract that must replace it.
- Do not use hardcoded personal names.
- Do not show fake child activity evidence on these pages.
- Remove all fake data once real typed read models land.

## Implementation Phases

Do not start these until the plan is accepted.

1. Add or extend domain contracts for portal/account read models and typed states.
2. Add tests for the contracts and label/state helpers.
3. Normalize layout ownership so Portal and Account both use the portal-owned surface shell.
4. Build a dedicated `PortalAccountSurface` or equivalent shared panel instead of extending generic stub controls.
5. Add per-surface tabs and body renderers.
6. Add explicit UI-check fake data behind a loud flag.
7. Validate Portal routes: Settings, Alerts, Channels.
8. Validate Account routes: Plan, Access, Support.
9. Run responsive visual checks for wide, narrow, and mobile-like widths.
10. Add hosted account/subscription contract stubs only after the UI shape is accepted.
11. Add Cloudflare/Stripe backend wiring in a separate backend slice.
12. Remove fake data after real read models and Rust/Tauri/cloud query bridges exist.

## Open Decisions

- Should Account remain a visible side-panel group, or should it collapse into Portal visually?
- Should Install, Updates, and Platforms become Support/Portal Ops rather than Devices?
- Which notification channels are MVP: in-app plus email first, or include disabled future rows for mobile push/SMS/WhatsApp/Telegram?
- Which support bundle data classes are allowed before a privacy review?
- Does Portal Settings own parent profile/auth state, or should that stay under Account?
- Is the existing multi-choice SVG enough, or do we need a new portal option matrix SVG for feature gates and channel routing?
- Should Plan and Access be separate routes, or Plan tab plus Access tab under one Account surface?
- Should Cloudflare state use D1, Durable Objects, or a simpler KV/catalog plus Stripe-webhook snapshot model for MVP?
- Are launch plan names Basic, Pro, Max?
- Are launch draft prices USD 5, USD 10, USD 20 per month?
- Are included device limits 3, 7, and 15?
- Should extra child-device seats be available on every plan or only Pro/Max?
- Is the draft extra child-device seat price USD 1/month?
- What are the external AI credit packs and usage price model?
- Should registration live only on the public web/account surface, or should the packaged portal also embed the same auth flow?

## Non-Goals For This Slice

- No production subscription integration.
- No Stripe/provider direct calls from the portal UI.
- No Cloudflare account backend implementation.
- No final pricing decision.
- No child activity report display.
- No support bundle implementation.
- No cloud storage of child evidence.
- No device grid on portal/account pages.
- No implementation until the plan is reviewed.
