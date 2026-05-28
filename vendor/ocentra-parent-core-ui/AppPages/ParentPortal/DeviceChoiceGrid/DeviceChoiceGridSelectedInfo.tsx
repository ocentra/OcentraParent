import type { ReactElement } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import type { DeviceChoiceGridIds, DeviceSlot } from './DeviceChoiceGridTypes';
import { DeviceKindIcon, DevicePlatformImage, getDeviceKind, getDevicePlatformIconHref } from './DeviceChoiceGridIcons';

type DeviceChoiceGridSelectedInfoProps = {
  cfg: DeviceChoiceGridConfig;
  ids: DeviceChoiceGridIds;
  infoX: number;
  infoY: number;
  infoW: number;
  selected: DeviceSlot | null;
};

export function DeviceChoiceGridSelectedInfo({
  cfg,
  ids,
  infoX,
  infoY,
  infoW,
  selected,
}: DeviceChoiceGridSelectedInfoProps): ReactElement {
  const infoH = cfg.layout.selectedInfoH;
  const iconX = infoX + cfg.layout.selectedInfoPadX;
  const iconY = infoY + (infoH - cfg.layout.selectedInfoIconBox) / 2;
  const platformIconHref = selected ? getDevicePlatformIconHref(selected) : null;
  const iconSize = cfg.layout.selectedInfoIconBox;
  const textColor =
    !selected || selected.status === 'empty' || selected.status === 'unsupported'
      ? cfg.colors.selectedInfoMuted
      : cfg.colors.selectedInfoText;
  const infoText = selected
    ? `${cfg.text.selectedInfoLabel} : ${selected.label || cfg.text.legend[selected.status]}`
    : `${cfg.text.selectedInfoLabel} : ${cfg.text.selectedInfoEmptyLabel}`;

  return (
    <g pointerEvents="none">
      <rect
        x={infoX}
        y={infoY}
        width={infoW}
        height={infoH}
        fill={cfg.colors.selectedInfoGlow}
        opacity={cfg.opacity.selectedInfoGlow}
        filter={`url(#${ids.selectedGlow})`}
      />
      <rect
        x={infoX}
        y={infoY}
        width={infoW}
        height={infoH}
        fill={cfg.colors.selectedInfoFill}
        stroke={cfg.colors.selectedInfoEdge}
        strokeWidth={cfg.stroke.selectedInfo}
        opacity={cfg.opacity.selectedInfo}
      />
      {selected && platformIconHref ? (
        <DevicePlatformImage slot={selected} x={iconX} y={iconY} size={iconSize} opacity={0.96} />
      ) : selected ? (
        <DeviceKindIcon
          kind={getDeviceKind(selected)}
          x={iconX}
          y={iconY}
          size={iconSize}
          color={cfg.colors.selectedInfoText}
        />
      ) : null}
      <text
        x={iconX + iconSize + cfg.layout.selectedInfoIconGap}
        y={infoY + infoH * 0.65}
        fill={textColor}
        fontFamily={cfg.text.font}
        fontSize={cfg.text.selectedInfoSize}
        fontWeight={cfg.text.optionWeight}
      >
        {infoText}
      </text>
    </g>
  );
}
