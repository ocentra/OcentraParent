const PortalBackgroundTheme = {
  Dark: 'dark',
  Light: 'light',
} as const;
type PortalBackgroundThemeValue = (typeof PortalBackgroundTheme)[keyof typeof PortalBackgroundTheme];

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

const PORTAL_BACKGROUND_APP_HEX_OPACITY_SCALE = 0.56;
const PORTAL_BACKGROUND_APP_LIGHT_TINT = {
  bgBaseStart: '#d7eafb',
  bgBaseMid: '#c2dcf5',
  bgBaseEnd: '#d0e6f8',
  vignetteMid: '#abcff0',
  vignetteEdge: '#a8c8e8',
} satisfies Partial<PortalBackgroundThemeColors>;

export const PortalBackgroundRuntime = {
  Api: {
    StaticConfigAsset: '/portal-background-config.json',
    ConfigEndpoint: '/__ocentra-parent/background-config',
  },
  Boot: {
    AriaLabel: 'Portal background',
    Id: 'portal-background-boot',
    IdPrefix: 'portalBackgroundBoot',
    PreserveAspectRatio: 'xMidYMid slice',
    ReadyAttribute: 'data-portal-bg-boot-ready',
    Style: 'display:block;height:100%;inset:0;position:absolute;width:100%;pointer-events:none',
  },
  UpdateEvent: 'portal-background-config-updated',
  Channel: 'ocentra-parent-background-config-channel',
  ContentType: {
    Json: 'application/json',
    Missing: '',
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

export function portalBackgroundRenderConfig(
  config: PortalBackgroundConfig,
  theme: PortalBackgroundThemeValue
): PortalBackgroundRenderConfig {
  const normalized = normalizePortalBackgroundConfig(config);
  return {
    beamBlur: normalized.beamBlur,
    beamColors: normalized.beamColors,
    blobBlur: normalized.blobBlur,
    blobColors: normalized.blobColors,
    colors: theme === PortalBackgroundTheme.Light ? normalized.themes.light : normalized.themes.dark,
    gap: normalized.gap,
    hexOpacity: normalized.hexOpacity,
    hexRadius: normalized.hexRadius,
    hexStrokeWidth: normalized.hexStrokeWidth,
    lightStrength: normalized.lightStrength,
  };
}

export function portalBackgroundAppRenderConfig(
  config: PortalBackgroundConfig,
  theme: PortalBackgroundThemeValue
): PortalBackgroundRenderConfig {
  const renderConfig = portalBackgroundRenderConfig(config, theme);
  if (theme === PortalBackgroundTheme.Light) {
    return {
      ...renderConfig,
      colors: {
        ...renderConfig.colors,
        ...PORTAL_BACKGROUND_APP_LIGHT_TINT,
      },
    };
  }
  if (theme !== PortalBackgroundTheme.Dark) {
    return renderConfig;
  }
  return {
    ...renderConfig,
    hexOpacity: renderConfig.hexOpacity * PORTAL_BACKGROUND_APP_HEX_OPACITY_SCALE,
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
