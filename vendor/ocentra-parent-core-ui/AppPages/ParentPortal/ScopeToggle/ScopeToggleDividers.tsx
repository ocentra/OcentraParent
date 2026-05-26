import type { CSSProperties, ReactElement } from 'react';
import type { ScopeToggleConfig } from './ScopeToggleConfig';
import type { ScopeToggleIds, ScopeToggleMetrics } from './ScopeToggleTypes';

export function ScopeToggleDividers({
  config,
  dividerGlowOpacity,
  ids,
  metrics,
  svgStyle,
}: {
  config: ScopeToggleConfig;
  dividerGlowOpacity: number;
  ids: ScopeToggleIds;
  metrics: ScopeToggleMetrics;
  svgStyle: CSSProperties;
}): ReactElement {
  return (
    <>
      {metrics.dividerXs.map((dividerX) => (
        <g key={`divider-${dividerX}`}>
          <line
            x1={dividerX}
            y1={metrics.trackY + config.effects.dividerTopInset}
            x2={dividerX}
            y2={metrics.trackY + metrics.trackHeight - config.effects.dividerBottomInset}
            stroke={config.colors.dividerGlow}
            strokeWidth={config.layout.dividerWidth + config.effects.dividerGlowStrokeExtra}
            opacity={dividerGlowOpacity}
            filter={`url(#${ids.dividerGlow})`}
            style={svgStyle}
          />
          <line
            x1={dividerX}
            y1={metrics.trackY + config.effects.dividerMainTopInset}
            x2={dividerX}
            y2={metrics.trackY + metrics.trackHeight - config.effects.dividerMainBottomInset}
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
