import type { CSSProperties, ReactElement } from 'react';

export type ScopeMultiChoiceFitMode = 'autoHeight' | 'fixedHeight';
export type ScopeMultiChoiceOverflowMode = 'visible' | 'hidden';

export type ScopeMultiChoiceOption = {
  readonly value: string;
  readonly label: string;
  readonly disabled?: boolean;
  readonly width?: number;
};

export type ScopeMultiChoiceTitleSlot = {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly centerX: number;
  readonly centerY: number;
};

export type ScopeMultiChoiceTitleRenderer = (slot: ScopeMultiChoiceTitleSlot) => ReactElement;

export type ScopeMultiChoiceColors = {
  readonly title: string;
  readonly titleBoxStroke: string;
  readonly titleBoxStrokeHover: string;
  readonly titleBoxGlow: string;
  readonly outerEdge: string;
  readonly outerEdgeGlow: string;
  readonly optionIdle: string;
  readonly optionHover: string;
  readonly optionSelected: string;
  readonly trackTop: string;
  readonly trackBottom: string;
  readonly trackStroke: string;
  readonly trackStrokeHover: string;
  readonly trackGlow: string;
  readonly selectedTop: string;
  readonly selectedBottom: string;
  readonly selectedStroke: string;
  readonly selectedGlow: string;
  readonly indicatorStroke: string;
  readonly indicatorCircleIdle: string;
  readonly indicatorCircleSelected: string;
  readonly indicatorCircleGlow: string;
  readonly shine: string;
  readonly shadow: string;
};

export type ScopeMultiChoiceOpacity = {
  readonly trackGlowIdle: number;
  readonly trackGlowHover: number;
  readonly titleGlowIdle: number;
  readonly titleGlowHover: number;
  readonly outerGlowIdle: number;
  readonly outerGlowHover: number;
  readonly selectedGlowIdle: number;
  readonly selectedGlowHover: number;
  readonly shineIdle: number;
  readonly shineHover: number;
  readonly disabled: number;
};

export type ScopeMultiChoiceConfig = {
  readonly svg: {
    readonly width: number;
    readonly height: number | undefined;
    readonly minHeight: number;
    readonly viewportInset: number;
    readonly fitMode: ScopeMultiChoiceFitMode;
    readonly overflowMode: ScopeMultiChoiceOverflowMode;
  };
  readonly layout: {
    readonly titleBoxX: number;
    readonly titleBoxY: number;
    readonly titleBoxMinWidth: number;
    readonly titleBoxPaddingX: number;
    readonly titleBoxHeight: number;
    readonly titleBoxRadius: number;
    readonly titleBoxRightRadius: number | undefined;
    readonly titleBoxBottomRadius: number;
    readonly centerTitleBoxOnTrack: boolean;
    readonly trackX: number;
    readonly trackY: number;
    readonly trackYWithoutTitle: number;
    readonly trackWidth: number;
    readonly optionMinWidth: number;
    readonly optionMaxWidth: number;
    readonly optionHeight: number;
    readonly optionGapX: number;
    readonly optionGapY: number;
    readonly distributeRowSpace: boolean;
    readonly maxExtraWidthPerOption: number;
    readonly maxOptions: number;
    readonly optionPaddingX: number;
    readonly outerPadX: number;
    readonly outerPadY: number;
    readonly outerRadius: number;
    readonly outerPaddingRight: number;
    readonly outerPaddingBottom: number;
  };
  readonly outerEdge: {
    readonly strokeWidth: number;
    readonly glowStrokeWidth: number;
  };
  readonly titleBox: {
    readonly strokeWidth: number;
    readonly innerStrokeWidth: number;
    readonly glowStrokeWidth: number;
  };
  readonly track: {
    readonly radius: number;
    readonly strokeWidth: number;
    readonly innerStrokeWidth: number;
    readonly glowStrokeWidth: number;
  };
  readonly optionButton: {
    readonly inset: number;
    readonly insetX: number;
    readonly insetY: number;
    readonly radius: number;
    readonly strokeWidth: number;
    readonly glowStrokeWidth: number;
  };
  readonly indicator: {
    readonly radius: number;
    readonly strokeWidth: number;
    readonly circleRadius: number;
    readonly circleStrokeWidth: number;
    readonly outerRingRadiusOffset: number;
    readonly outerRingStrokeWidth: number;
  };
  readonly text: {
    readonly title: string;
    readonly options: readonly ScopeMultiChoiceOption[];
    readonly titleFontSize: number;
    readonly optionFontSize: number;
    readonly fontWeight: number;
    readonly optionFontWeight: number;
    readonly fontFamily: string;
  };
  readonly colors: ScopeMultiChoiceColors;
  readonly opacity: ScopeMultiChoiceOpacity;
  readonly hover: {
    readonly pressScale: number;
  };
  readonly transition: {
    readonly root: string;
    readonly svg: string;
  };
};

export type ScopeMultiChoiceConfigOverride = {
  readonly [K in keyof ScopeMultiChoiceConfig]?: ScopeMultiChoiceConfig[K] extends object
    ? Partial<ScopeMultiChoiceConfig[K]>
    : ScopeMultiChoiceConfig[K];
};

export type ScopeMultiChoicePlacement = {
  x: number;
  y: number;
  width: number;
  height: number;
  row: number;
};

export type ScopeMultiChoiceMetrics = {
  readonly svgWidth: number;
  readonly svgHeight: number;
  readonly titleBoxX: number;
  readonly titleBoxY: number;
  readonly titleBoxWidth: number;
  readonly titleCenterX: number;
  readonly trackX: number;
  readonly trackY: number;
  readonly trackWidth: number;
  readonly trackHeight: number;
  readonly placements: readonly ScopeMultiChoicePlacement[];
};

export type ScopeMultiChoiceSvgProps = {
  readonly x?: number;
  readonly y?: number;
  readonly renderMode?: 'html' | 'svg';
  readonly selected?: readonly string[];
  readonly defaultSelected?: readonly string[];
  readonly title?: string;
  readonly showTitle?: boolean;
  readonly options?: readonly ScopeMultiChoiceOption[];
  readonly width?: number;
  readonly height?: number;
  readonly fitMode?: ScopeMultiChoiceFitMode;
  readonly overflowMode?: ScopeMultiChoiceOverflowMode;
  readonly multiSelect?: boolean;
  readonly disabled?: boolean;
  readonly className?: string;
  readonly style?: CSSProperties;
  readonly titleRenderer?: ScopeMultiChoiceTitleRenderer;
  readonly onChange?: (selected: readonly string[], option: ScopeMultiChoiceOption, index: number) => void;
  readonly config?: ScopeMultiChoiceConfigOverride;
};
