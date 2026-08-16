import type { ReactElement } from 'react';
import { LanNetworkMonitorsIcon, PortalGatewayIcon } from '../../../Common/NavSvgIcons/ParentNavSvgIcons';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import {
  DEVICE_CHOICE_DEFAULT_SCOPE_VALUES,
  type DeviceChoiceGridIds,
  type DeviceChoiceGridScopeIcon,
  type ScopeValue,
} from './DeviceChoiceGridTypes';

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
  scopeValues?: readonly ScopeValue[];
  scopeIcons?: Partial<Record<ScopeValue, DeviceChoiceGridScopeIcon>>;
};

const DEFAULT_SCOPE_ICONS: Record<ScopeValue, DeviceChoiceGridScopeIcon> = {
  lan: { render: LanNetworkMonitorsIcon },
  parent: { render: PortalGatewayIcon },
  portal: { render: PortalGatewayIcon },
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
  scopeValues = DEVICE_CHOICE_DEFAULT_SCOPE_VALUES,
  scopeIcons,
}: DeviceChoiceGridScopeSelectorProps): ReactElement {
  const titleGlowOpacity = hover ? cfg.opacity.titleGlowHover : cfg.opacity.titleGlow;
  const activeScopeValues = scopeValues.length > 0 ? scopeValues : DEVICE_CHOICE_DEFAULT_SCOPE_VALUES;

  return (
    <g aria-hidden="true" onClick={(event) => event.stopPropagation()}>
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
      {activeScopeValues.slice(1).map((scopeValue, index) => {
        const dividerX = titleX + scopeOptionW * (index + 1);
        return (
          <line
            key={`scope-divider:${scopeValue}`}
            x1={dividerX}
            y1={cfg.layout.titleY + 7}
            x2={dividerX}
            y2={cfg.layout.titleY + cfg.layout.titleH - 7}
            stroke={cfg.colors.titleEdge}
            strokeWidth="0.8"
            opacity="0.65"
          />
        );
      })}
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
      {activeScopeValues.map((scopeValue, index) => {
        const icon = scopeIcons?.[scopeValue] ?? DEFAULT_SCOPE_ICONS[scopeValue];
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
          <g key={scopeValue} style={{ cursor: disabled ? 'not-allowed' : 'pointer', outline: 'none' }}>
            <rect
              x={optionX}
              y={cfg.layout.titleY}
              width={scopeOptionW}
              height={cfg.layout.titleH}
              fill="transparent"
            />
            {renderScopeIcon(icon, {
              x: iconX,
              y: iconY,
              width: iconSize,
              height: iconSize,
              scope: scopeValue,
              selected,
            })}
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

function renderScopeIcon(
  icon: DeviceChoiceGridScopeIcon,
  props: {
    x: number;
    y: number;
    width: number;
    height: number;
    scope: ScopeValue;
    selected: boolean;
  }
): ReactElement {
  if ('href' in icon) {
    return (
      <image
        href={icon.href}
        x={props.x}
        y={props.y}
        width={props.width}
        height={props.height}
        preserveAspectRatio="xMidYMid meet"
        pointerEvents="none"
      />
    );
  }

  if (icon.foreignObject) {
    return (
      <foreignObject x={props.x} y={props.y} width={props.width} height={props.height} pointerEvents="none">
        {icon.render(props)}
      </foreignObject>
    );
  }

  return icon.render(props);
}
