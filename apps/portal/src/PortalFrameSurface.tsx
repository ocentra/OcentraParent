import { useEffect, useMemo, useRef, useState, type ReactElement } from 'react';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import { PictureViewerFrame } from '../../../vendor/ocentra-parent-core-ui/Common/PictureViewerFrame/PictureViewerFrame';
import {
  getPictureViewerFrameSpaceForOrientation,
  type PictureViewerFrameSurfaceControls,
} from '../../../vendor/ocentra-parent-core-ui/Common/PictureViewerFrame/PictureViewerFrameControls';
import type { PortalFrameContentTargetLayout } from './portal-frame-layout-types';

type FrameHostSize = {
  readonly height: number;
  readonly width: number;
};

const MinMeasuredFrameSize = 1;

export function PortalFrameBackdrop({
  ariaLabel,
  controls,
}: {
  readonly ariaLabel: PortalDisplayText;
  readonly controls: PictureViewerFrameSurfaceControls;
}): ReactElement {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const hostSize = useMeasuredFrameHost(hostRef);
  const fittedControls = useMemo(() => fitFrameControlsToHost(controls, hostSize), [controls, hostSize]);
  return (
    <div aria-hidden={true} className={PortalFrameTuner.Classes.FrameBackdrop} ref={hostRef}>
      <PictureViewerFrame
        ariaLabel={ariaLabel}
        className={PortalFrameTuner.Classes.FrameBackdropSvg}
        controls={fittedControls}
      />
    </div>
  );
}

export function PortalFrameBoundsOverlay({
  content,
}: {
  readonly content: PortalFrameContentTargetLayout;
}): ReactElement | null {
  if (!content.showFrameBounds && !content.showContentBounds) {
    return null;
  }
  return (
    <>
      {content.showFrameBounds ? (
        <div aria-hidden={true} className={PortalFrameTuner.Classes.FrameOuterBoundsOverlay} />
      ) : null}
      {content.showContentBounds ? (
        <div aria-hidden={true} className={PortalFrameTuner.Classes.FrameContentBoundsOverlay} />
      ) : null}
    </>
  );
}

function useMeasuredFrameHost(ref: React.RefObject<HTMLDivElement | null>): FrameHostSize | undefined {
  const [size, setSize] = useState<FrameHostSize>();
  useEffect(() => {
    const node = ref.current;
    if (node === null) {
      return undefined;
    }
    const observer = new ResizeObserver((entries) => {
      const box = entries[0]?.contentRect;
      if (box === undefined) {
        return;
      }
      const next = {
        height: Math.max(MinMeasuredFrameSize, Math.round(box.height)),
        width: Math.max(MinMeasuredFrameSize, Math.round(box.width)),
      };
      setSize((current) => (current?.height === next.height && current.width === next.width ? current : next));
    });
    observer.observe(node);
    return () => observer.disconnect();
  }, [ref]);
  return size;
}

function fitFrameControlsToHost(
  controls: PictureViewerFrameSurfaceControls,
  hostSize: FrameHostSize | undefined
): PictureViewerFrameSurfaceControls {
  if (hostSize === undefined) {
    return controls;
  }
  const viewBox = {
    h: hostSize.height,
    w: hostSize.width,
  };
  const baseSpace = getPictureViewerFrameSpaceForOrientation(controls.viewBox, controls.orientation);
  const hostSpace = getPictureViewerFrameSpaceForOrientation(viewBox, controls.orientation);
  return {
    ...controls,
    frameSpace: {
      h: hostSpace.h + frameSpaceDelta(controls.frameSpace.h, baseSpace.h),
      w: hostSpace.w + frameSpaceDelta(controls.frameSpace.w, baseSpace.w),
    },
    viewBox,
  };
}

function frameSpaceDelta(value: number, base: number): number {
  return Number.isFinite(value - base) ? value - base : 0;
}
