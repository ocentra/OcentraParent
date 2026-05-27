import type { CSSProperties, ReactElement } from 'react';
import type { GridToggleConfig } from './GridToggleConfig';
import type { GridToggleMetrics, GridToggleOption } from './GridToggleTypes';

type GridToggleCellsProps = {
  config: GridToggleConfig;
  disabled: boolean;
  hoveredIndex: number | null;
  metrics: GridToggleMetrics;
  options: GridToggleOption[];
  pressedIndex: number | null;
  selectedIndex: number;
  svgStyle: CSSProperties;
  onHoverChange: (index: number | null) => void;
  onPressChange: (index: number | null) => void;
  onSelect: (value: string) => void;
};

export function GridToggleCells({
  config,
  disabled,
  hoveredIndex,
  metrics,
  options,
  pressedIndex,
  selectedIndex,
  svgStyle,
  onHoverChange,
  onPressChange,
  onSelect,
}: GridToggleCellsProps): ReactElement {
  const optionCenterYBase = config.text.optionFontSize * 0.35;

  return (
    <>
      {options.map((option, index) => {
        const row = Math.floor(index / metrics.columnCount);
        const column = index % metrics.columnCount;
        const cellX =
          metrics.gridX + column * (metrics.cellWidth + config.layout.dividerWidth + config.layout.cellGapX);
        const cellY = metrics.gridY + row * (metrics.cellHeight + config.layout.dividerWidth + config.layout.cellGapY);
        const optionCenterX = cellX + metrics.cellWidth * 0.5;
        const optionCenterY = cellY + metrics.cellHeight * 0.5 + optionCenterYBase;
        const isSelected = index === selectedIndex;
        const isHovered = hoveredIndex === index;
        const isPressedCell = pressedIndex === index;

        return (
          <g
            key={option.value}
            role="button"
            tabIndex={disabled ? -1 : 0}
            aria-label={`Select ${option.label}`}
            onPointerEnter={() => onHoverChange(index)}
            onPointerLeave={() => {
              onHoverChange(null);
              onPressChange(null);
            }}
            onPointerDown={(event) => {
              event.stopPropagation();
              onPressChange(index);
            }}
            onPointerUp={(event) => {
              event.stopPropagation();
              onPressChange(null);
            }}
            onClick={(event) => {
              event.stopPropagation();
              onSelect(option.value);
            }}
            onKeyDown={(event) => {
              if (event.key === 'Enter' || event.key === ' ') {
                event.preventDefault();
                onSelect(option.value);
              }
            }}
            style={{ cursor: disabled ? 'not-allowed' : 'pointer', outline: 'none' }}
          >
            <rect
              x={cellX}
              y={cellY}
              width={metrics.cellWidth}
              height={metrics.cellHeight}
              fill="transparent"
              opacity={0}
            />
            {isHovered && !isSelected ? (
              <rect
                x={cellX + config.selectedCell.inset}
                y={cellY + config.selectedCell.inset}
                width={metrics.cellWidth - config.selectedCell.inset * 2}
                height={metrics.cellHeight - config.selectedCell.inset * 2}
                rx={config.selectedCell.radius}
                fill={config.colors.shine}
                opacity={isPressedCell ? 0.11 : 0.055}
                style={svgStyle}
              />
            ) : null}
            <text
              x={optionCenterX}
              y={optionCenterY}
              textAnchor="middle"
              fill={isSelected ? config.colors.optionSelected : config.colors.optionIdle}
              fontFamily={config.text.fontFamily}
              fontSize={config.text.optionFontSize}
              fontWeight={config.text.optionFontWeight}
              pointerEvents="none"
              style={svgStyle}
            >
              {option.label}
            </text>
          </g>
        );
      })}
    </>
  );
}
