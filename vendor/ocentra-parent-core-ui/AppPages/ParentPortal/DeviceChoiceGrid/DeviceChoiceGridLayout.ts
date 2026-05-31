import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import { clampGridCount, estimateTextWidth, topRoundRectPath } from './DeviceChoiceGridGeometry';
import { DEVICE_CHOICE_DEFAULT_SCOPE_VALUES, type DeviceSlot, type ScopeValue } from './DeviceChoiceGridTypes';

export type DeviceChoiceGridGridPlan = {
  columns: number;
  rows: number;
};

type DeviceChoiceGridCellBounds = {
  aspect: number;
  maxW: number;
  minW: number;
};

export type DeviceChoiceGridShape = {
  currentScope: ScopeValue;
  lanRows: number;
  lanColumns: number;
  portalRows: number;
  portalColumns: number;
  rowCount: number;
  columnCount: number;
  frameRows: number;
  frameColumns: number;
  totalSlots: number;
  lanTotalSlots: number;
};

export type DeviceChoiceGridLayoutMetrics = {
  activeGridX: number;
  activeGridY: number;
  cellH: number;
  cellW: number;
  connectorY: number;
  frameGridH: number;
  gridContentH: number;
  gridOuterH: number;
  gridOuterPath: string;
  gridOuterW: number;
  gridOuterX: number;
  gridViewportH: number;
  gridViewportW: number;
  gridViewportX: number;
  gridViewportY: number;
  infoH: number;
  infoY: number;
  maxScrollY: number;
  scopeOptionW: number;
  scopeSliderX: number;
  svgH: number;
  svgW: number;
  titleBottom: number;
  titleW: number;
  titleX: number;
  topCenters: number[];
  viewBoxH: number;
  viewBoxW: number;
};

export function createDeviceChoiceGridGridPlan({
  availableH,
  availableW,
  cfg,
  fallbackColumns,
  fallbackRows,
  itemCount,
  legendCount,
  requestedColumns,
  requestedRows,
}: {
  availableH: number;
  availableW: number;
  cfg: DeviceChoiceGridConfig;
  fallbackColumns: number;
  fallbackRows: number;
  itemCount: number;
  legendCount: number;
  requestedColumns?: number | undefined;
  requestedRows?: number | undefined;
}): DeviceChoiceGridGridPlan {
  void availableH;
  void legendCount;
  const desiredCount = clampGridCount(itemCount, 1, Number.MAX_SAFE_INTEGER);

  if (requestedRows !== undefined && requestedColumns !== undefined) {
    return {
      rows: clampGridCount(requestedRows, 1, desiredCount),
      columns: clampGridCount(requestedColumns, 1, desiredCount),
    };
  }

  if (requestedRows !== undefined) {
    const rows = clampGridCount(requestedRows, 1, desiredCount);
    return {
      rows,
      columns: clampGridCount(Math.ceil(desiredCount / rows), 1, desiredCount),
    };
  }

  if (requestedColumns !== undefined) {
    const columns = clampGridCount(requestedColumns, 1, desiredCount);
    return {
      rows: clampGridCount(Math.ceil(desiredCount / columns), 1, desiredCount),
      columns,
    };
  }

  const candidates = Array.from({ length: desiredCount }, (_, index) => index + 1).map((columns) => ({
    columns,
    rows: Math.ceil(desiredCount / columns),
  }));
  const fallbackPlan = {
    rows: clampGridCount(fallbackRows, 1, desiredCount),
    columns: clampGridCount(fallbackColumns, 1, desiredCount),
  };

  if (candidates.length === 0) {
    return fallbackPlan;
  }

  return candidates.reduce((best, candidate) => {
    const bestScore = scoreGridPlan(best, cfg, availableW, desiredCount);
    const candidateScore = scoreGridPlan(candidate, cfg, availableW, desiredCount);
    return candidateScore > bestScore ? candidate : best;
  }, fallbackPlan);
}

export function createDeviceChoiceGridShape({
  cfg,
  currentScope,
  lanRows,
  lanColumns,
  portalRows,
  portalColumns,
}: {
  cfg: DeviceChoiceGridConfig;
  currentScope: ScopeValue;
  lanRows: number;
  lanColumns: number;
  portalRows: number;
  portalColumns: number;
}): DeviceChoiceGridShape {
  const usesPortalGrid = currentScope === 'parent' || currentScope === 'portal';
  const rowCount = usesPortalGrid ? portalRows : lanRows;
  const columnCount = usesPortalGrid ? portalColumns : lanColumns;
  return {
    currentScope,
    lanRows,
    lanColumns,
    portalRows,
    portalColumns,
    rowCount,
    columnCount,
    frameRows: rowCount,
    frameColumns: columnCount,
    totalSlots: rowCount * columnCount,
    lanTotalSlots: lanRows * lanColumns,
  };
}

export function createDeviceChoiceGridLayout(
  cfg: DeviceChoiceGridConfig,
  shape: DeviceChoiceGridShape,
  items: DeviceSlot[],
  scopeValues: readonly ScopeValue[] = DEVICE_CHOICE_DEFAULT_SCOPE_VALUES
): DeviceChoiceGridLayoutMetrics {
  const svgW = Math.max(1, cfg.svg.width);
  const svgH = Math.max(1, cfg.svg.height);
  const cellBounds = createCellBounds(cfg, items);
  const gridSafePadX = Math.max(3, Math.ceil(cfg.effects.cellHoverPad + cfg.stroke.cellHoverGlow / 2));
  const gridSafePadY = Math.max(3, Math.ceil(cfg.effects.cellHoverPad + cfg.stroke.cellHoverGlow / 2));
  const minCellH = cellBounds.minW / cellBounds.aspect;
  const gridTop = gridTopY(cfg, cfg.statusOrder[shape.currentScope].length);
  const contentPadX = Math.max(cfg.layout.outerPad, cfg.svg.inset * 0.5);
  const cellWFromWidth =
    (svgW - contentPadX * 2 - cfg.layout.outerPad * 2 - gridSafePadX * 2 - (shape.frameColumns - 1) * cfg.layout.gapX) /
    Math.max(1, shape.frameColumns);
  const cellW = Number.isFinite(cellWFromWidth)
    ? Math.min(cellBounds.maxW, Math.max(cellBounds.minW, cellWFromWidth))
    : cellBounds.minW;
  const cellH = Math.max(minCellH, cellW / cellBounds.aspect);
  const activeGridW = shape.columnCount * cellW + (shape.columnCount - 1) * cfg.layout.gapX;
  const activeGridH = shape.rowCount * cellH + (shape.rowCount - 1) * cfg.layout.gapY;
  const frameGridW = shape.frameColumns * cellW + (shape.frameColumns - 1) * cfg.layout.gapX;
  const frameGridH = shape.frameRows * cellH + (shape.frameRows - 1) * cfg.layout.gapY;
  const availableOuterW = Math.max(frameGridW + cfg.layout.outerPad * 2 + gridSafePadX * 2, svgW - contentPadX * 2);
  const scopeIconW = cfg.layout.scopeIconSize + cfg.layout.scopeIconGap;
  const activeScopeValues = scopeValues.length > 0 ? scopeValues : DEVICE_CHOICE_DEFAULT_SCOPE_VALUES;
  const scopeOptionCount = activeScopeValues.length;
  const widestScopeLabelW = Math.max(
    ...activeScopeValues.map(
      (scopeValue) => estimateTextWidth(cfg.text.scopeOptions[scopeValue], cfg.text.optionSize) + scopeIconW
    )
  );
  const titleW = Math.max(cfg.layout.scopeOptionW * scopeOptionCount, widestScopeLabelW * scopeOptionCount + 36);
  const contentW = Math.max(availableOuterW, titleW) + contentPadX * 2;
  const gridOuterY = gridTop - cfg.layout.outerPad;
  const minOuterH =
    cfg.layout.outerPad * 2 + gridSafePadY * 2 + cfg.layout.selectedInfoH + cfg.layout.selectedInfoYGap + cellH;
  const availableOuterH = Math.max(minOuterH, svgH - cfg.svg.inset - gridOuterY);
  const contentH = gridOuterY + availableOuterH + cfg.svg.inset;
  let viewBoxW = Math.max(svgW, contentW);
  let viewBoxH = Math.max(svgH, contentH);
  const hostAspect = svgW / svgH;
  const viewBoxAspect = viewBoxW / viewBoxH;
  if (Number.isFinite(hostAspect) && Number.isFinite(viewBoxAspect) && hostAspect > 0) {
    if (viewBoxAspect > hostAspect) {
      viewBoxH = viewBoxW / hostAspect;
    } else if (viewBoxAspect < hostAspect) {
      viewBoxW = viewBoxH * hostAspect;
    }
  }
  const gridOuterX = Math.max(contentPadX, (viewBoxW - availableOuterW) / 2);
  const gridOuterW = availableOuterW;
  const gridOuterH = Math.max(availableOuterH, viewBoxH - cfg.svg.inset - gridOuterY);
  const infoH = cfg.layout.selectedInfoH;
  const infoY = gridOuterY + gridOuterH - infoH;
  const gridViewportX = gridOuterX + cfg.layout.outerPad;
  const gridViewportY = gridTop;
  const gridViewportW = Math.max(1, gridOuterW - cfg.layout.outerPad * 2);
  const gridViewportH = Math.max(cellH, infoY - cfg.layout.selectedInfoYGap - gridViewportY);
  const gridContentH = gridSafePadY * 2 + activeGridH;
  const maxScrollY = Math.max(0, gridContentH - gridViewportH);
  const gridContentW = Math.max(1, gridViewportW - gridSafePadX * 2);
  const gridContentX = gridViewportX + gridSafePadX;
  const gridX = gridContentX + (gridContentW - frameGridW) / 2;
  const activeGridX = gridX + (frameGridW - activeGridW) / 2;
  const activeGridY = gridViewportY + gridSafePadY;
  const titleX = (viewBoxW - titleW) / 2;
  const scopeOptionW = titleW / scopeOptionCount;
  const scopeSliderX = currentScopeSliderX(cfg, titleX, scopeOptionW, shape, activeScopeValues);

  return {
    activeGridX,
    activeGridY,
    cellH,
    cellW,
    connectorY: activeGridY - cfg.connector.trunkOffset,
    frameGridH,
    gridContentH,
    gridOuterH,
    gridOuterPath: topRoundRectPath(gridOuterX, gridOuterY, gridOuterW, gridOuterH, cfg.radius.outer),
    gridOuterW,
    gridOuterX,
    gridViewportH,
    gridViewportW,
    gridViewportX,
    gridViewportY,
    infoH,
    infoY,
    maxScrollY,
    scopeOptionW,
    scopeSliderX,
    svgH,
    svgW,
    titleBottom: cfg.layout.titleY + cfg.layout.titleH,
    titleW,
    titleX,
    topCenters: Array.from(
      { length: shape.columnCount },
      (_, column) => activeGridX + column * (cellW + cfg.layout.gapX) + cellW / 2
    ),
    viewBoxH,
    viewBoxW,
  };
}

function scoreGridPlan(
  plan: DeviceChoiceGridGridPlan,
  cfg: DeviceChoiceGridConfig,
  availableW: number,
  itemCount: number
): number {
  const cellBounds = createCellBounds(cfg, []);
  const contentPadX = Math.max(cfg.layout.outerPad, cfg.svg.inset * 0.5);
  const gridSafePadX = Math.max(3, Math.ceil(cfg.effects.cellHoverPad + cfg.stroke.cellHoverGlow / 2));
  const gridW = plan.columns * cellBounds.minW + (plan.columns - 1) * cfg.layout.gapX;
  const naturalW = gridW + cfg.layout.outerPad * 2 + gridSafePadX * 2 + contentPadX * 2;
  const widthOverflow = Math.max(0, naturalW - availableW);
  const widthFitScore = widthOverflow === 0 ? 1_000_000 : -widthOverflow * 10_000;
  const columnScore = plan.columns * 1_000;
  const rowPenalty = plan.rows * 10;
  const emptyCellPenalty = Math.max(0, plan.rows * plan.columns - itemCount) * cfg.layout.cellW;
  return widthFitScore + columnScore - rowPenalty - emptyCellPenalty;
}

function createCellBounds(cfg: DeviceChoiceGridConfig, items: DeviceSlot[]): DeviceChoiceGridCellBounds {
  const aspect = cfg.layout.cellW / Math.max(1, cfg.layout.cellH);
  const widestLabel = items.length
    ? Math.max(
        ...items.map((slot) => estimateTextWidth(slot.label, cfg.text.optionSize) + cfg.layout.cellW * 0.18),
        cfg.layout.cellW
      )
    : cfg.layout.cellW;
  const minW = Math.max(cfg.layout.cellW, widestLabel);
  const maxW = Math.max(minW, cfg.layout.cellMaxW);
  return { aspect, maxW, minW };
}

function gridTopY(cfg: DeviceChoiceGridConfig, legendCount: number): number {
  const titleBottom = cfg.layout.titleY + cfg.layout.titleH;
  const legendRows = Math.max(1, Math.ceil(legendCount / 2));
  const legendBottom =
    cfg.layout.legendY + Math.max(0, legendRows - 1) * cfg.layout.legendItemGap + cfg.layout.legendDotR * 2 + 1;
  return Math.max(
    cfg.layout.gridY,
    titleBottom + cfg.connector.trunkOffset * 2 + cfg.layout.outerPad,
    legendBottom + cfg.layout.outerPad
  );
}

function currentScopeSliderX(
  cfg: DeviceChoiceGridConfig,
  titleX: number,
  scopeOptionW: number,
  shape: DeviceChoiceGridShape,
  scopeValues: readonly ScopeValue[]
): number {
  const scopeIndex = Math.max(0, scopeValues.indexOf(shape.currentScope));
  return titleX + scopeOptionW * scopeIndex + cfg.layout.scopeInset;
}
