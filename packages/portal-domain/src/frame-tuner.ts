import {
  PortalFrameTuner as PortalFrameTunerImpl,
  PortalFrameShellNumberFields as PortalFrameShellNumberFieldsImpl,
  PortalFrameContentBooleanFields as PortalFrameContentBooleanFieldsImpl,
  PortalGoldenCardBooleanFields as PortalGoldenCardBooleanFieldsImpl,
  PortalFrameContentNumberFields as PortalFrameContentNumberFieldsImpl,
  PortalGoldenCardFrameNumberFields as PortalGoldenCardFrameNumberFieldsImpl,
  PortalGoldenCardContentNumberFields as PortalGoldenCardContentNumberFieldsImpl,
  PortalCarouselFrameNumberFields as PortalCarouselFrameNumberFieldsImpl,
  PortalCarouselContentNumberFields as PortalCarouselContentNumberFieldsImpl,
  PortalCarouselRailNumberFields as PortalCarouselRailNumberFieldsImpl,
  PortalFrameSlotNumberFields as PortalFrameSlotNumberFieldsImpl,
  PortalFrameGeometryNumberFields as PortalFrameGeometryNumberFieldsImpl,
  PortalFrameChromeNumberFields as PortalFrameChromeNumberFieldsImpl,
  PortalFrameOuterShapeNumberFields as PortalFrameOuterShapeNumberFieldsImpl,
  PortalFrameInnerShapeNumberFields as PortalFrameInnerShapeNumberFieldsImpl,
  PortalFrameOuterEdgeNumberFields as PortalFrameOuterEdgeNumberFieldsImpl,
  PortalFrameInnerEdgeNumberFields as PortalFrameInnerEdgeNumberFieldsImpl,
  PortalFrameOuterGapNumberFields as PortalFrameOuterGapNumberFieldsImpl,
  PortalFrameInnerGapNumberFields as PortalFrameInnerGapNumberFieldsImpl,
  PortalFrameOuterSegmentNumberFields as PortalFrameOuterSegmentNumberFieldsImpl,
  PortalFrameInnerSegmentNumberFields as PortalFrameInnerSegmentNumberFieldsImpl,
  portalFrameCssOpacity as portalFrameCssOpacityImpl,
  portalFrameCssNumber as portalFrameCssNumberImpl,
  portalFrameCssPercent as portalFrameCssPercentImpl,
  portalFrameCssPixel as portalFrameCssPixelImpl,
} from './frame-tuner-impl';

export const PortalFrameTuner = PortalFrameTunerImpl;
export type PortalFrameTargetValue = (typeof PortalFrameTuner.FrameTarget)[keyof typeof PortalFrameTuner.FrameTarget];
export type PortalFrameTunerFrameSectionValue =
  (typeof PortalFrameTuner.FrameSection)[keyof typeof PortalFrameTuner.FrameSection];
export type PortalFrameCssStyle = Record<string, string>;
export type PortalFrameCssValue = string;
export type PortalFrameBooleanField = { readonly path: readonly string[]; readonly label: string };
export type PortalFrameNumberField = {
  readonly path: readonly string[];
  readonly label: string;
  readonly min: number;
  readonly max: number;
  readonly step: number;
};
export type PortalFrameColorField = { readonly path: readonly string[]; readonly label: string };
export const PortalFrameShellNumberFields = PortalFrameShellNumberFieldsImpl;
export const PortalFrameContentBooleanFields = PortalFrameContentBooleanFieldsImpl;
export const PortalGoldenCardBooleanFields = PortalGoldenCardBooleanFieldsImpl;
export const PortalFrameContentNumberFields = PortalFrameContentNumberFieldsImpl;
export const PortalGoldenCardFrameNumberFields = PortalGoldenCardFrameNumberFieldsImpl;
export const PortalGoldenCardContentNumberFields = PortalGoldenCardContentNumberFieldsImpl;
export const PortalCarouselFrameNumberFields = PortalCarouselFrameNumberFieldsImpl;
export const PortalCarouselContentNumberFields = PortalCarouselContentNumberFieldsImpl;
export const PortalCarouselRailNumberFields = PortalCarouselRailNumberFieldsImpl;
export const PortalFrameSlotNumberFields = PortalFrameSlotNumberFieldsImpl;
export const PortalFrameGeometryNumberFields = PortalFrameGeometryNumberFieldsImpl;
export const PortalFrameChromeNumberFields = PortalFrameChromeNumberFieldsImpl;
export const PortalFrameOuterShapeNumberFields = PortalFrameOuterShapeNumberFieldsImpl;
export const PortalFrameInnerShapeNumberFields = PortalFrameInnerShapeNumberFieldsImpl;
export const PortalFrameOuterEdgeNumberFields = PortalFrameOuterEdgeNumberFieldsImpl;
export const PortalFrameInnerEdgeNumberFields = PortalFrameInnerEdgeNumberFieldsImpl;
export const PortalFrameOuterGapNumberFields = PortalFrameOuterGapNumberFieldsImpl;
export const PortalFrameInnerGapNumberFields = PortalFrameInnerGapNumberFieldsImpl;
export const PortalFrameOuterSegmentNumberFields = PortalFrameOuterSegmentNumberFieldsImpl;
export const PortalFrameInnerSegmentNumberFields = PortalFrameInnerSegmentNumberFieldsImpl;
export const portalFrameCssPixel = portalFrameCssPixelImpl;
export const portalFrameCssPercent = portalFrameCssPercentImpl;
export const portalFrameCssNumber = portalFrameCssNumberImpl;
export const portalFrameCssOpacity = portalFrameCssOpacityImpl;
