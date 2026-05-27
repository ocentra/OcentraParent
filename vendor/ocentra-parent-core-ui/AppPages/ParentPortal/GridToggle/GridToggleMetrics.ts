import type { GridToggleConfig } from './GridToggleConfig';
import {
  estimateGridToggleTextWidth,
  roundedGridToggleRectPath,
  roundedGridToggleRectPathByCorner,
} from './GridToggleGeometry';
import type {
  GridToggleMetrics,
  GridToggleOption,
  GridTogglePaths,
  GridToggleSelectionMetrics,
} from './GridToggleTypes';

export function calculateGridToggleMetrics(
  config: GridToggleConfig,
  titleText: string,
  options: GridToggleOption[],
  rowCount: number,
  columnCount: number
): GridToggleMetrics {
  const titleTextWidth = estimateGridToggleTextWidth(titleText, config.text.titleFontSize);
  const titleBoxWidth = Math.max(config.layout.titleBoxMinWidth, titleTextWidth + config.layout.titleBoxPaddingX * 2);
  const titleBoxX = config.layout.titleAnchorX;
  const titleBoxY = config.layout.titleBoxY;
  const gridX = titleBoxX + titleBoxWidth;
  const widestCellWidth = Math.max(
    ...options.map(
      (option) => estimateGridToggleTextWidth(option.label, config.text.optionFontSize) + config.layout.cellPaddingX * 2
    )
  );
  const cellWidth = Math.max(config.layout.cellMinWidth, widestCellWidth);
  const cellHeight = config.layout.cellHeight;
  const gridWidth =
    cellWidth * columnCount +
    config.layout.dividerWidth * (columnCount - 1) +
    config.layout.cellGapX * (columnCount - 1);
  const gridHeight =
    cellHeight * rowCount + config.layout.dividerWidth * (rowCount - 1) + config.layout.cellGapY * (rowCount - 1);
  const verticalDividerXs = Array.from(
    { length: Math.max(0, columnCount - 1) },
    (_, index) =>
      gridX +
      cellWidth * (index + 1) +
      config.layout.cellGapX * index +
      config.layout.dividerWidth * index +
      config.layout.cellGapX * 0.5
  );
  const horizontalDividerYs = Array.from(
    { length: Math.max(0, rowCount - 1) },
    (_, index) =>
      config.layout.gridY +
      cellHeight * (index + 1) +
      config.layout.cellGapY * index +
      config.layout.dividerWidth * index +
      config.layout.cellGapY * 0.5
  );

  return {
    svgWidth: Math.max(
      config.svg.width,
      gridX + gridWidth + config.layout.outerPaddingRight + config.svg.viewportInset
    ),
    svgHeight: Math.max(
      config.svg.height,
      config.layout.gridY + gridHeight + config.layout.outerPaddingBottom + config.svg.viewportInset
    ),
    titleBoxX,
    titleBoxY,
    titleBoxWidth,
    titleCenterX: titleBoxX + titleBoxWidth * 0.5,
    gridX,
    gridY: config.layout.gridY,
    gridWidth,
    gridHeight,
    cellWidth,
    cellHeight,
    rowCount,
    columnCount,
    verticalDividerXs,
    horizontalDividerYs,
  };
}

export function calculateGridToggleSelection(
  config: GridToggleConfig,
  metrics: GridToggleMetrics,
  selectedIndex: number
): GridToggleSelectionMetrics {
  const row = Math.floor(selectedIndex / metrics.columnCount);
  const column = selectedIndex % metrics.columnCount;
  const x =
    metrics.gridX +
    column * (metrics.cellWidth + config.layout.dividerWidth + config.layout.cellGapX) +
    config.selectedCell.inset;
  const y =
    metrics.gridY +
    row * (metrics.cellHeight + config.layout.dividerWidth + config.layout.cellGapY) +
    config.selectedCell.inset;

  return {
    column,
    row,
    x,
    y,
    width: metrics.cellWidth - config.selectedCell.inset * 2,
    height: metrics.cellHeight - config.selectedCell.inset * 2,
  };
}

export function calculateGridTogglePaths(
  config: GridToggleConfig,
  metrics: GridToggleMetrics,
  selection: GridToggleSelectionMetrics
): GridTogglePaths {
  return {
    titleBox: roundedGridToggleRectPathByCorner(
      metrics.titleBoxX,
      metrics.titleBoxY,
      metrics.titleBoxWidth,
      config.layout.titleBoxHeight,
      config.layout.titleBoxRadius,
      config.layout.titleBoxRightRadius,
      config.layout.titleBoxRightRadius,
      config.layout.titleBoxRadius
    ),
    outerEdge: roundedGridToggleRectPath(
      metrics.gridX - config.layout.outerPadX,
      metrics.gridY - config.layout.outerPadY,
      metrics.gridWidth + config.layout.outerPadX * 2,
      metrics.gridHeight + config.layout.outerPadY * 2,
      config.layout.outerRadius
    ),
    grid: roundedGridToggleRectPath(
      metrics.gridX,
      metrics.gridY,
      metrics.gridWidth,
      metrics.gridHeight,
      config.grid.radius
    ),
    selected: roundedGridToggleRectPath(
      selection.x,
      selection.y,
      selection.width,
      selection.height,
      config.selectedCell.radius
    ),
  };
}
