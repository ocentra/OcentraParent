import type { CSSProperties, ReactElement } from 'react';
import type { GridToggleConfig } from './GridToggleConfig';
import type { GridToggleIds, GridToggleMetrics, GridTogglePaths } from './GridToggleTypes';

export function GridToggleFrame({
  config,
  glowOpacity,
  ids,
  isHovering,
  metrics,
  outerGlowOpacity,
  paths,
  svgStyle,
  titleText,
  titleGlowOpacity,
}: {
  config: GridToggleConfig;
  glowOpacity: number;
  ids: GridToggleIds;
  isHovering: boolean;
  metrics: GridToggleMetrics;
  outerGlowOpacity: number;
  paths: GridTogglePaths;
  svgStyle: CSSProperties;
  titleText: string;
  titleGlowOpacity: number;
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
      <text
        x={metrics.titleCenterX}
        y={metrics.titleBoxY + config.layout.titleBoxHeight * 0.64}
        textAnchor="middle"
        fill={config.colors.title}
        fontFamily={config.text.fontFamily}
        fontSize={config.text.titleFontSize}
        fontWeight={config.text.fontWeight}
        style={svgStyle}
        pointerEvents="none"
      >
        {titleText}
      </text>
      <path
        d={paths.grid}
        fill="none"
        stroke={config.colors.gridGlow}
        strokeWidth={config.grid.glowStrokeWidth}
        strokeOpacity={glowOpacity}
        filter={`url(#${ids.gridGlow})`}
        style={svgStyle}
      />
      <path
        d={paths.grid}
        fill={`url(#${ids.grid})`}
        stroke={isHovering ? config.colors.gridStrokeHover : config.colors.gridStroke}
        strokeWidth={config.grid.strokeWidth}
        filter={`url(#${ids.shadow})`}
        style={svgStyle}
      />
      <path
        d={paths.grid}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.grid.innerStrokeWidth}
        strokeOpacity={config.effects.gridInnerStrokeOpacity}
        style={svgStyle}
      />
    </>
  );
}
