import { memo, useId, useMemo } from 'react';

import { mergeActionConfig } from './ActionConfig';
import { buildActionArrowPath, buildActionBoltPath, getActionCenterTransform } from './ActionGeometry';
import type { ActionProps } from './ActionTypes';

export const Action = memo(function Action({
  x = 0,
  y = 0,
  width,
  height,
  title = 'Action',
  className,
  preserveAspectRatio = 'xMidYMid meet',
  config: configOverride,
}: ActionProps) {
  const rawId = useId();
  const uid = rawId.replace(/[^a-zA-Z0-9_-]/g, '');
  const config = useMemo(() => mergeActionConfig(configOverride), [configOverride]);
  const { svg, arrow, bolt } = config;
  const arrowPath = useMemo(() => buildActionArrowPath(arrow), [arrow]);
  const boltPath = useMemo(() => buildActionBoltPath(bolt), [bolt]);
  const arrowTransform = useMemo(() => getActionCenterTransform(arrow), [arrow]);
  const boltTransform = useMemo(() => getActionCenterTransform(bolt), [bolt]);
  const renderW = width ?? svg.w;
  const renderH = height ?? svg.h;
  const arrowShineY1 = arrow.y + arrow.h * arrow.shineY;
  const arrowShineY2 = arrowShineY1 + arrow.h * arrow.shineHeight;
  const boltShineY1 = bolt.y + bolt.h * bolt.shineY;
  const boltShineY2 = boltShineY1 + bolt.h * bolt.shineHeight;

  return (
    <svg
      x={x}
      y={y}
      width={renderW}
      height={renderH}
      viewBox={`0 0 ${svg.w} ${svg.h}`}
      role="img"
      aria-label={title}
      className={className}
      preserveAspectRatio={preserveAspectRatio}
      style={{ overflow: 'visible' }}
    >
      <title>{title}</title>
      <defs>
        <filter id={`${uid}-frameGlow`} x="-30%" y="-30%" width="160%" height="160%">
          <feGaussianBlur stdDeviation="3" result="blur" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
        <filter id={`${uid}-arrowGlow`} x="-40%" y="-80%" width="180%" height="260%">
          <feGaussianBlur stdDeviation="5" result="blur" />
          <feColorMatrix
            in="blur"
            type="matrix"
            values={`0 0 0 0 0.12 0 0 0 0 0.84 0 0 0 0 1 0 0 0 ${arrow.glowOpacity} 0`}
            result="coloredBlur"
          />
          <feMerge>
            <feMergeNode in="coloredBlur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
        <filter id={`${uid}-boltGlow`} x="-60%" y="-60%" width="220%" height="220%">
          <feGaussianBlur stdDeviation="5" result="blur" />
          <feColorMatrix
            in="blur"
            type="matrix"
            values={`0 0 0 0 1 0 0 0 0 0.78 0 0 0 0 0.08 0 0 0 ${bolt.glowOpacity} 0`}
            result="coloredBlur"
          />
          <feMerge>
            <feMergeNode in="coloredBlur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
        <linearGradient
          id={`${uid}-arrowFill`}
          x1={arrow.x}
          y1={arrow.y}
          x2={arrow.x}
          y2={arrow.y + arrow.h}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0" stopColor={arrow.fillTop} />
          <stop offset="0.48" stopColor="#111827" />
          <stop offset="1" stopColor={arrow.fillBottom} />
        </linearGradient>
        <linearGradient
          id={`${uid}-boltFill`}
          x1={bolt.x}
          y1={bolt.y}
          x2={bolt.x + bolt.w}
          y2={bolt.y + bolt.h}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0" stopColor={bolt.fillTop} />
          <stop offset="0.45" stopColor="#ffd43b" />
          <stop offset="1" stopColor={bolt.fillBottom} />
        </linearGradient>
        <linearGradient
          id={`${uid}-arrowShine`}
          x1={arrow.x}
          y1={arrowShineY1}
          x2={arrow.x}
          y2={arrowShineY2}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0" stopColor="#ffffff" stopOpacity="0" />
          <stop offset="0.45" stopColor="#ffffff" stopOpacity={arrow.shineOpacity} />
          <stop offset="1" stopColor="#ffffff" stopOpacity="0" />
        </linearGradient>
        <linearGradient
          id={`${uid}-boltShine`}
          x1={bolt.x}
          y1={boltShineY1}
          x2={bolt.x + bolt.w}
          y2={boltShineY2}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0" stopColor="#ffffff" stopOpacity="0" />
          <stop offset="0.4" stopColor="#ffffff" stopOpacity={bolt.shineOpacity} />
          <stop offset="1" stopColor="#ffffff" stopOpacity="0" />
        </linearGradient>
      </defs>
      <rect width={svg.w} height={svg.h} fill={svg.bg} />
      {svg.borderWidth > 0 ? (
        <rect
          x={svg.borderWidth / 2}
          y={svg.borderWidth / 2}
          width={svg.w - svg.borderWidth}
          height={svg.h - svg.borderWidth}
          rx={svg.radius}
          ry={svg.radius}
          fill="none"
          stroke={svg.border}
          strokeWidth={svg.borderWidth}
          opacity={0.65 + svg.glowOpacity * 0.35}
          filter={`url(#${uid}-frameGlow)`}
        />
      ) : null}
      <g id={`${uid}-arrow-child`} transform={arrowTransform} filter={`url(#${uid}-arrowGlow)`}>
        <path
          d={arrowPath}
          fill={`url(#${uid}-arrowFill)`}
          stroke={arrow.stroke}
          strokeWidth={arrow.strokeWidth}
          strokeLinejoin="round"
        />
        <path d={arrowPath} fill={`url(#${uid}-arrowShine)`} stroke="none" />
        <path
          d={arrowPath}
          fill="none"
          stroke={arrow.innerStroke}
          strokeWidth={arrow.innerStrokeWidth}
          strokeLinejoin="round"
          opacity="0.7"
        />
      </g>
      <g id={`${uid}-thunder-bolt-child`} transform={boltTransform} filter={`url(#${uid}-boltGlow)`}>
        <path
          d={boltPath}
          fill={`url(#${uid}-boltFill)`}
          stroke={bolt.stroke}
          strokeWidth={bolt.strokeWidth}
          strokeLinejoin="round"
        />
        <path d={boltPath} fill={`url(#${uid}-boltShine)`} stroke="none" />
        <path
          d={boltPath}
          fill="none"
          stroke={bolt.innerStroke}
          strokeWidth={bolt.innerStrokeWidth}
          strokeLinejoin="round"
          opacity="0.72"
        />
      </g>
    </svg>
  );
});
