import {
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type ReactElement,
  type ReactNode,
} from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalUnifiedChrome } from '@ocentra-parent/portal-domain/unified-chrome';
import {
  getPictureViewerAnchoredFrame,
  getPictureViewerFrameGroupTransform,
  getPictureViewerFramePoints,
  getPictureViewerFrameTransform,
  normalizePictureViewerFrameControls,
  pictureViewerDarkenHex,
  pictureViewerFrameSegmentThickness,
  pictureViewerFrameSegments,
  type PictureViewerFrameControls,
  type PictureViewerFrameSegment,
} from '../../../vendor/ocentra-parent-core-ui/Common/PictureViewerFrame/PictureViewerFrameControls';

type HeaderFrameSize = {
  readonly height: number;
  readonly width: number;
};

type PortalHeaderSvgFrameProps = {
  readonly children?: ReactNode;
};

const initialHeaderFrameSize: HeaderFrameSize = {
  height: 1,
  width: 1,
};

const headerFrame = {
  cornerThicknessScale: 0.5,
  edgeRunWidth: 36,
  frameGroupInset: 2,
  frameHeight: 420,
  minFrameScale: 0.01,
  outerBottomInset: 24,
  outerCornerCut: 62,
  outerOpacity: 1,
  outerOutlineBoost: 0.75,
  outerOutlineOpacity: 1,
  outerSideInset: 24,
  outerTopInset: 24,
  topBottomEdgeThickness: 1,
} as const;

function scaledSegmentThickness(value: number | undefined): number {
  return Math.max(0.5, (value ?? 0) * headerFrame.cornerThicknessScale);
}

function scaleHeaderCornerSegments(frame: PictureViewerFrameControls): PictureViewerFrameControls {
  const segmentThicknesses = frame.segmentThicknesses;
  return {
    ...frame,
    segmentThicknesses: {
      ...segmentThicknesses,
      bottomCenterRun: headerFrame.topBottomEdgeThickness,
      bottomLeftConnector: headerFrame.topBottomEdgeThickness,
      bottomLeftRunEnd: scaledSegmentThickness(segmentThicknesses.bottomLeftRunEnd),
      bottomLeftRunMid: headerFrame.topBottomEdgeThickness,
      bottomLeftRunStart: headerFrame.topBottomEdgeThickness,
      bottomRightConnector: headerFrame.topBottomEdgeThickness,
      bottomRightRunEnd: headerFrame.topBottomEdgeThickness,
      bottomRightRunMid: headerFrame.topBottomEdgeThickness,
      bottomRightRunStart: scaledSegmentThickness(segmentThicknesses.bottomRightRunStart),
      leftBottomCorner: scaledSegmentThickness(segmentThicknesses.leftBottomCorner),
      leftSideRunEnd: scaledSegmentThickness(segmentThicknesses.leftSideRunEnd),
      leftSideRunStart: scaledSegmentThickness(segmentThicknesses.leftSideRunStart),
      leftTopCorner: scaledSegmentThickness(segmentThicknesses.leftTopCorner),
      rightBottomCorner: scaledSegmentThickness(segmentThicknesses.rightBottomCorner),
      rightSideRunEnd: scaledSegmentThickness(segmentThicknesses.rightSideRunEnd),
      rightSideRunStart: scaledSegmentThickness(segmentThicknesses.rightSideRunStart),
      rightTopCorner: scaledSegmentThickness(segmentThicknesses.rightTopCorner),
      topCenterRun: headerFrame.topBottomEdgeThickness,
      topLeftConnector: headerFrame.topBottomEdgeThickness,
      topLeftRunEnd: headerFrame.topBottomEdgeThickness,
      topLeftRunMid: headerFrame.topBottomEdgeThickness,
      topLeftRunStart: scaledSegmentThickness(segmentThicknesses.topLeftRunStart),
      topRightConnector: headerFrame.topBottomEdgeThickness,
      topRightRunEnd: scaledSegmentThickness(segmentThicknesses.topRightRunEnd),
      topRightRunMid: headerFrame.topBottomEdgeThickness,
      topRightRunStart: headerFrame.topBottomEdgeThickness,
    },
  };
}

function useMeasuredHeaderFrameSize(): [HeaderFrameSize, React.RefObject<HTMLSpanElement | null>] {
  const frameRef = useRef<HTMLSpanElement>(null);
  const [size, setSize] = useState(initialHeaderFrameSize);

  useLayoutEffect(() => {
    const updateSize = () => {
      const box = frameRef.current?.getBoundingClientRect();
      if (box === undefined) {
        return;
      }
      setSize({
        height: Math.max(1, Math.round(box.height)),
        width: Math.max(1, Math.round(box.width)),
      });
    };

    updateSize();
    const observer = new ResizeObserver(updateSize);
    if (frameRef.current !== null) {
      observer.observe(frameRef.current);
    }
    return () => observer.disconnect();
  }, []);

  return [size, frameRef];
}

function headerFrameScale(size: HeaderFrameSize): number {
  return Math.max(headerFrame.minFrameScale, size.height / headerFrame.frameHeight);
}

function headerFrameControls(size: HeaderFrameSize) {
  const frameScale = headerFrameScale(size);
  const viewBoxWidth = Math.max(1, size.width / frameScale);
  const cappedStepWidth = Math.min(headerFrame.edgeRunWidth, viewBoxWidth * 0.5);
  const base = normalizePictureViewerFrameControls({
    frameGroup: { inset: headerFrame.frameGroupInset, offsetX: 0, offsetY: 0 },
    orientation: PortalUnifiedChrome.Svg.FrameOrientationLandscape,
    outerAnchor: {
      bottomInset: headerFrame.outerBottomInset,
      sideInset: headerFrame.outerSideInset,
      topInset: headerFrame.outerTopInset,
    },
    viewBox: {
      h: headerFrame.frameHeight,
      w: viewBoxWidth,
    },
  });
  const color = PortalUnifiedChrome.Svg.FrameColorCyan;

  return {
    ...base,
    outerFrame: scaleHeaderCornerSegments({
      ...base.outerFrame,
      bottomTabDepth: 0,
      bottomTabDirection: PortalUnifiedChrome.Svg.BottomTabDown,
      bottomTabInset: 0,
      bottomTabWidth: cappedStepWidth,
      color,
      cornerCut: headerFrame.outerCornerCut,
      glowEnabled: false,
      glowOpacity: 0,
      glowWidthBoost: 0,
      opacity: headerFrame.outerOpacity,
      outlineEnabled: true,
      outlineOpacity: headerFrame.outerOutlineOpacity,
      outlineWidthBoost: headerFrame.outerOutlineBoost,
      topRise: 0,
      topStepInset: 0,
      topStepWidth: cappedStepWidth,
    }),
  };
}

function headerFrameFillPath(frame: PictureViewerFrameControls): string {
  const points = getPictureViewerFramePoints(frame);
  const firstPoint = points[0];
  if (firstPoint === undefined) {
    return '';
  }
  return `${points.map((point, index) => `${index === 0 ? 'M' : 'L'}${point.x} ${point.y}`).join(' ')} Z`;
}

function HeaderFrameLines({
  frame,
  segments,
}: {
  readonly frame: PictureViewerFrameControls;
  readonly segments: readonly PictureViewerFrameSegment[];
}): ReactElement {
  return (
    <g
      className={PortalUnifiedChrome.Classes.OutlineHeaderFrameSegmentGroup}
      opacity={frame.opacity}
      pointerEvents={PortalUnifiedChrome.Svg.PointerEventsNone}
    >
      {frame.outlineEnabled ? (
        <g className={PortalUnifiedChrome.Classes.OutlineHeaderFrameOuter} opacity={frame.outlineOpacity}>
          {segments.map((segment) => (
            <path
              d={segment.d}
              fill={PortalUnifiedChrome.Svg.FillNone}
              key={`${PortalUnifiedChrome.Svg.FrameLineVariant.Outline}-${segment.id}`}
              stroke={pictureViewerDarkenHex(frame.color, 0.48)}
              strokeLinecap={frame.lineCap}
              strokeLinejoin={PortalUnifiedChrome.Svg.StrokeLinejoinRound}
              strokeWidth={pictureViewerFrameSegmentThickness(frame, segment) + frame.outlineWidthBoost}
              vectorEffect={PortalUnifiedChrome.Svg.VectorEffectNonScalingStroke}
            />
          ))}
        </g>
      ) : null}
      {segments.map((segment) => (
        <path
          className={PortalUnifiedChrome.Classes.OutlineHeaderFrameLine}
          d={segment.d}
          fill={PortalUnifiedChrome.Svg.FillNone}
          key={`${PortalUnifiedChrome.Svg.FrameLineVariant.Line}-${segment.id}`}
          stroke={frame.color}
          strokeLinecap={frame.lineCap}
          strokeLinejoin={PortalUnifiedChrome.Svg.StrokeLinejoinRound}
          strokeWidth={pictureViewerFrameSegmentThickness(frame, segment)}
          vectorEffect={PortalUnifiedChrome.Svg.VectorEffectNonScalingStroke}
        />
      ))}
    </g>
  );
}

export function PortalHeaderSvgFrame({ children }: PortalHeaderSvgFrameProps = {}): ReactElement {
  const [size, frameRef] = useMeasuredHeaderFrameSize();
  const frameControls = useMemo(() => headerFrameControls(size), [size]);
  const frameScale = headerFrameScale(size);
  const frameLocalWidth = Math.max(1, size.width / frameScale);
  const outerFrame = useMemo(
    () =>
      getPictureViewerAnchoredFrame(
        frameControls,
        PortalUnifiedChrome.Svg.FrameKeyOuter,
        PortalUnifiedChrome.Svg.AnchorKeyOuter
      ),
    [frameControls]
  );
  const outerSegments = useMemo(() => pictureViewerFrameSegments(outerFrame), [outerFrame]);
  const outerFillPath = useMemo(() => headerFrameFillPath(outerFrame), [outerFrame]);
  const outerFrameCssClipPath = `path("${outerFillPath}")`;
  const blurSurfaceStyle = useMemo(
    () =>
      ({
        clipPath: outerFrameCssClipPath,
        WebkitClipPath: outerFrameCssClipPath,
      }) satisfies CSSProperties,
    [outerFrameCssClipPath]
  );

  return (
    <span
      aria-hidden={PortalDom.Attributes.True}
      className={PortalUnifiedChrome.Classes.OutlineHeaderFrame}
      ref={frameRef}
    >
      <svg
        className={PortalUnifiedChrome.Classes.OutlineHeaderFrameSvg}
        focusable={PortalDom.Attributes.False}
        preserveAspectRatio={PortalUnifiedChrome.Svg.PreserveAspectRatioNone}
        viewBox={`0 0 ${size.width} ${size.height}`}
      >
        <g transform={`scale(${frameScale})`}>
          <g transform={getPictureViewerFrameGroupTransform(frameControls)}>
            <g transform={getPictureViewerFrameTransform(frameControls)}>
              <foreignObject
                height={headerFrame.frameHeight}
                pointerEvents={PortalUnifiedChrome.Svg.PointerEventsNone}
                width={frameLocalWidth}
                x={0}
                y={0}
              >
                <div className={PortalUnifiedChrome.Classes.OutlineHeaderFrameBlur} style={blurSurfaceStyle} />
              </foreignObject>
              <path
                className={PortalUnifiedChrome.Classes.OutlineHeaderFrameFill}
                d={outerFillPath}
                pointerEvents={PortalUnifiedChrome.Svg.PointerEventsNone}
              />
              <HeaderFrameLines frame={outerFrame} segments={outerSegments} />
            </g>
          </g>
        </g>
        {children === undefined ? null : (
          <foreignObject
            className={PortalUnifiedChrome.Classes.OutlineHeaderFrameForeignObject}
            height={size.height}
            pointerEvents={PortalUnifiedChrome.Svg.PointerEventsNone}
            width={size.width}
            x={0}
            y={0}
          >
            <div className={PortalUnifiedChrome.Classes.OutlineHeaderFrameContent}>{children}</div>
          </foreignObject>
        )}
      </svg>
    </span>
  );
}
