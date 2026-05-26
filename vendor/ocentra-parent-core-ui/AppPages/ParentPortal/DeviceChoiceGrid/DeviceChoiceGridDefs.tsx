import type { ReactElement } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import type { DeviceChoiceGridIds } from './DeviceChoiceGridTypes';

type DeviceChoiceGridDefsProps = {
  cfg: DeviceChoiceGridConfig;
  ids: DeviceChoiceGridIds;
  outerGlowOpacity: number;
  selectedGlowOpacity: number;
  titleGlowOpacity: number;
};

export function DeviceChoiceGridDefs({
  cfg,
  ids,
  outerGlowOpacity,
  selectedGlowOpacity,
  titleGlowOpacity,
}: DeviceChoiceGridDefsProps): ReactElement {
  return (
    <defs>
      <linearGradient id={ids.cell} x1="0" y1="0" x2="0" y2="1">
        <stop offset={cfg.effects.gradientStart} stopColor={cfg.colors.cellFillA} />
        <stop offset={cfg.effects.gradientEnd} stopColor={cfg.colors.cellFillB} />
      </linearGradient>
      <linearGradient id={ids.selected} x1="0" y1="0" x2="0" y2="1">
        <stop offset={cfg.effects.gradientStart} stopColor={cfg.colors.selectedA} />
        <stop offset={cfg.effects.gradientEnd} stopColor={cfg.colors.selectedB} />
      </linearGradient>
      <linearGradient id={ids.shine} x1="0" y1="0" x2="0" y2="1">
        <stop offset={cfg.effects.shineStopA} stopColor={cfg.colors.shine} stopOpacity={cfg.effects.shineOpacityA} />
        <stop offset={cfg.effects.shineStopB} stopColor={cfg.colors.shine} stopOpacity={cfg.effects.shineOpacityB} />
        <stop offset={cfg.effects.shineStopC} stopColor={cfg.colors.shine} stopOpacity={cfg.effects.shineOpacityC} />
      </linearGradient>
      <filter
        id={ids.glow}
        x={cfg.effects.glowX}
        y={cfg.effects.glowY}
        width={cfg.effects.glowW}
        height={cfg.effects.glowH}
      >
        <feDropShadow
          dx="0"
          dy="0"
          stdDeviation={cfg.effects.glowBlur}
          floodColor={cfg.colors.outerGlow}
          floodOpacity={outerGlowOpacity}
        />
      </filter>
      <filter
        id={ids.titleGlow}
        x={cfg.effects.titleGlowX}
        y={cfg.effects.titleGlowY}
        width={cfg.effects.titleGlowW}
        height={cfg.effects.titleGlowH}
      >
        <feDropShadow
          dx="0"
          dy="0"
          stdDeviation={cfg.effects.titleGlowBlur}
          floodColor={cfg.colors.titleGlow}
          floodOpacity={titleGlowOpacity}
        />
      </filter>
      <filter
        id={ids.selectedGlow}
        x={cfg.effects.selectedGlowX}
        y={cfg.effects.selectedGlowY}
        width={cfg.effects.selectedGlowW}
        height={cfg.effects.selectedGlowH}
      >
        <feDropShadow
          dx="0"
          dy="0"
          stdDeviation={cfg.effects.selectedGlowBlur}
          floodColor={cfg.colors.selectedGlow}
          floodOpacity={selectedGlowOpacity}
        />
      </filter>
    </defs>
  );
}
