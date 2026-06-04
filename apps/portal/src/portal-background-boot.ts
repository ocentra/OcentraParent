import { PortalDom, PortalTheme, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import {
  loadPortalBackgroundConfig,
  portalBackgroundRenderConfig,
  readDefaultPortalBackgroundConfig,
} from './portal-background-config';
import { portalBackgroundSvgMarkup } from './portal-background-svg-markup';

export const PortalBackgroundBoot = {
  ReadyAttribute: 'data-portal-bg-boot-ready',
  ThemeAttribute: 'data-theme',
  Id: 'portal-background-boot',
  Style: 'display:block;height:100%;inset:0;position:absolute;width:100%;pointer-events:none',
} as const;

export function mountPortalBackgroundBootLayer(): void {
  if (typeof document === 'undefined') {
    return;
  }
  const host = ensurePortalBackgroundBootHost();
  renderPortalBackgroundBootHost(host, readDefaultPortalBackgroundConfig());
  void loadPortalBackgroundConfig().then((config) => {
    renderPortalBackgroundBootHost(host, config);
  });
}

export function removePortalBackgroundBootLayer(): void {
  document.getElementById(PortalBackgroundBoot.Id)?.remove();
  document.documentElement.removeAttribute(PortalBackgroundBoot.ReadyAttribute);
}

export function fadePortalBackgroundBootLayer(): void {
  document.getElementById(PortalBackgroundBoot.Id)?.classList.add(PortalDom.Classes.AppLoadingHide);
}

function ensurePortalBackgroundBootHost(): HTMLDivElement {
  const existing = document.getElementById(PortalBackgroundBoot.Id);
  if (existing instanceof HTMLDivElement) {
    return existing;
  }
  const host = document.createElement('div');
  host.id = PortalBackgroundBoot.Id;
  document.body.prepend(host);
  return host;
}

function renderPortalBackgroundBootHost(
  host: HTMLDivElement,
  config: Parameters<typeof portalBackgroundRenderConfig>[0]
): void {
  host.innerHTML = portalBackgroundSvgMarkup({
    ariaHidden: true,
    ariaLabel: 'Portal background',
    idPrefix: 'portalBackgroundBoot',
    preserveAspectRatio: 'xMidYMid slice',
    renderConfig: portalBackgroundRenderConfig(config, currentPortalBackgroundTheme()),
    style: PortalBackgroundBoot.Style,
  });
  document.documentElement.setAttribute(PortalBackgroundBoot.ReadyAttribute, 'true');
}

function currentPortalBackgroundTheme(): PortalThemeValue {
  return document.documentElement.getAttribute(PortalBackgroundBoot.ThemeAttribute) === PortalTheme.Light
    ? PortalTheme.Light
    : PortalTheme.Dark;
}
