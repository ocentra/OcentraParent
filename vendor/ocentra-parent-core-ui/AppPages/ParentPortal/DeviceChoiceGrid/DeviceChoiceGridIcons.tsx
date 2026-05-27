import type { ReactElement } from 'react';
import type { DeviceKind, DevicePlatformKind, DeviceSlot } from './DeviceChoiceGridTypes';

const DEVICE_PLATFORM_ICON_HREFS: Record<Exclude<DevicePlatformKind, 'unknown'>, string> = {
  android: '/images/android.png',
  ios: '/images/mactablet.png',
  linux: '/images/linuxlogo.png',
  macos: '/images/mac.png',
  router: '/images/router.png',
  windows: '/images/windowslogo.png',
};

export function getDeviceKind(slot: DeviceSlot): DeviceKind {
  return slot.device?.type ?? 'unknown';
}

export function getDevicePlatform(slot: DeviceSlot): DevicePlatformKind {
  if (slot.platform) {
    return slot.platform;
  }
  if (slot.device?.platform) {
    return slot.device.platform;
  }
  if (slot.device?.type === 'router') {
    return 'router';
  }
  return 'unknown';
}

export function getDevicePlatformIconHref(slot: DeviceSlot): string | null {
  const platform = getDevicePlatform(slot);
  return platform === 'unknown' ? null : DEVICE_PLATFORM_ICON_HREFS[platform];
}

export function DevicePlatformImage({
  slot,
  x,
  y,
  size,
  opacity = 1,
}: {
  slot: DeviceSlot;
  x: number;
  y: number;
  size: number;
  opacity?: number;
}): ReactElement | null {
  const href = getDevicePlatformIconHref(slot);
  if (!href) {
    return null;
  }

  return (
    <image
      href={href}
      x={x}
      y={y}
      width={size}
      height={size}
      preserveAspectRatio="xMidYMid meet"
      opacity={opacity}
      pointerEvents="none"
    />
  );
}

export function DeviceKindIcon({
  kind,
  x,
  y,
  size,
  color,
}: {
  kind: DeviceKind;
  x: number;
  y: number;
  size: number;
  color: string;
}): ReactElement {
  const midX = x + size / 2;
  const midY = y + size / 2;
  const s = size;

  if (kind === 'mobile') {
    return (
      <>
        <rect
          x={x + s * 0.28}
          y={y + s * 0.1}
          width={s * 0.44}
          height={s * 0.8}
          rx={s * 0.12}
          fill="none"
          stroke={color}
          strokeWidth="1.35"
        />
        <rect
          x={x + s * 0.36}
          y={y + s * 0.18}
          width={s * 0.28}
          height={s * 0.56}
          rx={s * 0.035}
          fill="none"
          stroke={color}
          strokeWidth="0.75"
          opacity="0.72"
        />
        <line
          x1={x + s * 0.43}
          y1={y + s * 0.8}
          x2={x + s * 0.57}
          y2={y + s * 0.8}
          stroke={color}
          strokeWidth="1"
          strokeLinecap="round"
        />
      </>
    );
  }

  if (kind === 'tablet') {
    return (
      <rect
        x={x + s * 0.2}
        y={y + s * 0.16}
        width={s * 0.6}
        height={s * 0.68}
        rx={s * 0.08}
        fill="none"
        stroke={color}
        strokeWidth="1.35"
      />
    );
  }

  if (kind === 'laptop') {
    return (
      <>
        <rect
          x={x + s * 0.18}
          y={y + s * 0.2}
          width={s * 0.64}
          height={s * 0.44}
          rx={s * 0.04}
          fill="none"
          stroke={color}
          strokeWidth="1.25"
        />
        <path
          d={`M${x + s * 0.08} ${y + s * 0.78}H${x + s * 0.92}L${x + s * 0.8} ${y + s * 0.66}H${x + s * 0.2}Z`}
          fill="none"
          stroke={color}
          strokeWidth="1.25"
        />
      </>
    );
  }

  if (kind === 'desktop') {
    return (
      <>
        <rect
          x={x + s * 0.16}
          y={y + s * 0.16}
          width={s * 0.68}
          height={s * 0.48}
          rx={s * 0.04}
          fill="none"
          stroke={color}
          strokeWidth="1.25"
        />
        <path
          d={`M${midX} ${y + s * 0.64}V${y + s * 0.78}M${x + s * 0.34} ${y + s * 0.8}H${x + s * 0.66}`}
          stroke={color}
          strokeWidth="1.25"
          strokeLinecap="round"
        />
      </>
    );
  }

  if (kind === 'router') {
    return (
      <>
        <rect
          x={x + s * 0.16}
          y={y + s * 0.46}
          width={s * 0.68}
          height={s * 0.28}
          rx={s * 0.08}
          fill="none"
          stroke={color}
          strokeWidth="1.25"
        />
        <path
          d={`M${x + s * 0.3} ${y + s * 0.42}Q${midX} ${y + s * 0.16} ${x + s * 0.7} ${y + s * 0.42}`}
          fill="none"
          stroke={color}
          strokeWidth="1.15"
          strokeLinecap="round"
        />
      </>
    );
  }

  return (
    <>
      <circle cx={midX} cy={midY} r={s * 0.28} fill="none" stroke={color} strokeWidth="1.25" />
      <path
        d={`M${midX} ${y + s * 0.28}V${midY}H${x + s * 0.7}`}
        stroke={color}
        strokeWidth="1.15"
        strokeLinecap="round"
      />
    </>
  );
}
