import { PortalTheme, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import savedPortalBackgroundConfig from '../public/portal-background-config.json';

export type PortalBackgroundThemeColors = {
  readonly bgBaseStart: string;
  readonly bgBaseMid: string;
  readonly bgBaseEnd: string;
  readonly vignetteCenter: string;
  readonly vignetteMid: string;
  readonly vignetteEdge: string;
  readonly hexStroke: string;
};

export type PortalBackgroundConfig = {
  readonly hexRadius: number;
  readonly gap: number;
  readonly hexStrokeWidth: number;
  readonly hexOpacity: number;
  readonly lightStrength: number;
  readonly blobBlur: number;
  readonly beamBlur: number;
  readonly themes: {
    readonly dark: PortalBackgroundThemeColors;
    readonly light: PortalBackgroundThemeColors;
  };
  readonly blobColors: readonly [string, string, string, string, string, string];
  readonly beamColors: readonly [string, string, string];
};

export type PortalBackgroundRenderConfig = {
  readonly beamBlur: number;
  readonly beamColors: readonly [string, string, string];
  readonly blobBlur: number;
  readonly blobColors: readonly [string, string, string, string, string, string];
  readonly colors: PortalBackgroundThemeColors;
  readonly gap: number;
  readonly hexOpacity: number;
  readonly hexRadius: number;
  readonly hexStrokeWidth: number;
  readonly lightStrength: number;
};

export const PortalBackgroundRuntime = {
  Api: {
    StaticConfigAsset: '/portal-background-config.json',
    ConfigEndpoint: '/__ocentra-parent/background-config',
  },
  UpdateEvent: 'portal-background-config-updated',
  Channel: 'ocentra-parent-background-config-channel',
  ContentType: {
    Json: 'application/json',
  },
  FetchCache: {
    NoStore: 'no-store',
  },
  HttpHeader: {
    ContentType: 'Content-Type',
  },
  HttpMethod: {
    Put: 'PUT',
  },
  ValueType: {
    Object: 'object',
    Number: 'number',
    String: 'string',
    Undefined: 'undefined',
  },
} as const;

export const DEFAULT_PORTAL_BACKGROUND_DARK_COLORS: PortalBackgroundThemeColors = {
  bgBaseStart: '#07111f',
  bgBaseMid: '#0a1a2f',
  bgBaseEnd: '#081423',
  vignetteCenter: '#12325a',
  vignetteMid: '#0c2240',
  vignetteEdge: '#02070d',
  hexStroke: '#9fc6ea',
};

export const DEFAULT_PORTAL_BACKGROUND_LIGHT_COLORS: PortalBackgroundThemeColors = {
  bgBaseStart: '#dcebfb',
  bgBaseMid: '#c5ddf7',
  bgBaseEnd: '#d7e8fb',
  vignetteCenter: '#9ec5f0',
  vignetteMid: '#b8d6f4',
  vignetteEdge: '#b5d0eb',
  hexStroke: '#6a97c7',
};

export const DEFAULT_PORTAL_BACKGROUND_CONFIG: PortalBackgroundConfig = {
  hexRadius: 40,
  gap: 4,
  hexStrokeWidth: 1,
  hexOpacity: 0.33,
  lightStrength: 1,
  blobBlur: 82,
  beamBlur: 28,
  themes: {
    dark: DEFAULT_PORTAL_BACKGROUND_DARK_COLORS,
    light: DEFAULT_PORTAL_BACKGROUND_LIGHT_COLORS,
  },
  blobColors: ['#f3efb0', '#a9efd8', '#98ebee', '#dff1a6', '#b9efc2', '#93e7d6'],
  beamColors: ['#f4efad', '#9de9e9', '#b8edb1'],
};

export const SAVED_PORTAL_BACKGROUND_CONFIG: PortalBackgroundConfig =
  normalizePortalBackgroundConfig(savedPortalBackgroundConfig);

export function portalBackgroundRenderConfig(
  config: PortalBackgroundConfig,
  theme: PortalThemeValue
): PortalBackgroundRenderConfig {
  const normalized = normalizePortalBackgroundConfig(config);
  return {
    beamBlur: normalized.beamBlur,
    beamColors: normalized.beamColors,
    blobBlur: normalized.blobBlur,
    blobColors: normalized.blobColors,
    colors: theme === PortalTheme.Light ? normalized.themes.light : normalized.themes.dark,
    gap: normalized.gap,
    hexOpacity: normalized.hexOpacity,
    hexRadius: normalized.hexRadius,
    hexStrokeWidth: normalized.hexStrokeWidth,
    lightStrength: normalized.lightStrength,
  };
}

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
  if (typeof window === PortalBackgroundRuntime.ValueType.Undefined) {
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

async function getPortalBackgroundConfig(url: string): Promise<PortalBackgroundConfig | undefined> {
  if (typeof window === PortalBackgroundRuntime.ValueType.Undefined) {
    return undefined;
  }
  const response = await fetch(url, { cache: PortalBackgroundRuntime.FetchCache.NoStore }).catch(() => undefined);
  if (response?.ok !== true) {
    return undefined;
  }
  return normalizePortalBackgroundConfig(await response.json());
}

export function readDefaultPortalBackgroundConfig(): PortalBackgroundConfig {
  return SAVED_PORTAL_BACKGROUND_CONFIG;
}

export function subscribePortalBackgroundConfig(onConfigChange: (config: PortalBackgroundConfig) => void): () => void {
  if (typeof window === PortalBackgroundRuntime.ValueType.Undefined) {
    return () => undefined;
  }
  const onLocalEvent = (event: Event): void => {
    const detail = (event as CustomEvent<PortalBackgroundConfig>).detail;
    onConfigChange(normalizePortalBackgroundConfig(detail));
  };
  const channel =
    typeof BroadcastChannel === PortalBackgroundRuntime.ValueType.Undefined
      ? undefined
      : new BroadcastChannel(PortalBackgroundRuntime.Channel);
  const onChannelMessage = (): void => {
    void loadPortalBackgroundConfig().then(onConfigChange);
  };
  channel?.addEventListener('message', onChannelMessage);
  window.addEventListener(PortalBackgroundRuntime.UpdateEvent, onLocalEvent);
  return () => {
    channel?.removeEventListener('message', onChannelMessage);
    channel?.close();
    window.removeEventListener(PortalBackgroundRuntime.UpdateEvent, onLocalEvent);
  };
}

export function normalizePortalBackgroundConfig(value: unknown): PortalBackgroundConfig {
  const source = recordOrUndefined(value);
  const defaults = DEFAULT_PORTAL_BACKGROUND_CONFIG;
  return {
    hexRadius: boundedNumber(source?.['hexRadius'], defaults.hexRadius, 20, 80),
    gap: boundedNumber(source?.['gap'], defaults.gap, 0, 20),
    hexStrokeWidth: boundedNumber(source?.['hexStrokeWidth'], defaults.hexStrokeWidth, 0.5, 3),
    hexOpacity: boundedNumber(source?.['hexOpacity'], defaults.hexOpacity, 0.05, 0.5),
    lightStrength: boundedNumber(source?.['lightStrength'], defaults.lightStrength, 0, 2),
    blobBlur: boundedNumber(source?.['blobBlur'], defaults.blobBlur, 30, 140),
    beamBlur: boundedNumber(source?.['beamBlur'], defaults.beamBlur, 8, 60),
    themes: normalizeThemeColors(recordOrUndefined(source?.['themes'])),
    blobColors: normalizeColorTuple(source?.['blobColors'], defaults.blobColors),
    beamColors: normalizeColorTriple(source?.['beamColors'], defaults.beamColors),
  };
}

function notifyPortalBackgroundConfigChanged(config: PortalBackgroundConfig): void {
  window.dispatchEvent(new CustomEvent(PortalBackgroundRuntime.UpdateEvent, { detail: config }));
  if (typeof BroadcastChannel === PortalBackgroundRuntime.ValueType.Undefined) {
    return;
  }
  const channel = new BroadcastChannel(PortalBackgroundRuntime.Channel);
  channel.postMessage(PortalBackgroundRuntime.UpdateEvent);
  channel.close();
}

function normalizeThemeColors(value: Record<string, unknown> | undefined): PortalBackgroundConfig['themes'] {
  return {
    dark: normalizeThemeColorSet(recordOrUndefined(value?.['dark']), DEFAULT_PORTAL_BACKGROUND_DARK_COLORS),
    light: normalizeThemeColorSet(recordOrUndefined(value?.['light']), DEFAULT_PORTAL_BACKGROUND_LIGHT_COLORS),
  };
}

function normalizeThemeColorSet(
  value: Record<string, unknown> | undefined,
  defaultColors: PortalBackgroundThemeColors
): PortalBackgroundThemeColors {
  return {
    bgBaseStart: colorValue(value?.['bgBaseStart'], defaultColors.bgBaseStart),
    bgBaseMid: colorValue(value?.['bgBaseMid'], defaultColors.bgBaseMid),
    bgBaseEnd: colorValue(value?.['bgBaseEnd'], defaultColors.bgBaseEnd),
    vignetteCenter: colorValue(value?.['vignetteCenter'], defaultColors.vignetteCenter),
    vignetteMid: colorValue(value?.['vignetteMid'], defaultColors.vignetteMid),
    vignetteEdge: colorValue(value?.['vignetteEdge'], defaultColors.vignetteEdge),
    hexStroke: colorValue(value?.['hexStroke'], defaultColors.hexStroke),
  };
}

function normalizeColorTuple(
  value: unknown,
  defaultColors: readonly [string, string, string, string, string, string]
): readonly [string, string, string, string, string, string] {
  const values = Array.isArray(value) ? value : defaultColors;
  return [
    colorValue(values[0], defaultColors[0]),
    colorValue(values[1], defaultColors[1]),
    colorValue(values[2], defaultColors[2]),
    colorValue(values[3], defaultColors[3]),
    colorValue(values[4], defaultColors[4]),
    colorValue(values[5], defaultColors[5]),
  ];
}

function normalizeColorTriple(
  value: unknown,
  defaultColors: readonly [string, string, string]
): readonly [string, string, string] {
  const values = Array.isArray(value) ? value : defaultColors;
  return [
    colorValue(values[0], defaultColors[0]),
    colorValue(values[1], defaultColors[1]),
    colorValue(values[2], defaultColors[2]),
  ];
}

function boundedNumber(value: unknown, defaultValue: number, min: number, max: number): number {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    return defaultValue;
  }
  return Math.min(max, Math.max(min, value));
}

function colorValue(value: unknown, defaultValue: string): string {
  if (typeof value !== 'string') {
    return defaultValue;
  }
  return /^#[0-9a-f]{6}$/iu.test(value) ? value : defaultValue;
}

function recordOrUndefined(value: unknown): Record<string, unknown> | undefined {
  if (typeof value !== PortalBackgroundRuntime.ValueType.Object || value === null || Array.isArray(value)) {
    return undefined;
  }
  return value as Record<string, unknown>;
}
