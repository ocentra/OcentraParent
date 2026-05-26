import type { ReactElement } from 'react';
import type { ScopeToggleConfig } from './ScopeToggleConfig';
import type { ScopeToggleIds, ScopeToggleMetrics } from './ScopeToggleTypes';

type ScopeToggleDefsProps = {
  config: ScopeToggleConfig;
  dividerGlowOpacity: number;
  glowOpacity: number;
  ids: ScopeToggleIds;
  metrics: ScopeToggleMetrics;
  outerGlowOpacity: number;
  sliderGlowOpacity: number;
  sliderGloss: {
    height: number;
    width: number;
    x: number;
    y: number;
  };
  titleGlowOpacity: number;
};

export function ScopeToggleDefs({
  config,
  dividerGlowOpacity,
  glowOpacity,
  ids,
  metrics,
  outerGlowOpacity,
  sliderGlowOpacity,
  sliderGloss,
  titleGlowOpacity,
}: ScopeToggleDefsProps): ReactElement {
  const sliderBottomY = sliderGloss.y + sliderGloss.height;
  const trackBottomY = metrics.trackY + metrics.trackHeight;

  return (
    <defs>
      <linearGradient id={ids.track} x1={0} y1={metrics.trackY} x2={0} y2={trackBottomY} gradientUnits="userSpaceOnUse">
        <stop offset={config.effects.gradientStart} stopColor={config.colors.trackTop} />
        <stop offset={config.effects.gradientEnd} stopColor={config.colors.trackBottom} />
      </linearGradient>
      <linearGradient
        id={ids.slider}
        x1={0}
        y1={sliderGloss.y}
        x2={0}
        y2={sliderBottomY}
        gradientUnits="userSpaceOnUse"
      >
        <stop offset={config.effects.gradientStart} stopColor={config.colors.sliderTop} />
        <stop offset={config.effects.gradientEnd} stopColor={config.colors.sliderBottom} />
      </linearGradient>
      <linearGradient
        id={ids.sliderShine}
        x1={0}
        y1={sliderGloss.y}
        x2={0}
        y2={sliderBottomY}
        gradientUnits="userSpaceOnUse"
      >
        <stop
          offset={config.effects.sliderShineStop0}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.sliderShineOpacity0}
        />
        <stop
          offset={config.effects.sliderShineStop1}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.sliderShineOpacity1}
        />
        <stop
          offset={config.effects.sliderShineStop2}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.sliderShineOpacity2}
        />
      </linearGradient>
      <linearGradient
        id={ids.sliderBottomGloss}
        x1={0}
        y1={sliderGloss.y}
        x2={0}
        y2={sliderBottomY}
        gradientUnits="userSpaceOnUse"
      >
        <stop
          offset={config.effects.sliderBottomGlossStop0}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.sliderBottomGlossOpacity0}
        />
        <stop
          offset={config.effects.sliderBottomGlossStop1}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.sliderBottomGlossOpacity1}
        />
        <stop
          offset={config.effects.sliderBottomGlossStop2}
          stopColor={config.colors.shine}
          stopOpacity={config.effects.sliderBottomGlossOpacity2}
        />
      </linearGradient>
      <filter
        id={ids.trackGlow}
        x={config.effects.trackGlowX}
        y={config.effects.trackGlowY}
        width={config.effects.trackGlowWidth}
        height={config.effects.trackGlowHeight}
      >
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.glowBlurA}
          floodColor={config.colors.trackGlow}
          floodOpacity={glowOpacity}
        />
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.glowBlurB}
          floodColor={config.colors.trackGlow}
          floodOpacity={glowOpacity * config.effects.trackGlowSecondOpacityMultiplier}
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
        id={ids.sliderGlow}
        x={config.effects.sliderGlowX}
        y={config.effects.sliderGlowY}
        width={config.effects.sliderGlowWidth}
        height={config.effects.sliderGlowHeight}
      >
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.glowBlurA}
          floodColor={config.colors.sliderGlow}
          floodOpacity={sliderGlowOpacity}
        />
        <feDropShadow
          dx={config.effects.glowDx}
          dy={config.effects.glowDy}
          stdDeviation={config.filters.glowBlurB}
          floodColor={config.colors.sliderGlow}
          floodOpacity={sliderGlowOpacity * config.effects.sliderGlowSecondOpacityMultiplier}
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
