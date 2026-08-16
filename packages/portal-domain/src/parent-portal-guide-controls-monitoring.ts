import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_NAV_LABELS } from './parent-portal-nav';

export const PARENT_PORTAL_MONITORING_EVIDENCE_GUIDE: ParentPortalGuideTopic = {
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
};
