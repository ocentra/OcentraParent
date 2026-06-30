import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_NAV_LABELS } from './parent-portal-nav';

export const PARENT_PORTAL_GUIDE_QUERY = {
  Topic: 'guideTopic',
  Page: 'guidePage',
} as const;

export const PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS = {
  Overview: 'rules-policy',
  Browser: 'browser-policy-guide',
  Apps: 'apps-policy-guide',
  Games: 'games-policy-guide',
  ScreenNetwork: 'screen-network-policy-guide',
  Tracking: 'tracking-policy-guide',
  Enforcement: 'enforcement-control',
} as const;

export const PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES = {
  Rules: 0,
  Schedule: 1,
  Budget: 2,
  Approvals: 3,
  Audit: 4,
} as const;

export const PARENT_PORTAL_CONTROL_GUIDES: readonly ParentPortalGuideTopic[] = [
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
        targetRoutePath: portalRouteHashPath(PortalRoute.Browser),
        targetNavLabel: 'WEB',
      },
      {
        label: 'Set browser rule',
        body: 'Start with explain-first or ask-parent before strict browser blocking.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.BrowserSettings),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Browser,
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
        targetRoutePath: portalRouteHashPath(PortalRoute.Activity),
        targetNavLabel: 'ACTIVITY',
      },
      {
        label: 'Open Reports',
        body: 'Use summaries when you need a day, week, or month view instead of raw events.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.Activity),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      },
    ],
  },
  {
    id: 'rules-policy',
    navLabel: 'RULES',
    rank: 5,
    title: 'Rules',
    subtitle: 'Family defaults, device overrides, schedules, approvals',
    detail: 'Your house, your choice',
    tone: 'gold',
    category: 'Control',
    subcategory: 'Policy',
    pages: [
      {
        eyebrow: 'HOUSE RULES',
        title: 'Start with a family rule, override only when needed',
        body: 'Rules are parent choices. Ocentra can show evidence, categories, previews, and explanations, but it should not secretly decide household policy. A family rule applies to everyone. A per-device rule is only for a child device that needs a different choice.',
        steps: [
          'Pick Family for the normal household default.',
          'Pick Per device only when one child or device needs a different rule.',
          'Choose the target: app, game, site, domain, browser state, category, schedule, or evidence source.',
          'Choose the result: allow, ask, explain, time-limit, block, or observe-only.',
          'Preview before strong enforcement so the matching evidence, schedule, conflict, and reason are visible.',
        ],
      },
      {
        eyebrow: 'POLICY DECISIONS',
        title: 'Rules become typed decisions on the child device',
        body: 'The parent app sends a typed rule or approval intent. The child-device agent checks rule version, target, evidence, schedule, child profile, device identity, local AI reference when used, and conflicts before producing a decision.',
        steps: [
          'Invalid rules do not activate.',
          'Missing evidence should become unknown, ask-parent, warn, or no-op according to your rule.',
          'Local AI can help classify, but parent rules decide the household action.',
          'Dry-run decisions explain what would happen before any adapter changes device behavior.',
        ],
      },
      {
        eyebrow: 'APPROVALS',
        title: 'Ask-parent is for exceptions and unclear cases',
        body: 'Permission requests should show what the child requested, what evidence is available, which rule matched, when the request expires, and what happens if the parent does not respond. Approvals, denials, and overrides need a visible history.',
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
        body: 'Start with allow, explain, or ask-parent. Move to block or timeout only after previews match your intent.',
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
        label: 'Manage rules',
        body: 'Use Rules to define family defaults, child-specific rules, schedules, and approvals.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.RuleManagement),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.RuleSet,
      },
      {
        label: 'Preview policy',
        body: 'Use dry-run preview before moving a rule to enforcement.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.Policy),
        targetNavLabel: 'RULES',
      },
    ],
  },
  {
    id: PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.Browser,
    navLabel: 'RULES',
    rank: 6,
    title: 'Browser Policy',
    subtitle: 'Sites, searches, video, downloads, managed and unmanaged browsers',
    detail: 'Web rules explained',
    tone: 'cyan',
    category: 'Control',
    subcategory: 'Browser',
    pages: [
      {
        eyebrow: 'BROWSER RULES',
        title: 'Pick targets parents can understand',
        body: 'Browser rules should start with a small set of visible targets: social media, browser games, search, video, downloads, unknown sites, blocked sites, managed browser time, and unmanaged browser use. The rule says what happens when that target is active.',
        steps: [
          'Block means the target is not allowed in that window.',
          'Ask means the child can request an exception and the parent decides.',
          'Allow means the target is explicitly permitted even if another broader rule is stricter.',
          'Observe means Ocentra records activity without changing what the child can do.',
        ],
      },
      {
        eyebrow: 'BROWSER SCHEDULE',
        title: 'Schedule windows decide when a rule can run',
        body: 'The schedule timeline is for time windows, not for every browser setting. A parent drags an action onto a day or time range, then opens the clip when they need finer browser targets inside that window.',
        steps: [
          'Use schedule for school, homework, bedtime, morning, weekend, and temporary exception windows.',
          'Keep the default empty space as observe-only unless a stronger action clip covers it.',
          'Use per-device override only when one child device needs a different week.',
          'Use audit to confirm what the final family plus per-device timeline will apply.',
        ],
      },
      {
        eyebrow: 'BROWSER BUDGET',
        title: 'Budgets count time, schedules place time',
        body: 'A budget is a number such as daily browser time, unmanaged browser time, social media minutes, or video minutes. The budget tab should hold the cap and counting rules. The schedule tab should only show when those caps or actions are active.',
        steps: [
          'Total browser time can include managed and unmanaged browser time.',
          'A target cap can count only one target such as social media, video, games, or downloads.',
          'When a cap ends, the parent chooses ask, block, allow school tools, or observe-only.',
          'A child-specific cap overrides the family cap only after the override is enabled.',
        ],
      },
      {
        eyebrow: 'BROWSER APPROVALS',
        title: 'Ask-parent needs a clear fallback',
        body: 'Approvals explain what a child can ask for, who is notified, how long the answer lasts, and what happens if the parent is quiet. Browser approvals are most useful for unknown sites, blocked sites, downloads, unmanaged browsers, and temporary time extensions.',
        steps: [
          'Show the request target and evidence before the parent decides.',
          'Use once, session, today, custom, or schedule duration labels.',
          'No answer should have a visible fallback such as deny, wait, observe, or current rule.',
          'Record approval, denial, timeout, and superseded states for later review.',
        ],
      },
      {
        eyebrow: 'BROWSER AUDIT',
        title: 'Audit shows the effective rule before apply',
        body: 'Audit is the checkpoint. It should show the family rule, child override, schedule window, budget cap, request fallback, capability support, and any conflict before the parent trusts the result.',
        steps: [
          'Show what matched and what did not match.',
          'Show family default and per-device override side by side.',
          'Show unsupported or degraded browser evidence instead of hiding it.',
          'Use audit again after a parent changes rules, schedules, budgets, or approvals.',
        ],
      },
    ],
    tips: [
      {
        label: 'Browser split',
        body: 'Exact tab evidence belongs to supported managed browsers. Unmanaged browser use is a visibility risk.',
        tone: 'cyan',
      },
      {
        label: 'Budget split',
        body: 'Do not put total caps into the timeline. Put caps in Budget and use Schedule for windows.',
        tone: 'gold',
      },
    ],
    actions: [
      {
        label: 'Browser setup',
        body: 'Return to Browser Manage and adjust rules, schedule, budget, approvals, or audit.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.BrowserSettings),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Browser,
      },
      {
        label: 'Browser state',
        body: 'Open Web to inspect managed, unmanaged, supported, unsupported, and stale evidence.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.Browser),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Web,
      },
    ],
  },
  {
    id: PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.Apps,
    navLabel: 'RULES',
    rank: 7,
    title: 'App Policy',
    subtitle: 'Installed apps, app categories, focus windows, per-device overrides',
    detail: 'Apps without clutter',
    tone: 'gold',
    category: 'Control',
    subcategory: 'Apps',
    pages: [
      {
        eyebrow: 'APP RULES',
        title: 'Rules should start from known app groups',
        body: 'App policy should let parents choose known apps, unknown apps, app categories, school tools, launch count, foreground time, and install or update behavior without forcing every app into one giant list.',
        steps: [
          'Use app groups for social, games, school, creative, browser, media, and unknown apps.',
          'Use allow for trusted school and family tools.',
          'Use ask for new or unknown apps when the parent wants a decision point.',
          'Use block only when the platform adapter can enforce it.',
        ],
      },
      {
        eyebrow: 'APP SCHEDULE',
        title: 'Windows tell when an app action applies',
        body: 'App schedules are useful for homework, bedtime, class time, chores, weekends, and temporary exceptions. A schedule window should not replace the app list; it should apply a chosen action to selected app targets.',
        steps: [
          'Use full-day clips only for simple household defaults.',
          'Use shorter windows for school, homework, bedtime, and family time.',
          'Leave unused time as observe-only if the parent only wants reporting.',
          'Use per-device override when one device has different school or app needs.',
        ],
      },
      {
        eyebrow: 'APP BUDGET',
        title: 'Caps belong with app counting rules',
        body: 'App budgets count minutes or sessions. They can apply to total app time, a category, one app, unknown apps, or foreground-only activity. Budget should define what counts before schedule decides when it counts.',
        steps: [
          'Total app time is not the same as one category cap.',
          'Foreground time is usually clearer for parents than background process time.',
          'A cap can end with ask, block, warn, or observe depending on the family rule.',
          'Show whether a child override inherits or replaces the family cap.',
        ],
      },
      {
        eyebrow: 'APP APPROVALS',
        title: 'Requests should explain app identity',
        body: 'App approvals need the app name, publisher or package id when available, device, requested duration, current rule, and fallback if the parent is quiet.',
        steps: [
          'Use ask for new apps, unknown apps, installs, extensions, and temporary app time.',
          'Show if an app is known, unknown, school-related, or recently installed.',
          'Use time-boxed approvals so exceptions expire.',
          'Record approval history so the parent can tune rules later.',
        ],
      },
      {
        eyebrow: 'APP AUDIT',
        title: 'Effective app policy should be readable',
        body: 'Audit should show which app target matched, whether the app evidence is fresh, which rule applied, whether a schedule or budget changed the result, and whether the adapter can enforce it.',
        steps: [
          'Show app identity and category source.',
          'Show family default, child override, and current schedule window.',
          'Show budget remaining or exhausted state.',
          'Show unsupported actions as unavailable, not applied.',
        ],
      },
    ],
    tips: [
      {
        label: 'App lists',
        body: 'Use groups first, then let parents drill into individual apps only when needed.',
        tone: 'gold',
      },
    ],
    actions: [
      {
        label: 'App policy',
        body: 'Return to the app policy workspace.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.PolicyApps),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Apps,
      },
    ],
  },
  {
    id: PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.Games,
    navLabel: 'RULES',
    rank: 8,
    title: 'Game Policy',
    subtitle: 'Game sessions, stores, browser games, weekend windows, caps',
    detail: 'Play time rules',
    tone: 'purple',
    category: 'Control',
    subcategory: 'Games',
    pages: [
      {
        eyebrow: 'GAME RULES',
        title: 'Separate games from school and creative tools',
        body: 'Game policy should make it clear whether the target is an installed game, browser game, game store, launcher, unknown game, or media that only looks game-like. Parents should see the difference before choosing allow, ask, limit, block, or observe.',
        steps: [
          'Keep school tools and creative tools out of game caps unless the parent chooses them.',
          'Treat browser games as browser targets and game targets when both evidence paths exist.',
          'Use ask for unknown launchers or stores.',
          'Use observe to learn play patterns before strict limits.',
        ],
      },
      {
        eyebrow: 'GAME SCHEDULE',
        title: 'Game windows are usually family rhythm windows',
        body: 'Game schedules should be easy to read as school day, homework, bedtime, weekend, and temporary reward windows. Parents should be able to set weekend rules without rebuilding every weekday.',
        steps: [
          'Use weekday and weekend presets as starting points.',
          'Use bedtime block windows when sleep is the main concern.',
          'Use temporary allow windows for rewards or family plans.',
          'Keep observe-only gaps visible in audit and reports.',
        ],
      },
      {
        eyebrow: 'GAME BUDGET',
        title: 'Caps explain how much play time is allowed',
        body: 'Game budgets can be daily, weekly, weekday, weekend, session-based, or category-based. The cap should explain whether it counts foreground play, launcher time, browser games, or all game-like activity.',
        steps: [
          'Daily caps are easier for younger kids.',
          'Weekly caps work better for weekend tradeoffs.',
          'Session caps prevent one very long play session.',
          'A cap ending should produce ask, block, warn, or observe based on parent choice.',
        ],
      },
      {
        eyebrow: 'GAME APPROVALS',
        title: 'Temporary extensions should expire',
        body: 'Game approvals should handle extra time, new game launch, store access, multiplayer windows, and exception requests. They should show when the approval ends.',
        steps: [
          'Use once or session duration for one-off play.',
          'Use today for a limited reward.',
          'Use custom only when the parent needs an exact override.',
          'Show what rule resumes when the approval expires.',
        ],
      },
      {
        eyebrow: 'GAME AUDIT',
        title: 'Audit should explain why play was allowed or stopped',
        body: 'Parents need to know whether a game was allowed because of schedule, remaining budget, approval, exception, missing evidence, or unsupported enforcement.',
        steps: [
          'Show game identity, category, and evidence source.',
          'Show remaining cap and current window.',
          'Show approval state if an exception changed the result.',
          'Show unsupported enforcement as degraded capability.',
        ],
      },
    ],
    tips: [
      {
        label: 'Weekend clarity',
        body: 'Game policy usually needs different weekday and weekend patterns.',
        tone: 'purple',
      },
    ],
    actions: [
      {
        label: 'Game policy',
        body: 'Return to the game policy workspace.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.PolicyGames),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Games,
      },
    ],
  },
  {
    id: PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.ScreenNetwork,
    navLabel: 'RULES',
    rank: 9,
    title: 'Screen And Network',
    subtitle: 'Screen summaries, network metadata, risk signals, privacy limits',
    detail: 'Evidence boundaries',
    tone: 'cyan',
    category: 'Control',
    subcategory: 'Evidence',
    pages: [
      {
        eyebrow: 'EVIDENCE RULES',
        title: 'Screen and network rules must show confidence',
        body: 'Screen and network signals can help parents understand risk, but they should not pretend to know more than they do. A rule should show whether it uses screen summary, network metadata, domain match, process evidence, or a stronger managed app or browser signal.',
        steps: [
          'Use observe for weak or new signals.',
          'Use ask when the parent wants a decision on ambiguous evidence.',
          'Use block only when a supported adapter and strong target exist.',
          'Show confidence and source near each decision.',
        ],
      },
      {
        eyebrow: 'EVIDENCE SCHEDULE',
        title: 'Schedule sensitive monitoring by family context',
        body: 'Screen and network schedules are useful for school hours, sleep hours, travel, and high-risk windows. They should be explicit because these controls can feel more sensitive than app or browser limits.',
        steps: [
          'Use school-hour windows for learning-related visibility.',
          'Use bedtime windows for device quiet time and risky network checks.',
          'Use observe-only windows when the parent only wants reports.',
          'Use per-device overrides for devices with different permissions.',
        ],
      },
      {
        eyebrow: 'EVIDENCE BUDGET',
        title: 'Budgets here are usually thresholds, not simple minutes',
        body: 'Screen and network budgets may be counts, rates, thresholds, or review limits instead of direct screen minutes. They should explain what is counted and what happens when the threshold is crossed.',
        steps: [
          'Count risky network events separately from normal traffic volume.',
          'Keep screen summary frequency separate from raw screen capture.',
          'Use thresholds for repeated unknown domains, unusual ports, or high-volume events.',
          'Show threshold resets and retention clearly.',
        ],
      },
      {
        eyebrow: 'EVIDENCE APPROVALS',
        title: 'Parents need consent and fallback wording',
        body: 'Approvals can ask for stronger monitoring, temporary screen analysis, network review, or remote support. The request must show what data is collected and how it is stored.',
        steps: [
          'Show local-only or export status before the parent approves.',
          'Use temporary approvals for stronger review modes.',
          'Show no-answer fallback such as wait, observe, deny, or current rule.',
          'Record who approved and what data class was affected.',
        ],
      },
      {
        eyebrow: 'EVIDENCE AUDIT',
        title: 'Audit protects against overclaiming',
        body: 'Audit should show what signal existed, what signal was missing, what rule used it, and what privacy or adapter limit prevented a stronger action.',
        steps: [
          'Show evidence source and confidence.',
          'Show redaction and retention state.',
          'Show whether an action was preview-only, applied, unavailable, or no-op.',
          'Show when AI classification helped but did not decide the rule.',
        ],
      },
    ],
    tips: [
      {
        label: 'Privacy honesty',
        body: 'Network metadata and screen summaries are useful, but exact content claims need stronger evidence.',
        tone: 'cyan',
      },
    ],
    actions: [
      {
        label: 'Screen policy',
        body: 'Return to the screen policy workspace.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.PolicyScreen),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Screen,
      },
      {
        label: 'Network policy',
        body: 'Return to the network policy workspace.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.PolicyNetwork),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Network,
      },
    ],
  },
  {
    id: PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.Tracking,
    navLabel: 'RULES',
    rank: 10,
    title: 'Tracking Policy',
    subtitle: 'Known places, unknown places, school hours, abnormal movement',
    detail: 'Location meaning',
    tone: 'cyan',
    category: 'Control',
    subcategory: 'Tracking',
    pages: [
      {
        eyebrow: 'TRACKING RULES',
        title: 'Location rules should describe safe, known, and unexpected places',
        body: 'Tracking policy is not a browser-style allow list. Parents usually need known places, safe places, school hours, unknown place alerts, unexpected movement, and device tracking availability.',
        steps: [
          'Known place means the parent has named the location.',
          'Safe place means the parent expects the child may be there.',
          'Unexpected place means the schedule or school-hour context makes it abnormal.',
          'Unavailable location should be shown as missing evidence, not as safe or unsafe.',
        ],
      },
      {
        eyebrow: 'TRACKING SCHEDULE',
        title: 'Time context makes location meaningful',
        body: 'A place can be normal after school but abnormal during school. Tracking schedules should let parents define school hours, commute windows, activities, sleep time, and exception windows.',
        steps: [
          'Use school-hour windows for expected campus or class locations.',
          'Use commute windows for movement between known places.',
          'Use activity windows for clubs, sports, or family events.',
          'Use observe-only if the parent only wants a report trail.',
        ],
      },
      {
        eyebrow: 'TRACKING BUDGET',
        title: 'Tracking budgets are frequency and alert limits',
        body: 'Tracking budgets are often update frequency, alert limits, stale thresholds, or review windows rather than usage minutes. They should explain battery, privacy, and evidence freshness tradeoffs.',
        steps: [
          'Use stale thresholds to show when location is too old.',
          'Use alert limits so parents are not flooded.',
          'Use high-frequency updates only for temporary safety windows.',
          'Show family default and child-device override separately.',
        ],
      },
      {
        eyebrow: 'TRACKING APPROVALS',
        title: 'Requests should be clear about place and duration',
        body: 'Tracking approvals can cover temporary places, one-time travel, stronger refresh, emergency unlock, or parent-requested check-in. They should expire.',
        steps: [
          'Show requested place or route context.',
          'Show duration and what happens when it expires.',
          'Show who is notified and what the fallback is.',
          'Record emergency and safety-related overrides in audit.',
        ],
      },
      {
        eyebrow: 'TRACKING AUDIT',
        title: 'Audit shows applied place logic',
        body: 'Audit should show place match, schedule context, last known timestamp, capability state, override, and final alert or observe result.',
        steps: [
          'Show known, safe, unknown, or unexpected place status.',
          'Show freshness and permission state.',
          'Show which schedule window gave the location meaning.',
          'Show notification delivery and parent response state.',
        ],
      },
    ],
    tips: [
      {
        label: 'Time plus place',
        body: 'Location only becomes useful when paired with expected time context.',
        tone: 'cyan',
      },
    ],
    actions: [
      {
        label: 'Tracking policy',
        body: 'Return to the tracking policy workspace.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.PolicyTracking),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Tracking,
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
        targetRoutePath: portalRouteHashPath(PortalRoute.Policy),
        targetNavLabel: 'RULES',
      },
      {
        label: 'Review audit',
        body: 'Inspect enforcement results, failures, expiry, and rollback state in policy history.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.AuditHistory),
        targetNavLabel: 'AUDIT',
      },
    ],
  },
] as const;
