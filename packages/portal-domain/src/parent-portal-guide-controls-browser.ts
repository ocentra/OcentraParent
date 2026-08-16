import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_NAV_LABELS } from './parent-portal-nav';

export const PARENT_PORTAL_BROWSER_CONTROL_GUIDE: ParentPortalGuideTopic = {
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
};
