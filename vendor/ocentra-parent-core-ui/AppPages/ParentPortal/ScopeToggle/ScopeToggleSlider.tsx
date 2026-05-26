import type { CSSProperties, ReactElement } from 'react';
import type { ScopeToggleConfig } from './ScopeToggleConfig';
import type { ScopeToggleIds, ScopeTogglePaths } from './ScopeToggleTypes';

export function ScopeToggleSlider({
  config,
  ids,
  paths,
  shineOpacity,
  sliderGlossOpacity,
  sliderWidth,
  sliderX,
  sliderY,
  svgStyle,
}: {
  config: ScopeToggleConfig;
  ids: ScopeToggleIds;
  paths: ScopeTogglePaths;
  shineOpacity: number;
  sliderGlossOpacity: number;
  sliderWidth: number;
  sliderX: number;
  sliderY: number;
  svgStyle: CSSProperties;
}): ReactElement {
  return (
    <>
      <path
        d={paths.slider}
        fill={`url(#${ids.slider})`}
        stroke={config.colors.sliderStroke}
        strokeWidth={config.slider.strokeWidth}
        filter={`url(#${ids.sliderGlow})`}
        style={svgStyle}
      />
      <path d={paths.slider} fill={`url(#${ids.sliderShine})`} opacity={shineOpacity} style={svgStyle} />
      <path d={paths.slider} fill={`url(#${ids.sliderBottomGloss})`} opacity={sliderGlossOpacity} style={svgStyle} />
      <path
        d={`M${sliderX + config.effects.sliderHighlightInsetX} ${sliderY + config.effects.sliderHighlightInsetY}H${sliderX + sliderWidth - config.effects.sliderHighlightInsetX}`}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.effects.sliderHighlightStrokeWidth}
        strokeLinecap="round"
        strokeOpacity={config.effects.sliderHighlightStrokeOpacity}
        style={svgStyle}
      />
      <path
        d={paths.slider}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.effects.sliderInnerStrokeWidth}
        strokeOpacity={config.effects.sliderInnerStrokeOpacity}
        style={svgStyle}
      />
    </>
  );
}
