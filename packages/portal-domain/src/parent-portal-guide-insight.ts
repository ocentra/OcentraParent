import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_NAV_LABELS } from './parent-portal-nav';

export const PARENT_PORTAL_INSIGHT_GUIDES: readonly ParentPortalGuideTopic[] = [
  {
    id: 'local-ai-evidence',
    navLabel: 'AI',
    rank: 7,
    title: 'AI And Evidence',
    subtitle: 'Local evaluator first, cited reports later',
    detail: 'Evidence-cited help',
    tone: 'cyan',
    category: 'Insight',
    subcategory: 'AI',
    pages: [
      {
        eyebrow: 'LOCAL AI',
        title: 'The child-device safety evaluator runs locally',
        body: 'Local AI helps classify page, video, app, domain, screen-summary, and recent activity context on the child device. It consumes typed evidence and parent rules, then returns typed output with confidence, reason, evidence refs, and degraded state.',
        steps: [
          'Local AI is not hidden household authority.',
          'Invalid or low-confidence output should be rejected or degraded to unknown, warn, ask-parent, or no-op.',
          'The evaluator cannot scan the OS, browser, network, or files directly.',
          'Policy turns local AI output into a decision only when a parent rule matches.',
        ],
      },
      {
        eyebrow: 'EVIDENCE ANSWERS',
        title: 'Parent explanations should come from evidence and reports',
        body: 'Parent-facing explanations should answer from stored evidence or parent-owned report bundles. They stay outside blocking decisions and should expose missing evidence, stale sources, unsupported devices, and uncertainty instead of pretending to know.',
        steps: [
          'Use local evidence, parent-owned reports, or explicitly authorized sources.',
          'Cite evidence refs, report windows, device source, and uncertainty.',
          'Never make remote/API AI required for blocking, timers, or ask-parent decisions.',
          'Show unavailable, local-only, remote-authorized, and missing-evidence states clearly.',
        ],
      },
      {
        eyebrow: 'EXTERNAL API',
        title: 'Remote AI is optional and parent-authorized',
        body: 'OpenAI or other API providers can help with richer explanations or report compilation only after the parent authorizes the source and custody boundary. Child activity should not leave the device or parent-owned storage by default.',
        steps: [
          'Show which provider is used, why, and what source data is allowed.',
          'Use no-retention or parent-owned-storage behavior where the feature requires it.',
          'Keep provider failures from disabling local child-device safety behavior.',
          'Never send raw screenshots, browser secrets, chat text, or unbounded activity by default.',
        ],
      },
    ],
    tips: [
      {
        label: 'Parent wording',
        body: 'AI explains and helps. Your rules decide what happens.',
        tone: 'cyan',
        targetPage: 0,
      },
      {
        label: 'API risk',
        body: 'External AI must be explicit, cited, and outside the normal child-device blocking path.',
        tone: 'gold',
        targetTopicId: 'api-providers',
        targetNavLabel: 'AI',
      },
    ],
    actions: [
      {
        label: 'Check AI status',
        body: 'Open Local AI for provider, model, cache, unavailable, and degraded states.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.AiRuntime),
        targetNavLabel: 'AI SETUP',
      },
      {
        label: 'Open reports',
        body: 'Use reports for evidence-cited parent explanations and summaries.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.Activity),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      },
    ],
  },
  {
    id: 'reports-summaries',
    navLabel: 'REPORTS',
    rank: 8,
    title: 'Reports And Summaries',
    subtitle: 'Daily, weekly, monthly child-device views',
    detail: 'Evidence into answers',
    tone: 'purple',
    category: 'Insight',
    subcategory: 'Reports',
    pages: [
      {
        eyebrow: 'REPORT TYPES',
        title: 'Reports should answer parent questions',
        body: 'Reports turn activity evidence into useful summaries: what happened today, what changed this week, which apps or sites dominated time, what alerts fired, what rules were previewed or enforced, and what needs parent attention.',
        steps: [
          'Daily report: current device health, top apps/sites, alerts, approvals, and notable changes.',
          'Weekly report: trends, repeated categories, time budgets, school versus entertainment, and rule outcomes.',
          'Monthly report: long-window summary, device changes, subscription/device limits, and export/archive status.',
          'Incident report: evidence before and after an alert, block, ask-parent request, or suspicious unknown.',
        ],
      },
      {
        eyebrow: 'SOURCE LABELS',
        title: 'Every report needs custody and evidence labels',
        body: 'A report should say whether it came from a live child agent, LAN, parent-device cache, parent-owned storage, or stateless compile request. It should cite evidence without copying raw child data into Ocentra-hosted storage by default.',
        steps: [
          'Show report window, child device, source, freshness, and custody.',
          'Use parent-owned storage or local cache for remote report access.',
          'Make missing evidence and platform gaps visible.',
          'Keep generated reports local or parent-owned unless a future explicit custody feature exists.',
        ],
      },
      {
        eyebrow: 'READING REPORTS',
        title: 'Use reports to tune rules without becoming technical',
        body: 'Reports should show quick takeaways for busy parents and deeper drill-down for technical parents. The parent should be able to jump from a report item to the rule, evidence, alert, device, or storage setting that explains it.',
        steps: [
          'Highlight what needs action first.',
          'Separate activity evidence from policy decisions and subscription/account status.',
          'Use plain words for unavailable, stale, unsupported, observe-only, and dry-run states.',
          'Link each recommendation to a parent-editable control.',
        ],
      },
    ],
    tips: [
      {
        label: 'Do not overwhelm',
        body: 'Start with top changes and action items, then let parents drill into evidence.',
        tone: 'cyan',
      },
      {
        label: 'Privacy',
        body: 'Reports are not stored by Ocentra by default. Use local cache or parent-owned storage.',
        tone: 'gold',
      },
    ],
    actions: [
      {
        label: 'Open Reports',
        body: 'Choose daily, weekly, monthly, or incident summary.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.Activity),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      },
      {
        label: 'Build report',
        body: 'Use the report compiler for stateless daily, weekly, monthly, or incident report generation.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.ReportCompiler),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      },
      {
        label: 'Connect storage',
        body: 'Use Drives if you want remote access, backup, or cross-device continuity.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.DriveConnections),
        targetNavLabel: 'DRIVES',
      },
    ],
  },
  {
    id: 'cited-memory',
    navLabel: 'MEMORY',
    rank: 9,
    title: 'Cited Memory',
    subtitle: 'Family knowledge with source references',
    detail: 'Local context',
    tone: 'purple',
    category: 'Insight',
    subcategory: 'Memory',
    pages: [
      {
        eyebrow: 'MEMORY',
        title: 'Memory helps explain but must cite evidence',
        body: 'Memory and graph references can help the system remember household context, recurring school sites, known games, trusted channels, past parent decisions, and family notes. They are derived indexes, not source truth.',
        steps: [
          'A memory item should say where it came from and when it was generated.',
          'Memory can support explanations, unknown handling, and ask-parent flows.',
          'Uncited memory cannot drive blocking or enforcement.',
          'Parents should review, revoke, export, or delete memory references.',
        ],
      },
      {
        eyebrow: 'FAMILY CONTEXT',
        title: 'Make repeated decisions easier without guessing',
        body: 'A parent may mark a domain as school-related for one child, allow a game only on weekends, or note that a channel is part of a class project. The UI should show source, child scope, and active rule before using that context.',
        steps: [
          'Scope memory to child, device, rule, schedule, source, or category.',
          'Show freshness and confidence.',
          'Link memory back to source evidence or parent action.',
          'Keep memory local or parent-owned by default.',
        ],
      },
    ],
    tips: [
      {
        label: 'Rule',
        body: 'Memory without a citation is a note, not evidence.',
        tone: 'gold',
      },
      {
        label: 'Control',
        body: 'Parents need edit, revoke, and export controls for memory.',
        tone: 'purple',
      },
    ],
    actions: [
      {
        label: 'Review memory',
        body: 'Open Memory to inspect source refs and freshness.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.Memory),
        targetNavLabel: 'MEMORY',
      },
      {
        label: 'Revoke context',
        body: 'Remove stale or wrong family context before it affects explanations.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.MemorySettings),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.MemorySet,
      },
    ],
  },
] as const;
