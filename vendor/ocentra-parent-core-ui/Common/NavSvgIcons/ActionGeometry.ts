import { defaultActionConfig } from './ActionConfig';
import type { ActionArrowConfig, ActionBoltConfig, ActionPoint } from './ActionTypes';

export function clampActionNumber(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

export function roundActionClosedPolygon(points: readonly ActionPoint[], radius: number): string {
  if (!points.length || radius <= 0) {
    return `${points.map((point, index) => `${index === 0 ? 'M' : 'L'} ${point.x} ${point.y}`).join(' ')} Z`;
  }

  const parts: string[] = [];

  points.forEach((current, index) => {
    const prev = points[(index - 1 + points.length) % points.length] ?? current;
    const next = points[(index + 1) % points.length] ?? current;
    const prevVector = { x: prev.x - current.x, y: prev.y - current.y };
    const nextVector = { x: next.x - current.x, y: next.y - current.y };
    const prevLength = Math.hypot(prevVector.x, prevVector.y) || 1;
    const nextLength = Math.hypot(nextVector.x, nextVector.y) || 1;
    const safeRadius = clampActionNumber(radius, 0, Math.min(prevLength, nextLength) * 0.42);
    const start = {
      x: current.x + (prevVector.x / prevLength) * safeRadius,
      y: current.y + (prevVector.y / prevLength) * safeRadius,
    };
    const end = {
      x: current.x + (nextVector.x / nextLength) * safeRadius,
      y: current.y + (nextVector.y / nextLength) * safeRadius,
    };

    parts.push(`${index === 0 ? 'M' : 'L'} ${start.x} ${start.y}`);
    parts.push(`Q ${current.x} ${current.y} ${end.x} ${end.y}`);
  });

  parts.push('Z');
  return parts.join(' ');
}

export function buildActionArrowPath(arrow: ActionArrowConfig): string {
  const arrowMidY = arrow.y + arrow.h / 2;
  const arrowHeadX = arrow.x + arrow.w - arrow.headW;
  const points: readonly ActionPoint[] = [
    { x: arrow.x, y: arrow.y },
    { x: arrowHeadX, y: arrow.y },
    { x: arrowHeadX, y: arrowMidY - arrow.headH / 2 },
    { x: arrow.x + arrow.w, y: arrowMidY },
    { x: arrowHeadX, y: arrowMidY + arrow.headH / 2 },
    { x: arrowHeadX, y: arrow.y + arrow.h },
    { x: arrow.x, y: arrow.y + arrow.h },
  ];

  return roundActionClosedPolygon(points, arrow.cornerRound);
}

export function buildActionBoltPath(bolt: ActionBoltConfig): string {
  const { x, y, w, h } = bolt;
  const points: readonly ActionPoint[] = [
    { x: x + w * bolt.topX, y },
    { x: x + w * bolt.upperLeftX, y: y + h * bolt.upperY },
    { x: x + w * bolt.centerLeftX, y: y + h * bolt.upperY },
    { x: x + w * bolt.bottomX, y: y + h },
    { x: x + w * bolt.lowerRightX, y: y + h * bolt.lowerY },
    { x: x + w * bolt.centerRightX, y: y + h * bolt.lowerY },
    { x: x + w * bolt.topRightX, y },
  ];

  return roundActionClosedPolygon(points, bolt.cornerRound);
}

export function getActionCenterTransform(item: {
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
  readonly scale?: number;
}): string {
  const scale = item.scale ?? 1;
  const cx = item.x + item.w / 2;
  const cy = item.y + item.h / 2;
  return `translate(${cx} ${cy}) scale(${scale}) translate(${-cx} ${-cy})`;
}

export function runActionSmokeTests(): readonly { readonly name: string; readonly pass: boolean }[] {
  const arrowPath = buildActionArrowPath(defaultActionConfig.arrow);
  const boltPath = buildActionBoltPath(defaultActionConfig.bolt);
  const arrowTransform = getActionCenterTransform(defaultActionConfig.arrow);
  const boltTransform = getActionCenterTransform(defaultActionConfig.bolt);

  return [
    { name: 'arrow path starts with M and closes with Z', pass: arrowPath.startsWith('M ') && arrowPath.endsWith('Z') },
    { name: 'bolt path starts with M and closes with Z', pass: boltPath.startsWith('M ') && boltPath.endsWith('Z') },
    { name: 'arrow uses rounded curve commands', pass: arrowPath.includes('Q ') },
    { name: 'bolt can be sharp when corner round is zero', pass: !boltPath.includes('Q ') },
    { name: 'arrow transform contains scale', pass: arrowTransform.includes('scale(1)') },
    { name: 'bolt transform contains configured scale', pass: boltTransform.includes('scale(1.07)') },
    {
      name: 'svg defaults to 256 square',
      pass: defaultActionConfig.svg.w === 256 && defaultActionConfig.svg.h === 256,
    },
  ];
}
