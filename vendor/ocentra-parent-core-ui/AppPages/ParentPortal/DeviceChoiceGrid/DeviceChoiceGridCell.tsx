import type { ReactElement } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import type { DeviceChoiceGridCellPosition, DeviceChoiceGridIds, DeviceSlot } from './DeviceChoiceGridTypes';

type DeviceChoiceGridCellProps = {
  active: boolean;
  cellH: number;
  cellW: number;
  cfg: DeviceChoiceGridConfig;
  disabled: boolean;
  ids: DeviceChoiceGridIds;
  item: DeviceSlot;
  over: boolean;
  position: DeviceChoiceGridCellPosition;
  press: boolean;
  shineOpacity: number;
  showAdd: boolean;
  onAddToPortal: (slot: DeviceSlot) => void;
  onHoverChange: (index: number | null) => void;
  onPressChange: (index: number | null) => void;
  onSelect: (index: number) => void;
  index: number;
};

export function DeviceChoiceGridCell({
  active,
  cellH,
  cellW,
  cfg,
  disabled,
  ids,
  item,
  over,
  position,
  press,
  shineOpacity,
  showAdd,
  index,
  onAddToPortal,
  onHoverChange,
  onPressChange,
  onSelect,
}: DeviceChoiceGridCellProps): ReactElement {
  const status = item.status ?? 'available';
  const statusColor = cfg.colors[status];
  const empty = status === 'empty';
  const disabledCell = disabled || empty;
  const ariaLabel = empty
    ? 'Empty LAN slot'
    : status === 'unsupported'
      ? `${item.label} is unsupported`
      : `Select ${item.label}`;
  const showLabel = !empty && item.label.trim().length > 0;
  const estimatedLabelW = item.label.length * cfg.text.optionSize * 0.62;
  const labelW = cellW - 20;
  const fittedLabelW = estimatedLabelW > labelW ? labelW : undefined;
  const labelX = position.x + cellW / 2;

  return (
    <g
      role={empty ? undefined : 'button'}
      tabIndex={disabledCell ? -1 : 0}
      aria-label={ariaLabel}
      onPointerEnter={() => {
        if (!empty) {
          onHoverChange(index);
        }
      }}
      onPointerLeave={() => {
        onHoverChange(null);
        onPressChange(null);
      }}
      onPointerDown={(event) => {
        event.stopPropagation();
        if (empty) return;
        onPressChange(index);
      }}
      onPointerUp={(event) => {
        event.stopPropagation();
        onPressChange(null);
      }}
      onClick={(event) => {
        event.stopPropagation();
        onSelect(index);
      }}
      onKeyDown={(event) => {
        if (event.key === 'Enter' || event.key === ' ') {
          event.preventDefault();
          onSelect(index);
        }
      }}
      style={{ cursor: disabledCell ? 'not-allowed' : 'pointer', outline: 'none' }}
    >
      {over && !active && !empty ? (
        <rect
          x={position.x - cfg.effects.cellHoverPad}
          y={position.y - cfg.effects.cellHoverPad}
          width={cellW + cfg.effects.cellHoverPad * 2}
          height={cellH + cfg.effects.cellHoverPad * 2}
          rx={cfg.radius.cell + cfg.effects.cellHoverPad}
          fill="none"
          stroke={statusColor}
          strokeWidth={cfg.stroke.cellHoverGlow}
          opacity={cfg.opacity.cellHoverGlow}
          filter={`url(#${ids.selectedGlow})`}
        />
      ) : null}
      <rect
        x={position.x}
        y={position.y}
        width={cellW}
        height={cellH}
        rx={cfg.radius.cell}
        fill={`url(#${ids.cell})`}
        opacity={
          status === 'unsupported'
            ? cfg.opacity.unsupportedCell
            : status === 'empty'
              ? cfg.opacity.emptyCell
              : active
                ? 0
                : press
                  ? cfg.opacity.cellPress
                  : over
                    ? cfg.opacity.cellHover
                    : cfg.opacity.cellFill
        }
      />
      <rect
        x={position.x}
        y={position.y}
        width={cellW}
        height={cellH}
        rx={cfg.radius.cell}
        fill="none"
        stroke={empty ? statusColor : over ? cfg.colors.cellEdgeHover : statusColor}
        strokeWidth={cfg.stroke.cell}
        opacity={empty ? 0.32 : over ? 0.9 : 0.66}
      />
      <rect
        x={position.x + cfg.effects.cellInnerInset}
        y={position.y + cfg.effects.cellInnerInset}
        width={cellW - cfg.effects.cellInnerInset * 2}
        height={cellH - cfg.effects.cellInnerInset * 2}
        rx={Math.max(0, cfg.radius.cell - cfg.effects.cellInnerInset)}
        fill="none"
        stroke={cfg.colors.cellInner}
        strokeWidth={cfg.stroke.cellInner}
        opacity="0.28"
      />
      {!empty ? (
        <circle
          cx={position.x + cellW - cfg.effects.statusDotInset}
          cy={position.y + cfg.effects.statusDotInset}
          r={cfg.effects.statusDotR}
          fill={statusColor}
          opacity={cfg.opacity.statusDot}
        />
      ) : null}
      {active ? (
        <>
          <rect
            x={position.x + cfg.effects.selectedInset}
            y={position.y + cfg.effects.selectedInset}
            width={cellW - cfg.effects.selectedInset * 2}
            height={cellH - cfg.effects.selectedInset * 2}
            rx={cfg.radius.selected}
            fill={`url(#${ids.selected})`}
            stroke={cfg.colors.selectedEdge}
            strokeWidth={cfg.stroke.selected}
            filter={`url(#${ids.selectedGlow})`}
          />
          <rect
            x={position.x + cfg.effects.selectedInset}
            y={position.y + cfg.effects.selectedInset}
            width={cellW - cfg.effects.selectedInset * 2}
            height={cellH - cfg.effects.selectedInset * 2}
            rx={cfg.radius.selected}
            fill={`url(#${ids.shine})`}
            opacity={shineOpacity}
          />
          <line
            x1={position.x + cfg.effects.selectedHighlightInsetX}
            y1={position.y + cfg.effects.selectedHighlightY}
            x2={position.x + cellW - cfg.effects.selectedHighlightInsetX}
            y2={position.y + cfg.effects.selectedHighlightY}
            stroke={cfg.colors.shine}
            strokeWidth={cfg.effects.selectedHighlightStroke}
            strokeLinecap="round"
            opacity={cfg.effects.selectedHighlightOpacity}
          />
        </>
      ) : null}
      {showLabel ? (
        <text
          x={labelX}
          y={position.y + cellH / 2 + cfg.text.optionSize * 0.35}
          textAnchor="middle"
          fill={
            empty || status === 'unsupported'
              ? cfg.colors.mutedText
              : active
                ? cfg.colors.selectedText
                : cfg.colors.idleText
          }
          fontFamily={cfg.text.font}
          fontSize={cfg.text.optionSize}
          fontWeight={cfg.text.optionWeight}
          pointerEvents="none"
          textLength={fittedLabelW}
          lengthAdjust={fittedLabelW ? 'spacingAndGlyphs' : undefined}
        >
          {item.label}
        </text>
      ) : null}
      {showAdd ? (
        <g
          role="button"
          aria-label={`Add ${item.label} to Parent Portal`}
          onClick={(event) => {
            event.stopPropagation();
            onAddToPortal(item);
          }}
          style={{ cursor: 'pointer' }}
        >
          <rect
            x={
              position.x + cellW - cfg.layout.addButtonSize - cfg.layout.addButtonInset - cfg.layout.addButtonCutoutPad
            }
            y={
              position.y + cellH - cfg.layout.addButtonSize - cfg.layout.addButtonInset - cfg.layout.addButtonCutoutPad
            }
            width={cfg.layout.addButtonSize + cfg.layout.addButtonCutoutPad * 2}
            height={cfg.layout.addButtonSize + cfg.layout.addButtonCutoutPad * 2}
            rx={cfg.radius.addButtonCutout}
            fill={cfg.colors.addButtonCutout}
            opacity={cfg.opacity.addButtonCutout}
          />
          <rect
            x={position.x + cellW - cfg.layout.addButtonSize - cfg.layout.addButtonInset}
            y={position.y + cellH - cfg.layout.addButtonSize - cfg.layout.addButtonInset}
            width={cfg.layout.addButtonSize}
            height={cfg.layout.addButtonSize}
            rx={cfg.radius.addButton}
            fill={cfg.colors.addButtonGlow}
            opacity={cfg.opacity.addButtonGlow}
            filter={`url(#${ids.selectedGlow})`}
          />
          <rect
            x={position.x + cellW - cfg.layout.addButtonSize - cfg.layout.addButtonInset}
            y={position.y + cellH - cfg.layout.addButtonSize - cfg.layout.addButtonInset}
            width={cfg.layout.addButtonSize}
            height={cfg.layout.addButtonSize}
            rx={cfg.radius.addButton}
            fill={cfg.colors.addButton}
            opacity={cfg.opacity.addButton}
            stroke={cfg.colors.addButtonEdge}
            strokeWidth={cfg.stroke.addButton}
          />
          <rect
            x={
              position.x +
              cellW -
              cfg.layout.addButtonSize -
              cfg.layout.addButtonInset +
              cfg.effects.addButtonInnerInset
            }
            y={
              position.y +
              cellH -
              cfg.layout.addButtonSize -
              cfg.layout.addButtonInset +
              cfg.effects.addButtonInnerInset
            }
            width={cfg.layout.addButtonSize - cfg.effects.addButtonInnerInset * 2}
            height={cfg.layout.addButtonSize - cfg.effects.addButtonInnerInset * 2}
            rx={Math.max(0, cfg.radius.addButton - cfg.effects.addButtonInnerInset)}
            fill="none"
            stroke={cfg.colors.shine}
            strokeWidth={cfg.effects.addButtonInnerStroke}
            opacity={cfg.effects.addButtonInnerOpacity}
          />
          <text
            x={position.x + cellW - cfg.layout.addButtonInset - cfg.layout.addButtonSize / 2}
            y={position.y + cellH - cfg.layout.addButtonInset - cfg.layout.addButtonSize * 0.24}
            textAnchor="middle"
            fill={cfg.colors.addButtonText}
            fontFamily={cfg.text.font}
            fontSize={cfg.text.addSize}
            fontWeight={cfg.text.titleWeight}
            pointerEvents="none"
          >
            +
          </text>
        </g>
      ) : null}
    </g>
  );
}
