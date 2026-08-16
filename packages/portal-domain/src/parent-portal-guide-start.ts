import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_NAV_LABELS } from './parent-portal-nav';

export const PARENT_PORTAL_START_GUIDES: readonly ParentPortalGuideTopic[] = [
  {
    id: 'setup-overall',
    navLabel: 'START HERE',
    rank: 1,
    title: 'Set Up Ocentra Parent',
    subtitle: 'The plain-language path from install to control',
    detail: 'Family setup map',
    tone: 'cyan',
    category: 'Start',
    subcategory: 'Setup overview',
    pages: [
      {
        eyebrow: 'START HERE',
        title: 'Set up the family console in a clear order',
        body: 'Ocentra Parent is a parent-owned control and observability app. Start by getting the parent app ready, pairing a child device, choosing simple household defaults, and confirming where data will live before turning on deeper reports or enforcement.',
        steps: [
          'Open the parent app and confirm the selected family console.',
          'Install or connect the child-device agent and verify live, stale, or unavailable status.',
          'Pair the child device before sending rules, approvals, alerts, or export settings.',
          'Choose the basic rule style first: allow, explain-first, ask-parent, time-limit, or block.',
        ],
      },
      {
        eyebrow: 'WHAT PARENTS CONTROL',
        title: 'Use simple controls first, then tune the details',
        body: 'The first experience should work for a busy parent who does not know browser internals or device policy. Advanced controls remain available, but the product should explain each control in plain language and show the exact device, source, and status before a parent relies on it.',
        steps: [
          'Quick Glance answers what is happening today and whether the child device is reachable.',
          'Guide explains browser, activity, rules, AI, reports, privacy, alerts, subscription, and platform limits.',
          'Manage is where parents pair devices, connect drives, tune alerts, change subscription, and export support data.',
          'Every action should say whether it is advisory, dry-run, observe-only, enforcement-ready, unavailable, or blocked by platform limits.',
        ],
      },
      {
        eyebrow: 'SAFE FIRST ORDER',
        title: 'Do not turn on powerful controls blindly',
        body: 'The product should make the safe order obvious: observe first, preview rules, then enforce only where the child-device agent reports a supported adapter. If a source is missing, unsupported, stale, or unmanaged, the app should say so instead of pretending control exists.',
        steps: [
          'Confirm evidence sources before writing strong rules.',
          'Use dry-run previews before enabling block, terminate, or strict timeout behavior.',
          'Keep data custody visible before enabling remote access, drives, exports, reports, or generated explanations.',
          'Review alerts and quiet hours so the system reduces anxiety instead of creating noise.',
        ],
      },
      {
        eyebrow: 'WHERE EACH THING LIVES',
        title: 'Use Guide to learn, then Manage to change settings',
        body: 'Every roadmap feature should have a parent-facing home. Device trust lives under Devices and LAN Pairing. Browser choices live under Web and Browser Setup. AI model and API choices live under AI Setup and API Keys. Data movement lives under Drives and Export/Delete. Reports live under Reports and Report Compiler. Billing state lives under Subscription and Entitlements. Install proof lives under Platforms and Updates.',
        steps: [
          'Quick Glance is for current state, source health, and today summaries.',
          'Guide is for plain-language explanation, risks, examples, and setup order.',
          'Manage is for per-device settings, provider setup, exports, alerts, subscription, updates, and audit.',
          'Support and Capability explain what is live, stale, unsupported, unavailable, planned, or manual-only.',
        ],
      },
    ],
    tips: [
      {
        label: 'For non-technical parents',
        body: 'Start with explain-first, ask-parent, and daily reports. Tighten rules only after evidence looks correct.',
        tone: 'cyan',
      },
      {
        label: 'For technical parents',
        body: 'Use per-device evidence state, custody labels, local AI status, and rule previews before enforcement.',
        tone: 'purple',
      },
    ],
    actions: [
      {
        label: 'First action',
        body: 'Pair a child device and confirm the selected device before relying on any rule or report.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.Devices),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      },
      {
        label: 'LAN trust',
        body: 'Use explicit LAN pairing before a parent device can control a child agent on the home network.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.LanPairing),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      },
      {
        label: 'Privacy check',
        body: 'Connect parent-owned storage only when you want backup, remote reports, or cross-device continuity.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.DriveConnections),
        targetNavLabel: 'DRIVES',
      },
      {
        label: 'Data movement',
        body: 'Review export, sync, retention, delete, and audit before moving family data across boundaries.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.ExportRetention),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Export,
      },
      {
        label: 'Plan setup',
        body: 'Review the plan, device limit, and paid feature boundaries before adding more devices.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.Subscription),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Plan,
      },
    ],
  },
  {
    id: 'setup-devices',
    navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
    rank: 2,
    title: 'Devices And Pairing',
    subtitle: 'Parent app, child agent, trusted devices',
    detail: 'Pair before control',
    tone: 'cyan',
    category: 'Setup',
    subcategory: 'Devices',
    pages: [
      {
        eyebrow: 'DEVICE SETUP',
        title: 'The child-device agent is the authority',
        body: 'The parent portal does not monitor the child by itself. The child-device agent observes activity, stores evidence, validates parent requests, and executes policy or enforcement locally. The parent app sends typed requests and displays validated status.',
        steps: [
          'Install the child-device agent on the child device, starting with Windows-first support.',
          'Open the parent app on the parent device and select the child device.',
          'Pair over loopback or explicit LAN so anonymous control is rejected.',
          'Confirm device health, evidence-store status, and link source before applying rules.',
        ],
      },
      {
        eyebrow: 'PER DEVICE',
        title: 'Each child device can have its own limits',
        body: 'Parents need to understand which rule applies to which child, device, schedule, browser, app, or platform. A laptop, phone, game PC, and school device may need different visibility and control because the OS capabilities differ.',
        steps: [
          'Use family defaults for broad household behavior.',
          'Override per child or per device when a device has different school or platform needs.',
          'Show offline, stale, unavailable, and wrong-device states clearly.',
          'Revoke a parent device before it can send another rule or approval.',
        ],
      },
    ],
    tips: [
      {
        label: 'Important',
        body: 'A browser tab in the parent app is not a child-device agent. It can only talk to a trusted agent.',
        tone: 'gold',
      },
      {
        label: 'LAN safety',
        body: 'LAN control must be explicit, paired, origin-checked, and rejected when stale or wrong-device.',
        tone: 'cyan',
      },
    ],
    actions: [
      {
        label: 'Pair device',
        body: 'Open Devices to add, select, revoke, or inspect a child device.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.Devices),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      },
      {
        label: 'Pair LAN',
        body: 'Use LAN Pairing for explicit trusted-device proof, stale rejection, and revocation.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.LanPairing),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      },
      {
        label: 'Check source',
        body: 'Confirm whether the view is live local, LAN, parent cache, parent-owned storage, or unavailable.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.CapabilityStatus),
        targetNavLabel: 'CAPABILITY',
      },
      {
        label: 'Platform status',
        body: 'Check which desktop and mobile surfaces are supported before claiming device control.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.PlatformsInstall),
        targetNavLabel: 'PLATFORMS',
      },
    ],
  },
] as const;
