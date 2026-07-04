import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';

export const PARENT_PORTAL_ENFORCEMENT_CONTROL_GUIDE: ParentPortalGuideTopic = {
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
};
