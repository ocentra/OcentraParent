import type {
  ScopeMultiChoiceConfig,
  ScopeMultiChoiceMetrics,
  ScopeMultiChoiceOption,
  ScopeMultiChoicePlacement,
} from './ScopeMultiChoiceTypes';

export function estimateScopeMultiChoiceTextWidth(text: string, fontSize: number): number {
  return Math.ceil(text.length * fontSize * 0.62);
}

export function clampScopeMultiChoiceNumber(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

export function roundedScopeMultiChoiceRectPath(
  x: number,
  y: number,
  width: number,
  height: number,
  topLeftRadius: number,
  topRightRadius: number,
  bottomRightRadius: number,
  bottomLeftRadius: number
): string {
  const safeWidth = Math.max(0, width);
  const safeHeight = Math.max(0, height);
  const maxRadius = Math.min(safeWidth * 0.5, safeHeight * 0.5);
  const tl = Math.min(topLeftRadius, maxRadius);
  const tr = Math.min(topRightRadius, maxRadius);
  const br = Math.min(bottomRightRadius, maxRadius);
  const bl = Math.min(bottomLeftRadius, maxRadius);
  const right = x + safeWidth;
  const bottom = y + safeHeight;

  return [
    `M${x + tl} ${y}`,
    `H${right - tr}`,
    `C${right - tr * 0.45} ${y} ${right} ${y + tr * 0.45} ${right} ${y + tr}`,
    `V${bottom - br}`,
    `C${right} ${bottom - br * 0.45} ${right - br * 0.45} ${bottom} ${right - br} ${bottom}`,
    `H${x + bl}`,
    `C${x + bl * 0.45} ${bottom} ${x} ${bottom - bl * 0.45} ${x} ${bottom - bl}`,
    `V${y + tl}`,
    `C${x} ${y + tl * 0.45} ${x + tl * 0.45} ${y} ${x + tl} ${y}`,
    'Z',
  ].join('');
}

export function getScopeMultiChoiceLabel(label: string, maxWidth: number, fontSize: number): string {
  if (estimateScopeMultiChoiceTextWidth(label, fontSize) <= maxWidth) {
    return label;
  }

  const maxChars = Math.max(1, Math.floor(maxWidth / (fontSize * 0.62)) - 3);
  return `${label.slice(0, maxChars)}...`;
}

export function getScopeMultiChoiceOptionWidth(
  config: ScopeMultiChoiceConfig,
  option: ScopeMultiChoiceOption,
  includeIndicatorColumn = true
): number {
  if (option.width !== undefined) {
    return clampScopeMultiChoiceNumber(option.width, config.layout.optionMinWidth, config.layout.optionMaxWidth);
  }

  const indicatorSize = includeIndicatorColumn ? config.layout.optionHeight - config.optionButton.insetY * 2 : 0;
  const measuredWidth =
    estimateScopeMultiChoiceTextWidth(option.label, config.text.optionFontSize) +
    config.layout.optionPaddingX * 2 +
    indicatorSize +
    config.optionButton.insetX * 2;

  return clampScopeMultiChoiceNumber(measuredWidth, config.layout.optionMinWidth, config.layout.optionMaxWidth);
}

function distributeScopeMultiChoiceRowSpace(
  config: ScopeMultiChoiceConfig,
  placements: ScopeMultiChoicePlacement[],
  trackWidth: number
): ScopeMultiChoicePlacement[] {
  if (!config.layout.distributeRowSpace) {
    return placements;
  }

  const rows = new Map<number, ScopeMultiChoicePlacement[]>();
  placements.forEach((placement) => {
    const rowPlacements = rows.get(placement.row) ?? [];
    rowPlacements.push(placement);
    rows.set(placement.row, rowPlacements);
  });

  rows.forEach((rowPlacements) => {
    const gapTotal = config.layout.optionGapX * Math.max(0, rowPlacements.length - 1);
    const usedWidth = rowPlacements.reduce((sum, placement) => sum + placement.width, 0) + gapTotal;
    const freeWidth = Math.max(0, trackWidth - usedWidth);
    const extraPerOption = Math.min(
      config.layout.maxExtraWidthPerOption,
      freeWidth / Math.max(1, rowPlacements.length)
    );
    let currentX = 0;

    rowPlacements.forEach((placement) => {
      placement.x = currentX;
      placement.width += extraPerOption;
      currentX += placement.width + config.layout.optionGapX;
    });
  });

  return placements;
}

export function calculateScopeMultiChoiceMetrics(
  config: ScopeMultiChoiceConfig,
  titleText: string,
  options: readonly ScopeMultiChoiceOption[],
  width?: number,
  showTitle = true,
  includeIndicatorColumn = true
): ScopeMultiChoiceMetrics {
  const titleTextWidth = estimateScopeMultiChoiceTextWidth(titleText, config.text.titleFontSize);
  const titleBoxWidth = Math.max(config.layout.titleBoxMinWidth, titleTextWidth + config.layout.titleBoxPaddingX * 2);
  const requestedSvgWidth = width ?? config.svg.width;
  const availableTrackWidth = Math.max(
    config.layout.optionMinWidth,
    requestedSvgWidth - config.layout.trackX - config.layout.outerPaddingRight - config.svg.viewportInset
  );
  const trackWidth = Math.min(config.layout.trackWidth, availableTrackWidth);
  const trackY = showTitle ? config.layout.trackY : config.layout.trackYWithoutTitle;
  const placements: ScopeMultiChoicePlacement[] = [];
  let x = 0;
  let y = 0;
  let row = 0;

  options.forEach((option) => {
    const optionWidth = Math.min(getScopeMultiChoiceOptionWidth(config, option, includeIndicatorColumn), trackWidth);
    if (x > 0 && x + optionWidth > trackWidth) {
      x = 0;
      y += config.layout.optionHeight + config.layout.optionGapY;
      row += 1;
    }

    placements.push({ x, y, width: optionWidth, height: config.layout.optionHeight, row });
    x += optionWidth + config.layout.optionGapX;
  });

  distributeScopeMultiChoiceRowSpace(config, placements, trackWidth);

  const lastPlacement = placements[placements.length - 1];
  const trackHeight = lastPlacement ? lastPlacement.y + lastPlacement.height : config.layout.optionHeight;
  const titleBoxY = config.layout.centerTitleBoxOnTrack
    ? trackY + (trackHeight - config.layout.titleBoxHeight) * 0.5
    : config.layout.titleBoxY;
  const svgWidth = Math.max(
    requestedSvgWidth,
    config.layout.trackX + trackWidth + config.layout.outerPaddingRight + config.svg.viewportInset
  );
  const svgHeight = Math.max(
    config.svg.minHeight,
    trackY + trackHeight + config.layout.outerPaddingBottom + config.svg.viewportInset
  );

  return {
    svgWidth,
    svgHeight,
    titleBoxX: config.layout.titleBoxX,
    titleBoxY,
    titleBoxWidth,
    titleCenterX: config.layout.titleBoxX + titleBoxWidth * 0.5,
    trackX: config.layout.trackX,
    trackY,
    trackWidth,
    trackHeight,
    placements,
  };
}
