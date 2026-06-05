import { DEFAULT_PORTAL_BACKGROUND_CONFIG, portalBackgroundAppRenderConfig } from './portal-background';
import { portalBackgroundSvgMarkup } from './portal-background-svg-markup';
import { BrowserChildInterventionAssets } from './browser-child-intervention-assets';
import { BrowserChildInterventionPageStyle } from './browser-child-intervention-style';
import { PortalUnifiedChrome } from './unified-chrome';

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

type BrowserChildInterventionPageContent = {
  readonly accent: 'approval' | 'blocked' | 'checking' | 'limited' | 'manual' | 'warning';
  readonly badge: string;
  readonly nextStep: string;
  readonly requestButton: string;
  readonly requestPlaceholder: string;
  readonly statusLine: string;
  readonly summary: string;
  readonly title: string;
};

export const BrowserChildInterventionPageDefaults = {
  BlockMarker: 'OCENTRA_MANAGED_BROWSER_BLOCKED',
  DefaultBridge: 'browser-child-intervention-template',
  ProductName: 'Ocentra',
  RequestEventName: 'ocentra-child-approval-request',
  RequestStatusReady: 'Request ready for parent review.',
  ThemeAuto: 'auto',
} as const;

export const BrowserChildInterventionPageSamples: readonly BrowserChildInterventionPageModel[] = [
  {
    action: 'block',
    blockMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    bridge: BrowserChildInterventionPageDefaults.DefaultBridge,
    deliveryState: 'block-page-rendered',
    outcome: 'blocked',
    parentRequestEnabled: true,
    reason: 'Your family rule blocks this exact video URL.',
    requestedUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
    ruleId: 'blocked-youtube-video-url',
    ruleLabel: 'Disallowed YouTube video URL',
    ruleMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    targetType: 'video',
  },
  {
    action: 'approval-hold',
    blockMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    bridge: BrowserChildInterventionPageDefaults.DefaultBridge,
    deliveryState: 'approval-hold-rendered',
    outcome: 'approval-required',
    parentRequestEnabled: true,
    reason: 'Creating a social account needs parent approval first.',
    requestedUrl: 'https://www.instagram.com/accounts/emailsignup/',
    ruleId: 'social-signup-approval-hold',
    ruleLabel: 'Social account creation approval hold',
    ruleMarker: 'OCENTRA_MANAGED_BROWSER_APPROVAL_HOLD',
    targetType: 'social-account-creation',
  },
  {
    action: 'warn',
    blockMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    bridge: BrowserChildInterventionPageDefaults.DefaultBridge,
    deliveryState: 'warn-page-rendered',
    outcome: 'warned',
    parentRequestEnabled: true,
    reason: 'Short-video feeds are set to warning mode for this device.',
    requestedUrl: 'https://www.tiktok.com/@ocentra/video/1',
    ruleId: 'social-short-video-warning',
    ruleLabel: 'Social short-video route warning',
    ruleMarker: 'OCENTRA_MANAGED_BROWSER_WARNED',
    targetType: 'social-short-video-feed',
  },
  {
    action: 'checking-hold',
    blockMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    bridge: BrowserChildInterventionPageDefaults.DefaultBridge,
    deliveryState: 'checking-hold-rendered',
    outcome: 'held',
    parentRequestEnabled: false,
    reason: 'Ocentra is checking the game page against the latest family rules.',
    requestedUrl: 'https://poki.com/en/g/example-game',
    ruleId: 'browser-game-checking-hold',
    ruleLabel: 'Browser game checking hold',
    ruleMarker: 'OCENTRA_MANAGED_BROWSER_CHECKING_HOLD',
    targetType: 'browser-game',
  },
  {
    action: 'time-limit',
    blockMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    bridge: BrowserChildInterventionPageDefaults.DefaultBridge,
    deliveryState: 'block-page-rendered',
    outcome: 'blocked',
    parentRequestEnabled: true,
    reason: 'The daily browser-game quota for this device has been used.',
    requestedUrl: 'https://www.xbox.com/en-US/play',
    ruleId: 'cloud-gaming-quota-reached',
    ruleLabel: 'Cloud gaming quota reached',
    ruleMarker: 'OCENTRA_MANAGED_BROWSER_TIME_LIMIT',
    targetType: 'cloud-gaming',
  },
  {
    action: 'parent-review',
    blockMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    bridge: BrowserChildInterventionPageDefaults.DefaultBridge,
    deliveryState: 'manual-required',
    outcome: 'manual-required',
    parentRequestEnabled: true,
    reason: 'This browser needs parent setup before Ocentra can unblock it automatically.',
    requestedUrl: 'https://example.com/manual-review',
    ruleId: 'manual-parent-review',
    ruleLabel: 'Manual parent review',
    ruleMarker: 'OCENTRA_MANAGED_BROWSER_MANUAL_REVIEW',
    targetType: 'browser-session',
  },
] as const;

export function renderBrowserChildInterventionPage(model: BrowserChildInterventionPageModel): string {
  const content = contentForModel(model);
  const requestEnabled = model.parentRequestEnabled;
  const pageTheme = model.theme ?? BrowserChildInterventionPageDefaults.ThemeAuto;
  const hasSiteBackdrop = model.backdrop !== undefined;
  const themeAttribute =
    pageTheme === BrowserChildInterventionPageDefaults.ThemeAuto
      ? ''
      : ` data-ocentra-theme="${escapeAttribute(pageTheme)}" data-theme="${escapeAttribute(pageTheme)}"`;
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
<html lang="en"${themeAttribute}>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="color-scheme" content="dark light" />
    <title>${escapeHtml(content.title)}</title>
    <style>${BrowserChildInterventionPageStyle}</style>
  </head>
  <body data-ocentra-intervention-state="${escapeAttribute(content.accent)}" data-ocentra-site-backdrop="${String(
    hasSiteBackdrop
  )}">
    ${renderChildPageBackground(model.backdrop)}
    <main class="ocentra-child-page" aria-labelledby="ocentra-child-title">
      <section class="ocentra-child-panel">
        ${renderPortalOutlineHeader(content)}
        <div class="ocentra-child-layout">
          <section class="ocentra-child-copy">
            <div class="ocentra-child-rule-mark">
              <div class="ocentra-child-illustration" aria-hidden="true">${BrowserChildInterventionPageStateSvg}</div>
              <p class="ocentra-child-status">${escapeHtml(content.statusLine)}</p>
              <span class="ocentra-child-rule-pill">${escapeHtml(content.badge)}</span>
            </div>
            <div class="ocentra-child-copy-main">
              <h1 id="ocentra-child-title">${escapeHtml(content.title)}</h1>
              <p class="ocentra-child-summary">${escapeHtml(content.summary)}</p>
              <div class="ocentra-child-actions">
                ${requestEnabled ? requestForm(content, requestPayload) : waitingPanel(content)}
              </div>
            </div>
          </section>
          <aside class="ocentra-child-reason" aria-label="Intervention details">
            <h2>Why this appeared</h2>
            <dl>
              ${detailRow('Rule', model.ruleLabel)}
              ${detailRow('Reason', model.reason)}
              ${detailRow('Target', readableTargetType(model.targetType))}
              ${detailRow('Delivery', readableToken(model.deliveryState))}
              ${detailRow('Request', model.requestedUrl)}
            </dl>
            <p class="ocentra-child-next-step">${escapeHtml(content.nextStep)}</p>
          </aside>
        </div>
      </section>
      <p hidden>${escapeHtml(model.blockMarker)}</p>
      <p hidden>${escapeHtml(model.ruleMarker)}</p>
      <p hidden>rule:${escapeHtml(model.ruleId)}</p>
      <p hidden>bridge:${escapeHtml(model.bridge)}</p>
    </main>
    <script>${BrowserChildInterventionPageScript.replace('__PAYLOAD__', escapeJsonForHtml(requestPayload))}</script>
  </body>
</html>`;
}

function contentForModel(model: BrowserChildInterventionPageModel): BrowserChildInterventionPageContent {
  if (model.action === 'approval-hold') {
    return {
      accent: 'approval',
      badge: 'Parent approval',
      nextStep: 'Your parent can approve or deny the request from their Ocentra portal.',
      requestButton: 'Ask parent',
      requestPlaceholder: 'Tell your parent why you need this page.',
      statusLine: 'This needs a parent decision.',
      summary: 'The page is paused until your parent says it is okay to continue.',
      title: 'Ask your parent to continue',
    };
  }
  if (model.action === 'warn') {
    return {
      accent: 'warning',
      badge: 'Warning',
      nextStep: 'Choose another page or ask your parent to review this one.',
      requestButton: 'Ask parent',
      requestPlaceholder: 'Add a short note for your parent.',
      statusLine: 'This page may not match your family settings.',
      summary: 'Your family settings ask you to stop and make a different choice here.',
      title: 'This page needs a different choice',
    };
  }
  if (model.action === 'checking-hold') {
    return {
      accent: 'checking',
      badge: 'Checking',
      nextStep: 'This hold clears automatically when the check finishes.',
      requestButton: 'Request review',
      requestPlaceholder: 'Add a note for your parent.',
      statusLine: 'Ocentra is checking this page.',
      summary: 'The browser is waiting while Ocentra compares this page with your family rules.',
      title: 'Ocentra is checking this page',
    };
  }
  if (model.action === 'time-limit') {
    return {
      accent: 'limited',
      badge: 'Time limit',
      nextStep: 'Your parent can add more time from the Ocentra portal.',
      requestButton: 'Ask for more time',
      requestPlaceholder: 'Tell your parent why you need more time.',
      statusLine: 'Your time for this activity is used up.',
      summary: 'The daily quota for this kind of browsing has been reached on this device.',
      title: 'Your browser time is used up',
    };
  }
  if (model.action === 'parent-review') {
    return {
      accent: 'manual',
      badge: 'Parent review',
      nextStep: 'Your parent may need to finish setup before this browser can continue.',
      requestButton: 'Ask parent',
      requestPlaceholder: 'Add anything your parent should know.',
      statusLine: 'This needs parent review.',
      summary: 'Ocentra needs a parent decision before it can continue with this page.',
      title: 'Parent review is needed',
    };
  }
  return {
    accent: 'blocked',
    badge: 'Blocked',
    nextStep: 'You can go back, choose another site, or send a request to your parent.',
    requestButton: 'Ask parent',
    requestPlaceholder: 'Tell your parent why you want this unblocked.',
    statusLine: 'A family rule is active here.',
    summary: 'This page is blocked by your family settings on this device.',
    title: 'This page is blocked',
  };
}

function requestForm(content: BrowserChildInterventionPageContent, payload: Record<string, string>): string {
  return `<form class="ocentra-child-request" data-ocentra-request-form>
    <label for="ocentra-child-note">Message to parent</label>
    <textarea id="ocentra-child-note" name="reason" rows="3" maxlength="240" placeholder="${escapeAttribute(
      content.requestPlaceholder
    )}" data-ocentra-request-note></textarea>
    <button class="ocentra-child-primary" type="submit" data-ocentra-request-button>
      ${BrowserChildInterventionPageAskSvg}
      <span>${escapeHtml(content.requestButton)}</span>
    </button>
    <output class="ocentra-child-request-status" data-ocentra-request-status aria-live="polite"></output>
    <script type="application/json" data-ocentra-request-payload>${escapeJsonForHtml(payload)}</script>
  </form>`;
}

function waitingPanel(content: BrowserChildInterventionPageContent): string {
  return `<div class="ocentra-child-waiting" aria-live="polite">
    <span class="ocentra-child-waiting-dot" aria-hidden="true"></span>
    <span>${escapeHtml(content.nextStep)}</span>
  </div>`;
}

function renderChildPageBackground(backdrop: BrowserChildInterventionPageBackdrop | undefined): string {
  const common = {
    ariaHidden: true,
    ariaLabel: 'Portal background',
    preserveAspectRatio: 'xMidYMid slice',
    style: 'display:block;height:100%;inset:0;position:absolute;width:100%;pointer-events:none',
  } as const;
  const siteBackdrop =
    backdrop === undefined
      ? ''
      : `<div class="ocentra-child-site-backdrop" aria-hidden="true">
    <img alt="" src="${escapeAttribute(backdrop.imageUrl)}" />
  </div>`;
  if (backdrop !== undefined) {
    return `${siteBackdrop}<div class="ocentra-child-background" aria-hidden="true">
    ${portalBackgroundSvgMarkup({
      ...common,
      className: 'ocentra-child-background-svg ocentra-child-background-svg-site',
      idPrefix: 'ocentraChildBackgroundSiteBackdrop',
      renderConfig: portalBackgroundSiteBackdropRenderConfig(),
    })}
  </div>`;
  }
  return `${siteBackdrop}<div class="ocentra-child-background" aria-hidden="true">
    ${portalBackgroundSvgMarkup({
      ...common,
      className: 'ocentra-child-background-svg ocentra-child-background-svg-clean',
      idPrefix: 'ocentraChildBackgroundClean',
      renderConfig: portalBackgroundCleanRenderConfig(),
    })}
  </div>`;
}

function portalBackgroundCleanRenderConfig() {
  const renderConfig = portalBackgroundAppRenderConfig(DEFAULT_PORTAL_BACKGROUND_CONFIG, 'dark');
  return {
    ...renderConfig,
    lightStrength: 0,
  };
}

function portalBackgroundSiteBackdropRenderConfig() {
  const renderConfig = portalBackgroundCleanRenderConfig();
  return {
    ...renderConfig,
    colors: {
      ...renderConfig.colors,
      bgBaseEnd: 'transparent',
      bgBaseMid: 'transparent',
      bgBaseStart: 'transparent',
      vignetteCenter: 'transparent',
      vignetteEdge: 'transparent',
      vignetteMid: 'transparent',
    },
    lightStrength: 0,
  };
}

function renderPortalOutlineHeader(content: BrowserChildInterventionPageContent): string {
  const backLabel = 'Back';
  return `<header data-oc-shell-header-extension="true" class="${PortalUnifiedChrome.Classes.OutlineHeader} ocentra-child-outline-header">
    <button aria-label="${escapeAttribute(backLabel)}" class="${PortalUnifiedChrome.Classes.OutlineHeaderAction} ocentra-child-outline-header-back" type="button" data-ocentra-back-button>
      ${renderPortalHeaderActionContent(BrowserChildInterventionAssets.BackIconDataUrl, backLabel)}
    </button>
    ${renderPortalHeaderConnector()}
    <div class="${PortalUnifiedChrome.Classes.OutlineHeaderBrand}" aria-label="${BrowserChildInterventionPageDefaults.ProductName} Parent">
      <span class="${PortalUnifiedChrome.Classes.OutlineHeaderBrandPart}">Ocentra</span>
      <span aria-hidden="true" class="${PortalUnifiedChrome.Classes.OutlineHeaderBrandLogoMount}">
        <img alt="" class="${PortalUnifiedChrome.Classes.OutlineHeaderBrandLogo}" src="${escapeAttribute(
          BrowserChildInterventionAssets.HeaderLogoDataUrl
        )}" />
      </span>
      <span class="${PortalUnifiedChrome.Classes.OutlineHeaderBrandPartMuted}">Parent</span>
    </div>
    ${renderPortalHeaderConnector()}
    <span aria-label="${escapeAttribute(content.badge)}" class="${PortalUnifiedChrome.Classes.OutlineHeaderAction} ocentra-child-outline-header-status" role="status">
      ${renderPortalHeaderActionContent(BrowserChildInterventionAssets.BlockedIconDataUrl, content.badge)}
    </span>
  </header>`;
}

function renderPortalHeaderActionContent(iconDataUrl: string, label: string): string {
  return `<span class="${PortalUnifiedChrome.Classes.OutlineHeaderActionContent}">
    <span aria-hidden="true" class="${PortalUnifiedChrome.Classes.OutlineHeaderActionIcon}">
      <img alt="" class="${PortalUnifiedChrome.Classes.OutlineHeaderActionIconImage}" src="${escapeAttribute(
        iconDataUrl
      )}" />
    </span>
    <span class="${PortalUnifiedChrome.Classes.OutlineHeaderActionLabel}">${escapeHtml(label)}</span>
  </span>`;
}

function renderPortalHeaderConnector(): string {
  return `<span aria-hidden="true" class="${PortalUnifiedChrome.Classes.OutlineHeaderConnector}"></span>`;
}

function detailRow(label: string, value: string): string {
  return `<div class="ocentra-child-detail-row"><dt>${escapeHtml(label)}</dt><dd>${escapeHtml(value)}</dd></div>`;
}

function readableTargetType(value: string): string {
  return readableToken(value);
}

function readableToken(value: string): string {
  return value.replaceAll('-', ' ');
}

function escapeHtml(value: string): string {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

function escapeAttribute(value: string): string {
  return escapeHtml(value);
}

function escapeJsonForHtml(value: unknown): string {
  return JSON.stringify(value)
    .replaceAll('<', '\\u003c')
    .replaceAll('>', '\\u003e')
    .replaceAll('&', '\\u0026')
    .replaceAll("'", '\\u0027');
}

const BrowserChildInterventionPageStateSvg = `<svg viewBox="0 0 220 180" focusable="false" aria-hidden="true">
  <path class="ocentra-child-screen" d="M36 32h148a15 15 0 0 1 15 15v86a15 15 0 0 1-15 15H36a15 15 0 0 1-15-15V47a15 15 0 0 1 15-15Z"/>
  <path class="ocentra-child-screen-line" d="M46 62h60M46 84h93M46 106h48"/>
  <path class="ocentra-child-shield" d="M146 67 174 78v21c0 20-12 35-28 43-16-8-28-23-28-43V78l28-11Z"/>
  <path class="ocentra-child-check" d="M133 103l9 9 19-24"/>
  <path class="ocentra-child-base" d="M86 148h48l8 18H78l8-18Z"/>
</svg>`;

const BrowserChildInterventionPageAskSvg = `<svg viewBox="0 0 20 20" focusable="false" aria-hidden="true">
  <path d="M3 10.5 17 3l-4.2 14-3.1-5.1L3 10.5Z" fill="none" stroke="currentColor" stroke-linejoin="round" stroke-width="1.8"/>
</svg>`;

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
