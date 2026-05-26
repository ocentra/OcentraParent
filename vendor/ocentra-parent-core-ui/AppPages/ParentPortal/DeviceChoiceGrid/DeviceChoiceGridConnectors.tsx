import type { ReactElement } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import type { DeviceChoiceGridIds } from './DeviceChoiceGridTypes';

type DeviceChoiceGridConnectorsProps = {
  cfg: DeviceChoiceGridConfig;
  ids: DeviceChoiceGridIds;
  activeGridY: number;
  cellH: number;
  chainClipId?: string;
  columnCount: number;
  connectorY: number;
  rowCount: number;
  scrollY?: number;
  titleBottom: number;
  titleCenterX: number;
  topCenters: number[];
};

export function DeviceChoiceGridConnectors({
  cfg,
  ids,
  activeGridY,
  cellH,
  chainClipId,
  columnCount,
  connectorY,
  rowCount,
  scrollY = 0,
  titleBottom,
  titleCenterX,
  topCenters,
}: DeviceChoiceGridConnectorsProps): ReactElement | null {
  if (!cfg.connector.enabled || topCenters.length === 0) {
    return null;
  }

  const wirePath = [
    `M${titleCenterX} ${titleBottom}`,
    `V${connectorY}`,
    `H${topCenters[columnCount - 1]}`,
    `M${topCenters[0]} ${connectorY}`,
    `H${titleCenterX}`,
  ].join(' ');

  return (
    <g pointerEvents="none" strokeLinecap="round" strokeLinejoin="round">
      <path
        d={wirePath}
        fill="none"
        stroke={cfg.colors.wireGlow}
        strokeWidth={cfg.connector.glowWidth}
        opacity={cfg.opacity.wireGlow}
        filter={`url(#${ids.titleGlow})`}
      />
      <path
        d={wirePath}
        fill="none"
        stroke={cfg.colors.wire}
        strokeWidth={cfg.connector.width}
        opacity={cfg.opacity.wire}
      />
      {topCenters.map((x) => (
        <g key={`top-${x}`}>
          <line
            x1={x}
            y1={connectorY}
            x2={x}
            y2={activeGridY + cfg.connector.branchInset}
            stroke={cfg.colors.wireGlow}
            strokeWidth={cfg.connector.glowWidth}
            opacity={cfg.opacity.wireGlow}
            filter={`url(#${ids.titleGlow})`}
          />
          <line
            x1={x}
            y1={connectorY}
            x2={x}
            y2={activeGridY + cfg.connector.branchInset}
            stroke={cfg.colors.wire}
            strokeWidth={cfg.connector.width}
            opacity={cfg.opacity.wire}
          />
          <circle
            cx={x}
            cy={activeGridY + cfg.connector.branchInset}
            r={cfg.connector.dotR}
            fill={cfg.colors.wire}
            opacity={cfg.opacity.wire}
          />
        </g>
      ))}
      <g
        clipPath={chainClipId ? `url(#${chainClipId})` : undefined}
        transform={scrollY > 0 ? `translate(0 ${-scrollY})` : undefined}
      >
        {Array.from({ length: columnCount * Math.max(0, rowCount - 1) }, (_, index) => {
          const col = index % columnCount;
          const row = Math.floor(index / columnCount);
          const x = topCenters[col] ?? topCenters[0] ?? 0;
          const y1 = activeGridY + row * (cellH + cfg.layout.gapY) + cellH;
          const y2 = activeGridY + (row + 1) * (cellH + cfg.layout.gapY);
          return (
            <g key={`chain-${index}`}>
              <line
                x1={x}
                y1={y1}
                x2={x}
                y2={y2}
                stroke={cfg.colors.wireGlow}
                strokeWidth={cfg.connector.chainGlowWidth}
                opacity={cfg.opacity.wireGlow}
                filter={`url(#${ids.titleGlow})`}
              />
              <line
                x1={x}
                y1={y1}
                x2={x}
                y2={y2}
                stroke={cfg.colors.wire}
                strokeWidth={cfg.connector.chainWidth}
                opacity={cfg.opacity.wire}
              />
              <circle cx={x} cy={y1} r={cfg.connector.smallDotR} fill={cfg.colors.wire} opacity={cfg.opacity.wire} />
              <circle cx={x} cy={y2} r={cfg.connector.smallDotR} fill={cfg.colors.wire} opacity={cfg.opacity.wire} />
            </g>
          );
        })}
      </g>
    </g>
  );
}
