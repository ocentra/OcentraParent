import { useId, useMemo } from 'react';
import {
  getPictureViewerAnchoredFrame,
  getPictureViewerFramePoints,
  normalizePictureViewerFrameControls,
  pictureViewerDarkenHex,
  pictureViewerFrameSegmentThickness,
  pictureViewerFrameSegments,
  type PictureViewerFrameControls,
  type PictureViewerFrameSegment,
} from '../../Common/PictureViewerFrame/PictureViewerFrameControls';

type ParentPortalPanelFrameProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  color: string;
  active?: boolean;
  fill?: string;
  fillOpacity?: number | string;
  cornerThicknessScale?: number;
  outerTabWidth?: number | undefined;
  innerTabWidth?: number | undefined;
};

const CORNER_SEGMENT_IDS = [
  'leftTopCorner',
  'topLeftRunStart',
  'topRightRunEnd',
  'rightTopCorner',
  'rightSideRunStart',
  'rightSideRunEnd',
  'rightBottomCorner',
  'bottomRightRunStart',
  'bottomLeftRunEnd',
  'leftBottomCorner',
  'leftSideRunStart',
  'leftSideRunEnd',
] as const;

const OUTER_SEGMENT_THICKNESSES = {
  topLeftRunMid: 2,
  topRightRunMid: 2,
  bottomRightRunMid: 2,
  bottomLeftRunMid: 2,
  leftSideRunMid: 1,
  rightSideRunMid: 1,
  leftSideRunStart: 6,
  leftSideRunEnd: 6,
  leftTopCorner: 6,
  topLeftRunStart: 6,
  topRightRunEnd: 6,
  rightTopCorner: 6,
  rightSideRunStart: 6,
  rightSideRunEnd: 6,
  rightBottomCorner: 6,
  bottomRightRunStart: 6,
  bottomLeftRunEnd: 6,
  leftBottomCorner: 6,
  topLeftRunEnd: 4,
  topLeftConnector: 4,
  topCenterRun: 4,
  topRightConnector: 4,
  topRightRunStart: 4,
  bottomRightRunEnd: 4,
  bottomRightConnector: 4,
  bottomCenterRun: 4,
  bottomLeftConnector: 4,
  bottomLeftRunStart: 4,
} satisfies PictureViewerFrameControls['segmentThicknesses'];

const INNER_SEGMENT_THICKNESSES = {
  ...OUTER_SEGMENT_THICKNESSES,
  topLeftRunMid: 1,
  topRightRunMid: 1,
  bottomRightRunMid: 1,
  bottomLeftRunMid: 1,
  leftSideRunMid: 1,
  rightSideRunMid: 1,
  leftSideRunStart: 3,
  leftSideRunEnd: 3,
  leftTopCorner: 3,
  topLeftRunStart: 3,
  topRightRunEnd: 3,
  rightTopCorner: 3,
  rightSideRunStart: 3,
  rightSideRunEnd: 3,
  rightBottomCorner: 3,
  bottomRightRunStart: 3,
  bottomLeftRunEnd: 3,
  leftBottomCorner: 3,
  topLeftRunEnd: 1,
  topLeftConnector: 1,
  topCenterRun: 1,
  topRightConnector: 1,
  topRightRunStart: 1,
  bottomRightRunEnd: 1,
  bottomRightConnector: 1,
  bottomCenterRun: 1,
  bottomLeftConnector: 1,
  bottomLeftRunStart: 1,
} satisfies PictureViewerFrameControls['segmentThicknesses'];

function clampNumber(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

function scaledCornerSegments(frame: PictureViewerFrameControls, scale: number): PictureViewerFrameControls {
  if (scale === 1) {
    return frame;
  }

  const segmentThicknesses = { ...frame.segmentThicknesses };
  for (const segmentId of CORNER_SEGMENT_IDS) {
    const value = segmentThicknesses[segmentId];
    if (typeof value === 'number') {
      segmentThicknesses[segmentId] = Math.max(0.5, value * scale);
    }
  }

  return {
    ...frame,
    segmentThicknesses,
  };
}

function framePath(frame: PictureViewerFrameControls): string {
  const points = getPictureViewerFramePoints(frame);
  return points.map((point, index) => `${index === 0 ? 'M' : 'L'} ${point.x} ${point.y}`).join(' ') + ' Z';
}

function panelFrameControls({
  w,
  h,
  color,
  active,
  cornerThicknessScale,
  outerTabWidth: requestedOuterTabWidth,
  innerTabWidth: requestedInnerTabWidth,
}: {
  w: number;
  h: number;
  color: string;
  active: boolean;
  cornerThicknessScale: number;
  outerTabWidth?: number | undefined;
  innerTabWidth?: number | undefined;
}) {
  const safeW = Math.max(1, w);
  const safeH = Math.max(1, h);
  const shortest = Math.min(safeW, safeH);
  const outerCorner = clampNumber(shortest * 0.072, 14, 20);
  const innerCorner = clampNumber(shortest * 0.052, 10, 14);
  const outerTabWidth = clampNumber(requestedOuterTabWidth ?? 112, 58, Math.max(58, safeW - outerCorner * 4));
  const innerTabWidth = clampNumber(requestedInnerTabWidth ?? 88, 48, Math.max(48, safeW - innerCorner * 4));

  const base = normalizePictureViewerFrameControls({
    orientation: 'landscape',
    viewBox: { w: safeW, h: safeH },
    frameSpace: { w: safeW, h: safeH },
    frameGroup: { inset: 0, offsetX: 0, offsetY: 0 },
    outerAnchor: { sideInset: 6, topInset: 6, bottomInset: 6 },
    innerAnchor: { sideInset: 13, topInset: 14, bottomInset: 14 },
  });

  return {
    ...base,
    navArrows: {
      ...base.navArrows,
      enabled: false,
    },
    outerFrame: scaledCornerSegments(
      {
        ...base.outerFrame,
        color,
        glowColor: color,
        glowEnabled: true,
        glowOpacity: active ? 0.34 : 0.2,
        glowBlur: active ? 13 : 9,
        glowWidthBoost: active ? 5 : 3.5,
        outlineOpacity: 0.92,
        outlineWidthBoost: 2,
        topRise: 0,
        cornerCut: outerCorner,
        topStepWidth: outerTabWidth,
        topStepInset: 0,
        bottomTabWidth: outerTabWidth,
        bottomTabDepth: 0,
        bottomTabInset: 0,
        bottomTabDirection: 'down',
        topLeftThickness: 0.75,
        topCenterThickness: 1.05,
        topRightThickness: 0.75,
        leftSideThickness: 0.72,
        rightSideThickness: 0.72,
        bottomLeftThickness: 0.75,
        bottomCenterThickness: 1.05,
        bottomRightThickness: 0.75,
        topLeftStartGap: 16,
        topLeftEndGap: 16,
        topRightStartGap: 16,
        topRightEndGap: 16,
        bottomLeftStartGap: 16,
        bottomLeftEndGap: 16,
        bottomRightStartGap: 16,
        bottomRightEndGap: 16,
        leftSideStartGap: 16,
        leftSideEndGap: 16,
        rightSideStartGap: 16,
        rightSideEndGap: 16,
        lineCap: 'round',
        segmentThicknesses: OUTER_SEGMENT_THICKNESSES,
        opacity: 1,
      },
      cornerThicknessScale
    ),
    innerFrame: scaledCornerSegments(
      {
        ...base.innerFrame,
        color,
        glowColor: color,
        glowEnabled: active,
        glowOpacity: active ? 0.2 : 0,
        glowBlur: 7,
        glowWidthBoost: 3,
        outlineOpacity: 0.52,
        outlineWidthBoost: 1.3,
        topRise: 0,
        cornerCut: innerCorner,
        topStepWidth: innerTabWidth,
        topStepInset: 0,
        bottomTabWidth: innerTabWidth,
        bottomTabDepth: 0,
        bottomTabInset: 0,
        bottomTabDirection: 'down',
        topLeftThickness: 0.55,
        topCenterThickness: 0.8,
        topRightThickness: 0.55,
        leftSideThickness: 0.55,
        rightSideThickness: 0.55,
        bottomLeftThickness: 0.55,
        bottomCenterThickness: 0.8,
        bottomRightThickness: 0.55,
        topLeftStartGap: 12,
        topLeftEndGap: 12,
        topRightStartGap: 12,
        topRightEndGap: 12,
        bottomLeftStartGap: 12,
        bottomLeftEndGap: 12,
        bottomRightStartGap: 12,
        bottomRightEndGap: 12,
        leftSideStartGap: 12,
        leftSideEndGap: 12,
        rightSideStartGap: 12,
        rightSideEndGap: 12,
        lineCap: 'round',
        segmentThicknesses: INNER_SEGMENT_THICKNESSES,
        opacity: active ? 0.88 : 0.58,
      },
      cornerThicknessScale
    ),
  };
}

function ParentPortalPanelFrameLines({
  frame,
  segments,
  filterId,
}: {
  frame: PictureViewerFrameControls;
  segments: PictureViewerFrameSegment[];
  filterId: string;
}) {
  return (
    <g opacity={frame.opacity ?? 1} pointerEvents="none">
      {frame.glowEnabled ? (
        <g filter={`url(#${filterId})`} opacity={frame.glowOpacity}>
          {segments.map((segment) => (
            <path
              key={`parent-portal-panel-frame-glow-${segment.id}`}
              d={segment.d}
              fill="none"
              stroke={frame.glowColor}
              strokeWidth={pictureViewerFrameSegmentThickness(frame, segment) + frame.glowWidthBoost}
              strokeLinecap={frame.lineCap}
              strokeLinejoin="round"
              vectorEffect="non-scaling-stroke"
            />
          ))}
        </g>
      ) : null}
      {frame.outlineEnabled ? (
        <g opacity={frame.outlineOpacity}>
          {segments.map((segment) => (
            <path
              key={`parent-portal-panel-frame-outline-${segment.id}`}
              d={segment.d}
              fill="none"
              stroke={pictureViewerDarkenHex(frame.color, 0.48)}
              strokeWidth={pictureViewerFrameSegmentThickness(frame, segment) + frame.outlineWidthBoost}
              strokeLinecap={frame.lineCap}
              strokeLinejoin="round"
              vectorEffect="non-scaling-stroke"
            />
          ))}
        </g>
      ) : null}
      {segments.map((segment) => (
        <path
          key={`parent-portal-panel-frame-${segment.id}`}
          d={segment.d}
          fill="none"
          stroke={frame.color}
          strokeWidth={pictureViewerFrameSegmentThickness(frame, segment)}
          strokeLinecap={frame.lineCap}
          strokeLinejoin="round"
          vectorEffect="non-scaling-stroke"
        />
      ))}
    </g>
  );
}

export function ParentPortalPanelFrame({
  x,
  y,
  w,
  h,
  color,
  active = false,
  fill,
  fillOpacity = 1,
  cornerThicknessScale = 1,
  outerTabWidth,
  innerTabWidth,
}: ParentPortalPanelFrameProps) {
  const rawId = useId().replace(/[^a-zA-Z0-9_-]/g, '');
  const frameControls = useMemo(
    () => panelFrameControls({ w, h, color, active, cornerThicknessScale, outerTabWidth, innerTabWidth }),
    [active, color, cornerThicknessScale, h, innerTabWidth, outerTabWidth, w]
  );
  const outerFrame = useMemo(
    () => getPictureViewerAnchoredFrame(frameControls, 'outerFrame', 'outerAnchor'),
    [frameControls]
  );
  const innerFrame = useMemo(
    () => getPictureViewerAnchoredFrame(frameControls, 'innerFrame', 'innerAnchor'),
    [frameControls]
  );
  const outerSegments = useMemo(() => pictureViewerFrameSegments(outerFrame), [outerFrame]);
  const innerSegments = useMemo(() => pictureViewerFrameSegments(innerFrame), [innerFrame]);
  const outerFramePath = useMemo(() => framePath(outerFrame), [outerFrame]);
  const outerGlowId = `parentPortalPanelFrameOuterGlow-${rawId}`;
  const innerGlowId = `parentPortalPanelFrameInnerGlow-${rawId}`;

  return (
    <g transform={`translate(${x} ${y})`} pointerEvents="none">
      <defs>
        <filter id={outerGlowId} x="-40%" y="-40%" width="180%" height="180%">
          <feGaussianBlur stdDeviation={outerFrame.glowBlur} />
        </filter>
        <filter id={innerGlowId} x="-40%" y="-40%" width="180%" height="180%">
          <feGaussianBlur stdDeviation={innerFrame.glowBlur} />
        </filter>
      </defs>
      {fill ? <path d={outerFramePath} fill={fill} opacity={fillOpacity} pointerEvents="none" /> : null}
      <ParentPortalPanelFrameLines frame={outerFrame} segments={outerSegments} filterId={outerGlowId} />
      <ParentPortalPanelFrameLines frame={innerFrame} segments={innerSegments} filterId={innerGlowId} />
    </g>
  );
}
