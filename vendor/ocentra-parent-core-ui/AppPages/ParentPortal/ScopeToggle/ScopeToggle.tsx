import { useId, useMemo, useState, type CSSProperties, type ReactElement } from 'react';
import { defaultScopeToggleConfig, mergeScopeToggleConfig } from './ScopeToggleConfig';
import { ScopeToggleDefs } from './ScopeToggleDefs';
import { ScopeToggleDividers } from './ScopeToggleDividers';
import { ScopeToggleFrame } from './ScopeToggleFrame';
import { calculateScopeToggleMetrics, calculateScopeTogglePaths } from './ScopeToggleMetrics';
import {
  getNextScopeToggleValue,
  getSelectedScopeToggleIndex,
  normalizeScopeToggleOptions,
} from './ScopeToggleOptions';
import { ScopeToggleSlider } from './ScopeToggleSlider';
import { ScopeToggleText } from './ScopeToggleText';
import type { ScopeToggleIds, ScopeToggleOption, ScopeToggleProps } from './ScopeToggleTypes';

export function ScopeToggle({
  x = 0,
  y = 0,
  renderMode = 'html',
  value,
  defaultValue,
  title,
  options,
  leftOption,
  rightOption,
  disabled = false,
  className,
  style,
  titleRenderer,
  onChange,
  config: configOverride,
}: ScopeToggleProps): ReactElement {
  const [isHovering, setIsHovering] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const rawId = useId();
  const uid = rawId.replace(/[^a-zA-Z0-9_-]/g, '');
  const config = useMemo(() => mergeScopeToggleConfig(defaultScopeToggleConfig, configOverride), [configOverride]);
  const normalizedOptions = useMemo(
    () => normalizeScopeToggleOptions(config, options, leftOption, rightOption),
    [config, options, leftOption, rightOption]
  );
  const firstOption = normalizedOptions[0] ?? { value: 'family', label: 'Family' };
  const fallbackValue = defaultValue ?? firstOption.value;
  const [internalValue, setInternalValue] = useState<string>(fallbackValue);
  const selectedValue = value ?? internalValue;
  const selectedIndex = getSelectedScopeToggleIndex(selectedValue, normalizedOptions);
  const selectedOption = normalizedOptions[selectedIndex] ?? firstOption;
  const titleText = title ?? config.text.title;
  const metrics = calculateScopeToggleMetrics(config, titleText, normalizedOptions);
  const sliderX =
    metrics.trackX + selectedIndex * (metrics.optionWidth + config.layout.dividerWidth) + config.slider.inset;
  const sliderY = metrics.trackY + config.slider.inset;
  const sliderWidth = metrics.optionWidth - config.slider.inset - config.slider.gapFromDivider;
  const sliderHeight = metrics.trackHeight - config.slider.inset * 2;
  const paths = calculateScopeTogglePaths(config, metrics, sliderX, sliderY, sliderWidth, sliderHeight);
  const ids = scopeToggleIds(uid);

  const glowOpacity = isHovering ? config.opacity.trackGlowHover : config.opacity.trackGlowIdle;
  const sliderGlowOpacity = isHovering ? config.opacity.sliderGlowHover : config.opacity.sliderGlowIdle;
  const shineOpacity = isHovering ? config.opacity.shineHover : config.opacity.shineIdle;
  const sliderGlossOpacity = isHovering ? config.opacity.sliderGlossHover : config.opacity.sliderGlossIdle;
  const titleGlowOpacity = isHovering ? config.opacity.titleGlowHover : config.opacity.titleGlowIdle;
  const outerGlowOpacity = isHovering ? config.opacity.outerGlowHover : config.opacity.outerGlowIdle;
  const dividerGlowOpacity = isHovering ? config.opacity.dividerGlowHover : config.opacity.dividerGlowIdle;
  const scale = disabled ? 1 : isPressed ? config.hover.pressScale : 1;

  const rootStyle: CSSProperties = {
    width: metrics.svgWidth,
    height: config.svg.height,
    opacity: disabled ? config.opacity.disabled : 1,
    transform: `scale(${scale})`,
    transformOrigin: 'center',
    transition: config.transition.root,
    cursor: disabled ? 'not-allowed' : 'pointer',
    ...style,
  };
  const svgStyle: CSSProperties = {
    transition: config.transition.svg,
  };

  const setSelected = (nextValue: string, option: ScopeToggleOption, index: number) => {
    if (disabled) {
      return;
    }
    if (value === undefined) {
      setInternalValue(nextValue);
    }
    onChange?.(nextValue, option, index);
  };
  const advanceSelected = () => {
    const nextValue = getNextScopeToggleValue(selectedValue, normalizedOptions);
    const nextIndex = getSelectedScopeToggleIndex(nextValue, normalizedOptions);
    setSelected(nextValue, normalizedOptions[nextIndex] ?? firstOption, nextIndex);
  };
  const svgContent = (
    <>
      <ScopeToggleDefs
        config={config}
        dividerGlowOpacity={dividerGlowOpacity}
        glowOpacity={glowOpacity}
        ids={ids}
        metrics={metrics}
        outerGlowOpacity={outerGlowOpacity}
        sliderGlowOpacity={sliderGlowOpacity}
        sliderGloss={{ x: sliderX, y: sliderY, width: sliderWidth, height: sliderHeight }}
        titleGlowOpacity={titleGlowOpacity}
      />
      <ScopeToggleFrame
        config={config}
        glowOpacity={glowOpacity}
        ids={ids}
        isHovering={isHovering}
        metrics={metrics}
        outerGlowOpacity={outerGlowOpacity}
        paths={paths}
        svgStyle={svgStyle}
        titleGlowOpacity={titleGlowOpacity}
      />
      <ScopeToggleDividers
        config={config}
        dividerGlowOpacity={dividerGlowOpacity}
        ids={ids}
        metrics={metrics}
        svgStyle={svgStyle}
      />
      <ScopeToggleSlider
        config={config}
        ids={ids}
        paths={paths}
        shineOpacity={shineOpacity}
        sliderGlossOpacity={sliderGlossOpacity}
        sliderWidth={sliderWidth}
        sliderX={sliderX}
        sliderY={sliderY}
        svgStyle={svgStyle}
      />
      <ScopeToggleText
        config={config}
        disabled={disabled}
        metrics={metrics}
        options={normalizedOptions}
        selectedIndex={selectedIndex}
        svgStyle={svgStyle}
        titleText={titleText}
        {...(titleRenderer ? { titleRenderer } : {})}
        onOptionSelect={(option, index) => setSelected(option.value, option, index)}
      />
    </>
  );

  if (renderMode === 'svg') {
    return (
      <svg
        x={x}
        y={y}
        viewBox={`${-config.svg.viewportInset} ${-config.svg.viewportInset} ${metrics.svgWidth + config.svg.viewportInset * 2} ${config.svg.height + config.svg.viewportInset * 2}`}
        width={metrics.svgWidth}
        height={config.svg.height}
        role="group"
        aria-label={`${titleText}: ${selectedOption.label}`}
        overflow="visible"
        opacity={disabled ? config.opacity.disabled : 1}
        onPointerEnter={() => setIsHovering(true)}
        onPointerLeave={() => {
          setIsHovering(false);
          setIsPressed(false);
        }}
        onPointerDown={() => setIsPressed(true)}
        onPointerUp={() => setIsPressed(false)}
        onPointerCancel={() => setIsPressed(false)}
        onClick={advanceSelected}
      >
        {svgContent}
      </svg>
    );
  }

  return (
    <div
      className={className}
      style={rootStyle}
      role="group"
      aria-label={`${titleText}: ${selectedOption.label}`}
      onPointerEnter={() => setIsHovering(true)}
      onPointerLeave={() => {
        setIsHovering(false);
        setIsPressed(false);
      }}
      onPointerDown={() => setIsPressed(true)}
      onPointerUp={() => setIsPressed(false)}
      onPointerCancel={() => setIsPressed(false)}
      onClick={advanceSelected}
    >
      <svg
        viewBox={`${-config.svg.viewportInset} ${-config.svg.viewportInset} ${metrics.svgWidth + config.svg.viewportInset * 2} ${config.svg.height + config.svg.viewportInset * 2}`}
        width={metrics.svgWidth}
        height={config.svg.height}
        role="img"
        aria-label={`${titleText}: ${selectedOption.label}`}
      >
        {svgContent}
      </svg>
    </div>
  );
}

export const ScopeToggleSvg = ScopeToggle;

function scopeToggleIds(uid: string): ScopeToggleIds {
  return {
    dividerGlow: `${uid}-dividerGlow`,
    outerGlow: `${uid}-outerGlow`,
    shadow: `${uid}-shadow`,
    slider: `${uid}-slider`,
    sliderBottomGloss: `${uid}-sliderBottomGloss`,
    sliderGlow: `${uid}-sliderGlow`,
    sliderShine: `${uid}-sliderShine`,
    titleGlow: `${uid}-titleGlow`,
    track: `${uid}-track`,
    trackGlow: `${uid}-trackGlow`,
  };
}
