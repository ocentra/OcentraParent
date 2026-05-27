import type { ReactElement } from 'react';
import { LanNetworkMonitorsIcon, PortalGatewayIcon } from '../../../Common/NavSvgIcons';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import type { DeviceChoiceGridIds, ScopeValue } from './DeviceChoiceGridTypes';

type DeviceChoiceGridScopeSelectorProps = {
  cfg: DeviceChoiceGridConfig;
  currentScope: ScopeValue;
  disabled: boolean;
  hover: boolean;
  ids: DeviceChoiceGridIds;
  scopeOptionW: number;
  scopeSliderX: number;
  titleW: number;
  titleX: number;
  onScopeSelect: (scope: ScopeValue) => void;
};

const SCOPE_ICONS = {
  lan: LanNetworkMonitorsIcon,
  parent: PortalGatewayIcon,
} as const;

function estimateScopeLabelWidth(label: string, size: number): number {
  return Math.ceil(label.length * size * 0.58);
}

export function DeviceChoiceGridScopeSelector({
  cfg,
  currentScope,
  disabled,
  hover,
  ids,
  scopeOptionW,
  scopeSliderX,
  titleW,
  titleX,
  onScopeSelect,
}: DeviceChoiceGridScopeSelectorProps): ReactElement {
  const titleGlowOpacity = hover ? cfg.opacity.titleGlowHover : cfg.opacity.titleGlow;

  return (
    <g role="group" aria-label="Scope selector" onClick={(event) => event.stopPropagation()}>
      <rect
        x={titleX}
        y={cfg.layout.titleY}
        width={titleW}
        height={cfg.layout.titleH}
        rx={cfg.radius.title}
        fill="transparent"
        stroke={cfg.colors.titleGlow}
        strokeWidth={cfg.stroke.titleGlow}
        opacity={titleGlowOpacity}
        filter={`url(#${ids.titleGlow})`}
      />
      <rect
        x={titleX}
        y={cfg.layout.titleY}
        width={titleW}
        height={cfg.layout.titleH}
        rx={cfg.radius.title}
        fill="rgba(255,255,255,0.06)"
        stroke={hover ? cfg.colors.titleEdgeHover : cfg.colors.titleEdge}
        strokeWidth={cfg.stroke.title}
      />
      <line
        x1={titleX + scopeOptionW}
        y1={cfg.layout.titleY + 7}
        x2={titleX + scopeOptionW}
        y2={cfg.layout.titleY + cfg.layout.titleH - 7}
        stroke={cfg.colors.titleEdge}
        strokeWidth="0.8"
        opacity="0.65"
      />
      <rect
        x={scopeSliderX}
        y={cfg.layout.titleY + cfg.layout.scopeInset}
        width={scopeOptionW - cfg.layout.scopeInset * 2}
        height={cfg.layout.titleH - cfg.layout.scopeInset * 2}
        rx={cfg.radius.selected}
        fill={`url(#${ids.selected})`}
        stroke={cfg.colors.selectedEdge}
        strokeWidth={cfg.stroke.selected}
        filter={`url(#${ids.selectedGlow})`}
      />
      {(['lan', 'parent'] as ScopeValue[]).map((scopeValue, index) => {
        const Icon = SCOPE_ICONS[scopeValue];
        const label = cfg.text.scopeOptions[scopeValue];
        const optionX = titleX + index * scopeOptionW;
        const optionCenterX = optionX + scopeOptionW / 2;
        const iconSize = cfg.layout.scopeIconSize;
        const labelW = estimateScopeLabelWidth(label, cfg.text.optionSize);
        const groupW = iconSize + cfg.layout.scopeIconGap + labelW;
        const iconX = optionCenterX - groupW / 2;
        const iconY = cfg.layout.titleY + (cfg.layout.titleH - iconSize) / 2;
        const labelX = iconX + iconSize + cfg.layout.scopeIconGap + labelW / 2;
        const selected = currentScope === scopeValue;

        return (
          <g
            key={scopeValue}
            role="button"
            tabIndex={disabled ? -1 : 0}
            aria-label={`Select ${label}`}
            onClick={(event) => {
              event.stopPropagation();
              if (!disabled) {
                onScopeSelect(scopeValue);
              }
            }}
            onKeyDown={(event) => {
              if (event.key === 'Enter' || event.key === ' ') {
                event.preventDefault();
                if (!disabled) {
                  onScopeSelect(scopeValue);
                }
              }
            }}
            style={{ cursor: disabled ? 'not-allowed' : 'pointer', outline: 'none' }}
          >
            <rect
              x={optionX}
              y={cfg.layout.titleY}
              width={scopeOptionW}
              height={cfg.layout.titleH}
              fill="transparent"
            />
            <Icon x={iconX} y={iconY} width={iconSize} height={iconSize} />
            <text
              x={labelX}
              y={cfg.layout.titleY + cfg.layout.titleH * 0.64}
              textAnchor="middle"
              fill={selected ? cfg.colors.scopeSelectedText : cfg.colors.scopeIdleText}
              fontFamily={cfg.text.font}
              fontSize={cfg.text.optionSize}
              fontWeight={cfg.text.optionWeight}
              pointerEvents="none"
            >
              {label}
            </text>
          </g>
        );
      })}
    </g>
  );
}
