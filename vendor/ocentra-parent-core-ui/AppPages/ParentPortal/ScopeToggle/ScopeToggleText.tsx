import type { CSSProperties, ReactElement } from 'react';
import type { ScopeToggleConfig } from './ScopeToggleConfig';
import type { ScopeToggleMetrics, ScopeToggleOption } from './ScopeToggleTypes';

type ScopeToggleTextProps = {
  config: ScopeToggleConfig;
  disabled: boolean;
  metrics: ScopeToggleMetrics;
  options: ScopeToggleOption[];
  selectedIndex: number;
  svgStyle: CSSProperties;
  titleText: string;
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
  onOptionSelect,
}: ScopeToggleTextProps): ReactElement {
  const optionCenterY = metrics.trackY + metrics.trackHeight * 0.5 + config.text.optionFontSize * 0.35;

  return (
    <>
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
      {options.map((option, index) => {
        const optionCenterX =
          metrics.trackX + index * (metrics.optionWidth + config.layout.dividerWidth) + metrics.optionWidth * 0.5;
        const isSelected = index === selectedIndex;

        return (
          <g
            key={option.value}
            role="button"
            tabIndex={disabled ? -1 : 0}
            aria-label={`Select ${option.label}`}
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
