import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  PortalFrameTuner,
  portalFrameCssNumber,
  portalFrameCssOpacity,
  portalFrameCssPercent,
  portalFrameCssPixel,
  type PortalFrameCssStyle,
} from '@ocentra-parent/portal-domain/frame-tuner';
import {
  getPictureViewerFrameSpaceForOrientation,
  type PictureViewerFrameSurfaceControls,
} from '../../../vendor/ocentra-parent-core-ui/Common/PictureViewerFrame/PictureViewerFrameControls';
import type {
  PortalCarouselLayout,
  PortalFrameContentTargetLayout,
  PortalGoldenCardLayout,
} from './portal-frame-layout-types';

export function carouselStyle(carousel: PortalCarouselLayout): PortalFrameCssStyle {
  return {
    [PortalFrameTuner.CssVar.CarouselCardMinWidth]: portalFrameCssPixel(carousel.cardMinWidth),
    [PortalFrameTuner.CssVar.CarouselCardWidthPercent]: portalFrameCssPercent(carousel.cardWidthPercent),
    [PortalFrameTuner.CssVar.CarouselFrameHeightAdjust]: portalFrameCssPixel(carousel.frameHeightAdjust),
    [PortalFrameTuner.CssVar.CarouselFrameInsetX]: portalFrameCssPixel(carousel.frameInsetX),
    [PortalFrameTuner.CssVar.CarouselFrameInsetY]: portalFrameCssPixel(carousel.frameInsetY),
    [PortalFrameTuner.CssVar.CarouselFrameOffsetX]: portalFrameCssPixel(carousel.frameGroup.offsetX),
    [PortalFrameTuner.CssVar.CarouselFrameOffsetY]: portalFrameCssPixel(carousel.frameGroup.offsetY),
    [PortalFrameTuner.CssVar.CarouselFrameOpacity]: portalFrameCssNumber(carousel.frameOpacity),
    [PortalFrameTuner.CssVar.CarouselFrameWidthAdjust]: portalFrameCssPixel(carousel.frameWidthAdjust),
    [PortalFrameTuner.CssVar.CarouselMinHeight]: portalFrameCssPixel(carousel.minHeight),
    [PortalFrameTuner.CssVar.CarouselPaddingBottom]: portalFrameCssPixel(carousel.paddingBottom),
    [PortalFrameTuner.CssVar.CarouselPaddingLeft]: portalFrameCssPixel(carousel.paddingLeft),
    [PortalFrameTuner.CssVar.CarouselPaddingRight]: portalFrameCssPixel(carousel.paddingRight),
    [PortalFrameTuner.CssVar.CarouselPaddingTop]: portalFrameCssPixel(carousel.paddingTop),
    [PortalFrameTuner.CssVar.CarouselRailGap]: portalFrameCssPixel(carousel.railGap),
    [PortalFrameTuner.CssVar.CarouselRailPaddingBottom]: portalFrameCssPixel(carousel.railPadBottom),
    [PortalFrameTuner.CssVar.CarouselRailPaddingLeft]: portalFrameCssPixel(carousel.railPadLeft),
    [PortalFrameTuner.CssVar.CarouselRailPaddingRight]: portalFrameCssPixel(carousel.railPadRight),
    [PortalFrameTuner.CssVar.CarouselRailPaddingTop]: portalFrameCssPixel(carousel.railPadTop),
    [PortalFrameTuner.CssVar.CarouselTitleLeft]: portalFrameCssPixel(carousel.titleLeft),
    [PortalFrameTuner.CssVar.CarouselTitleRight]: portalFrameCssPixel(carousel.titleRight),
    [PortalFrameTuner.CssVar.CarouselTitleTop]: portalFrameCssPixel(carousel.titleTop),
  };
}

export function goldenCardStyle(goldenCard: PortalGoldenCardLayout): PortalFrameCssStyle {
  return {
    [PortalFrameTuner.CssVar.GoldenCardContentBoundsOpacity]: portalFrameCssOpacity(goldenCard.showContentBounds),
    [PortalFrameTuner.CssVar.GoldenCardContentInsetX]: portalFrameCssPixel(goldenCard.contentInsetX),
    [PortalFrameTuner.CssVar.GoldenCardContentInsetY]: portalFrameCssPixel(goldenCard.contentInsetY),
    [PortalFrameTuner.CssVar.GoldenCardContentOffsetX]: portalFrameCssPixel(goldenCard.contentOffsetX),
    [PortalFrameTuner.CssVar.GoldenCardContentOffsetY]: portalFrameCssPixel(goldenCard.contentOffsetY),
    [PortalFrameTuner.CssVar.GoldenCardContentOpacity]: portalFrameCssOpacity(goldenCard.showContent),
    [PortalFrameTuner.CssVar.GoldenCardFrameBoundsOpacity]: portalFrameCssOpacity(goldenCard.showFrameBounds),
    [PortalFrameTuner.CssVar.GoldenCardFrameGlowBlur]: portalFrameCssPixel(goldenCard.frameGlowBlur),
    [PortalFrameTuner.CssVar.GoldenCardFrameGlowOpacity]: portalFrameCssNumber(goldenCard.frameGlowOpacity),
    [PortalFrameTuner.CssVar.GoldenCardFrameHeightAdjust]: portalFrameCssPixel(goldenCard.frameHeightAdjust),
    [PortalFrameTuner.CssVar.GoldenCardFrameInset]: portalFrameCssPixel(goldenCard.frameInset),
    [PortalFrameTuner.CssVar.GoldenCardFrameOffsetX]: portalFrameCssPixel(goldenCard.frameGroup.offsetX),
    [PortalFrameTuner.CssVar.GoldenCardFrameOffsetY]: portalFrameCssPixel(goldenCard.frameGroup.offsetY),
    [PortalFrameTuner.CssVar.GoldenCardFrameOpacity]: portalFrameCssNumber(goldenCard.frameOpacity),
    [PortalFrameTuner.CssVar.GoldenCardFrameWidthAdjust]: portalFrameCssPixel(goldenCard.frameWidthAdjust),
    [PortalFrameTuner.CssVar.GoldenCardGlyphSize]: portalFrameCssPixel(goldenCard.glyphSize),
    [PortalFrameTuner.CssVar.GoldenCardHeight]: portalFrameCssPixel(goldenCard.height),
    [PortalFrameTuner.CssVar.GoldenCardPaddingX]: portalFrameCssPixel(goldenCard.paddingX),
    [PortalFrameTuner.CssVar.GoldenCardPaddingY]: portalFrameCssPixel(goldenCard.paddingY),
    [PortalFrameTuner.CssVar.GoldenCardTitleOffsetX]: portalFrameCssPixel(goldenCard.titleOffsetX),
    [PortalFrameTuner.CssVar.GoldenCardTitleOffsetY]: portalFrameCssPixel(goldenCard.titleOffsetY),
  };
}

export function frameContentStyle(
  content: PortalFrameContentTargetLayout,
  controls?: PictureViewerFrameSurfaceControls
): PortalFrameCssStyle {
  return {
    ...frameContentClipStyle(controls),
    [PortalFrameTuner.CssVar.ContentGap]: portalFrameCssPixel(content.gap),
    [PortalFrameTuner.CssVar.ContentInsetX]: portalFrameCssPixel(content.insetX),
    [PortalFrameTuner.CssVar.ContentInsetY]: portalFrameCssPixel(content.insetY),
    [PortalFrameTuner.CssVar.ContentOffsetX]: portalFrameCssPixel(content.offsetX),
    [PortalFrameTuner.CssVar.ContentOffsetY]: portalFrameCssPixel(content.offsetY),
    [PortalFrameTuner.CssVar.SlotHeightAdjust]: portalFrameCssPixel(content.slotHeightAdjust),
    [PortalFrameTuner.CssVar.SlotWidthAdjust]: portalFrameCssPixel(content.slotWidthAdjust),
  };
}

export function frameHostClassName(
  baseClassName:
    | typeof PortalDom.Classes.AppMain
    | typeof PortalDom.Classes.SidebarDeviceFrame
    | typeof PortalDom.Classes.SidebarNavFrame,
  content: PortalFrameContentTargetLayout
) {
  return [
    baseClassName,
    content.showFrameBounds || content.showContentBounds ? PortalFrameTuner.Classes.FrameDebugHost : undefined,
    content.showContent ? undefined : PortalFrameTuner.Classes.FrameContentHidden,
  ]
    .filter(Boolean)
    .join(PortalDom.Classes.ClassNameSeparator);
}

function frameContentClipStyle(controls: PictureViewerFrameSurfaceControls | undefined): PortalFrameCssStyle {
  if (controls === undefined) {
    return {};
  }
  const frameSpace =
    controls.frameSpace ?? getPictureViewerFrameSpaceForOrientation(controls.viewBox, controls.orientation);
  const innerWidth = Math.max(1, frameSpace.w - controls.innerAnchor.sideInset * 2);
  const innerHeight = Math.max(
    1,
    frameSpace.h - controls.innerAnchor.topInset - controls.innerAnchor.bottomInset - controls.innerFrame.bottomTabDepth
  );
  const cornerCut = Math.max(0, controls.innerFrame.cornerCut);
  return {
    [PortalFrameTuner.CssVar.ContentClipX]: portalFrameCssPercent(percent(cornerCut, innerWidth)),
    [PortalFrameTuner.CssVar.ContentClipY]: portalFrameCssPercent(percent(cornerCut, innerHeight)),
  };
}

function percent(value: number, total: number): number {
  return Math.min(40, Math.max(0, (value / total) * 100));
}
