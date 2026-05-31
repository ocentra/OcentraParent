import type { ActionConfig, ActionConfigOverride } from './ActionTypes';

export const defaultActionConfig: ActionConfig = {
  svg: {
    w: 256,
    h: 256,
    bg: 'transparent',
    border: '#36d8ff',
    borderWidth: 0,
    radius: 18,
    glowOpacity: 0,
  },
  arrow: {
    x: -9,
    y: 84,
    w: 278,
    h: 82,
    scale: 1,
    headW: 62,
    headH: 185,
    cornerRound: 7.5,
    fillTop: '#09616d',
    fillBottom: '#0b2900',
    stroke: '#00ccff',
    strokeWidth: 3,
    innerStroke: '#ffffff',
    innerStrokeWidth: 0.25,
    shineOpacity: 0,
    shineY: 0,
    shineHeight: 0.02,
    glowOpacity: 1,
  },
  bolt: {
    x: -8,
    y: -2,
    w: 204,
    h: 260,
    scale: 1.07,
    cornerRound: 0,
    topX: 0.8,
    topRightX: 0.82,
    upperLeftX: 0.09,
    upperY: 0.49,
    centerLeftX: 0.57,
    centerRightX: 0.64,
    lowerRightX: 1,
    lowerY: 0.36,
    bottomX: 0.29,
    fillTop: '#fff58a',
    fillBottom: '#f08c00',
    stroke: '#fff2a8',
    strokeWidth: 5,
    innerStroke: '#fffbd1',
    innerStrokeWidth: 3.5,
    shineOpacity: 0.48,
    shineY: 0,
    shineHeight: 0.02,
    glowOpacity: 0.82,
  },
};

export function mergeActionConfig(override?: ActionConfigOverride): ActionConfig {
  if (!override) {
    return defaultActionConfig;
  }

  return {
    svg: { ...defaultActionConfig.svg, ...override.svg },
    arrow: { ...defaultActionConfig.arrow, ...override.arrow },
    bolt: { ...defaultActionConfig.bolt, ...override.bolt },
  };
}
