import { BrowserChildInterventionPageStyle } from './browser-child-intervention-style';
import { BrowserChildInterventionPageSamples as BrowserChildInterventionPageSamplesImpl } from './browser-child-intervention-page-samples';
import {
  browserChildInterventionPageEscapeHtml,
  browserChildInterventionPageEscapeJsonForHtml,
  browserChildInterventionPageRenderBody,
  browserChildInterventionPageThemeAttribute,
} from './browser-child-intervention-page-support';

export type BrowserChildInterventionPageTheme = 'auto' | 'dark' | 'light';

export type BrowserChildInterventionPageAction =
  | 'approval-hold'
  | 'block'
  | 'checking-hold'
  | 'parent-review'
  | 'time-limit'
  | 'unknown'
  | 'warn';

export type BrowserChildInterventionPageBackdrop = {
  readonly imageUrl: string;
  readonly label?: string;
};

export type BrowserChildInterventionPageModel = {
  readonly action: BrowserChildInterventionPageAction;
  readonly backdrop?: BrowserChildInterventionPageBackdrop | undefined;
  readonly blockMarker: string;
  readonly bridge: string;
  readonly childName?: string;
  readonly deliveryState: string;
  readonly outcome: string;
  readonly parentRequestEnabled: boolean;
  readonly reason: string;
  readonly requestedUrl: string;
  readonly ruleId: string;
  readonly ruleLabel: string;
  readonly ruleMarker: string;
  readonly targetType: string;
  readonly theme?: BrowserChildInterventionPageTheme;
};

export type BrowserChildInterventionPageContent = {
  readonly accent: 'approval' | 'blocked' | 'checking' | 'limited' | 'manual' | 'warning';
  readonly badge: string;
  readonly nextStep: string;
  readonly requestButton: string;
  readonly requestPlaceholder: string;
  readonly statusLine: string;
  readonly summary: string;
  readonly title: string;
};

const BrowserChildInterventionPageContentByAction: Readonly<
  Record<
    BrowserChildInterventionPageAction,
    {
      readonly accent: BrowserChildInterventionPageContent['accent'];
      readonly badge: string;
      readonly nextStep: string;
      readonly requestButton: string;
      readonly requestPlaceholder: string;
      readonly statusLine: string;
      readonly summary: string;
      readonly title: string;
    }
  >
> = {
  'approval-hold': {
    accent: 'approval',
    badge: 'Parent approval',
    nextStep: 'Your parent can approve or deny the request from their Ocentra portal.',
    requestButton: 'Ask parent',
    requestPlaceholder: 'Tell your parent why you need this page.',
    statusLine: 'This needs a parent decision.',
    summary: 'The page is paused until your parent says it is okay to continue.',
    title: 'Ask your parent to continue',
  },
  block: {
    accent: 'blocked',
    badge: 'Blocked',
    nextStep: 'You can go back, choose another site, or send a request to your parent.',
    requestButton: 'Ask parent',
    requestPlaceholder: 'Tell your parent why you want this unblocked.',
    statusLine: 'A family rule is active here.',
    summary: 'This page is blocked by your family settings on this device.',
    title: 'This page is blocked',
  },
  'checking-hold': {
    accent: 'checking',
    badge: 'Checking',
    nextStep: 'This hold clears automatically when the check finishes.',
    requestButton: 'Request review',
    requestPlaceholder: 'Add a note for your parent.',
    statusLine: 'Ocentra is checking this page.',
    summary: 'The browser is waiting while Ocentra compares this page with your family rules.',
    title: 'Ocentra is checking this page',
  },
  'parent-review': {
    accent: 'manual',
    badge: 'Parent review',
    nextStep: 'Your parent may need to finish setup before this browser can continue.',
    requestButton: 'Ask parent',
    requestPlaceholder: 'Add anything your parent should know.',
    statusLine: 'This needs parent review.',
    summary: 'Ocentra needs a parent decision before it can continue with this page.',
    title: 'Parent review is needed',
  },
  'time-limit': {
    accent: 'limited',
    badge: 'Time limit',
    nextStep: 'Your parent can add more time from the Ocentra portal.',
    requestButton: 'Ask for more time',
    requestPlaceholder: 'Tell your parent why you need more time.',
    statusLine: 'Your time for this activity is used up.',
    summary: 'The daily quota for this kind of browsing has been reached on this device.',
    title: 'Your browser time is used up',
  },
  unknown: {
    accent: 'blocked',
    badge: 'Blocked',
    nextStep: 'You can go back, choose another site, or send a request to your parent.',
    requestButton: 'Ask parent',
    requestPlaceholder: 'Tell your parent why you want this unblocked.',
    statusLine: 'A family rule is active here.',
    summary: 'This page is blocked by your family settings on this device.',
    title: 'This page is blocked',
  },
  warn: {
    accent: 'warning',
    badge: 'Warning',
    nextStep: 'Choose another page or ask your parent to review this one.',
    requestButton: 'Ask parent',
    requestPlaceholder: 'Add a short note for your parent.',
    statusLine: 'This page may not match your family settings.',
    summary: 'Your family settings ask you to stop and make a different choice here.',
    title: 'This page needs a different choice',
  },
};

export const BrowserChildInterventionPageDefaults = {
  BlockMarker: 'OCENTRA_MANAGED_BROWSER_BLOCKED',
  DefaultBridge: 'browser-child-intervention-template',
  ProductName: 'Ocentra',
  RequestEventName: 'ocentra-child-approval-request',
  RequestStatusReady: 'Request ready for parent review.',
  ThemeAuto: 'auto',
} as const;

export const BrowserChildInterventionPageSamples = BrowserChildInterventionPageSamplesImpl;

export function renderBrowserChildInterventionPage(model: BrowserChildInterventionPageModel): string {
  const content = contentForModel(model);
  const requestEnabled = model.parentRequestEnabled;
  const requestPayload = {
    action: model.action,
    bridge: model.bridge,
    deliveryState: model.deliveryState,
    outcome: model.outcome,
    requestedUrl: model.requestedUrl,
    ruleId: model.ruleId,
    targetType: model.targetType,
  };
  return `<!doctype html>
<html lang="en"${browserChildInterventionPageThemeAttribute(model.theme)}>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="color-scheme" content="dark light" />
    <title>${browserChildInterventionPageEscapeHtml(content.title)}</title>
    <style>${BrowserChildInterventionPageStyle}</style>
  </head>
  <body data-ocentra-intervention-state="${browserChildInterventionPageEscapeHtml(content.accent)}" data-ocentra-site-backdrop="${String(
    model.backdrop !== undefined
  )}">
    ${browserChildInterventionPageRenderBody(model, content, requestEnabled, requestPayload)}
    <script>${BrowserChildInterventionPageScript.replace(
      '__PAYLOAD__',
      browserChildInterventionPageEscapeJsonForHtml(requestPayload)
    )}</script>
  </body>
</html>`;
}

function contentForModel(model: BrowserChildInterventionPageModel): BrowserChildInterventionPageContent {
  return BrowserChildInterventionPageContentByAction[model.action];
}

const BrowserChildInterventionPageScript = `(() => {
  const payload = __PAYLOAD__;
  const form = document.querySelector('[data-ocentra-request-form]');
  const note = document.querySelector('[data-ocentra-request-note]');
  const status = document.querySelector('[data-ocentra-request-status]');
  const back = document.querySelector('[data-ocentra-back-button]');
  form?.addEventListener('submit', (event) => {
    event.preventDefault();
    const detail = {
      ...payload,
      childReason: note instanceof HTMLTextAreaElement ? note.value.trim() : '',
      requestedAt: new Date().toISOString(),
    };
    window.dispatchEvent(new CustomEvent('${BrowserChildInterventionPageDefaults.RequestEventName}', { detail }));
    window.parent?.postMessage({ type: '${BrowserChildInterventionPageDefaults.RequestEventName}', detail }, '*');
    if (status !== null) {
      status.textContent = '${BrowserChildInterventionPageDefaults.RequestStatusReady}';
    }
  });
  back?.addEventListener('click', () => {
    if (history.length > 1) {
      history.back();
      return;
    }
    window.dispatchEvent(new CustomEvent('ocentra-child-back-request', { detail: payload }));
  });
})();`;
