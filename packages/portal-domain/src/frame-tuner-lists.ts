import { PortalFrameTuner, type PortalFrameColorField } from './frame-tuner';

const key = PortalFrameTuner.LayoutKey;

export const PortalFrameColorFields: readonly PortalFrameColorField[] = [
  { path: [key.OuterFrame, key.Color], label: PortalFrameTuner.Text.OuterColor },
  { path: [key.OuterFrame, key.GlowColor], label: PortalFrameTuner.Text.OuterGlowColor },
  { path: [key.InnerFrame, key.Color], label: PortalFrameTuner.Text.InnerColor },
  { path: [key.InnerFrame, key.GlowColor], label: PortalFrameTuner.Text.InnerGlowColor },
] as const;

export const PortalFrameTunerFrameSections = [
  { id: PortalFrameTuner.FrameSection.OuterAnchors, label: PortalFrameTuner.Text.OuterAnchorGroup },
  { id: PortalFrameTuner.FrameSection.Content, label: PortalFrameTuner.Text.ContentGroup },
  { id: PortalFrameTuner.FrameSection.InnerAnchors, label: PortalFrameTuner.Text.InnerAnchorGroup },
  { id: PortalFrameTuner.FrameSection.OuterFrame, label: PortalFrameTuner.Text.OuterFrameGroup },
  { id: PortalFrameTuner.FrameSection.OuterEdges, label: PortalFrameTuner.Text.OuterEdgeGroup },
  { id: PortalFrameTuner.FrameSection.OuterSegments, label: PortalFrameTuner.Text.OuterSegmentGroup },
  { id: PortalFrameTuner.FrameSection.OuterGaps, label: PortalFrameTuner.Text.OuterGapGroup },
  { id: PortalFrameTuner.FrameSection.InnerFrame, label: PortalFrameTuner.Text.InnerFrameGroup },
  { id: PortalFrameTuner.FrameSection.InnerEdges, label: PortalFrameTuner.Text.InnerEdgeGroup },
  { id: PortalFrameTuner.FrameSection.InnerSegments, label: PortalFrameTuner.Text.InnerSegmentGroup },
  { id: PortalFrameTuner.FrameSection.InnerGaps, label: PortalFrameTuner.Text.InnerGapGroup },
  { id: PortalFrameTuner.FrameSection.Colors, label: PortalFrameTuner.Text.ColorsGroup },
  { id: PortalFrameTuner.FrameSection.Viewport, label: PortalFrameTuner.Text.ViewportGroup },
] as const;
