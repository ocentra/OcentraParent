import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_NAV_LABELS } from './parent-portal-nav';

export const PARENT_PORTAL_RULES_POLICY_GUIDE: ParentPortalGuideTopic = {
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
    {
      eyebrow: 'CONFLICTS',
      title: 'Conflicts should be explained, not hidden',
      body: 'When a rule conflicts with another rule, a schedule, or evidence confidence, the UI should explain the conflict instead of silently choosing a path. A parent should understand why one rule wins before changing it.',
      steps: [
        'Show the policy source that won.',
        'Show why the losing option was rejected.',
        'Use evidence and schedule timing to explain contradictions.',
        'Keep preview results visible after save.',
      ],
    },
  ],
  tips: [
    {
      label: 'Simple first',
      body: 'Start with Family defaults and only add per-device exceptions when a child really needs them.',
      tone: 'cyan',
    },
    {
      label: 'Explain the winner',
      body: 'If a rule is blocked by a stronger one, show the stronger rule and the reason.',
      tone: 'gold',
    },
  ],
  actions: [
    {
      label: 'Open Rules',
      body: 'Review policy rules and default household choices.',
      tone: 'gold',
      targetRoutePath: portalRouteHashPath(PortalRoute.Policy),
      targetNavLabel: 'RULES',
    },
    {
      label: 'Open Approvals',
      body: 'Inspect pending approvals, expired requests, and previous overrides.',
      tone: 'cyan',
      targetRoutePath: portalRouteHashPath(PortalRoute.Approvals),
      targetNavLabel: PARENT_PORTAL_NAV_LABELS.Approvals,
    },
    {
      label: 'Open Audit',
      body: 'Review the audit trail for rule changes and policy decisions.',
      tone: 'gold',
      targetRoutePath: portalRouteHashPath(PortalRoute.AuditHistory),
      targetNavLabel: 'AUDIT',
    },
  ],
};
