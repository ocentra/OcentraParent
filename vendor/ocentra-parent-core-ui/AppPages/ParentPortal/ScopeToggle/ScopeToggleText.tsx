import type { CSSProperties, ReactElement } from 'react';
import type { ScopeToggleConfig } from './ScopeToggleConfig';
import { estimateScopeToggleTextWidth } from './ScopeToggleGeometry';
import type { ScopeToggleMetrics, ScopeToggleOption, ScopeToggleTitleRenderer } from './ScopeToggleTypes';

type ScopeToggleTextProps = {
  config: ScopeToggleConfig;
  disabled: boolean;
  metrics: ScopeToggleMetrics;
  options: ScopeToggleOption[];
  selectedIndex: number;
  svgStyle: CSSProperties;
  titleText: string;
  titleRenderer?: ScopeToggleTitleRenderer;
  onOptionSelect: (option: ScopeToggleOption, index: number) => void;
};

export function ScopeToggleText({
  config,
  disabled,
  metrics,
  options,
  selectedIndex,
  svgStyle,
  titleText,
  titleRenderer,
  onOptionSelect,
}: ScopeToggleTextProps): ReactElement {
  const optionCenterY = metrics.trackY + metrics.trackHeight * 0.5 + config.text.optionFontSize * 0.35;
  const titleSlot = {
    x: metrics.titleBoxX,
    y: metrics.titleBoxY,
    width: metrics.titleBoxWidth,
    height: config.layout.titleBoxHeight,
    centerX: metrics.titleCenterX,
    centerY: metrics.titleBoxY + config.layout.titleBoxHeight * 0.5,
  };

  return (
    <>
      {titleRenderer ? (
        <g pointerEvents="none" style={svgStyle}>
          {titleRenderer(titleSlot)}
        </g>
      ) : (
        <text
          x={metrics.titleCenterX}
          y={metrics.titleBoxY + config.layout.titleBoxHeight * 0.64}
          textAnchor="middle"
          fill={config.colors.title}
          fontFamily={config.text.fontFamily}
          fontSize={config.text.titleFontSize}
          fontWeight={config.text.fontWeight}
          style={svgStyle}
          pointerEvents="none"
        >
          {titleText}
        </text>
      )}
      {options.map((option, index) => {
        const optionCenterX =
          metrics.trackX + index * (metrics.optionWidth + config.layout.dividerWidth) + metrics.optionWidth * 0.5;
        const isSelected = index === selectedIndex;
        const textWidth = estimateScopeToggleTextWidth(option.label, config.text.optionFontSize);
        const iconSize = option.iconHref ? config.layout.optionIconSize : 0;
        const iconGap = option.iconHref ? config.layout.optionIconGap : 0;
        const contentWidth = iconSize + iconGap + textWidth;
        const iconX = optionCenterX - contentWidth * 0.5;
        const iconY = metrics.trackY + (metrics.trackHeight - iconSize) * 0.5;
        const textX = iconX + iconSize + iconGap + textWidth * 0.5;

        return (
          <g
            key={option.value}
            role="button"
            tabIndex={disabled ? -1 : 0}
            aria-label={`Select ${option.label}`}
            aria-disabled={disabled || undefined}
            onClick={(event) => {
              event.stopPropagation();
              onOptionSelect(option, index);
            }}
            onKeyDown={(event) => {
              if (event.key === 'Enter' || event.key === ' ') {
                event.preventDefault();
                onOptionSelect(option, index);
              }
            }}
            style={{ cursor: disabled ? 'not-allowed' : 'pointer', outline: 'none' }}
          >
            <rect
              x={metrics.trackX + index * (metrics.optionWidth + config.layout.dividerWidth)}
              y={metrics.trackY}
              width={metrics.optionWidth}
              height={metrics.trackHeight}
              fill="transparent"
            />
            {option.iconHref ? (
              <image
                href={option.iconHref}
                x={iconX}
                y={iconY}
                width={iconSize}
                height={iconSize}
                opacity={isSelected ? config.opacity.optionIconSelected : config.opacity.optionIconIdle}
                preserveAspectRatio="xMidYMid meet"
                pointerEvents="none"
                style={svgStyle}
              />
            ) : null}
            <text
              x={textX}
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
