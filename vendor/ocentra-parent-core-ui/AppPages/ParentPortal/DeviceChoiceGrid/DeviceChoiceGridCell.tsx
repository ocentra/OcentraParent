import type { ReactElement } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import type { DeviceChoiceGridCellPosition, DeviceChoiceGridIds, DeviceSlot } from './DeviceChoiceGridTypes';
import { DeviceKindIcon, DevicePlatformImage, getDeviceKind, getDevicePlatformIconHref } from './DeviceChoiceGridIcons';

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
  showEdit: boolean;
  onAddToPortal: (slot: DeviceSlot) => void;
  onEditDevice: (slot: DeviceSlot) => void;
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
  showEdit,
  index,
  onAddToPortal,
  onEditDevice,
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
  const badgeLabel = item.badge?.trim() ?? '';
  const showBadge = false;
  const showKindIcon = !empty;
  const showPlatformIcon = showKindIcon && getDevicePlatformIconHref(item) !== null;
  const iconSize = Math.max(14, Math.min(18, cellH * 0.36));
  const statusDotX = position.x + 13;
  const statusDotY = position.y + cellH / 2;
  const iconX = position.x + 24;
  const iconY = position.y + (cellH - iconSize) / 2;
  const iconColor = active ? cfg.colors.selectedText : statusColor;
  const labelInsetLeft = showKindIcon ? iconSize + 39 : 28;
  const estimatedLabelW = item.label.length * cfg.text.optionSize * 0.62;
  const estimatedBadgeW = badgeLabel.length * Math.max(8, cfg.text.optionSize * 0.72) * 0.62;
  const labelW = Math.max(24, cellW - labelInsetLeft - 14);
  const fittedLabelW = estimatedLabelW > labelW ? labelW : undefined;
  const fittedBadgeW = estimatedBadgeW > labelW ? labelW : undefined;
  const labelX = position.x + labelInsetLeft + labelW / 2;
  const labelY = position.y + cellH / 2 + cfg.text.optionSize * (showBadge && cellH >= 36 ? 0.02 : 0.32);
  const editButtonSize = Math.max(14, Math.min(17, cellH * 0.34));
  const editButtonX = position.x + cellW - editButtonSize - 4;
  const editButtonY = position.y + 4;

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
          cx={statusDotX}
          cy={statusDotY}
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
      {showKindIcon ? (
        showPlatformIcon ? (
          <DevicePlatformImage slot={item} x={iconX} y={iconY} size={iconSize} opacity={active ? 1 : 0.92} />
        ) : (
          <DeviceKindIcon kind={getDeviceKind(item)} x={iconX} y={iconY} size={iconSize} color={iconColor} />
        )
      ) : null}
      {showLabel ? (
        <text
          x={labelX}
          y={labelY}
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
      {showBadge ? (
        <text
          x={labelX}
          y={position.y + cellH - 7}
          textAnchor="middle"
          fill={active ? cfg.colors.selectedText : statusColor}
          fontFamily={cfg.text.font}
          fontSize={Math.max(8, cfg.text.optionSize * 0.72)}
          fontWeight={cfg.text.optionWeight}
          opacity={active ? 0.74 : 0.82}
          pointerEvents="none"
          textLength={fittedBadgeW}
          lengthAdjust={fittedBadgeW ? 'spacingAndGlyphs' : undefined}
        >
          {badgeLabel}
        </text>
      ) : null}
      {showEdit ? (
        <g
          role="button"
          aria-label={`Edit ${item.label}`}
          tabIndex={0}
          onClick={(event) => {
            event.stopPropagation();
            onEditDevice(item);
          }}
          onKeyDown={(event) => {
            if (event.key !== 'Enter' && event.key !== ' ') return;
            event.preventDefault();
            event.stopPropagation();
            onEditDevice(item);
          }}
          style={{ cursor: 'pointer' }}
        >
          <title>Edit device name and type</title>
          <rect
            x={editButtonX - 2}
            y={editButtonY - 2}
            width={editButtonSize + 4}
            height={editButtonSize + 4}
            rx={5}
            fill="rgba(2, 12, 22, 0.82)"
            stroke={active ? cfg.colors.selectedText : statusColor}
            strokeWidth={0.72}
            opacity={0.9}
          />
          <path
            d={`M${editButtonX + editButtonSize * 0.24} ${editButtonY + editButtonSize * 0.72}L${
              editButtonX + editButtonSize * 0.32
            } ${editButtonY + editButtonSize * 0.5}L${editButtonX + editButtonSize * 0.66} ${
              editButtonY + editButtonSize * 0.16
            }L${editButtonX + editButtonSize * 0.82} ${editButtonY + editButtonSize * 0.32}L${
              editButtonX + editButtonSize * 0.48
            } ${editButtonY + editButtonSize * 0.66}Z`}
            fill="none"
            stroke={active ? cfg.colors.selectedText : statusColor}
            strokeWidth={1.15}
            strokeLinejoin="round"
          />
          <path
            d={`M${editButtonX + editButtonSize * 0.2} ${editButtonY + editButtonSize * 0.78}H${
              editButtonX + editButtonSize * 0.56
            }`}
            stroke={active ? cfg.colors.selectedText : statusColor}
            strokeWidth={1.05}
            strokeLinecap="round"
          />
        </g>
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
