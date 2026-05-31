export type ActionFrameConfig = {
  readonly w: number;
  readonly h: number;
  readonly bg: string;
  readonly border: string;
  readonly borderWidth: number;
  readonly radius: number;
  readonly glowOpacity: number;
};

export type ActionArrowConfig = {
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
  readonly scale: number;
  readonly headW: number;
  readonly headH: number;
  readonly cornerRound: number;
  readonly fillTop: string;
  readonly fillBottom: string;
  readonly stroke: string;
  readonly strokeWidth: number;
  readonly innerStroke: string;
  readonly innerStrokeWidth: number;
  readonly shineOpacity: number;
  readonly shineY: number;
  readonly shineHeight: number;
  readonly glowOpacity: number;
};

export type ActionBoltConfig = {
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
  readonly scale: number;
  readonly cornerRound: number;
  readonly topX: number;
  readonly topRightX: number;
  readonly upperLeftX: number;
  readonly upperY: number;
  readonly centerLeftX: number;
  readonly centerRightX: number;
  readonly lowerRightX: number;
  readonly lowerY: number;
  readonly bottomX: number;
  readonly fillTop: string;
  readonly fillBottom: string;
  readonly stroke: string;
  readonly strokeWidth: number;
  readonly innerStroke: string;
  readonly innerStrokeWidth: number;
  readonly shineOpacity: number;
  readonly shineY: number;
  readonly shineHeight: number;
  readonly glowOpacity: number;
};

export type ActionConfig = {
  readonly svg: ActionFrameConfig;
  readonly arrow: ActionArrowConfig;
  readonly bolt: ActionBoltConfig;
};

export type ActionConfigOverride = {
  readonly svg?: Partial<ActionFrameConfig>;
  readonly arrow?: Partial<ActionArrowConfig>;
  readonly bolt?: Partial<ActionBoltConfig>;
};

export type ActionPoint = {
  readonly x: number;
  readonly y: number;
};

export type ActionProps = {
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly title?: string;
  readonly className?: string;
  readonly preserveAspectRatio?: string;
  readonly config?: ActionConfigOverride;
};
