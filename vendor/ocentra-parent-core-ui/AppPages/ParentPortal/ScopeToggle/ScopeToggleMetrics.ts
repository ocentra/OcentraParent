import type { ScopeToggleConfig } from './ScopeToggleConfig';
import { estimateScopeToggleTextWidth, roundedRectPath, roundedRectPathByCorner } from './ScopeToggleGeometry';
import type { ScopeToggleMetrics, ScopeToggleOption, ScopeTogglePaths } from './ScopeToggleTypes';

export function calculateScopeToggleMetrics(
  config: ScopeToggleConfig,
  titleText: string,
  options: ScopeToggleOption[]
): ScopeToggleMetrics {
  const titleTextWidth = estimateScopeToggleTextWidth(titleText, config.text.titleFontSize);
  const titleBoxWidth = Math.max(config.layout.titleBoxMinWidth, titleTextWidth + config.layout.titleBoxPaddingX * 2);
  const titleBoxX = config.layout.titleAnchorX;
  const titleBoxY = config.layout.titleBoxY;
  const trackX = titleBoxX + titleBoxWidth;
  const widestOptionWidth = Math.max(
    ...options.map((option) => scopeToggleOptionContentWidth(config, option) + config.layout.optionPaddingX * 2)
  );
  const optionCount = options.length;
  const minOptionWidth = (config.layout.trackMinWidth - config.layout.dividerWidth * (optionCount - 1)) / optionCount;
  const optionWidth = Math.max(widestOptionWidth, minOptionWidth);
  const trackWidth = optionWidth * optionCount + config.layout.dividerWidth * (optionCount - 1);
  const dividerXs = Array.from(
    { length: optionCount - 1 },
    (_, index) => trackX + optionWidth * (index + 1) + config.layout.dividerWidth * index
  );
  const svgWidth = Math.max(
    config.svg.width,
    trackX + trackWidth + config.layout.outerPaddingRight + config.svg.viewportInset
  );

  return {
    svgWidth,
    titleBoxX,
    titleBoxY,
    titleBoxWidth,
    titleCenterX: titleBoxX + titleBoxWidth * 0.5,
    trackX,
    trackY: config.layout.trackY,
    trackWidth,
    trackHeight: config.layout.trackHeight,
    optionWidth,
    dividerXs,
  };
}

function scopeToggleOptionContentWidth(config: ScopeToggleConfig, option: ScopeToggleOption): number {
  const textWidth = estimateScopeToggleTextWidth(option.label, config.text.optionFontSize);
  if (!option.iconHref) {
    return textWidth;
  }
  return textWidth + config.layout.optionIconSize + config.layout.optionIconGap;
}

export function calculateScopeTogglePaths(
  config: ScopeToggleConfig,
  metrics: ScopeToggleMetrics,
  sliderX: number,
  sliderY: number,
  sliderWidth: number,
  sliderHeight: number
): ScopeTogglePaths {
  return {
    titleBox: roundedRectPathByCorner(
      metrics.titleBoxX,
      metrics.titleBoxY,
      metrics.titleBoxWidth,
      config.layout.titleBoxHeight,
      config.layout.titleBoxRadius,
      config.layout.titleBoxRightRadius,
      config.layout.titleBoxRightRadius,
      config.layout.titleBoxRadius
    ),
    outerEdge: roundedRectPath(
      metrics.trackX - config.layout.outerPadX,
      metrics.trackY - config.layout.outerPadY,
      metrics.trackWidth + config.layout.outerPadX * 2,
      metrics.trackHeight + config.layout.outerPadY * 2,
      config.layout.outerRadius
    ),
    track: roundedRectPath(
      metrics.trackX,
      metrics.trackY,
      metrics.trackWidth,
      metrics.trackHeight,
      config.track.radius
    ),
    slider: roundedRectPath(sliderX, sliderY, sliderWidth, sliderHeight, config.slider.radius),
  };
}
