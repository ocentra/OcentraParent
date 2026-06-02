import type { ReactElement } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import type { DeviceChoiceGridIds, DeviceSlot } from './DeviceChoiceGridTypes';
import { DeviceKindIcon, DevicePlatformImage, getDeviceKind, getDevicePlatformIconHref } from './DeviceChoiceGridIcons';

const DEVICE_SELECTED_INFO_COPY = {
  AgentSource: 'Child agent',
  ControlOffline: 'Offline',
  ControlPending: 'Setup needed',
  ControlTarget: 'Policy target',
  ControlUnsupported: 'Unsupported for child policy',
  ControlVisibleOnly: 'Visible only',
  Empty: '',
  InfrastructureRole: 'Network infrastructure',
  LanSource: 'LAN neighbor',
  NotReported: 'Not reported',
  ServiceRole: 'Service state',
  Separator: ' | ',
} as const;

const DEVICE_TOKEN_SPLIT_PATTERN = /[-_\s]+/;
const DEVICE_SELECTED_INFO_TEXT_FACTOR = 0.58;

type DeviceSelectedInfoLines = {
  readonly title: string;
  readonly meta: string;
  readonly detail: string;
};

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
  const lines = selected ? selectedDeviceInfoLines(selected, cfg) : emptySelectedInfoLines(cfg);
  const textX = iconX + iconSize + cfg.layout.selectedInfoIconGap;
  const textW = Math.max(1, infoW - (textX - infoX) - cfg.layout.selectedInfoPadX);
  const primaryFontSize = cfg.text.selectedInfoSize;
  const secondaryFontSize = Math.max(9, primaryFontSize * 0.78);
  const titleTextLength = fitTextLength(lines.title, textW, primaryFontSize);
  const metaTextLength = fitTextLength(lines.meta, textW, secondaryFontSize);
  const detailTextLength = lines.detail ? fitTextLength(lines.detail, textW, secondaryFontSize) : undefined;

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
        x={textX}
        y={infoY + 18}
        fill={textColor}
        fontFamily={cfg.text.font}
        fontSize={primaryFontSize}
        fontWeight={cfg.text.optionWeight}
        textLength={titleTextLength}
        lengthAdjust={titleTextLength ? 'spacingAndGlyphs' : undefined}
      >
        {lines.title}
      </text>
      <text
        x={textX}
        y={infoY + 37}
        fill={textColor}
        fontFamily={cfg.text.font}
        fontSize={secondaryFontSize}
        fontWeight={620}
        opacity={0.82}
        textLength={metaTextLength}
        lengthAdjust={metaTextLength ? 'spacingAndGlyphs' : undefined}
      >
        {lines.meta}
      </text>
      {lines.detail ? (
        <text
          x={textX}
          y={infoY + 53}
          fill={textColor}
          fontFamily={cfg.text.font}
          fontSize={secondaryFontSize}
          fontWeight={580}
          opacity={0.68}
          textLength={detailTextLength}
          lengthAdjust={detailTextLength ? 'spacingAndGlyphs' : undefined}
        >
          {lines.detail}
        </text>
      ) : null}
    </g>
  );
}

function emptySelectedInfoLines(cfg: DeviceChoiceGridConfig): DeviceSelectedInfoLines {
  return {
    title: `${cfg.text.selectedInfoLabel}: ${cfg.text.selectedInfoEmptyLabel}`,
    meta: DEVICE_SELECTED_INFO_COPY.NotReported,
    detail: DEVICE_SELECTED_INFO_COPY.Empty,
  };
}

function selectedDeviceInfoLines(slot: DeviceSlot, cfg: DeviceChoiceGridConfig): DeviceSelectedInfoLines {
  const title = `${cfg.text.selectedInfoLabel}: ${slot.label || cfg.text.legend[slot.status]}`;
  return {
    title,
    meta: compactDeviceLine([deviceSourceLabel(slot), deviceControlLabel(slot), deviceNetworkLabel(slot)]),
    detail: compactDeviceLine([devicePlatformLabel(slot), deviceHardwareLabel(slot), deviceStateLabel(slot)]),
  };
}

function deviceSourceLabel(slot: DeviceSlot): string {
  if (!slot.device) return DEVICE_SELECTED_INFO_COPY.ServiceRole;
  if (deviceSlotHasAgent(slot)) return DEVICE_SELECTED_INFO_COPY.AgentSource;
  if (deviceSlotIsInfrastructure(slot)) return DEVICE_SELECTED_INFO_COPY.InfrastructureRole;
  return DEVICE_SELECTED_INFO_COPY.LanSource;
}

function deviceControlLabel(slot: DeviceSlot): string {
  if (deviceSlotIsInfrastructure(slot) || slot.status === 'unsupported') {
    return DEVICE_SELECTED_INFO_COPY.ControlUnsupported;
  }
  if (slot.status === 'offline') return DEVICE_SELECTED_INFO_COPY.ControlOffline;
  if (slot.status === 'connected' && deviceSlotHasAgent(slot)) return DEVICE_SELECTED_INFO_COPY.ControlTarget;
  if (deviceSlotHasAgent(slot)) return DEVICE_SELECTED_INFO_COPY.ControlPending;
  return DEVICE_SELECTED_INFO_COPY.ControlVisibleOnly;
}

function deviceNetworkLabel(slot: DeviceSlot): string {
  return firstDeviceText(slot.device?.ip, slot.device?.hostname, slot.device?.networkInterface);
}

function devicePlatformLabel(slot: DeviceSlot): string {
  return humanizeDeviceToken(slot.device?.platform ?? slot.platform);
}

function deviceHardwareLabel(slot: DeviceSlot): string {
  return firstDeviceText(slot.device?.cpuModel, slot.device?.gpuModel, slot.device?.model, slot.device?.manufacturer);
}

function deviceStateLabel(slot: DeviceSlot): string {
  return humanizeDeviceToken(slot.badge) || humanizeDeviceToken(slot.status);
}

function compactDeviceLine(parts: readonly string[]): string {
  return parts.filter((part) => part.trim().length > 0).join(DEVICE_SELECTED_INFO_COPY.Separator);
}

function firstDeviceText(...values: readonly (string | undefined)[]): string {
  return (
    values.find((value) => value !== undefined && value.trim().length > 0)?.trim() ?? DEVICE_SELECTED_INFO_COPY.Empty
  );
}

function deviceSlotHasAgent(slot: DeviceSlot): boolean {
  return Boolean(slot.device?.agentStatus?.trim());
}

function deviceSlotIsInfrastructure(slot: DeviceSlot): boolean {
  return slot.badge === 'infrastructure' || slot.device?.type === 'router' || slot.device?.platform === 'router';
}

function humanizeDeviceToken(value: string | undefined): string {
  const trimmed = value?.trim();
  if (!trimmed) return DEVICE_SELECTED_INFO_COPY.Empty;
  return trimmed
    .split(DEVICE_TOKEN_SPLIT_PATTERN)
    .filter((part) => part.length > 0)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function fitTextLength(value: string, maxWidth: number, fontSize: number): number | undefined {
  const estimate = value.length * fontSize * DEVICE_SELECTED_INFO_TEXT_FACTOR;
  return estimate > maxWidth ? maxWidth : undefined;
}
