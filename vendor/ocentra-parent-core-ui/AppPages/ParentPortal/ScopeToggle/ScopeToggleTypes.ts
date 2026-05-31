import type { CSSProperties, ReactElement } from 'react';
import type { ScopeToggleConfig } from './ScopeToggleConfig';

export type ScopeToggleOption = {
  value: string;
  label: string;
  iconHref?: string;
};

export type ScopeToggleTitleSlot = {
  x: number;
  y: number;
  width: number;
  height: number;
  centerX: number;
  centerY: number;
};

export type ScopeToggleTitleRenderer = (slot: ScopeToggleTitleSlot) => ReactElement;

export type DeepPartial<T> = T extends readonly (infer U)[]
  ? readonly U[]
  : T extends (...args: never[]) => unknown
    ? T
    : T extends object
      ? { [K in keyof T]?: DeepPartial<T[K]> }
      : T;

export type ScopeToggleProps = {
  x?: number;
  y?: number;
  renderMode?: 'html' | 'svg';
  value?: string;
  defaultValue?: string;
  title?: string;
  options?: ScopeToggleOption[];
  leftOption?: string;
  rightOption?: string;
  disabled?: boolean;
  className?: string;
  style?: CSSProperties;
  titleRenderer?: ScopeToggleTitleRenderer;
  onChange?: (value: string, option: ScopeToggleOption, index: number) => void;
  config?: DeepPartial<ScopeToggleConfig>;
};

export type ScopeToggleMetrics = {
  svgWidth: number;
  titleBoxX: number;
  titleBoxY: number;
  titleBoxWidth: number;
  titleCenterX: number;
  trackX: number;
  trackY: number;
  trackWidth: number;
  trackHeight: number;
  optionWidth: number;
  dividerXs: number[];
};

export type ScopeTogglePaths = {
  outerEdge: string;
  slider: string;
  titleBox: string;
  track: string;
};

export type ScopeToggleIds = {
  dividerGlow: string;
  outerGlow: string;
  shadow: string;
  slider: string;
  sliderBottomGloss: string;
  sliderGlow: string;
  sliderShine: string;
  titleGlow: string;
  track: string;
  trackGlow: string;
};
