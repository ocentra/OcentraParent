import type { ScopeMultiChoiceConfig, ScopeMultiChoiceConfigOverride } from './ScopeMultiChoiceTypes';

export const defaultScopeMultiChoiceConfig: ScopeMultiChoiceConfig = {
  svg: {
    width: 640,
    height: undefined,
    minHeight: 150,
    viewportInset: 12,
    fitMode: 'autoHeight',
    overflowMode: 'visible',
  },
  layout: {
    titleBoxX: 12,
    titleBoxY: 10,
    titleBoxMinWidth: 118,
    titleBoxPaddingX: 18,
    titleBoxHeight: 32,
    titleBoxRadius: 8,
    titleBoxBottomRadius: 0,
    trackX: 12,
    trackY: 42,
    trackYWithoutTitle: 12,
    trackWidth: 596,
    optionMinWidth: 118,
    optionMaxWidth: 260,
    optionHeight: 42,
    optionGapX: 8,
    optionGapY: 8,
    distributeRowSpace: true,
    maxExtraWidthPerOption: 82,
    maxOptions: 36,
    optionPaddingX: 14,
    outerPadX: 4,
    outerPadY: 4,
    outerRadius: 12,
    outerPaddingRight: 10,
    outerPaddingBottom: 10,
  },
  outerEdge: {
    strokeWidth: 0.75,
    glowStrokeWidth: 1.1,
  },
  titleBox: {
    strokeWidth: 1.1,
    innerStrokeWidth: 0.5,
    glowStrokeWidth: 2.3,
  },
  track: {
    radius: 10,
    strokeWidth: 1.1,
    innerStrokeWidth: 0.55,
    glowStrokeWidth: 1.5,
  },
  optionButton: {
    inset: 5,
    radius: 7,
    strokeWidth: 1,
    glowStrokeWidth: 1.8,
  },
  indicator: {
    radius: 7,
    strokeWidth: 1,
    circleRadius: 6.2,
    circleStrokeWidth: 1.7,
  },
  text: {
    title: 'Choice',
    options: [
      { value: 'a', label: 'Option A' },
      { value: 'b', label: 'Option B' },
    ],
    titleFontSize: 14,
    optionFontSize: 13,
    fontWeight: 800,
    optionFontWeight: 800,
    fontFamily: 'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif',
  },
  colors: {
    title: '#ffd98a',
    titleBoxStroke: '#c7a75a',
    titleBoxStrokeHover: '#ffe8a8',
    titleBoxGlow: '#c7a75a',
    outerEdge: '#bfeeff',
    outerEdgeGlow: '#55dfff',
    optionIdle: '#d4e4f0',
    optionHover: '#ffffff',
    optionSelected: '#06131e',
    trackTop: '#1c2b3f',
    trackBottom: '#0d1724',
    trackStroke: '#6f849d',
    trackStrokeHover: '#d6ecff',
    trackGlow: '#53c7ff',
    selectedTop: '#bff3ff',
    selectedBottom: '#4cc8ff',
    selectedStroke: '#f4fdff',
    selectedGlow: '#38dfff',
    indicatorStroke: '#8fffc0',
    indicatorCircleIdle: '#7cffaa',
    indicatorCircleSelected: '#39ff88',
    indicatorCircleGlow: '#4dff91',
    shine: '#ffffff',
    shadow: '#020617',
  },
  opacity: {
    trackGlowIdle: 0.2,
    trackGlowHover: 0.46,
    titleGlowIdle: 0.14,
    titleGlowHover: 0.28,
    outerGlowIdle: 0.18,
    outerGlowHover: 0.44,
    selectedGlowIdle: 0.34,
    selectedGlowHover: 0.7,
    shineIdle: 0.26,
    shineHover: 0.42,
    disabled: 0.45,
  },
  hover: {
    pressScale: 0.992,
  },
  transition: {
    root: 'transform 140ms ease, opacity 160ms ease',
    svg: 'opacity 160ms ease, fill 180ms ease, stroke 180ms ease, filter 180ms ease',
  },
};

export function mergeScopeMultiChoiceConfig(
  base: ScopeMultiChoiceConfig,
  override?: ScopeMultiChoiceConfigOverride
): ScopeMultiChoiceConfig {
  if (!override) {
    return base;
  }

  const merged: ScopeMultiChoiceConfig = {
    ...base,
    ...override,
    svg: { ...base.svg, ...override.svg },
    layout: { ...base.layout, ...override.layout },
    outerEdge: { ...base.outerEdge, ...override.outerEdge },
    titleBox: { ...base.titleBox, ...override.titleBox },
    track: { ...base.track, ...override.track },
    optionButton: { ...base.optionButton, ...override.optionButton },
    indicator: { ...base.indicator, ...override.indicator },
    text: { ...base.text, ...override.text },
    colors: { ...base.colors, ...override.colors },
    opacity: { ...base.opacity, ...override.opacity },
    hover: { ...base.hover, ...override.hover },
    transition: { ...base.transition, ...override.transition },
  };

  return merged;
}
