import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import {
  defaultPortalAppLayoutContentDraft,
  type PortalAppLayoutContentDraft,
} from '@ocentra-parent/portal-domain/app-layout';
import {
  normalizePictureViewerFrameControls,
  type PictureViewerFrameSurfaceControls,
} from '../../../vendor/ocentra-parent-core-ui/Common/PictureViewerFrame/PictureViewerFrameControls';
import {
  DEFAULT_PARENT_PORTAL_SVG_CONTROLS,
  normalizeParentPortalSvgControls,
  type ParentPortalSvgControls,
} from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';

export type PortalFrameShellLayout = {
  readonly sidebarWidth: number;
  readonly shellEdge: number;
  readonly bodyInset: number;
  readonly frameGap: number;
  readonly sideBottomHeight: number;
  readonly sideStackGap: number;
};

export type PortalFramePreviewLayout = {
  readonly showContent: boolean;
};

export type PortalGoldenCardLayout = {
  readonly showContentBounds: boolean;
  readonly showContent: boolean;
  readonly showFrameBounds: boolean;
  readonly height: number;
  readonly paddingX: number;
  readonly paddingY: number;
  readonly contentInsetX: number;
  readonly contentInsetY: number;
  readonly contentOffsetX: number;
  readonly contentOffsetY: number;
  readonly frameInset: number;
  readonly frameWidthAdjust: number;
  readonly frameHeightAdjust: number;
  readonly frameGroup: {
    readonly offsetX: number;
    readonly offsetY: number;
  };
  readonly frameOpacity: number;
  readonly frameGlowBlur: number;
  readonly frameGlowOpacity: number;
  readonly glyphSize: number;
  readonly titleOffsetX: number;
  readonly titleOffsetY: number;
};

export type PortalCarouselLayout = {
  readonly minHeight: number;
  readonly paddingTop: number;
  readonly paddingRight: number;
  readonly paddingBottom: number;
  readonly paddingLeft: number;
  readonly frameInsetX: number;
  readonly frameInsetY: number;
  readonly frameWidthAdjust: number;
  readonly frameHeightAdjust: number;
  readonly frameGroup: {
    readonly offsetX: number;
    readonly offsetY: number;
  };
  readonly frameOpacity: number;
  readonly titleLeft: number;
  readonly titleRight: number;
  readonly titleTop: number;
  readonly railGap: number;
  readonly railPadTop: number;
  readonly railPadRight: number;
  readonly railPadBottom: number;
  readonly railPadLeft: number;
  readonly cardMinWidth: number;
  readonly cardWidthPercent: number;
};

export type PortalFrameContentTargetLayout = {
  readonly showContentBounds: boolean;
  readonly showContent: boolean;
  readonly showFrameBounds: boolean;
  readonly insetX: number;
  readonly insetY: number;
  readonly offsetX: number;
  readonly offsetY: number;
  readonly slotHeightAdjust: number;
  readonly slotWidthAdjust: number;
  readonly gap: number;
};

export type PortalFrameContentLayout = {
  readonly sideTop: PortalFrameContentTargetLayout;
  readonly sideBottom: PortalFrameContentTargetLayout;
  readonly main: PortalFrameContentTargetLayout;
};

export type PortalParentPortalLayout = {
  readonly mainApp: ParentPortalSvgControls;
  readonly chatInterface: ParentPortalSvgControls;
  readonly contentDraft: PortalAppLayoutContentDraft;
};

export type PortalFrameLayout = {
  readonly carousel: PortalCarouselLayout;
  readonly content: PortalFrameContentLayout;
  readonly goldenCard: PortalGoldenCardLayout;
  readonly parentPortal: PortalParentPortalLayout;
  readonly shell: PortalFrameShellLayout;
  readonly preview: PortalFramePreviewLayout;
  readonly sideTop: PictureViewerFrameSurfaceControls;
  readonly sideBottom: PictureViewerFrameSurfaceControls;
  readonly main: PictureViewerFrameSurfaceControls;
};

const text = PortalFrameTuner;

export const DEFAULT_PORTAL_FRAME_LAYOUT: PortalFrameLayout = {
  carousel: {
    minHeight: 270,
    paddingTop: 42,
    paddingRight: 48,
    paddingBottom: 28,
    paddingLeft: 48,
    frameInsetX: 0,
    frameInsetY: 0,
    frameWidthAdjust: 0,
    frameHeightAdjust: 0,
    frameGroup: {
      offsetX: 0,
      offsetY: 0,
    },
    frameOpacity: 1,
    titleLeft: 0,
    titleRight: 0,
    titleTop: 0,
    railGap: 16,
    railPadTop: 18,
    railPadRight: 3,
    railPadBottom: 20,
    railPadLeft: 3,
    cardMinWidth: 292,
    cardWidthPercent: 31,
  },
  content: {
    sideTop: contentTarget(14, 22, 7, true),
    sideBottom: contentTarget(14, 14, 8, true),
    main: contentTarget(22, 22, 10, true),
  },
  goldenCard: {
    showContentBounds: false,
    showContent: true,
    showFrameBounds: false,
    height: 182,
    paddingX: 20,
    paddingY: 26,
    contentInsetX: 20,
    contentInsetY: 24,
    contentOffsetX: 0,
    contentOffsetY: 0,
    frameInset: -1,
    frameWidthAdjust: 0,
    frameHeightAdjust: 0,
    frameGroup: {
      offsetX: 0,
      offsetY: 0,
    },
    frameOpacity: 0.9,
    frameGlowBlur: 7,
    frameGlowOpacity: 0.22,
    glyphSize: 42,
    titleOffsetX: 0,
    titleOffsetY: 0,
  },
  parentPortal: defaultParentPortalLayout(),
  shell: {
    sidebarWidth: 270,
    shellEdge: 8,
    bodyInset: 22,
    frameGap: -2,
    sideBottomHeight: 126,
    sideStackGap: 10,
  },
  preview: {
    showContent: true,
  },
  sideTop: normalizePictureViewerFrameControls({
    orientation: text.Orientation.Portrait,
    viewBox: { w: 960, h: 1800 },
    frameSpace: { w: 1800, h: 960 },
    frameGroup: { inset: 4, offsetX: 0, offsetY: 0 },
    outerAnchor: { sideInset: 20, topInset: 20, bottomInset: 20 },
    innerAnchor: { sideInset: 54, topInset: 58, bottomInset: 58 },
    outerFrame: frame(text.Color.Cyan, 86, 420, 0.24, 12, 5, 0.86),
    innerFrame: frame(text.Color.Cyan, 64, 340, 0, 8, 3, 0.64),
  } as Partial<PictureViewerFrameSurfaceControls>),
  sideBottom: normalizePictureViewerFrameControls({
    orientation: text.Orientation.Landscape,
    viewBox: { w: 1000, h: 420 },
    frameSpace: { w: 1000, h: 420 },
    frameGroup: { inset: 4, offsetX: 0, offsetY: 0 },
    outerAnchor: { sideInset: 20, topInset: 20, bottomInset: 20 },
    innerAnchor: { sideInset: 46, topInset: 48, bottomInset: 48 },
    outerFrame: frame(text.Color.Cyan, 42, 360, 0.22, 10, 4, 0.84),
    innerFrame: frame(text.Color.Cyan, 28, 300, 0, 7, 2, 0.62),
  } as Partial<PictureViewerFrameSurfaceControls>),
  main: normalizePictureViewerFrameControls({
    orientation: text.Orientation.Landscape,
    viewBox: { w: 1800, h: 960 },
    frameSpace: { w: 1800, h: 960 },
    frameGroup: { inset: 4, offsetX: 0, offsetY: 0 },
    outerAnchor: { sideInset: 20, topInset: 20, bottomInset: 20 },
    innerAnchor: { sideInset: 58, topInset: 54, bottomInset: 54 },
    outerFrame: frame(text.Color.Cyan, 62, 520, 0.22, 11, 5, 0.86),
    innerFrame: frame(text.Color.Cyan, 42, 420, 0, 8, 3, 0.62),
  } as Partial<PictureViewerFrameSurfaceControls>),
};

export function defaultParentPortalLayout(): PortalParentPortalLayout {
  return {
    mainApp: defaultParentPortalControls(),
    chatInterface: defaultParentPortalControls(),
    contentDraft: defaultPortalAppLayoutContentDraft(),
  };
}

export function defaultParentPortalControls(): ParentPortalSvgControls {
  return normalizeParentPortalSvgControls({
    ...DEFAULT_PARENT_PORTAL_SVG_CONTROLS,
    layout: {
      ...DEFAULT_PARENT_PORTAL_SVG_CONTROLS.layout,
      topY: 15,
    },
  });
}

function contentTarget(
  insetX: number,
  insetY: number,
  gap: number,
  showBounds: boolean
): PortalFrameContentTargetLayout {
  return {
    showContentBounds: showBounds,
    showContent: true,
    showFrameBounds: showBounds,
    insetX,
    insetY,
    offsetX: 0,
    offsetY: 0,
    slotHeightAdjust: 0,
    slotWidthAdjust: 0,
    gap,
  };
}

function frame(
  color: (typeof PortalFrameTuner.Color)[keyof typeof PortalFrameTuner.Color],
  cornerCut: number,
  tabWidth: number,
  glowOpacity: number,
  glowBlur: number,
  glowWidth: number,
  opacity: number
) {
  return {
    color,
    glowColor: color,
    cornerCut,
    topRise: 0,
    topStepInset: 0,
    topStepWidth: tabWidth,
    bottomTabWidth: tabWidth,
    bottomTabDepth: 0,
    bottomTabInset: 0,
    glowEnabled: glowOpacity > 0,
    glowOpacity,
    glowBlur,
    glowWidthBoost: glowWidth,
    outlineOpacity: 0.72,
    outlineWidthBoost: 2,
    opacity,
  };
}
