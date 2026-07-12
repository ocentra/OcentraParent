import { decodeDisplayText } from './display-text';
import type { PortalFrameBooleanField, PortalFrameNumberField } from './frame-tuner-impl';

export const PortalFrameTunerLayoutKey = {
  ParentPortal: 'parentPortal',
  ContentDraft: 'contentDraft',
  SidePanelFoldouts: 'sidePanelFoldouts',
  MainPanelTop: 'mainPanelTop',
  MainPanelBottom: 'mainPanelBottom',
  Id: 'id',
  Label: 'label',
  Detail: 'detail',
  RoutePath: 'routePath',
  Icon: 'icon',
  Tone: 'tone',
  Buttons: 'buttons',
  Orientation: 'orientation',
  Content: 'content',
  Shell: 'shell',
  Preview: 'preview',
  Carousel: 'carousel',
  GoldenCard: 'goldenCard',
  ShowContent: 'showContent',
  MinHeight: 'minHeight',
  SidebarWidth: 'sidebarWidth',
  ShellEdge: 'shellEdge',
  BodyInset: 'bodyInset',
  FrameGap: 'frameGap',
  SideBottomHeight: 'sideBottomHeight',
  SideStackGap: 'sideStackGap',
  ShowBounds: 'showBounds',
  ShowContentBounds: 'showContentBounds',
  ContentInsetX: 'insetX',
  ContentInsetY: 'insetY',
  ContentOffsetX: 'offsetX',
  ContentOffsetY: 'offsetY',
  ContentGap: 'gap',
  ShowFrameBounds: 'showFrameBounds',
  ViewBox: 'viewBox',
  FrameSpace: 'frameSpace',
  FrameGroup: 'frameGroup',
  SlotHeightAdjust: 'slotHeightAdjust',
  SlotWidthAdjust: 'slotWidthAdjust',
  OuterAnchor: 'outerAnchor',
  InnerAnchor: 'innerAnchor',
  OuterFrame: 'outerFrame',
  InnerFrame: 'innerFrame',
  SegmentThicknesses: 'segmentThicknesses',
  Segment: {
    TopLeftRunStart: 'topLeftRunStart',
    TopLeftRunMid: 'topLeftRunMid',
    TopLeftRunEnd: 'topLeftRunEnd',
    TopLeftConnector: 'topLeftConnector',
    TopCenterRun: 'topCenterRun',
    TopRightConnector: 'topRightConnector',
    TopRightRunStart: 'topRightRunStart',
    TopRightRunMid: 'topRightRunMid',
    TopRightRunEnd: 'topRightRunEnd',
    RightTopCorner: 'rightTopCorner',
    RightSideRunStart: 'rightSideRunStart',
    RightSideRunMid: 'rightSideRunMid',
    RightSideRunEnd: 'rightSideRunEnd',
    RightBottomCorner: 'rightBottomCorner',
    BottomRightRunStart: 'bottomRightRunStart',
    BottomRightRunMid: 'bottomRightRunMid',
    BottomRightRunEnd: 'bottomRightRunEnd',
    BottomRightConnector: 'bottomRightConnector',
    BottomCenterRun: 'bottomCenterRun',
    BottomLeftConnector: 'bottomLeftConnector',
    BottomLeftRunStart: 'bottomLeftRunStart',
    BottomLeftRunMid: 'bottomLeftRunMid',
    BottomLeftRunEnd: 'bottomLeftRunEnd',
    LeftBottomCorner: 'leftBottomCorner',
    LeftSideRunStart: 'leftSideRunStart',
    LeftSideRunMid: 'leftSideRunMid',
    LeftSideRunEnd: 'leftSideRunEnd',
    LeftTopCorner: 'leftTopCorner',
  },
  Width: 'w',
  Height: 'h',
  Inset: 'inset',
  OffsetX: 'offsetX',
  OffsetY: 'offsetY',
  PaddingTop: 'paddingTop',
  PaddingRight: 'paddingRight',
  PaddingBottom: 'paddingBottom',
  PaddingLeft: 'paddingLeft',
  SideInset: 'sideInset',
  TopInset: 'topInset',
  BottomInset: 'bottomInset',
  CornerCut: 'cornerCut',
  TopRise: 'topRise',
  TopStepWidth: 'topStepWidth',
  TopStepInset: 'topStepInset',
  BottomTabWidth: 'bottomTabWidth',
  BottomTabDepth: 'bottomTabDepth',
  BottomTabInset: 'bottomTabInset',
  TopLeftThickness: 'topLeftThickness',
  TopCenterThickness: 'topCenterThickness',
  TopRightThickness: 'topRightThickness',
  LeftSideThickness: 'leftSideThickness',
  RightSideThickness: 'rightSideThickness',
  BottomLeftThickness: 'bottomLeftThickness',
  BottomCenterThickness: 'bottomCenterThickness',
  BottomRightThickness: 'bottomRightThickness',
  TopGroupThickness: 'topGroupThickness',
  BottomGroupThickness: 'bottomGroupThickness',
  CornerGroupThickness: 'cornerGroupThickness',
  ThinLineGroupThickness: 'thinLineGroupThickness',
  TopLeftStartGap: 'topLeftStartGap',
  TopLeftEndGap: 'topLeftEndGap',
  TopRightStartGap: 'topRightStartGap',
  TopRightEndGap: 'topRightEndGap',
  BottomLeftStartGap: 'bottomLeftStartGap',
  BottomLeftEndGap: 'bottomLeftEndGap',
  BottomRightStartGap: 'bottomRightStartGap',
  BottomRightEndGap: 'bottomRightEndGap',
  LeftSideStartGap: 'leftSideStartGap',
  LeftSideEndGap: 'leftSideEndGap',
  RightSideStartGap: 'rightSideStartGap',
  RightSideEndGap: 'rightSideEndGap',
  GlowOpacity: 'glowOpacity',
  GlowBlur: 'glowBlur',
  GlowWidthBoost: 'glowWidthBoost',
  OutlineOpacity: 'outlineOpacity',
  OutlineWidthBoost: 'outlineWidthBoost',
  Opacity: 'opacity',
  Color: 'color',
  GlowColor: 'glowColor',
  FrameInset: 'frameInset',
  FrameInsetX: 'frameInsetX',
  FrameInsetY: 'frameInsetY',
  CardHeight: 'height',
  CardMinWidth: 'cardMinWidth',
  CardWidthPercent: 'cardWidthPercent',
  FrameWidthAdjust: 'frameWidthAdjust',
  FrameHeightAdjust: 'frameHeightAdjust',
  FrameOpacity: 'frameOpacity',
  FrameGlowBlur: 'frameGlowBlur',
  FrameGlowOpacity: 'frameGlowOpacity',
  PaddingX: 'paddingX',
  PaddingY: 'paddingY',
  GlyphSize: 'glyphSize',
  TitleOffsetX: 'titleOffsetX',
  TitleOffsetY: 'titleOffsetY',
  TitleLeft: 'titleLeft',
  TitleRight: 'titleRight',
  TitleTop: 'titleTop',
  RailGap: 'railGap',
  RailPaddingTop: 'railPadTop',
  RailPaddingRight: 'railPadRight',
  RailPaddingBottom: 'railPadBottom',
  RailPaddingLeft: 'railPadLeft',
} as const;

const key = PortalFrameTunerLayoutKey;

const PortalFrameTunerText = {
  ShowFrameBounds: decodeDisplayText('Purple frame bounds'),
  ShowContentBounds: decodeDisplayText('Yellow content bounds'),
  ShowFrameContent: decodeDisplayText('Show content'),
} as const;

export const PortalFrameShellNumberFields: readonly PortalFrameNumberField[] = [
  field([key.SidebarWidth], 'Sidebar width', 220, 340, 1),
  field([key.SideBottomHeight], 'Bottom frame height', 96, 180, 1),
  field([key.SideStackGap], 'Side frame gap', 0, 24, 1),
  field([key.ShellEdge], 'Shell edge', 0, 20, 1),
  field([key.BodyInset], 'Body inset', 10, 40, 1),
  field([key.FrameGap], 'Frame gap', -10, 20, 1),
] as const;

export const PortalFrameContentBooleanFields: readonly PortalFrameBooleanField[] = [
  booleanField([key.ShowFrameBounds], PortalFrameTunerText.ShowFrameBounds),
  booleanField([key.ShowContentBounds], PortalFrameTunerText.ShowContentBounds),
  booleanField([key.ShowContent], PortalFrameTunerText.ShowFrameContent),
] as const;

export const PortalGoldenCardBooleanFields: readonly PortalFrameBooleanField[] = [
  booleanField([key.ShowFrameBounds], PortalFrameTunerText.ShowFrameBounds),
  booleanField([key.ShowContentBounds], PortalFrameTunerText.ShowContentBounds),
  booleanField([key.ShowContent], PortalFrameTunerText.ShowFrameContent),
] as const;

export const PortalFrameContentNumberFields: readonly PortalFrameNumberField[] = [
  field([key.ContentInsetX], 'Inset X', 0, 80, 1),
  field([key.ContentInsetY], 'Inset Y', 0, 80, 1),
  field([key.ContentOffsetX], 'Offset X', -80, 80, 1),
  field([key.ContentOffsetY], 'Offset Y', -80, 80, 1),
  field([key.ContentGap], 'Gap', 0, 40, 1),
] as const;

export const PortalGoldenCardFrameNumberFields: readonly PortalFrameNumberField[] = [
  field([key.CardHeight], 'Card height', 130, 260, 1),
  field([key.FrameInset], 'Frame inset', -24, 24, 1),
  field([key.FrameWidthAdjust], 'Frame width +/-', -80, 120, 1),
  field([key.FrameHeightAdjust], 'Frame height +/-', -80, 120, 1),
  field([key.FrameGroup, key.OffsetX], 'Frame offset X', -80, 80, 1),
  field([key.FrameGroup, key.OffsetY], 'Frame offset Y', -80, 80, 1),
  field([key.FrameOpacity], 'Frame opacity', 0, 1, 0.01),
  field([key.FrameGlowBlur], 'Frame glow blur', 0, 24, 1),
  field([key.FrameGlowOpacity], 'Frame glow opacity', 0, 0.6, 0.01),
] as const;

export const PortalGoldenCardContentNumberFields: readonly PortalFrameNumberField[] = [
  field([key.PaddingX], 'Padding X', 0, 60, 1),
  field([key.PaddingY], 'Padding Y', 0, 70, 1),
  field([key.ContentInsetX], 'Content bounds X', 0, 80, 1),
  field([key.ContentInsetY], 'Content bounds Y', 0, 80, 1),
  field([key.ContentOffsetX], 'Content offset X', -80, 80, 1),
  field([key.ContentOffsetY], 'Content offset Y', -80, 80, 1),
  field([key.GlyphSize], 'Glyph size', 24, 64, 1),
  field([key.TitleOffsetX], 'Title offset X', -80, 80, 1),
  field([key.TitleOffsetY], 'Title offset Y', -40, 40, 1),
] as const;

export const PortalCarouselFrameNumberFields: readonly PortalFrameNumberField[] = [
  field([key.MinHeight], 'Carousel height', 190, 420, 1),
  field([key.FrameInsetX], 'Frame inset X', -80, 80, 1),
  field([key.FrameInsetY], 'Frame inset Y', -80, 80, 1),
  field([key.FrameWidthAdjust], 'Frame width +/-', -160, 200, 1),
  field([key.FrameHeightAdjust], 'Frame height +/-', -120, 160, 1),
  field([key.FrameGroup, key.OffsetX], 'Frame offset X', -120, 120, 1),
  field([key.FrameGroup, key.OffsetY], 'Frame offset Y', -120, 120, 1),
  field([key.FrameOpacity], 'Frame opacity', 0, 1, 0.01),
] as const;

export const PortalCarouselContentNumberFields: readonly PortalFrameNumberField[] = [
  field([key.PaddingTop], 'Padding top', 0, 100, 1),
  field([key.PaddingRight], 'Padding right', 0, 120, 1),
  field([key.PaddingBottom], 'Padding bottom', 0, 100, 1),
  field([key.PaddingLeft], 'Padding left', 0, 120, 1),
  field([key.TitleLeft], 'Title left', 0, 180, 1),
  field([key.TitleRight], 'Title right', 0, 180, 1),
  field([key.TitleTop], 'Title top', -20, 80, 1),
] as const;

export const PortalCarouselRailNumberFields: readonly PortalFrameNumberField[] = [
  field([key.RailGap], 'Rail gap', 0, 48, 1),
  field([key.RailPaddingTop], 'Rail padding top', 0, 80, 1),
  field([key.RailPaddingRight], 'Rail padding right', 0, 80, 1),
  field([key.RailPaddingBottom], 'Rail padding bottom', 0, 80, 1),
  field([key.RailPaddingLeft], 'Rail padding left', 0, 80, 1),
  field([key.CardMinWidth], 'Card min width', 220, 520, 1),
  field([key.CardWidthPercent], 'Card width %', 28, 96, 1),
] as const;

export const PortalFrameSlotNumberFields: readonly PortalFrameNumberField[] = [
  field([key.SlotWidthAdjust], 'Frame box width +/-', -180, 240, 1),
  field([key.SlotHeightAdjust], 'Frame box height +/-', -180, 240, 1),
] as const;

export const PortalFrameGeometryNumberFields: readonly PortalFrameNumberField[] = [
  field([key.ViewBox, key.Width], 'Camera width', 700, 2200, 1),
  field([key.ViewBox, key.Height], 'Camera height', 700, 2400, 1),
  field([key.FrameSpace, key.Width], 'Path width', 300, 2600, 1),
  field([key.FrameSpace, key.Height], 'Path height', 180, 2400, 1),
  field([key.FrameGroup, key.Inset], 'Uniform margin', 0, 60, 1),
  field([key.FrameGroup, key.OffsetX], 'Move frame X', -120, 120, 1),
  field([key.FrameGroup, key.OffsetY], 'Move frame Y', -120, 120, 1),
  field([key.OuterAnchor, key.SideInset], 'Left/right padding', 0, 120, 1),
  field([key.OuterAnchor, key.TopInset], 'Top padding', 0, 140, 1),
  field([key.OuterAnchor, key.BottomInset], 'Bottom padding', 0, 140, 1),
  field([key.InnerAnchor, key.SideInset], 'Inner left/right padding', 0, 160, 1),
  field([key.InnerAnchor, key.TopInset], 'Inner top padding', 0, 180, 1),
  field([key.InnerAnchor, key.BottomInset], 'Inner bottom padding', 0, 180, 1),
] as const;

export const PortalFrameChromeNumberFields: readonly PortalFrameNumberField[] = [
  field([key.OuterFrame, key.CornerCut], 'Outer corner cut', 20, 140, 1),
  field([key.OuterFrame, key.TopRise], 'Outer top rise', 0, 120, 1),
  field([key.OuterFrame, key.TopStepWidth], 'Outer top step', 120, 920, 1),
  field([key.OuterFrame, key.BottomTabWidth], 'Outer bottom tab', 120, 920, 1),
  field([key.OuterFrame, key.GlowOpacity], 'Outer glow opacity', 0, 0.8, 0.01),
  field([key.OuterFrame, key.GlowBlur], 'Outer glow blur', 0, 40, 1),
  field([key.OuterFrame, key.GlowWidthBoost], 'Outer glow width', 0, 16, 1),
  field([key.InnerFrame, key.CornerCut], 'Inner corner cut', 10, 120, 1),
  field([key.InnerFrame, key.TopStepWidth], 'Inner top step', 120, 820, 1),
  field([key.InnerFrame, key.BottomTabWidth], 'Inner bottom tab', 120, 820, 1),
  field([key.InnerFrame, key.OutlineOpacity], 'Inner outline opacity', 0, 1, 0.01),
  field([key.InnerFrame, key.Opacity], 'Inner opacity', 0, 1, 0.01),
] as const;

export const PortalFrameOuterShapeNumberFields: readonly PortalFrameNumberField[] = [
  field([key.OuterFrame, key.SideInset], 'Side inset', 0, 160, 1),
  field([key.OuterFrame, key.CornerCut], 'Corner cut', 0, 160, 1),
  field([key.OuterFrame, key.TopRise], 'Top rise', 0, 140, 1),
  field([key.OuterFrame, key.TopStepWidth], 'Top step width', 40, 1000, 1),
  field([key.OuterFrame, key.TopStepInset], 'Top step inset', 0, 140, 1),
  field([key.OuterFrame, key.BottomTabWidth], 'Bottom tab width', 40, 1000, 1),
  field([key.OuterFrame, key.BottomTabDepth], 'Bottom tab depth', 0, 160, 1),
  field([key.OuterFrame, key.BottomTabInset], 'Bottom tab inset', 0, 140, 1),
  field([key.OuterFrame, key.GlowOpacity], 'Glow opacity', 0, 0.8, 0.01),
  field([key.OuterFrame, key.GlowBlur], 'Glow blur', 0, 40, 1),
  field([key.OuterFrame, key.GlowWidthBoost], 'Glow width', 0, 16, 1),
  field([key.OuterFrame, key.OutlineOpacity], 'Outline opacity', 0, 1, 0.01),
  field([key.OuterFrame, key.OutlineWidthBoost], 'Outline width', 0, 12, 1),
  field([key.OuterFrame, key.Opacity], 'Opacity', 0, 1, 0.01),
] as const;

export const PortalFrameInnerShapeNumberFields: readonly PortalFrameNumberField[] = [
  field([key.InnerFrame, key.SideInset], 'Side inset', 0, 160, 1),
  field([key.InnerFrame, key.CornerCut], 'Corner cut', 0, 160, 1),
  field([key.InnerFrame, key.TopRise], 'Top rise', 0, 140, 1),
  field([key.InnerFrame, key.TopStepWidth], 'Top step width', 40, 1000, 1),
  field([key.InnerFrame, key.TopStepInset], 'Top step inset', 0, 140, 1),
  field([key.InnerFrame, key.BottomTabWidth], 'Bottom tab width', 40, 1000, 1),
  field([key.InnerFrame, key.BottomTabDepth], 'Bottom tab depth', 0, 160, 1),
  field([key.InnerFrame, key.BottomTabInset], 'Bottom tab inset', 0, 140, 1),
  field([key.InnerFrame, key.GlowOpacity], 'Glow opacity', 0, 0.8, 0.01),
  field([key.InnerFrame, key.GlowBlur], 'Glow blur', 0, 40, 1),
  field([key.InnerFrame, key.GlowWidthBoost], 'Glow width', 0, 16, 1),
  field([key.InnerFrame, key.OutlineOpacity], 'Outline opacity', 0, 1, 0.01),
  field([key.InnerFrame, key.OutlineWidthBoost], 'Outline width', 0, 12, 1),
  field([key.InnerFrame, key.Opacity], 'Opacity', 0, 1, 0.01),
] as const;

export const PortalFrameOuterEdgeNumberFields: readonly PortalFrameNumberField[] = [
  field([key.OuterFrame, key.TopGroupThickness], 'Top group', 0, 40, 1),
  field([key.OuterFrame, key.BottomGroupThickness], 'Bottom group', 0, 40, 1),
  field([key.OuterFrame, key.CornerGroupThickness], 'Corner group', 0, 40, 1),
  field([key.OuterFrame, key.ThinLineGroupThickness], 'Thin line group', 0, 20, 1),
  field([key.OuterFrame, key.TopLeftThickness], 'Top left', 0, 40, 1),
  field([key.OuterFrame, key.TopCenterThickness], 'Top center', 0, 40, 1),
  field([key.OuterFrame, key.TopRightThickness], 'Top right', 0, 40, 1),
  field([key.OuterFrame, key.LeftSideThickness], 'Left side', 0, 40, 1),
  field([key.OuterFrame, key.RightSideThickness], 'Right side', 0, 40, 1),
  field([key.OuterFrame, key.BottomLeftThickness], 'Bottom left', 0, 40, 1),
  field([key.OuterFrame, key.BottomCenterThickness], 'Bottom center', 0, 40, 1),
  field([key.OuterFrame, key.BottomRightThickness], 'Bottom right', 0, 40, 1),
] as const;

export const PortalFrameInnerEdgeNumberFields: readonly PortalFrameNumberField[] = [
  field([key.InnerFrame, key.TopGroupThickness], 'Top group', 0, 40, 1),
  field([key.InnerFrame, key.BottomGroupThickness], 'Bottom group', 0, 40, 1),
  field([key.InnerFrame, key.CornerGroupThickness], 'Corner group', 0, 40, 1),
  field([key.InnerFrame, key.ThinLineGroupThickness], 'Thin line group', 0, 20, 1),
  field([key.InnerFrame, key.TopLeftThickness], 'Top left', 0, 40, 1),
  field([key.InnerFrame, key.TopCenterThickness], 'Top center', 0, 40, 1),
  field([key.InnerFrame, key.TopRightThickness], 'Top right', 0, 40, 1),
  field([key.InnerFrame, key.LeftSideThickness], 'Left side', 0, 40, 1),
  field([key.InnerFrame, key.RightSideThickness], 'Right side', 0, 40, 1),
  field([key.InnerFrame, key.BottomLeftThickness], 'Bottom left', 0, 40, 1),
  field([key.InnerFrame, key.BottomCenterThickness], 'Bottom center', 0, 40, 1),
  field([key.InnerFrame, key.BottomRightThickness], 'Bottom right', 0, 40, 1),
] as const;

export const PortalFrameOuterGapNumberFields: readonly PortalFrameNumberField[] = [
  field([key.OuterFrame, key.TopLeftStartGap], 'Top left start', 0, 200, 1),
  field([key.OuterFrame, key.TopLeftEndGap], 'Top left end', 0, 200, 1),
  field([key.OuterFrame, key.TopRightStartGap], 'Top right start', 0, 200, 1),
  field([key.OuterFrame, key.TopRightEndGap], 'Top right end', 0, 200, 1),
  field([key.OuterFrame, key.BottomLeftStartGap], 'Bottom left start', 0, 200, 1),
  field([key.OuterFrame, key.BottomLeftEndGap], 'Bottom left end', 0, 200, 1),
  field([key.OuterFrame, key.BottomRightStartGap], 'Bottom right start', 0, 200, 1),
  field([key.OuterFrame, key.BottomRightEndGap], 'Bottom right end', 0, 200, 1),
  field([key.OuterFrame, key.LeftSideStartGap], 'Left side start', 0, 200, 1),
  field([key.OuterFrame, key.LeftSideEndGap], 'Left side end', 0, 200, 1),
  field([key.OuterFrame, key.RightSideStartGap], 'Right side start', 0, 200, 1),
  field([key.OuterFrame, key.RightSideEndGap], 'Right side end', 0, 200, 1),
] as const;

export const PortalFrameInnerGapNumberFields: readonly PortalFrameNumberField[] = [
  field([key.InnerFrame, key.TopLeftStartGap], 'Top left start', 0, 200, 1),
  field([key.InnerFrame, key.TopLeftEndGap], 'Top left end', 0, 200, 1),
  field([key.InnerFrame, key.TopRightStartGap], 'Top right start', 0, 200, 1),
  field([key.InnerFrame, key.TopRightEndGap], 'Top right end', 0, 200, 1),
  field([key.InnerFrame, key.BottomLeftStartGap], 'Bottom left start', 0, 200, 1),
  field([key.InnerFrame, key.BottomLeftEndGap], 'Bottom left end', 0, 200, 1),
  field([key.InnerFrame, key.BottomRightStartGap], 'Bottom right start', 0, 200, 1),
  field([key.InnerFrame, key.BottomRightEndGap], 'Bottom right end', 0, 200, 1),
  field([key.InnerFrame, key.LeftSideStartGap], 'Left side start', 0, 200, 1),
  field([key.InnerFrame, key.LeftSideEndGap], 'Left side end', 0, 200, 1),
  field([key.InnerFrame, key.RightSideStartGap], 'Right side start', 0, 200, 1),
  field([key.InnerFrame, key.RightSideEndGap], 'Right side end', 0, 200, 1),
] as const;

export const PortalFrameOuterSegmentNumberFields: readonly PortalFrameNumberField[] = [
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopLeftRunStart],
    'Top left start',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopLeftRunMid],
    'Top left mid',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopLeftRunEnd],
    'Top left end',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopLeftConnector],
    'Top left connector',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopCenterRun],
    'Top center run',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopRightConnector],
    'Top right connector',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopRightRunStart],
    'Top right start',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopRightRunMid],
    'Top right mid',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.TopRightRunEnd],
    'Top right end',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.RightTopCorner],
    'Right top corner',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.RightSideRunStart],
    'Right side start',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.RightSideRunMid],
    'Right side mid',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.RightSideRunEnd],
    'Right side end',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.RightBottomCorner],
    'Right bottom corner',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomRightRunStart],
    'Bottom right start',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomRightRunMid],
    'Bottom right mid',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomRightRunEnd],
    'Bottom right end',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomRightConnector],
    'Bottom right connector',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomCenterRun],
    'Bottom center run',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomLeftConnector],
    'Bottom left connector',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomLeftRunStart],
    'Bottom left start',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomLeftRunMid],
    'Bottom left mid',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.BottomLeftRunEnd],
    'Bottom left end',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.LeftBottomCorner],
    'Left bottom corner',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.LeftSideRunStart],
    'Left side start',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.LeftSideRunMid],
    'Left side mid',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.LeftSideRunEnd],
    'Left side end',
    0,
    40,
    1
  ),
  field(
    [key.OuterFrame, key.SegmentThicknesses, PortalFrameTunerLayoutKey.Segment.LeftTopCorner],
    'Left top corner',
    0,
    40,
    1
  ),
] as const;

export const PortalFrameInnerSegmentNumberFields: readonly PortalFrameNumberField[] =
  PortalFrameOuterSegmentNumberFields.map((fieldValue) => ({
    ...fieldValue,
    path: [key.InnerFrame, ...fieldValue.path.slice(1)],
  }));

function booleanField(path: readonly string[], label: ReturnType<typeof decodeDisplayText>): PortalFrameBooleanField {
  return {
    path,
    label,
  };
}

function field(path: readonly string[], label: string, min: number, max: number, step: number): PortalFrameNumberField {
  return {
    path,
    label: decodeDisplayText(label),
    min,
    max,
    step,
  };
}
