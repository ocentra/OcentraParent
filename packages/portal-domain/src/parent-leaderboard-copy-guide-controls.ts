import type { ParentLeaderboardCopyGuideTopic } from './parent-leaderboard-copy-guide-types';

export const PARENT_LEADERBOARD_COPY_CONTROL_GUIDES: readonly ParentLeaderboardCopyGuideTopic[] = [
  {
    id: 'browser-control',
    navLabel: 'WEB',
    rank: 3,
    title: 'Browser Control',
    subtitle: 'Managed, unmanaged, supported, unsupported',
    detail: 'Web visibility',
    tone: 'cyan',
    category: 'Control',
    subcategory: 'Browser evidence',
    pages: [
      {
        eyebrow: 'BROWSER BASICS',
        title: 'Managed browsers provide exact tab evidence',
        body: 'A managed browser is the strongest web path because the child-device agent can record supported browser tab evidence: URL, title, normalized domain, timestamp, source id, and evidence id. That evidence is stored locally before the parent app, reports, AI, or policy use it.',
        steps: [
          'Supported managed browser: exact URL and tab evidence can be available.',
          'Supported but unmanaged browser: visible as a bypass risk, not exact tab proof.',
          'Unsupported browser: the app may detect it but cannot claim URL control yet.',
          'Unmanaged or unknown browser use should be visible per child device so a parent can decide whether to allow, warn, ask, or block.',
        ],
      },
      {
        eyebrow: 'WHY UNMANAGED IS DIFFERENT',
        title: 'Process and network data cannot prove the open tab',
        body: 'Seeing Chrome, Edge, Firefox, or a network domain is not the same as knowing the active browser tab. Ocentra should not guess exact URLs from process names or network metadata. The UI must show the difference so parents know when the product has strong browser evidence and when it only sees possible bypass.',
        steps: [
          'Process evidence can show a browser is running.',
          'Network evidence can show domains, IPs, ports, and unusual traffic where available.',
          'Only the managed browser boundary can claim exact supported tab URL evidence.',
          'No page body text, form values, secrets, cookies, keystrokes, or decrypted HTTPS payloads are captured by default.',
        ],
      },
      {
        eyebrow: 'PARENT CHOICES',
        title: 'Choose what to do with each browser state',
        body: 'Parents need clear options per device. They may encourage the managed browser, allow unsupported browsers for school compatibility, warn on unmanaged browsers, ask-parent for unknown browsers, or block unmanaged browser use only where enforcement support exists.',
        steps: [
          'Use allow for trusted managed browser sessions and school sites.',
          'Use explain-first when the child uses a browser path that reduces visibility.',
          'Use ask-parent for exceptions such as school login issues or compatibility needs.',
          'Use block only when the child-device agent reports a supported enforcement path.',
        ],
      },
    ],
    tips: [
      {
        label: 'Plain language',
        body: 'Managed means Ocentra can see supported tab evidence. Unmanaged means the child may be outside that visibility path.',
        tone: 'cyan',
      },
      {
        label: 'Risk',
        body: 'Do not promise exact URLs for unsupported or unmanaged browsers.',
        tone: 'red',
      },
    ],
    actions: [
      {
        label: 'Review Web',
        body: 'Open Web to inspect supported, unsupported, managed, unmanaged, stale, and permission-limited states.',
        tone: 'cyan',
        targetRoutePath: '#/browser',
        targetNavLabel: 'WEB',
      },
      {
        label: 'Set browser rule',
        body: 'Start with explain-first or ask-parent before strict browser blocking.',
        tone: 'gold',
        targetRoutePath: '#/browser-settings',
        targetNavLabel: 'BROWSER SETUP',
      },
    ],
  },
  {
    id: 'monitoring-evidence',
    navLabel: 'ACTIVITY',
    rank: 4,
    title: 'Monitoring And Evidence',
    subtitle: 'Apps, games, network, screen summaries',
    detail: 'What we observe',
    tone: 'purple',
    category: 'Visibility',
    subcategory: 'Evidence sources',
    pages: [
      {
        eyebrow: 'WHAT WE CAN SHOW',
        title: 'Monitoring starts with stored local facts',
        body: 'Ocentra Parent is built around real, typed, timestamped evidence. The child-device agent writes evidence locally first, then query views, policy, AI, reports, and the parent portal can reference it. If evidence is missing or degraded, the app should say so.',
        steps: [
          'Process and window evidence show apps and foreground windows where the platform allows it.',
          'App and game sessions show running time, foreground time, run count, and known or unknown state.',
          'Network flow summaries show process, domain/IP, port, protocol, counts, and unusual indicators where available.',
          'Screen analysis is optional, parent-controlled, local-only, and stores summaries after temporary image deletion.',
        ],
      },
      {
        eyebrow: 'ACTIVITY DETAIL',
        title: 'Parents need context, not raw noise',
        body: 'The Activity area should help a parent answer what happened, when it happened, which source observed it, how confident it is, and whether a rule or alert used it. It should not become an endless list of raw logs.',
        steps: [
          'Group recent activity by child device, app, browser, site, game, category, and time window.',
          'Show live, stale, unavailable, unsupported, or degraded state near each source.',
          'Use timelines and report cards for summaries instead of long developer logs.',
          'Keep copy/export diagnostics separate from parent-facing activity summaries.',
        ],
      },
      {
        eyebrow: 'WHAT WE DO NOT DO',
        title: 'Monitoring has privacy limits',
        body: 'The normal product path does not capture keystrokes, chat content, browser secrets, cookies, decrypted HTTPS payloads, or raw screenshots as a long-term store. Sensitive data classes need explicit parent settings, local processing, deletion status, and clear custody labels.',
        steps: [
          'Browser evidence is URL/title/domain, not page body scraping.',
          'Network evidence is metadata, not decrypted content.',
          'Screen evidence stores summaries by default, not permanent images.',
          'Unknown states remain unknown until there is enough evidence for a stronger claim.',
        ],
      },
    ],
    tips: [
      {
        label: 'Good UX',
        body: 'Parents should see summary, source, confidence, and next action before raw evidence details.',
        tone: 'cyan',
      },
      {
        label: 'Honesty',
        body: 'Unsupported, permission-limited, and stale states are product information, not errors to hide.',
        tone: 'gold',
      },
    ],
    actions: [
      {
        label: 'Open Activity',
        body: 'Review live timeline, source labels, and local evidence status.',
        tone: 'purple',
        targetRoutePath: '#/activity',
        targetNavLabel: 'ACTIVITY',
      },
      {
        label: 'Open Reports',
        body: 'Use summaries when you need a day, week, or month view instead of raw events.',
        tone: 'cyan',
        targetRoutePath: '#/report-settings',
        targetNavLabel: 'REPORT SETUP',
      },
    ],
  },
  {
    id: 'rules-policy',
    navLabel: 'RULES',
    rank: 5,
    title: 'Rules And Policy',
    subtitle: 'House rules, schedules, approvals',
    detail: 'Your house your rule',
    tone: 'gold',
    category: 'Control',
    subcategory: 'Policy',
    pages: [
      {
        eyebrow: 'HOUSE RULES',
        title: 'Parent rules are the household authority',
        body: 'Ocentra provides evidence, categories, local AI, previews, and enforcement adapters. It does not secretly decide your household policy. A parent-authored rule says what should happen for a child, device, schedule, app, site, category, or evidence source.',
        steps: [
          'Choose who the rule applies to: family, child, device, browser, app, or schedule.',
          'Choose a target: app, process, game, site, domain, category, video, network indicator, or screen-derived category.',
          'Choose the result: allow, warn, explain-first, time-limit, ask-parent, block, or observe-only.',
          'Preview the rule before enforcement so evidence, schedule, conflict, and reason are visible.',
        ],
      },
      {
        eyebrow: 'POLICY DECISIONS',
        title: 'Rules become decisions on the child device',
        body: 'The parent app sends a typed rule or approval intent. The child-device agent validates rule version, target, evidence, schedule, child profile, device identity, local AI reference when used, and conflict resolution before producing a policy decision.',
        steps: [
          'Invalid rules do not activate.',
          'Missing evidence should become unknown, ask-parent, warn, or no-op according to your rule.',
          'Local AI can help classify, but parent rules decide the household action.',
          'Dry-run decisions explain what would happen before any adapter changes device behavior.',
        ],
      },
      {
        eyebrow: 'APPROVALS',
        title: 'Ask-parent needs a clear parent path',
        body: 'Permission requests should show what the child requested, what evidence is available, which rule matched, when the request expires, and what happens if the parent does not respond. Approvals, denials, and overrides need an audit trail.',
        steps: [
          'Use ask-parent for ambiguous or exception-heavy situations.',
          'Time-box temporary approvals so they do not become permanent loopholes.',
          'Show expired, pending, approved, denied, and superseded states.',
          'Keep approval history visible for later reports and rule tuning.',
        ],
      },
    ],
    tips: [
      {
        label: 'Best default',
        body: 'Start advisory and ask-parent. Move to block or timeout only after previews match your intent.',
        tone: 'cyan',
      },
      {
        label: 'Safety',
        body: 'A category label alone should never block. A matching parent rule and typed decision must exist.',
        tone: 'gold',
      },
    ],
    actions: [
      {
        label: 'Create rule',
        body: 'Use Rules to define family defaults, child-specific rules, schedules, and approvals.',
        tone: 'gold',
        targetRoutePath: '#/rule-management',
        targetNavLabel: 'RULE SETUP',
      },
      {
        label: 'Preview policy',
        body: 'Use dry-run preview before moving a rule to enforcement.',
        tone: 'cyan',
        targetRoutePath: '#/policy',
        targetNavLabel: 'RULES',
      },
    ],
  },
  {
    id: 'enforcement-control',
    navLabel: 'RULES',
    rank: 6,
    title: 'Enforcement',
    subtitle: 'Block, timeout, terminate, rollback',
    detail: 'After policy preview',
    tone: 'red',
    category: 'Control',
    subcategory: 'Enforcement',
    pages: [
      {
        eyebrow: 'HIGHER BAR',
        title: 'Enforcement changes device behavior',
        body: 'Blocking, terminating, timing out, or rolling back activity requires a stronger safety bar. The child-device agent can enforce only after a typed policy decision references evidence, rules, and supported adapter capability. The portal does not enforce directly.',
        steps: [
          'Dry-run first so parents can see what would happen.',
          'Confirm adapter support for the target platform and target type.',
          'Record action, result, evidence refs, policy decision, timer state, and rollback state.',
          'Show unavailable or degraded instead of pretending every platform can block everything.',
        ],
      },
      {
        eyebrow: 'PARENT RESULT',
        title: 'Parents must see what happened and why',
        body: 'A parent should be able to tell whether a rule would enforce, actually enforced, failed, expired, rolled back, was superseded, or did nothing. This matters for browser bypass, game time limits, network restrictions, screen-derived risks, and temporary approvals.',
        steps: [
          'Show decision reason and matching parent rule.',
          'Show adapter status: succeeded, failed, unavailable, no-op, expired, or rolled back.',
          'Show timer creation, expiry, cancellation, and recovery after restart.',
          'Keep billing and cloud outages outside critical local safety logic.',
        ],
      },
    ],
    tips: [
      {
        label: 'Do not rush',
        body: 'Preview and explain before enabling strong blocking.',
        tone: 'gold',
      },
      {
        label: 'Platform limits',
        body: 'Windows comes first. macOS, Linux, Android, and iOS need separate proof before enforcement claims.',
        tone: 'red',
      },
    ],
    actions: [
      {
        label: 'Use dry-run',
        body: 'Enable dry-run preview before enforcement adapters affect behavior.',
        tone: 'cyan',
        targetRoutePath: '#/policy',
        targetNavLabel: 'RULES',
      },
      {
        label: 'Review audit',
        body: 'Inspect enforcement results, failures, expiry, and rollback state in policy history.',
        tone: 'gold',
        targetRoutePath: '#/enforcement',
        targetNavLabel: 'ENFORCE',
      },
    ],
  },
] as const;
