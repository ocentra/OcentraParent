import type { CSSProperties, ReactElement } from 'react';
import type { GridToggleConfig } from './GridToggleConfig';
import type { GridToggleIds, GridTogglePaths, GridToggleSelectionMetrics } from './GridToggleTypes';

export function GridToggleSelectedCell({
  config,
  ids,
  paths,
  selectedGlossOpacity,
  selection,
  shineOpacity,
  svgStyle,
}: {
  config: GridToggleConfig;
  ids: GridToggleIds;
  paths: GridTogglePaths;
  selectedGlossOpacity: number;
  selection: GridToggleSelectionMetrics;
  shineOpacity: number;
  svgStyle: CSSProperties;
}): ReactElement {
  return (
    <>
      <path
        d={paths.selected}
        fill={`url(#${ids.selected})`}
        stroke={config.colors.selectedStroke}
        strokeWidth={config.selectedCell.strokeWidth}
        filter={`url(#${ids.selectedGlow})`}
        style={svgStyle}
      />
      <path d={paths.selected} fill={`url(#${ids.selectedShine})`} opacity={shineOpacity} style={svgStyle} />
      <path
        d={paths.selected}
        fill={`url(#${ids.selectedBottomGloss})`}
        opacity={selectedGlossOpacity}
        style={svgStyle}
      />
      <path
        d={`M${selection.x + config.effects.selectedHighlightInsetX} ${selection.y + config.effects.selectedHighlightInsetY}H${selection.x + selection.width - config.effects.selectedHighlightInsetX}`}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.effects.selectedHighlightStrokeWidth}
        strokeLinecap="round"
        strokeOpacity={config.effects.selectedHighlightStrokeOpacity}
        style={svgStyle}
      />
      <path
        d={paths.selected}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.effects.selectedInnerStrokeWidth}
        strokeOpacity={config.effects.selectedInnerStrokeOpacity}
        style={svgStyle}
      />
    </>
  );
}
