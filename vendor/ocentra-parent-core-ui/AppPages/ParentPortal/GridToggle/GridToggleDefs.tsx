import type { ReactElement } from 'react';
import type { GridToggleConfig } from './GridToggleConfig';
import type { GridToggleIds, GridToggleMetrics, GridToggleSelectionMetrics } from './GridToggleTypes';

type GridToggleDefsProps = {
  config: GridToggleConfig;
  dividerGlowOpacity: number;
  glowOpacity: number;
  ids: GridToggleIds;
  metrics: GridToggleMetrics;
  outerGlowOpacity: number;
  selectedGlowOpacity: number;
  selection: GridToggleSelectionMetrics;
  titleGlowOpacity: number;
};

export function GridToggleDefs({
  config,
  dividerGlowOpacity,
  glowOpacity,
  ids,
  metrics,
  outerGlowOpacity,
  selectedGlowOpacity,
  selection,
  titleGlowOpacity,
}: GridToggleDefsProps): ReactElement {
  const selectedBottomY = selection.y + selection.height;
  const gridBottomY = metrics.gridY + metrics.gridHeight;

  return (
    <defs>
      <linearGradient id={ids.grid} x1={0} y1={metrics.gridY} x2={0} y2={gridBottomY} gradientUnits="userSpaceOnUse">
        <stop offset={config.effects.gradientStart} stopColor={config.colors.gridTop} />
        <stop offset={config.effects.gradientEnd} stopColor={config.colors.gridBottom} />
      </linearGradient>
      <linearGradient
        id={ids.selected}
        x1={0}
        y1={selection.y}
        x2={0}
        y2={selectedBottomY}
        gradientUnits="userSpaceOnUse"
      >
        <stop offset={config.effects.gradientStart} stopColor={config.colors.selectedTop} />
        <stop offset={config.effects.gradientEnd} stopColor={config.colors.selectedBottom} />
      </linearGradient>
      <linearGradient
        id={ids.selectedShine}
        x1={0}
        y1={selection.y}
        x2={0}
        y2={selectedBottomY}
        gradientUnits="userSpaceOnUse"
      >
        <stop
          offset={config.effects.selectedShineStop0}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.selectedShineOpacity0}
        />
        <stop
          offset={config.effects.selectedShineStop1}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.selectedShineOpacity1}
        />
        <stop
          offset={config.effects.selectedShineStop2}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.selectedShineOpacity2}
        />
      </linearGradient>
      <linearGradient
        id={ids.selectedBottomGloss}
        x1={0}
        y1={selection.y}
        x2={0}
        y2={selectedBottomY}
        gradientUnits="userSpaceOnUse"
      >
        <stop
          offset={config.effects.selectedBottomGlossStop0}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.selectedBottomGlossOpacity0}
        />
        <stop
          offset={config.effects.selectedBottomGlossStop1}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.selectedBottomGlossOpacity1}
        />
        <stop
          offset={config.effects.selectedBottomGlossStop2}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.selectedBottomGlossOpacity2}
        />
      </linearGradient>
      <filter
        id={ids.gridGlow}
        x={config.effects.gridGlowX}
        y={config.effects.gridGlowY}
        width={config.effects.gridGlowWidth}
        height={config.effects.gridGlowHeight}
      >
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.glowBlurA}
          floodColor={config.colors.gridGlow}
          floodOpacity={glowOpacity}
        />
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.glowBlurB}
          floodColor={config.colors.gridGlow}
          floodOpacity={glowOpacity * config.effects.gridGlowSecondOpacityMultiplier}
        />
      </filter>
      <filter
        id={ids.titleGlow}
        x={config.effects.titleGlowX}
        y={config.effects.titleGlowY}
        width={config.effects.titleGlowWidth}
        height={config.effects.titleGlowHeight}
      >
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.titleGlowBlurA}
          floodColor={config.colors.titleBoxGlow}
          floodOpacity={titleGlowOpacity}
        />
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.titleGlowBlurB}
          floodColor={config.colors.titleBoxGlow}
          floodOpacity={titleGlowOpacity * config.effects.titleGlowSecondOpacityMultiplier}
        />
      </filter>
      <filter
        id={ids.outerGlow}
        x={config.effects.outerGlowX}
        y={config.effects.outerGlowY}
        width={config.effects.outerGlowWidth}
        height={config.effects.outerGlowHeight}
      >
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.outerGlowBlurA}
          floodColor={config.colors.outerEdgeGlow}
          floodOpacity={outerGlowOpacity}
        />
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.outerGlowBlurB}
          floodColor={config.colors.outerEdgeGlow}
          floodOpacity={outerGlowOpacity * config.effects.outerGlowSecondOpacityMultiplier}
        />
      </filter>
      <filter
        id={ids.dividerGlow}
        x={config.effects.dividerGlowX}
        y={config.effects.dividerGlowY}
        width={config.effects.dividerGlowWidth}
        height={config.effects.dividerGlowHeight}
      >
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.dividerGlowBlur}
          floodColor={config.colors.dividerGlow}
          floodOpacity={dividerGlowOpacity}
        />
      </filter>
      <filter
        id={ids.selectedGlow}
        x={config.effects.selectedGlowX}
        y={config.effects.selectedGlowY}
        width={config.effects.selectedGlowWidth}
        height={config.effects.selectedGlowHeight}
      >
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.glowBlurA}
          floodColor={config.colors.selectedGlow}
          floodOpacity={selectedGlowOpacity}
        />
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.glowBlurB}
          floodColor={config.colors.selectedGlow}
          floodOpacity={selectedGlowOpacity * config.effects.selectedGlowSecondOpacityMultiplier}
        />
      </filter>
      <filter
        id={ids.shadow}
        x={config.effects.shadowX}
        y={config.effects.shadowY}
        width={config.effects.shadowWidth}
        height={config.effects.shadowHeight}
      >
        <feDropShadow
          dx={config.effects.shadowDx}
          dy={config.filters.shadowDy}
          stdDeviation={config.filters.shadowBlur}
          floodColor={config.colors.shadow}
          floodOpacity={config.effects.shadowOpacity}
        />
      </filter>
    </defs>
  );
}
