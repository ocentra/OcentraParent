import type { CSSProperties, KeyboardEvent } from 'react';
import { useId, useMemo, useState } from 'react';

import { defaultScopeMultiChoiceConfig, mergeScopeMultiChoiceConfig } from './ScopeMultiChoiceConfig';
import {
  calculateScopeMultiChoiceMetrics,
  getScopeMultiChoiceLabel,
  roundedScopeMultiChoiceRectPath,
} from './ScopeMultiChoiceMetrics';
import {
  normalizeScopeMultiChoiceOptions,
  scopeMultiChoiceSelectionLabel,
  toggleScopeMultiChoiceValue,
} from './ScopeMultiChoiceOptions';
import type { ScopeMultiChoiceOption, ScopeMultiChoiceSvgProps } from './ScopeMultiChoiceTypes';

export function ScopeMultiChoice({
  x = 0,
  y = 0,
  renderMode = 'html',
  selected,
  defaultSelected = [],
  title,
  showTitle = true,
  options,
  width,
  height,
  fitMode,
  overflowMode,
  multiSelect = true,
  disabled = false,
  className,
  style,
  titleRenderer,
  onChange,
  config: configOverride,
}: ScopeMultiChoiceSvgProps) {
  const [isHovering, setIsHovering] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const [hoveredValue, setHoveredValue] = useState<string | null>(null);
  const [internalSelected, setInternalSelected] = useState<readonly string[]>(defaultSelected);
  const rawId = useId();
  const uid = rawId.replace(/[^a-zA-Z0-9_-]/g, '');
  const config = useMemo(() => {
    const merged = mergeScopeMultiChoiceConfig(defaultScopeMultiChoiceConfig, configOverride);

    return {
      ...merged,
      svg: {
        ...merged.svg,
        height: height ?? merged.svg.height,
        fitMode: fitMode ?? merged.svg.fitMode,
        overflowMode: overflowMode ?? merged.svg.overflowMode,
      },
    };
  }, [configOverride, fitMode, height, overflowMode]);
  const normalizedOptions = useMemo(() => normalizeScopeMultiChoiceOptions(config, options), [config, options]);
  const selectedValues = selected ?? internalSelected;
  const titleText = title ?? config.text.title;
  const metrics = calculateScopeMultiChoiceMetrics(config, titleText, normalizedOptions, width, showTitle, multiSelect);
  const renderedSvgHeight =
    config.svg.fitMode === 'fixedHeight' && config.svg.height !== undefined ? config.svg.height : metrics.svgHeight;
  const viewBoxX = 0;
  const viewBoxY = 0;
  const viewBoxW = metrics.svgWidth;
  const viewBoxH = metrics.svgHeight;
  const titleClipId = `${uid}-titleClip`;
  const titleClipX = metrics.titleBoxX - config.svg.viewportInset;
  const titleClipY = metrics.titleBoxY - config.svg.viewportInset;
  const titleClipWidth = Math.max(1, metrics.trackX - titleClipX);
  const titleClipHeight = config.layout.titleBoxHeight + config.svg.viewportInset * 2;
  const titleOverlayClipId = `${uid}-titleOverlayClip`;
  const titleIconGlowId = `${uid}-titleIconGlow`;
  const indicatorOuterGlowId = `${uid}-indicatorOuterGlow`;
  const titleOverlayClipX = metrics.titleBoxX - config.svg.viewportInset;
  const titleOverlayClipY = metrics.titleBoxY - config.svg.viewportInset;
  const titleOverlayClipWidth = metrics.titleBoxWidth + config.svg.viewportInset * 2;
  const titleOverlayClipHeight = config.layout.titleBoxHeight + config.svg.viewportInset * 2;
  const glowOpacity = isHovering ? config.opacity.trackGlowHover : config.opacity.trackGlowIdle;
  const outerGlowOpacity = isHovering ? config.opacity.outerGlowHover : config.opacity.outerGlowIdle;
  const titleGlowOpacity = isHovering ? config.opacity.titleGlowHover : config.opacity.titleGlowIdle;
  const selectedGlowOpacity = isHovering ? config.opacity.selectedGlowHover : config.opacity.selectedGlowIdle;
  const rootOpacity = disabled ? config.opacity.disabled : 1;
  const scale = disabled ? 1 : isPressed ? config.hover.pressScale : 1;
  const titleBoxRightRadius = config.layout.titleBoxRightRadius ?? config.layout.titleBoxRadius;
  const titleBoxPath = roundedScopeMultiChoiceRectPath(
    metrics.titleBoxX,
    metrics.titleBoxY,
    metrics.titleBoxWidth,
    config.layout.titleBoxHeight,
    config.layout.titleBoxRadius,
    titleBoxRightRadius,
    config.layout.titleBoxRightRadius ?? config.layout.titleBoxBottomRadius,
    config.layout.titleBoxBottomRadius
  );
  const outerEdgePath = roundedScopeMultiChoiceRectPath(
    metrics.trackX - config.layout.outerPadX,
    metrics.trackY - config.layout.outerPadY,
    metrics.trackWidth + config.layout.outerPadX * 2,
    metrics.trackHeight + config.layout.outerPadY * 2,
    config.layout.outerRadius,
    config.layout.outerRadius,
    config.layout.outerRadius,
    config.layout.outerRadius
  );
  const trackPath = roundedScopeMultiChoiceRectPath(
    metrics.trackX,
    metrics.trackY,
    metrics.trackWidth,
    metrics.trackHeight,
    config.track.radius,
    config.track.radius,
    config.track.radius,
    config.track.radius
  );
  const rootStyle: CSSProperties = {
    width: metrics.svgWidth,
    height: renderedSvgHeight,
    overflow: config.svg.overflowMode,
    opacity: rootOpacity,
    transform: `scale(${scale})`,
    transformOrigin: 'center',
    transition: config.transition.root,
    ...style,
  };
  const svgStyle: CSSProperties = { transition: config.transition.svg };
  const titleSlot = {
    x: metrics.titleBoxX,
    y: metrics.titleBoxY,
    width: metrics.titleBoxWidth,
    height: config.layout.titleBoxHeight,
    centerX: metrics.titleCenterX,
    centerY: metrics.titleBoxY + config.layout.titleBoxHeight * 0.5,
  };

  function commitSelected(nextSelected: readonly string[], option: ScopeMultiChoiceOption, index: number) {
    if (selected === undefined) {
      setInternalSelected(nextSelected);
    }
    onChange?.(nextSelected, option, index);
  }

  function handleOptionSelect(option: ScopeMultiChoiceOption, index: number) {
    if (disabled || option.disabled) {
      return;
    }
    commitSelected(toggleScopeMultiChoiceValue(selectedValues, option.value, multiSelect), option, index);
  }

  function handleOptionKeyDown(event: KeyboardEvent<SVGGElement>, option: ScopeMultiChoiceOption, index: number) {
    if (event.key !== 'Enter' && event.key !== ' ') {
      return;
    }
    event.preventDefault();
    handleOptionSelect(option, index);
  }

  const svgContent = (
    <>
      <defs>
        <linearGradient
          id={`${uid}-track`}
          x1={0}
          y1={metrics.trackY}
          x2={0}
          y2={metrics.trackY + metrics.trackHeight}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset={0} stopColor={config.colors.trackTop} />
          <stop offset={1} stopColor={config.colors.trackBottom} />
        </linearGradient>
        <linearGradient
          id={`${uid}-selected`}
          x1={0}
          y1={0}
          x2={0}
          y2={config.layout.optionHeight}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset={0} stopColor={config.colors.selectedTop} />
          <stop offset={1} stopColor={config.colors.selectedBottom} />
        </linearGradient>
        <filter id={`${uid}-trackGlow`} x="-20%" y="-30%" width="140%" height="170%">
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={2}
            floodColor={config.colors.trackGlow}
            floodOpacity={glowOpacity}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={6}
            floodColor={config.colors.trackGlow}
            floodOpacity={glowOpacity * 0.34}
          />
        </filter>
        <filter id={`${uid}-outerGlow`} x="-20%" y="-20%" width="140%" height="140%">
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={2}
            floodColor={config.colors.outerEdgeGlow}
            floodOpacity={outerGlowOpacity}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={6}
            floodColor={config.colors.outerEdgeGlow}
            floodOpacity={outerGlowOpacity * 0.38}
          />
        </filter>
        <filter id={`${uid}-titleGlow`} x="-35%" y="-35%" width="170%" height="170%">
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={3.2}
            floodColor={config.colors.titleBoxGlow}
            floodOpacity={titleGlowOpacity}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={8}
            floodColor={config.colors.titleBoxGlow}
            floodOpacity={titleGlowOpacity * 0.45}
          />
        </filter>
        <filter id={`${uid}-selectedGlow`} x="-35%" y="-75%" width="170%" height="250%">
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={2.2}
            floodColor={config.colors.selectedGlow}
            floodOpacity={selectedGlowOpacity}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={6}
            floodColor={config.colors.selectedGlow}
            floodOpacity={selectedGlowOpacity * 0.36}
          />
        </filter>
        <filter id={titleIconGlowId} x="-60%" y="-60%" width="220%" height="220%">
          <feDropShadow dx={0} dy={0} stdDeviation={1.4} floodColor="#fff2a8" floodOpacity={0.58} />
          <feDropShadow dx={0} dy={0} stdDeviation={4.2} floodColor="#53c7ff" floodOpacity={0.34} />
        </filter>
        <filter id={indicatorOuterGlowId} x="-90%" y="-90%" width="280%" height="280%">
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={1.2}
            floodColor={config.colors.indicatorCircleGlow}
            floodOpacity={0.9}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={4.2}
            floodColor={config.colors.indicatorCircleGlow}
            floodOpacity={0.42}
          />
        </filter>
        <clipPath id={titleClipId}>
          <rect x={titleClipX} y={titleClipY} width={titleClipWidth} height={titleClipHeight} />
        </clipPath>
        <clipPath id={titleOverlayClipId}>
          <rect
            x={titleOverlayClipX}
            y={titleOverlayClipY}
            width={titleOverlayClipWidth}
            height={titleOverlayClipHeight}
          />
        </clipPath>
      </defs>

      {showTitle ? (
        <g clipPath={`url(#${titleClipId})`}>
          <path
            d={titleBoxPath}
            fill="none"
            stroke={config.colors.titleBoxGlow}
            strokeWidth={config.titleBox.glowStrokeWidth}
            strokeOpacity={titleGlowOpacity}
            filter={`url(#${uid}-titleGlow)`}
            style={svgStyle}
          />
          <path
            d={titleBoxPath}
            fill="transparent"
            stroke={isHovering ? config.colors.titleBoxStrokeHover : config.colors.titleBoxStroke}
            strokeWidth={config.titleBox.strokeWidth}
            style={svgStyle}
          />
          <path
            d={titleBoxPath}
            fill="none"
            stroke={config.colors.shine}
            strokeWidth={config.titleBox.innerStrokeWidth}
            strokeOpacity={isHovering ? 0.34 : 0.18}
            style={svgStyle}
          />
          {!titleRenderer ? (
            <text
              x={metrics.titleCenterX}
              y={metrics.titleBoxY + config.layout.titleBoxHeight * 0.64}
              textAnchor="middle"
              fill={config.colors.title}
              fontFamily={config.text.fontFamily}
              fontSize={config.text.titleFontSize}
              fontWeight={config.text.fontWeight}
              pointerEvents="none"
              style={svgStyle}
            >
              {titleText}
            </text>
          ) : null}
        </g>
      ) : null}

      <path d={outerEdgePath} fill={`url(#${uid}-track)`} stroke="none" style={svgStyle} />
      <path
        d={outerEdgePath}
        fill="none"
        stroke={config.colors.outerEdgeGlow}
        strokeWidth={config.outerEdge.glowStrokeWidth}
        strokeOpacity={outerGlowOpacity}
        filter={`url(#${uid}-outerGlow)`}
        style={svgStyle}
      />
      <path
        d={outerEdgePath}
        fill="none"
        stroke={config.colors.outerEdge}
        strokeWidth={config.outerEdge.strokeWidth}
        strokeOpacity={isHovering ? 0.72 : 0.42}
        style={svgStyle}
      />

      <path
        d={trackPath}
        fill="none"
        stroke={config.colors.trackGlow}
        strokeWidth={config.track.glowStrokeWidth}
        strokeOpacity={glowOpacity}
        filter={`url(#${uid}-trackGlow)`}
        style={svgStyle}
      />
      <path
        d={trackPath}
        fill={`url(#${uid}-track)`}
        stroke={isHovering ? config.colors.trackStrokeHover : config.colors.trackStroke}
        strokeWidth={config.track.strokeWidth}
        style={svgStyle}
      />
      <path
        d={trackPath}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.track.innerStrokeWidth}
        strokeOpacity={0.24}
        style={svgStyle}
      />

      {normalizedOptions.map((option, index) => {
        const placement = metrics.placements[index];
        if (!placement) {
          return null;
        }

        const isSelected = selectedValues.includes(option.value);
        const isOptionHovered = hoveredValue === option.value;
        const optionX = metrics.trackX + placement.x;
        const optionY = metrics.trackY + placement.y;
        const selectedX = optionX + config.optionButton.insetX;
        const selectedY = optionY + config.optionButton.insetY;
        const selectedHeight = placement.height - config.optionButton.insetY * 2;
        const showIndicator = multiSelect;
        const indicatorSize = showIndicator ? selectedHeight : 0;
        const selectedWidth = placement.width - config.optionButton.insetX * 2;
        const labelWidth = Math.max(1, selectedWidth - indicatorSize);
        const indicatorX = selectedX + labelWidth;
        const labelPath = roundedScopeMultiChoiceRectPath(
          selectedX,
          selectedY,
          labelWidth,
          selectedHeight,
          config.optionButton.radius,
          showIndicator ? 0 : config.optionButton.radius,
          showIndicator ? 0 : config.optionButton.radius,
          config.optionButton.radius
        );
        const indicatorPath = roundedScopeMultiChoiceRectPath(
          indicatorX,
          selectedY,
          indicatorSize,
          indicatorSize,
          0,
          config.indicator.radius,
          config.indicator.radius,
          0
        );
        const label = getScopeMultiChoiceLabel(
          option.label,
          labelWidth - config.layout.optionPaddingX,
          config.text.optionFontSize
        );
        const textColor = isSelected
          ? config.colors.optionSelected
          : isOptionHovered
            ? config.colors.optionHover
            : config.colors.optionIdle;

        return (
          <g
            key={option.value}
            role="button"
            aria-label={option.label}
            aria-pressed={isSelected}
            aria-disabled={disabled || option.disabled || undefined}
            tabIndex={disabled || option.disabled ? -1 : 0}
            opacity={option.disabled ? config.opacity.disabled : 1}
            onPointerEnter={() => setHoveredValue(option.value)}
            onPointerLeave={() => setHoveredValue(null)}
            onClick={(event) => {
              event.stopPropagation();
              handleOptionSelect(option, index);
            }}
            onKeyDown={(event) => handleOptionKeyDown(event, option, index)}
            style={{ cursor: disabled || option.disabled ? 'not-allowed' : 'pointer', outline: 'none', ...svgStyle }}
          >
            <rect x={optionX} y={optionY} width={placement.width} height={placement.height} fill="transparent" />
            {isSelected ? (
              <path
                d={labelPath}
                fill="none"
                stroke={config.colors.selectedGlow}
                strokeWidth={config.optionButton.glowStrokeWidth}
                strokeOpacity={selectedGlowOpacity}
                filter={`url(#${uid}-selectedGlow)`}
                style={svgStyle}
              />
            ) : null}
            <path
              d={labelPath}
              fill={isSelected ? `url(#${uid}-selected)` : 'transparent'}
              stroke={
                isSelected
                  ? config.colors.selectedStroke
                  : isOptionHovered
                    ? config.colors.trackStrokeHover
                    : config.colors.trackStroke
              }
              strokeWidth={config.optionButton.strokeWidth}
              strokeOpacity={isSelected ? 1 : isOptionHovered ? 0.8 : 0.58}
              style={svgStyle}
            />
            {isSelected ? (
              <path
                d={labelPath}
                fill={config.colors.shine}
                opacity={isHovering ? config.opacity.shineHover : config.opacity.shineIdle}
                style={{ mixBlendMode: 'screen', ...svgStyle }}
              />
            ) : null}
            {showIndicator ? (
              <>
                <path
                  d={indicatorPath}
                  fill={isSelected ? `url(#${uid}-track)` : 'transparent'}
                  stroke={isSelected || isOptionHovered ? config.colors.indicatorStroke : config.colors.trackStroke}
                  strokeWidth={config.indicator.strokeWidth}
                  strokeOpacity={isSelected ? 1 : isOptionHovered ? 0.82 : 0.62}
                  style={svgStyle}
                />
                {isSelected ? (
                  <circle
                    cx={indicatorX + indicatorSize * 0.5}
                    cy={selectedY + indicatorSize * 0.5}
                    r={config.indicator.circleRadius + config.indicator.outerRingRadiusOffset}
                    fill="none"
                    stroke={config.colors.indicatorCircleGlow}
                    strokeWidth={config.indicator.outerRingStrokeWidth}
                    strokeOpacity={0.98}
                    filter={`url(#${indicatorOuterGlowId})`}
                    style={svgStyle}
                  />
                ) : null}
                <circle
                  cx={indicatorX + indicatorSize * 0.5}
                  cy={selectedY + indicatorSize * 0.5}
                  r={config.indicator.circleRadius}
                  fill={isSelected ? config.colors.indicatorCircleSelected : 'transparent'}
                  stroke={
                    isSelected
                      ? config.colors.indicatorCircleSelected
                      : isOptionHovered
                        ? config.colors.indicatorCircleIdle
                        : config.colors.trackStroke
                  }
                  strokeWidth={config.indicator.circleStrokeWidth}
                  strokeOpacity={isSelected ? 1 : isOptionHovered ? 0.98 : 0.68}
                  style={svgStyle}
                />
              </>
            ) : null}
            <text
              x={selectedX + labelWidth * 0.5}
              y={optionY + placement.height * 0.5 + config.text.optionFontSize * 0.34}
              textAnchor="middle"
              fill={textColor}
              fontFamily={config.text.fontFamily}
              fontSize={config.text.optionFontSize}
              fontWeight={config.text.optionFontWeight}
              pointerEvents="none"
              style={svgStyle}
            >
              {label}
            </text>
          </g>
        );
      })}
      {showTitle && titleRenderer ? (
        <g clipPath={`url(#${titleOverlayClipId})`} filter={`url(#${titleIconGlowId})`} pointerEvents="none">
          {titleRenderer(titleSlot)}
        </g>
      ) : null}
    </>
  );

  if (renderMode === 'svg') {
    return (
      <svg
        x={x}
        y={y}
        viewBox={`${viewBoxX} ${viewBoxY} ${viewBoxW} ${viewBoxH}`}
        width={metrics.svgWidth}
        height={renderedSvgHeight}
        role="group"
        aria-label={`${titleText}: ${scopeMultiChoiceSelectionLabel(selectedValues)}`}
        overflow={config.svg.overflowMode}
        opacity={rootOpacity}
        onPointerEnter={() => setIsHovering(true)}
        onPointerLeave={() => {
          setIsHovering(false);
          setIsPressed(false);
          setHoveredValue(null);
        }}
        onPointerDown={() => setIsPressed(true)}
        onPointerUp={() => setIsPressed(false)}
        onPointerCancel={() => setIsPressed(false)}
      >
        {svgContent}
      </svg>
    );
  }

  return (
    <div
      className={className}
      style={rootStyle}
      onPointerEnter={() => setIsHovering(true)}
      onPointerLeave={() => {
        setIsHovering(false);
        setIsPressed(false);
        setHoveredValue(null);
      }}
      onPointerDown={() => setIsPressed(true)}
      onPointerUp={() => setIsPressed(false)}
      onPointerCancel={() => setIsPressed(false)}
    >
      <svg
        viewBox={`${viewBoxX} ${viewBoxY} ${viewBoxW} ${viewBoxH}`}
        width={metrics.svgWidth}
        height={renderedSvgHeight}
        role="group"
        aria-label={`${titleText}: ${scopeMultiChoiceSelectionLabel(selectedValues)}`}
        style={{ display: 'block', overflow: config.svg.overflowMode }}
      >
        {svgContent}
      </svg>
    </div>
  );
}
