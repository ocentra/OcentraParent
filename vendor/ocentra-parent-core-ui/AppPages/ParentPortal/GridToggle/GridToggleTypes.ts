import type { CSSProperties } from 'react';
import type { GridToggleConfig } from './GridToggleConfig';

export type GridToggleOption = {
  value: string;
  label: string;
};

export type DeepPartial<T> = T extends readonly (infer U)[]
  ? readonly U[]
  : T extends (...args: never[]) => unknown
    ? T
    : T extends object
      ? { [K in keyof T]?: DeepPartial<T[K]> }
      : T;

export type GridToggleProps = {
  value?: string;
  defaultValue?: string;
  title?: string;
  options?: GridToggleOption[];
  rows?: number;
  columns?: number;
  disabled?: boolean;
  className?: string;
  style?: CSSProperties;
  onChange?: (value: string, option: GridToggleOption, index: number, row: number, column: number) => void;
  config?: DeepPartial<GridToggleConfig>;
};

export type GridToggleMetrics = {
  svgWidth: number;
  svgHeight: number;
  titleBoxX: number;
  titleBoxY: number;
  titleBoxWidth: number;
  titleCenterX: number;
  gridX: number;
  gridY: number;
  gridWidth: number;
  gridHeight: number;
  cellWidth: number;
  cellHeight: number;
  rowCount: number;
  columnCount: number;
  verticalDividerXs: number[];
  horizontalDividerYs: number[];
};

export type GridToggleSelectionMetrics = {
  column: number;
  height: number;
  row: number;
  width: number;
  x: number;
  y: number;
};

export type GridTogglePaths = {
  grid: string;
  outerEdge: string;
  selected: string;
  titleBox: string;
};

export type GridToggleIds = {
  dividerGlow: string;
  grid: string;
  gridGlow: string;
  outerGlow: string;
  selected: string;
  selectedBottomGloss: string;
  selectedGlow: string;
  selectedShine: string;
  shadow: string;
  titleGlow: string;
};
