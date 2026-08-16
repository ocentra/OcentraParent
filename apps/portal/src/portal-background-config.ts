import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  DEFAULT_PORTAL_BACKGROUND_CONFIG,
  DEFAULT_PORTAL_BACKGROUND_DARK_COLORS,
  DEFAULT_PORTAL_BACKGROUND_LIGHT_COLORS,
  PortalBackgroundRuntime,
  normalizePortalBackgroundConfig,
  portalBackgroundAppRenderConfig,
  portalBackgroundRenderConfig,
  type PortalBackgroundConfig,
  type PortalBackgroundRenderConfig,
  type PortalBackgroundThemeColors,
} from '@ocentra-parent/portal-domain/portal-background';
import savedPortalBackgroundConfig from '../public/portal-background-config.json';

export {
  DEFAULT_PORTAL_BACKGROUND_CONFIG,
  DEFAULT_PORTAL_BACKGROUND_DARK_COLORS,
  DEFAULT_PORTAL_BACKGROUND_LIGHT_COLORS,
  PortalBackgroundRuntime,
  normalizePortalBackgroundConfig,
  portalBackgroundAppRenderConfig,
  portalBackgroundRenderConfig,
  type PortalBackgroundConfig,
  type PortalBackgroundRenderConfig,
  type PortalBackgroundThemeColors,
};

export const SAVED_PORTAL_BACKGROUND_CONFIG: PortalBackgroundConfig =
  normalizePortalBackgroundConfig(savedPortalBackgroundConfig);

export async function loadPortalBackgroundConfig(): Promise<PortalBackgroundConfig> {
  const endpointConfig = await getPortalBackgroundConfig(PortalBackgroundRuntime.Api.ConfigEndpoint);
  if (endpointConfig !== undefined) {
    return endpointConfig;
  }
  const staticConfig = await getPortalBackgroundConfig(PortalBackgroundRuntime.Api.StaticConfigAsset);
  if (staticConfig !== undefined) {
    return staticConfig;
  }
  return SAVED_PORTAL_BACKGROUND_CONFIG;
}

export async function savePortalBackgroundConfig(
  config: PortalBackgroundConfig
): Promise<PortalBackgroundConfig | undefined> {
  const normalized = normalizePortalBackgroundConfig(config);
  if (typeof window === PortalDom.Runtime.Undefined) {
    return undefined;
  }
  const response = await fetch(PortalBackgroundRuntime.Api.ConfigEndpoint, {
    body: JSON.stringify(normalized, null, 2),
    headers: {
      [PortalBackgroundRuntime.HttpHeader.ContentType]: PortalBackgroundRuntime.ContentType.Json,
    },
    method: PortalBackgroundRuntime.HttpMethod.Put,
  }).catch(() => undefined);
  if (response?.ok !== true) {
    return undefined;
  }
  notifyPortalBackgroundConfigChanged(normalized);
  return normalized;
}

async function getPortalBackgroundConfig(
  url: (typeof PortalBackgroundRuntime.Api)[keyof typeof PortalBackgroundRuntime.Api]
): Promise<PortalBackgroundConfig | undefined> {
  if (typeof window === PortalDom.Runtime.Undefined) {
    return undefined;
  }
  const response = await fetch(url, { cache: PortalBackgroundRuntime.FetchCache.NoStore }).catch(() => undefined);
  if (response?.ok !== true) {
    return undefined;
  }
  const contentType =
    response.headers.get(PortalBackgroundRuntime.HttpHeader.ContentType) ?? PortalBackgroundRuntime.ContentType.Missing;
  if (!contentType.toLowerCase().includes(PortalBackgroundRuntime.ContentType.Json)) {
    return undefined;
  }
  return normalizePortalBackgroundConfig(await response.json());
}

export function readDefaultPortalBackgroundConfig(): PortalBackgroundConfig {
  return SAVED_PORTAL_BACKGROUND_CONFIG;
}

export function subscribePortalBackgroundConfig(onConfigChange: (config: PortalBackgroundConfig) => void): () => void {
  if (typeof window === PortalDom.Runtime.Undefined) {
    return () => undefined;
  }
  const onLocalEvent = (event: Event): void => {
    const detail = (event as CustomEvent<PortalBackgroundConfig>).detail;
    onConfigChange(normalizePortalBackgroundConfig(detail));
  };
  const channel =
    typeof BroadcastChannel === PortalDom.Runtime.Undefined
      ? undefined
      : new BroadcastChannel(PortalBackgroundRuntime.Channel);
  const onChannelMessage = (): void => {
    void loadPortalBackgroundConfig().then(onConfigChange);
  };
  channel?.addEventListener(PortalDom.Events.Message, onChannelMessage);
  window.addEventListener(PortalBackgroundRuntime.UpdateEvent, onLocalEvent);
  return () => {
    channel?.removeEventListener(PortalDom.Events.Message, onChannelMessage);
    channel?.close();
    window.removeEventListener(PortalBackgroundRuntime.UpdateEvent, onLocalEvent);
  };
}

function notifyPortalBackgroundConfigChanged(config: PortalBackgroundConfig): void {
  window.dispatchEvent(new CustomEvent(PortalBackgroundRuntime.UpdateEvent, { detail: config }));
  if (typeof BroadcastChannel === PortalDom.Runtime.Undefined) {
    return;
  }
  const channel = new BroadcastChannel(PortalBackgroundRuntime.Channel);
  channel.postMessage(PortalBackgroundRuntime.UpdateEvent);
  channel.close();
}
