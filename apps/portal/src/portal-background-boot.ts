import { PortalDom, PortalTheme, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import { PortalBackgroundRuntime } from '@ocentra-parent/portal-domain/portal-background';
import {
  loadPortalBackgroundConfig,
  portalBackgroundRenderConfig,
  readDefaultPortalBackgroundConfig,
} from './portal-background-config';
import { portalBackgroundSvgMarkup } from './portal-background-svg-markup';

export function mountPortalBackgroundBootLayer(): void {
  if (typeof document === PortalDom.Runtime.Undefined) {
    return;
  }
  const host = ensurePortalBackgroundBootHost();
  renderPortalBackgroundBootHost(host, readDefaultPortalBackgroundConfig());
  void loadPortalBackgroundConfig().then((config) => {
    renderPortalBackgroundBootHost(host, config);
  });
}

export function removePortalBackgroundBootLayer(): void {
  document.getElementById(PortalBackgroundRuntime.Boot.Id)?.remove();
  document.documentElement.removeAttribute(PortalBackgroundRuntime.Boot.ReadyAttribute);
}

export function fadePortalBackgroundBootLayer(): void {
  document.getElementById(PortalBackgroundRuntime.Boot.Id)?.classList.add(PortalDom.Classes.AppLoadingHide);
}

function ensurePortalBackgroundBootHost(): HTMLDivElement {
  const existing = document.getElementById(PortalBackgroundRuntime.Boot.Id);
  if (existing instanceof HTMLDivElement) {
    return existing;
  }
  const host = document.createElement(PortalDom.Tags.Division);
  host.id = PortalBackgroundRuntime.Boot.Id;
  document.body.prepend(host);
  return host;
}

function renderPortalBackgroundBootHost(
  host: HTMLDivElement,
  config: Parameters<typeof portalBackgroundRenderConfig>[0]
): void {
  host.innerHTML = portalBackgroundSvgMarkup({
    ariaHidden: true,
    ariaLabel: PortalBackgroundRuntime.Boot.AriaLabel,
    idPrefix: PortalBackgroundRuntime.Boot.IdPrefix,
    preserveAspectRatio: PortalBackgroundRuntime.Boot.PreserveAspectRatio,
    renderConfig: portalBackgroundRenderConfig(config, currentPortalBackgroundTheme()),
    style: PortalBackgroundRuntime.Boot.Style,
  });
  document.documentElement.setAttribute(PortalBackgroundRuntime.Boot.ReadyAttribute, PortalDom.Attributes.True);
}

function currentPortalBackgroundTheme(): PortalThemeValue {
  return document.documentElement.getAttribute(PortalDom.Attributes.DataTheme) === PortalTheme.Light
    ? PortalTheme.Light
    : PortalTheme.Dark;
}
