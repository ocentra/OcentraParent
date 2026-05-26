import type { CSSProperties, ReactElement } from 'react';
import type { ScopeToggleConfig } from './ScopeToggleConfig';
import type { ScopeToggleIds, ScopeToggleMetrics, ScopeTogglePaths } from './ScopeToggleTypes';

export function ScopeToggleFrame({
  config,
  ids,
  isHovering,
  metrics,
  paths,
  outerGlowOpacity,
  titleGlowOpacity,
  glowOpacity,
  svgStyle,
}: {
  config: ScopeToggleConfig;
  ids: ScopeToggleIds;
  isHovering: boolean;
  metrics: ScopeToggleMetrics;
  paths: ScopeTogglePaths;
  outerGlowOpacity: number;
  titleGlowOpacity: number;
  glowOpacity: number;
  svgStyle: CSSProperties;
}): ReactElement {
  return (
    <>
      <path
        d={paths.outerEdge}
        fill="none"
        stroke={config.colors.outerEdgeGlow}
        strokeWidth={config.outerEdge.glowStrokeWidth}
        strokeOpacity={outerGlowOpacity}
        filter={`url(#${ids.outerGlow})`}
        style={svgStyle}
      />
      <path
        d={paths.outerEdge}
        fill="none"
        stroke={config.colors.outerEdge}
        strokeWidth={config.outerEdge.strokeWidth}
        strokeOpacity={isHovering ? config.effects.outerStrokeOpacityHover : config.effects.outerStrokeOpacityIdle}
        style={svgStyle}
      />
      <path
        d={paths.titleBox}
        fill="none"
        stroke={config.colors.titleBoxGlow}
        strokeWidth={config.titleBox.glowStrokeWidth}
        strokeOpacity={titleGlowOpacity}
        filter={`url(#${ids.titleGlow})`}
        style={svgStyle}
      />
      <path
        d={paths.titleBox}
        fill="transparent"
        stroke={isHovering ? config.colors.titleBoxStrokeHover : config.colors.titleBoxStroke}
        strokeWidth={config.titleBox.strokeWidth}
        style={svgStyle}
      />
      <path
        d={paths.titleBox}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.titleBox.innerStrokeWidth}
        strokeOpacity={
          isHovering ? config.effects.titleInnerStrokeOpacityHover : config.effects.titleInnerStrokeOpacityIdle
        }
        style={svgStyle}
      />
      <path
        d={paths.track}
        fill="none"
        stroke={config.colors.trackGlow}
        strokeWidth={config.track.glowStrokeWidth}
        strokeOpacity={glowOpacity}
        filter={`url(#${ids.trackGlow})`}
        style={svgStyle}
      />
      <path
        d={paths.track}
        fill={`url(#${ids.track})`}
        stroke={isHovering ? config.colors.trackStrokeHover : config.colors.trackStroke}
        strokeWidth={config.track.strokeWidth}
        filter={`url(#${ids.shadow})`}
        style={svgStyle}
      />
      <path
        d={paths.track}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.track.innerStrokeWidth}
        strokeOpacity={config.effects.trackInnerStrokeOpacity}
        style={svgStyle}
      />
    </>
  );
}
