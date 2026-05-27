import { useId, useMemo, useState, type CSSProperties, type ReactElement } from 'react';
import { defaultGridToggleConfig, mergeGridToggleConfig } from './GridToggleConfig';
import { GridToggleCells } from './GridToggleCells';
import { GridToggleDefs } from './GridToggleDefs';
import { GridToggleDividers } from './GridToggleDividers';
import { GridToggleFrame } from './GridToggleFrame';
import { clampGridToggleInt } from './GridToggleGeometry';
import {
  calculateGridToggleMetrics,
  calculateGridTogglePaths,
  calculateGridToggleSelection,
} from './GridToggleMetrics';
import { getSelectedGridToggleIndex, normalizeGridToggleOptions } from './GridToggleOptions';
import { GridToggleSelectedCell } from './GridToggleSelectedCell';
import type { GridToggleIds, GridToggleOption, GridToggleProps } from './GridToggleTypes';

export function GridToggle({
  value,
  defaultValue,
  title,
  options,
  rows,
  columns,
  disabled = false,
  className,
  style,
  onChange,
  config: configOverride,
}: GridToggleProps): ReactElement {
  const [isHovering, setIsHovering] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const [hoveredIndex, setHoveredIndex] = useState<number | null>(null);
  const [pressedIndex, setPressedIndex] = useState<number | null>(null);
  const rawId = useId();
  const uid = rawId.replace(/[^a-zA-Z0-9_-]/g, '');
  const config = useMemo(() => mergeGridToggleConfig(defaultGridToggleConfig, configOverride), [configOverride]);
  const rowCount = clampGridToggleInt(rows ?? config.layout.rows, 1, config.layout.maxRows);
  const columnCount = clampGridToggleInt(columns ?? config.layout.columns, 1, config.layout.maxColumns);
  const totalCells = rowCount * columnCount;
  const normalizedOptions = useMemo(
    () => normalizeGridToggleOptions(options, totalCells, config.text.options),
    [options, totalCells, config.text.options]
  );
  const fallbackValue = defaultValue ?? normalizedOptions[0].value;
  const [internalValue, setInternalValue] = useState<string>(fallbackValue);
  const selectedValue = value ?? internalValue;
  const selectedIndex = getSelectedGridToggleIndex(selectedValue, normalizedOptions);
  const selectedOption = normalizedOptions[selectedIndex] ?? normalizedOptions[0];
  const titleText = title ?? config.text.title;
  const metrics = calculateGridToggleMetrics(config, titleText, normalizedOptions, rowCount, columnCount);
  const selection = calculateGridToggleSelection(config, metrics, selectedIndex);
  const paths = calculateGridTogglePaths(config, metrics, selection);
  const ids = gridToggleIds(uid);

  const glowOpacity = isHovering ? config.opacity.gridGlowHover : config.opacity.gridGlowIdle;
  const selectedGlowOpacity = isHovering ? config.opacity.selectedGlowHover : config.opacity.selectedGlowIdle;
  const shineOpacity = isHovering ? config.opacity.shineHover : config.opacity.shineIdle;
  const selectedGlossOpacity = isHovering ? config.opacity.selectedGlossHover : config.opacity.selectedGlossIdle;
  const titleGlowOpacity = isHovering ? config.opacity.titleGlowHover : config.opacity.titleGlowIdle;
  const outerGlowOpacity = isHovering ? config.opacity.outerGlowHover : config.opacity.outerGlowIdle;
  const dividerGlowOpacity = isHovering ? config.opacity.dividerGlowHover : config.opacity.dividerGlowIdle;
  const scale = disabled ? 1 : isPressed ? config.hover.pressScale : 1;

  const rootStyle: CSSProperties = {
    width: metrics.svgWidth,
    height: metrics.svgHeight,
    opacity: disabled ? config.opacity.disabled : 1,
    transform: `scale(${scale})`,
    transformOrigin: 'center',
    transition: config.transition.root,
    cursor: disabled ? 'not-allowed' : 'default',
    ...style,
  };
  const svgStyle: CSSProperties = {
    transition: config.transition.svg,
  };

  const setSelected = (nextValue: string) => {
    if (disabled) {
      return;
    }
    const nextIndex = getSelectedGridToggleIndex(nextValue, normalizedOptions);
    const nextOption = normalizedOptions[nextIndex] ?? normalizedOptions[0];
    const nextRow = Math.floor(nextIndex / metrics.columnCount);
    const nextColumn = nextIndex % metrics.columnCount;
    if (value === undefined) {
      setInternalValue(nextValue);
    }
    onChange?.(nextValue, nextOption, nextIndex, nextRow, nextColumn);
  };

  return (
    <div
      className={className}
      style={rootStyle}
      onPointerEnter={() => setIsHovering(true)}
      onPointerLeave={() => {
        setIsHovering(false);
        setIsPressed(false);
        setHoveredIndex(null);
        setPressedIndex(null);
      }}
      onPointerDown={() => setIsPressed(true)}
      onPointerUp={() => setIsPressed(false)}
      onPointerCancel={() => setIsPressed(false)}
    >
      <svg
        viewBox={`${-config.svg.viewportInset} ${-config.svg.viewportInset} ${metrics.svgWidth + config.svg.viewportInset * 2} ${metrics.svgHeight + config.svg.viewportInset * 2}`}
        width={metrics.svgWidth}
        height={metrics.svgHeight}
        role="img"
        aria-label={`${titleText}: ${selectedOption.label}`}
      >
        <GridToggleDefs
          config={config}
          dividerGlowOpacity={dividerGlowOpacity}
          glowOpacity={glowOpacity}
          ids={ids}
          metrics={metrics}
          outerGlowOpacity={outerGlowOpacity}
          selectedGlowOpacity={selectedGlowOpacity}
          selection={selection}
          titleGlowOpacity={titleGlowOpacity}
        />
        <GridToggleFrame
          config={config}
          glowOpacity={glowOpacity}
          ids={ids}
          isHovering={isHovering}
          metrics={metrics}
          outerGlowOpacity={outerGlowOpacity}
          paths={paths}
          svgStyle={svgStyle}
          titleText={titleText}
          titleGlowOpacity={titleGlowOpacity}
        />
        <GridToggleDividers
          config={config}
          dividerGlowOpacity={dividerGlowOpacity}
          ids={ids}
          metrics={metrics}
          svgStyle={svgStyle}
        />
        <GridToggleSelectedCell
          config={config}
          ids={ids}
          paths={paths}
          selectedGlossOpacity={selectedGlossOpacity}
          selection={selection}
          shineOpacity={shineOpacity}
          svgStyle={svgStyle}
        />
        <GridToggleCells
          config={config}
          disabled={disabled}
          hoveredIndex={hoveredIndex}
          metrics={metrics}
          options={normalizedOptions}
          pressedIndex={pressedIndex}
          selectedIndex={selectedIndex}
          svgStyle={svgStyle}
          onHoverChange={setHoveredIndex}
          onPressChange={setPressedIndex}
          onSelect={setSelected}
        />
      </svg>
    </div>
  );
}

export const GridToggleSvg = GridToggle;

function gridToggleIds(uid: string): GridToggleIds {
  return {
    dividerGlow: `${uid}-dividerGlow`,
    grid: `${uid}-grid`,
    gridGlow: `${uid}-gridGlow`,
    outerGlow: `${uid}-outerGlow`,
    selected: `${uid}-selected`,
    selectedBottomGloss: `${uid}-selectedBottomGloss`,
    selectedGlow: `${uid}-selectedGlow`,
    selectedShine: `${uid}-selectedShine`,
    shadow: `${uid}-shadow`,
    titleGlow: `${uid}-titleGlow`,
  };
}
