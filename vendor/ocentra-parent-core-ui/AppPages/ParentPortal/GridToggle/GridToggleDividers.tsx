import type { CSSProperties, ReactElement } from 'react';
import type { GridToggleConfig } from './GridToggleConfig';
import type { GridToggleIds, GridToggleMetrics } from './GridToggleTypes';

export function GridToggleDividers({
  config,
  dividerGlowOpacity,
  ids,
  metrics,
  svgStyle,
}: {
  config: GridToggleConfig;
  dividerGlowOpacity: number;
  ids: GridToggleIds;
  metrics: GridToggleMetrics;
  svgStyle: CSSProperties;
}): ReactElement {
  return (
    <>
      {metrics.verticalDividerXs.map((dividerX) => (
        <g key={`v-divider-${dividerX}`}>
          <line
            x1={dividerX}
            y1={metrics.gridY + config.effects.dividerInset}
            x2={dividerX}
            y2={metrics.gridY + metrics.gridHeight - config.effects.dividerInset}
            stroke={config.colors.dividerGlow}
            strokeWidth={config.layout.dividerWidth + config.effects.dividerGlowStrokeExtra}
            opacity={dividerGlowOpacity}
            filter={`url(#${ids.dividerGlow})`}
            style={svgStyle}
          />
          <line
            x1={dividerX}
            y1={metrics.gridY + config.effects.dividerInset}
            x2={dividerX}
            y2={metrics.gridY + metrics.gridHeight - config.effects.dividerInset}
            stroke={config.colors.divider}
            strokeWidth={config.layout.dividerWidth + config.effects.dividerMainStrokeExtra}
            opacity={config.opacity.divider}
            style={svgStyle}
          />
        </g>
      ))}
      {metrics.horizontalDividerYs.map((dividerY) => (
        <g key={`h-divider-${dividerY}`}>
          <line
            x1={metrics.gridX + config.effects.dividerInset}
            y1={dividerY}
            x2={metrics.gridX + metrics.gridWidth - config.effects.dividerInset}
            y2={dividerY}
            stroke={config.colors.dividerGlow}
            strokeWidth={config.layout.dividerWidth + config.effects.dividerGlowStrokeExtra}
            opacity={dividerGlowOpacity}
            filter={`url(#${ids.dividerGlow})`}
            style={svgStyle}
          />
          <line
            x1={metrics.gridX + config.effects.dividerInset}
            y1={dividerY}
            x2={metrics.gridX + metrics.gridWidth - config.effects.dividerInset}
            y2={dividerY}
            stroke={config.colors.divider}
            strokeWidth={config.layout.dividerWidth + config.effects.dividerMainStrokeExtra}
            opacity={config.opacity.divider}
            style={svgStyle}
          />
        </g>
      ))}
    </>
  );
}
