import { DEFAULT_PORTAL_BACKGROUND_CONFIG, portalBackgroundAppRenderConfig } from './portal-background';
import { portalBackgroundSvgMarkup } from './portal-background-svg-markup';
import { BrowserChildInterventionAssets } from './browser-child-intervention-assets';
import { PortalUnifiedChrome } from './unified-chrome';
import type {
  BrowserChildInterventionPageBackdrop,
  BrowserChildInterventionPageContent,
  BrowserChildInterventionPageModel,
  BrowserChildInterventionPageTheme,
} from './browser-child-intervention-page-impl';

export const BrowserChildInterventionPageStateSvg = `<svg viewBox="0 0 220 180" focusable="false" aria-hidden="true">
  <path class="ocentra-child-screen" d="M36 32h148a15 15 0 0 1 15 15v86a15 15 0 0 1-15 15H36a15 15 0 0 1-15-15V47a15 15 0 0 1 15-15Z"/>
  <path class="ocentra-child-screen-line" d="M46 62h60M46 84h93M46 106h48"/>
  <path class="ocentra-child-shield" d="M146 67 174 78v21c0 20-12 35-28 43-16-8-28-23-28-43V78l28-11Z"/>
  <path class="ocentra-child-check" d="M133 103l9 9 19-24"/>
  <path class="ocentra-child-base" d="M86 148h48l8 18H78l8-18Z"/>
</svg>`;

export const BrowserChildInterventionPageAskSvg = `<svg viewBox="0 0 20 20" focusable="false" aria-hidden="true">
  <path d="M3 10.5 17 3l-4.2 14-3.1-5.1L3 10.5Z" fill="none" stroke="currentColor" stroke-linejoin="round" stroke-width="1.8"/>
</svg>`;

export function browserChildInterventionPageRenderRequestForm(
  content: BrowserChildInterventionPageContent,
  payload: Record<string, string>
): string {
  return `<form class="ocentra-child-request" data-ocentra-request-form>
    <label for="ocentra-child-note">Message to parent</label>
    <textarea id="ocentra-child-note" name="reason" rows="3" maxlength="240" placeholder="${browserChildInterventionPageEscapeAttribute(
      content.requestPlaceholder
    )}" data-ocentra-request-note></textarea>
    <button class="ocentra-child-primary" type="submit" data-ocentra-request-button>
      ${BrowserChildInterventionPageAskSvg}
      <span>${browserChildInterventionPageEscapeHtml(content.requestButton)}</span>
    </button>
    <output class="ocentra-child-request-status" data-ocentra-request-status aria-live="polite"></output>
    <script type="application/json" data-ocentra-request-payload>${browserChildInterventionPageEscapeJsonForHtml(
      payload
    )}</script>
  </form>`;
}

export function browserChildInterventionPageRenderWaitingPanel(content: BrowserChildInterventionPageContent): string {
  return `<div class="ocentra-child-waiting" aria-live="polite">
    <span class="ocentra-child-waiting-dot" aria-hidden="true"></span>
    <span>${browserChildInterventionPageEscapeHtml(content.nextStep)}</span>
  </div>`;
}

export function browserChildInterventionPageRenderBackground(
  backdrop: BrowserChildInterventionPageBackdrop | undefined
): string {
  const common = {
    ariaHidden: true,
    ariaLabel: 'Portal background',
    preserveAspectRatio: 'xMidYMid slice',
    style: 'display:block;height:100%;inset:0;position:absolute;width:100%;pointer-events:none',
  } as const;
  const siteBackdrop = browserChildInterventionPageSiteBackdropMarkup(backdrop);
  const backgroundVariant = browserChildInterventionPageBackgroundVariant(backdrop);
  return `${siteBackdrop}<div class="ocentra-child-background" aria-hidden="true">
    ${portalBackgroundSvgMarkup({
      ...common,
      className: `ocentra-child-background-svg ${backgroundVariant.className}`,
      idPrefix: backgroundVariant.idPrefix,
      renderConfig: backgroundVariant.renderConfig,
    })}
  </div>`;
}

function browserChildInterventionPageSiteBackdropMarkup(
  backdrop: BrowserChildInterventionPageBackdrop | undefined
): string {
  if (backdrop === undefined) {
    return '';
  }

  return `<div class="ocentra-child-site-backdrop" aria-hidden="true">
    <img alt="" src="${browserChildInterventionPageEscapeAttribute(backdrop.imageUrl)}" />
  </div>`;
}

function browserChildInterventionPageBackgroundVariant(backdrop: BrowserChildInterventionPageBackdrop | undefined): {
  readonly className: 'ocentra-child-background-svg-clean' | 'ocentra-child-background-svg-site';
  readonly idPrefix: 'ocentraChildBackgroundClean' | 'ocentraChildBackgroundSiteBackdrop';
  readonly renderConfig: ReturnType<typeof browserChildInterventionPageCleanRenderConfig>;
} {
  if (backdrop === undefined) {
    return {
      className: 'ocentra-child-background-svg-clean',
      idPrefix: 'ocentraChildBackgroundClean',
      renderConfig: browserChildInterventionPageCleanRenderConfig(),
    };
  }

  return {
    className: 'ocentra-child-background-svg-site',
    idPrefix: 'ocentraChildBackgroundSiteBackdrop',
    renderConfig: browserChildInterventionPageSiteBackdropRenderConfig(),
  };
}

export function browserChildInterventionPageThemeAttribute(
  theme: BrowserChildInterventionPageTheme | undefined
): string {
  return theme === undefined || theme === 'auto'
    ? ''
    : ` data-ocentra-theme="${browserChildInterventionPageEscapeAttribute(theme)}" data-theme="${browserChildInterventionPageEscapeAttribute(theme)}"`;
}

export function browserChildInterventionPageRenderBody(
  model: BrowserChildInterventionPageModel,
  content: BrowserChildInterventionPageContent,
  requestEnabled: boolean,
  requestPayload: Record<string, string>
): string {
  return `<main class="ocentra-child-page" aria-labelledby="ocentra-child-title">
      <section class="ocentra-child-panel">
        ${browserChildInterventionPageRenderHeader(content)}
        <div class="ocentra-child-layout">
          <section class="ocentra-child-copy">
            <div class="ocentra-child-rule-mark">
              <div class="ocentra-child-illustration" aria-hidden="true">${BrowserChildInterventionPageStateSvg}</div>
              <p class="ocentra-child-status">${browserChildInterventionPageEscapeHtml(content.statusLine)}</p>
              <span class="ocentra-child-rule-pill">${browserChildInterventionPageEscapeHtml(content.badge)}</span>
            </div>
            <div class="ocentra-child-copy-main">
              <h1 id="ocentra-child-title">${browserChildInterventionPageEscapeHtml(content.title)}</h1>
              <p class="ocentra-child-summary">${browserChildInterventionPageEscapeHtml(content.summary)}</p>
              <div class="ocentra-child-actions">
                ${browserChildInterventionPageRenderRequestPanel(requestEnabled, content, requestPayload)}
              </div>
            </div>
          </section>
          <aside class="ocentra-child-reason" aria-label="Intervention details">
            <h2>Why this appeared</h2>
            <dl>
              ${browserChildInterventionPageDetailRow('Rule', model.ruleLabel)}
              ${browserChildInterventionPageDetailRow('Reason', model.reason)}
              ${browserChildInterventionPageDetailRow(
                'Target',
                browserChildInterventionPageReadableTargetType(model.targetType)
              )}
              ${browserChildInterventionPageDetailRow(
                'Delivery',
                browserChildInterventionPageReadableToken(model.deliveryState)
              )}
              ${browserChildInterventionPageDetailRow('Request', model.requestedUrl)}
            </dl>
            <p class="ocentra-child-next-step">${browserChildInterventionPageEscapeHtml(content.nextStep)}</p>
          </aside>
        </div>
      </section>
      <p hidden>${browserChildInterventionPageEscapeHtml(model.blockMarker)}</p>
      <p hidden>${browserChildInterventionPageEscapeHtml(model.ruleMarker)}</p>
      <p hidden>rule:${browserChildInterventionPageEscapeHtml(model.ruleId)}</p>
      <p hidden>bridge:${browserChildInterventionPageEscapeHtml(model.bridge)}</p>
    </main>`;
}

function browserChildInterventionPageRenderRequestPanel(
  requestEnabled: boolean,
  content: BrowserChildInterventionPageContent,
  requestPayload: Record<string, string>
): string {
  if (requestEnabled) {
    return browserChildInterventionPageRenderRequestForm(content, requestPayload);
  }

  return browserChildInterventionPageRenderWaitingPanel(content);
}

function browserChildInterventionPageCleanRenderConfig() {
  const renderConfig = portalBackgroundAppRenderConfig(DEFAULT_PORTAL_BACKGROUND_CONFIG, 'dark');
  return {
    ...renderConfig,
    lightStrength: 0,
  };
}

function browserChildInterventionPageSiteBackdropRenderConfig() {
  const renderConfig = browserChildInterventionPageCleanRenderConfig();
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

export function browserChildInterventionPageRenderHeader(content: BrowserChildInterventionPageContent): string {
  const backLabel = 'Back';
  return `<header data-oc-shell-header-extension="true" class="${PortalUnifiedChrome.Classes.OutlineHeader} ocentra-child-outline-header">
    <button aria-label="${browserChildInterventionPageEscapeAttribute(backLabel)}" class="${PortalUnifiedChrome.Classes.OutlineHeaderAction} ocentra-child-outline-header-back" type="button" data-ocentra-back-button>
      ${browserChildInterventionPageRenderHeaderActionContent(BrowserChildInterventionAssets.BackIconDataUrl, backLabel)}
    </button>
    ${browserChildInterventionPageRenderHeaderConnector()}
    <div class="${PortalUnifiedChrome.Classes.OutlineHeaderBrand}" aria-label="Ocentra Parent">
      <span class="${PortalUnifiedChrome.Classes.OutlineHeaderBrandPart}">Ocentra</span>
      <span aria-hidden="true" class="${PortalUnifiedChrome.Classes.OutlineHeaderBrandLogoMount}">
        <img alt="" class="${PortalUnifiedChrome.Classes.OutlineHeaderBrandLogo}" src="${browserChildInterventionPageEscapeAttribute(
          BrowserChildInterventionAssets.HeaderLogoDataUrl
        )}" />
      </span>
      <span class="${PortalUnifiedChrome.Classes.OutlineHeaderBrandPartMuted}">Parent</span>
    </div>
    ${browserChildInterventionPageRenderHeaderConnector()}
    <span aria-label="${browserChildInterventionPageEscapeAttribute(content.badge)}" class="${PortalUnifiedChrome.Classes.OutlineHeaderAction} ocentra-child-outline-header-status" role="status">
      ${browserChildInterventionPageRenderHeaderActionContent(BrowserChildInterventionAssets.BlockedIconDataUrl, content.badge)}
    </span>
  </header>`;
}

function browserChildInterventionPageRenderHeaderActionContent(iconDataUrl: string, label: string): string {
  return `<span class="${PortalUnifiedChrome.Classes.OutlineHeaderActionContent}">
    <span aria-hidden="true" class="${PortalUnifiedChrome.Classes.OutlineHeaderActionIcon}">
      <img alt="" class="${PortalUnifiedChrome.Classes.OutlineHeaderActionIconImage}" src="${browserChildInterventionPageEscapeAttribute(
        iconDataUrl
      )}" />
    </span>
    <span class="${PortalUnifiedChrome.Classes.OutlineHeaderActionLabel}">${browserChildInterventionPageEscapeHtml(label)}</span>
  </span>`;
}

function browserChildInterventionPageRenderHeaderConnector(): string {
  return `<span aria-hidden="true" class="${PortalUnifiedChrome.Classes.OutlineHeaderConnector}"></span>`;
}

export function browserChildInterventionPageDetailRow(label: string, value: string): string {
  return `<div class="ocentra-child-detail-row"><dt>${browserChildInterventionPageEscapeHtml(label)}</dt><dd>${browserChildInterventionPageEscapeHtml(value)}</dd></div>`;
}

export function browserChildInterventionPageReadableTargetType(value: string): string {
  return browserChildInterventionPageReadableToken(value);
}

export function browserChildInterventionPageReadableToken(value: string): string {
  return value.replaceAll('-', ' ');
}

export function browserChildInterventionPageEscapeHtml(value: string): string {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

export function browserChildInterventionPageEscapeAttribute(value: string): string {
  return browserChildInterventionPageEscapeHtml(value);
}

export function browserChildInterventionPageEscapeJsonForHtml(value: unknown): string {
  return JSON.stringify(value)
    .replaceAll('<', '\\u003c')
    .replaceAll('>', '\\u003e')
    .replaceAll('&', '\\u0026')
    .replaceAll("'", '\\u0027');
}
