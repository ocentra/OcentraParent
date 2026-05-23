import type { ParentLeaderboardCopyGuideTopic } from './parent-leaderboard-copy-guide-types';

export const PARENT_LEADERBOARD_COPY_START_GUIDES: readonly ParentLeaderboardCopyGuideTopic[] = [
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
          'Keep data custody visible before enabling remote access, drives, exports, reports, or assistant features.',
          'Review alerts and quiet hours so the system reduces anxiety instead of creating noise.',
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
        targetRoutePath: '#/devices',
        targetNavLabel: 'DEVICES',
      },
      {
        label: 'Privacy check',
        body: 'Connect parent-owned storage only when you want backup, remote reports, or cross-device continuity.',
        tone: 'cyan',
        targetRoutePath: '#/drive-connections',
        targetNavLabel: 'DRIVES',
      },
    ],
  },
  {
    id: 'setup-devices',
    navLabel: 'DEVICES',
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
        targetRoutePath: '#/devices',
        targetNavLabel: 'DEVICES',
      },
      {
        label: 'Check source',
        body: 'Confirm whether the view is live local, LAN, parent cache, parent-owned storage, or unavailable.',
        tone: 'gold',
        targetRoutePath: '#/diagnostics',
        targetNavLabel: 'SUPPORT',
      },
    ],
  },
] as const;
