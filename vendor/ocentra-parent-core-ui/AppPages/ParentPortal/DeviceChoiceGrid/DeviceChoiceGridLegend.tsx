import type { ReactElement } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import { estimateTextWidth } from './DeviceChoiceGridGeometry';
import type { DeviceStatus } from './DeviceChoiceGridTypes';

export function DeviceChoiceGridLegend({
  cfg,
  statuses,
}: {
  cfg: DeviceChoiceGridConfig;
  statuses: DeviceStatus[];
}): ReactElement {
  const columnCount = Math.min(2, Math.max(1, statuses.length));
  const columnWidth = Math.max(
    112,
    ...statuses.map(
      (status) =>
        cfg.layout.legendDotR * 2 +
        cfg.layout.legendTextOffset +
        estimateTextWidth(cfg.text.legend[status], cfg.text.legendSize) +
        24
    )
  );

  return (
    <g pointerEvents="none">
      {statuses.map((status, index) => {
        const row = Math.floor(index / columnCount);
        const column = index % columnCount;
        const dotX = cfg.layout.legendX + column * columnWidth;
        const dotY = cfg.layout.legendY + row * cfg.layout.legendItemGap + cfg.layout.legendDotR;
        const showDivider = column === 0 && index + 1 < statuses.length;

        return (
          <g key={`legend-${status}`}>
            <circle
              cx={dotX}
              cy={dotY}
              r={cfg.layout.legendDotR}
              fill={cfg.colors[status]}
              opacity={status === 'unsupported' || status === 'empty' ? 0.55 : 0.95}
            />
            <text
              x={dotX + cfg.layout.legendTextOffset}
              y={cfg.layout.legendY + row * cfg.layout.legendItemGap + cfg.layout.legendDotR * 2 + 1}
              fill={cfg.colors.legendText}
              fontFamily={cfg.text.font}
              fontSize={cfg.text.legendSize}
              fontWeight={cfg.text.optionWeight}
            >
              {cfg.text.legend[status]}
            </text>
            {showDivider ? (
              <line
                x1={cfg.layout.legendX + columnWidth - 12}
                y1={dotY - cfg.layout.legendDotR - 2}
                x2={cfg.layout.legendX + columnWidth - 12}
                y2={dotY + cfg.layout.legendDotR + 3}
                stroke={cfg.colors.legendText}
                strokeWidth={0.8}
                opacity={0.42}
              />
            ) : null}
          </g>
        );
      })}
    </g>
  );
}
