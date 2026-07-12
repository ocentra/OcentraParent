import { PortalFrameTuner, type PortalFrameTargetValue } from '@ocentra-parent/portal-domain/frame-tuner';
import {
  defaultPortalAppLayoutSurfaceContent,
  normalizePortalAppLayoutContentDraft,
  type PortalAppLayoutSurfaceKey,
} from '@ocentra-parent/portal-domain/app-layout';
import {
  normalizePictureViewerFrameControls,
  type PictureViewerFrameSurfaceControls,
} from '../../../vendor/ocentra-parent-core-ui/Common/PictureViewerFrame/PictureViewerFrameControls';
import {
  DEFAULT_PORTAL_FRAME_LAYOUT,
  defaultParentPortalControls,
  type PortalCarouselLayout,
  type PortalFrameContentTargetLayout,
  type PortalFrameLayout,
  type PortalGoldenCardLayout,
  type PortalParentPortalLayout,
} from './portal-frame-layout-types';
import {
  normalizeParentPortalSvgControls,
  type ParentPortalSvgControls,
} from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';

const text = PortalFrameTuner;

export function normalizePortalFrameLayout(value: unknown): PortalFrameLayout {
  return {
    carousel: normalizeCarouselLayout(valueAt(value, [text.LayoutKey.Carousel])),
    content: {
      sideTop: normalizeFrameContent(
        valueAt(value, [text.LayoutKey.Content, text.FrameTarget.SideTop]),
        DEFAULT_PORTAL_FRAME_LAYOUT.content.sideTop
      ),
      sideBottom: normalizeFrameContent(
        valueAt(value, [text.LayoutKey.Content, text.FrameTarget.SideBottom]),
        DEFAULT_PORTAL_FRAME_LAYOUT.content.sideBottom
      ),
      main: normalizeFrameContent(
        valueAt(value, [text.LayoutKey.Content, text.FrameTarget.Main]),
        DEFAULT_PORTAL_FRAME_LAYOUT.content.main
      ),
    },
    shell: {
      sidebarWidth: numberAt(value, [text.LayoutKey.Shell, text.LayoutKey.SidebarWidth], 270),
      shellEdge: numberAt(value, [text.LayoutKey.Shell, text.LayoutKey.ShellEdge], 8),
      bodyInset: numberAt(value, [text.LayoutKey.Shell, text.LayoutKey.BodyInset], 22),
      frameGap: numberAt(value, [text.LayoutKey.Shell, text.LayoutKey.FrameGap], -2),
      sideBottomHeight: numberAt(value, [text.LayoutKey.Shell, text.LayoutKey.SideBottomHeight], 126),
      sideStackGap: numberAt(value, [text.LayoutKey.Shell, text.LayoutKey.SideStackGap], 10),
    },
    goldenCard: normalizeGoldenCardLayout(valueAt(value, [text.LayoutKey.GoldenCard])),
    parentPortal: normalizeParentPortalLayout(valueAt(value, [text.LayoutKey.ParentPortal])),
    preview: {
      showContent: valueAt(value, [text.LayoutKey.Preview, text.LayoutKey.ShowContent]) !== false,
    },
    sideTop: normalizePictureViewerFrameControls(
      (valueAt(value, [text.FrameTarget.SideTop]) ??
        valueAt(value, [text.FrameTarget.Side]) ??
        DEFAULT_PORTAL_FRAME_LAYOUT.sideTop) as Partial<PictureViewerFrameSurfaceControls>
    ),
    sideBottom: normalizePictureViewerFrameControls(
      (valueAt(value, [text.FrameTarget.SideBottom]) ??
        DEFAULT_PORTAL_FRAME_LAYOUT.sideBottom) as Partial<PictureViewerFrameSurfaceControls>
    ),
    main: normalizePictureViewerFrameControls(
      valueAt(value, [text.FrameTarget.Main]) as Partial<PictureViewerFrameSurfaceControls>
    ),
  };
}

export function frameTargetControls(
  layout: PortalFrameLayout,
  target: PortalFrameTargetValue
): PictureViewerFrameSurfaceControls {
  if (target === PortalFrameTuner.FrameTarget.Main) {
    return layout.main;
  }
  return target === PortalFrameTuner.FrameTarget.SideBottom ? layout.sideBottom : layout.sideTop;
}

export function frameContentTarget(
  layout: PortalFrameLayout,
  target: PortalFrameTargetValue
): PortalFrameContentTargetLayout {
  if (target === PortalFrameTuner.FrameTarget.Main) {
    return layout.content.main;
  }
  return target === PortalFrameTuner.FrameTarget.SideBottom ? layout.content.sideBottom : layout.content.sideTop;
}

export function resetPortalFrameTarget(layout: PortalFrameLayout, target: PortalFrameTargetValue): PortalFrameLayout {
  return normalizePortalFrameLayout(PORTAL_FRAME_TARGET_RESETTERS[target](layout));
}

export function resetPortalGoldenCard(layout: PortalFrameLayout): PortalFrameLayout {
  return normalizePortalFrameLayout({
    ...layout,
    goldenCard: DEFAULT_PORTAL_FRAME_LAYOUT.goldenCard,
  });
}

export function resetPortalCarousel(layout: PortalFrameLayout): PortalFrameLayout {
  return normalizePortalFrameLayout({
    ...layout,
    carousel: DEFAULT_PORTAL_FRAME_LAYOUT.carousel,
  });
}

export function resetPortalParentPortalSurface(
  layout: PortalFrameLayout,
  surface: PortalAppLayoutSurfaceKey
): PortalFrameLayout {
  return normalizePortalFrameLayout(
    setPortalFrameLayoutValue(
      layout,
      [text.LayoutKey.ParentPortal, surface],
      DEFAULT_PORTAL_FRAME_LAYOUT.parentPortal[surface]
    )
  );
}

export function resetPortalParentPortalContent(
  layout: PortalFrameLayout,
  surface: PortalAppLayoutSurfaceKey
): PortalFrameLayout {
  return normalizePortalFrameLayout(
    setPortalFrameLayoutValue(
      layout,
      [text.LayoutKey.ParentPortal, text.LayoutKey.ContentDraft, surface],
      defaultPortalAppLayoutSurfaceContent(surface)
    )
  );
}

function normalizeParentPortalLayout(value: unknown): PortalParentPortalLayout {
  return {
    mainApp: normalizeParentPortalControls(valueAt(value, [text.AppSurface.MainApp])),
    chatInterface: normalizeParentPortalControls(valueAt(value, [text.AppSurface.ChatInterface])),
    contentDraft: normalizePortalAppLayoutContentDraft(valueAt(value, [text.LayoutKey.ContentDraft])),
  };
}

function normalizeParentPortalControls(value: unknown): ParentPortalSvgControls {
  const fallback = defaultParentPortalControls();
  if (!isRecord(value)) {
    return fallback;
  }
  const group = text.ParentPortalControlGroup;
  const canvas = recordAt(value, group.Canvas);
  const layout = recordAt(value, group.Layout);
  const colors = recordAt(value, group.Colors);
  const chrome = recordAt(value, group.Chrome);
  return normalizeParentPortalSvgControls({
    canvas: { ...fallback.canvas, ...canvas },
    layout: { ...fallback.layout, ...layout },
    colors: { ...fallback.colors, ...colors },
    chrome: { ...fallback.chrome, ...chrome },
  });
}

function recordAt(root: Record<PropertyKey, unknown>, key: PropertyKey): Record<PropertyKey, unknown> {
  const value = root[key];
  return isRecord(value) ? value : {};
}

function numberAt(root: unknown, path: readonly PropertyKey[], fallback: number): number {
  const value = Number(valueAt(root, path));
  return Number.isFinite(value) ? value : fallback;
}

function booleanAt(root: unknown, path: readonly PropertyKey[], fallback: boolean): boolean {
  const value = valueAt(root, path);
  return value === true || value === false ? value : fallback;
}

function normalizeFrameContent(
  value: unknown,
  fallback: PortalFrameContentTargetLayout
): PortalFrameContentTargetLayout {
  const legacyBounds = booleanAt(value, [text.LayoutKey.ShowBounds], fallback.showFrameBounds);
  return {
    showContentBounds: booleanAt(value, [text.LayoutKey.ShowContentBounds], legacyBounds),
    showContent: booleanAt(value, [text.LayoutKey.ShowContent], fallback.showContent),
    showFrameBounds: booleanAt(value, [text.LayoutKey.ShowFrameBounds], legacyBounds),
    insetX: numberAt(value, [text.LayoutKey.ContentInsetX], fallback.insetX),
    insetY: numberAt(value, [text.LayoutKey.ContentInsetY], fallback.insetY),
    offsetX: numberAt(value, [text.LayoutKey.ContentOffsetX], fallback.offsetX),
    offsetY: numberAt(value, [text.LayoutKey.ContentOffsetY], fallback.offsetY),
    slotHeightAdjust: numberAt(value, [text.LayoutKey.SlotHeightAdjust], fallback.slotHeightAdjust),
    slotWidthAdjust: numberAt(value, [text.LayoutKey.SlotWidthAdjust], fallback.slotWidthAdjust),
    gap: numberAt(value, [text.LayoutKey.ContentGap], fallback.gap),
  };
}

function normalizeGoldenCardLayout(value: unknown): PortalGoldenCardLayout {
  const fallback = DEFAULT_PORTAL_FRAME_LAYOUT.goldenCard;
  const key = text.LayoutKey;
  return {
    showContentBounds: booleanAt(value, [key.ShowContentBounds], fallback.showContentBounds),
    showContent: booleanAt(value, [key.ShowContent], fallback.showContent),
    showFrameBounds: booleanAt(value, [key.ShowFrameBounds], fallback.showFrameBounds),
    height: numberAt(value, [key.CardHeight], fallback.height),
    paddingX: numberAt(value, [key.PaddingX], fallback.paddingX),
    paddingY: numberAt(value, [key.PaddingY], fallback.paddingY),
    contentInsetX: numberAt(value, [key.ContentInsetX], fallback.contentInsetX),
    contentInsetY: numberAt(value, [key.ContentInsetY], fallback.contentInsetY),
    contentOffsetX: numberAt(value, [key.ContentOffsetX], fallback.contentOffsetX),
    contentOffsetY: numberAt(value, [key.ContentOffsetY], fallback.contentOffsetY),
    frameInset: numberAt(value, [key.FrameInset], fallback.frameInset),
    frameWidthAdjust: numberAt(value, [key.FrameWidthAdjust], fallback.frameWidthAdjust),
    frameHeightAdjust: numberAt(value, [key.FrameHeightAdjust], fallback.frameHeightAdjust),
    frameGroup: {
      offsetX: numberAt(value, [key.FrameGroup, key.OffsetX], fallback.frameGroup.offsetX),
      offsetY: numberAt(value, [key.FrameGroup, key.OffsetY], fallback.frameGroup.offsetY),
    },
    frameOpacity: numberAt(value, [key.FrameOpacity], fallback.frameOpacity),
    frameGlowBlur: numberAt(value, [key.FrameGlowBlur], fallback.frameGlowBlur),
    frameGlowOpacity: numberAt(value, [key.FrameGlowOpacity], fallback.frameGlowOpacity),
    glyphSize: numberAt(value, [key.GlyphSize], fallback.glyphSize),
    titleOffsetX: numberAt(value, [key.TitleOffsetX], fallback.titleOffsetX),
    titleOffsetY: numberAt(value, [key.TitleOffsetY], fallback.titleOffsetY),
  };
}

const PORTAL_FRAME_TARGET_RESETTERS: Record<PortalFrameTargetValue, (layout: PortalFrameLayout) => PortalFrameLayout> =
  {
    [PortalFrameTuner.FrameTarget.Main]: (layout) => ({
      ...layout,
      content: { ...layout.content, main: DEFAULT_PORTAL_FRAME_LAYOUT.content.main },
      main: cloneFrameControls(DEFAULT_PORTAL_FRAME_LAYOUT.main),
    }),
    [PortalFrameTuner.FrameTarget.SideBottom]: (layout) => ({
      ...layout,
      content: { ...layout.content, sideBottom: DEFAULT_PORTAL_FRAME_LAYOUT.content.sideBottom },
      sideBottom: cloneFrameControls(DEFAULT_PORTAL_FRAME_LAYOUT.sideBottom),
    }),
    [PortalFrameTuner.FrameTarget.SideTop]: (layout) => ({
      ...layout,
      content: { ...layout.content, sideTop: DEFAULT_PORTAL_FRAME_LAYOUT.content.sideTop },
      sideTop: cloneFrameControls(DEFAULT_PORTAL_FRAME_LAYOUT.sideTop),
    }),
    [PortalFrameTuner.FrameTarget.Side]: (layout) => ({
      ...layout,
      content: { ...layout.content, sideTop: DEFAULT_PORTAL_FRAME_LAYOUT.content.sideTop },
      sideTop: cloneFrameControls(DEFAULT_PORTAL_FRAME_LAYOUT.sideTop),
    }),
  };

function normalizeCarouselLayout(value: unknown): PortalCarouselLayout {
  const fallback = DEFAULT_PORTAL_FRAME_LAYOUT.carousel;
  const key = text.LayoutKey;
  return {
    minHeight: numberAt(value, [key.MinHeight], fallback.minHeight),
    paddingTop: numberAt(value, [key.PaddingTop], fallback.paddingTop),
    paddingRight: numberAt(value, [key.PaddingRight], fallback.paddingRight),
    paddingBottom: numberAt(value, [key.PaddingBottom], fallback.paddingBottom),
    paddingLeft: numberAt(value, [key.PaddingLeft], fallback.paddingLeft),
    frameInsetX: numberAt(value, [key.FrameInsetX], fallback.frameInsetX),
    frameInsetY: numberAt(value, [key.FrameInsetY], fallback.frameInsetY),
    frameWidthAdjust: numberAt(value, [key.FrameWidthAdjust], fallback.frameWidthAdjust),
    frameHeightAdjust: numberAt(value, [key.FrameHeightAdjust], fallback.frameHeightAdjust),
    frameGroup: {
      offsetX: numberAt(value, [key.FrameGroup, key.OffsetX], fallback.frameGroup.offsetX),
      offsetY: numberAt(value, [key.FrameGroup, key.OffsetY], fallback.frameGroup.offsetY),
    },
    frameOpacity: numberAt(value, [key.FrameOpacity], fallback.frameOpacity),
    titleLeft: numberAt(value, [key.TitleLeft], fallback.titleLeft),
    titleRight: numberAt(value, [key.TitleRight], fallback.titleRight),
    titleTop: numberAt(value, [key.TitleTop], fallback.titleTop),
    railGap: numberAt(value, [key.RailGap], fallback.railGap),
    railPadTop: numberAt(value, [key.RailPaddingTop], fallback.railPadTop),
    railPadRight: numberAt(value, [key.RailPaddingRight], fallback.railPadRight),
    railPadBottom: numberAt(value, [key.RailPaddingBottom], fallback.railPadBottom),
    railPadLeft: numberAt(value, [key.RailPaddingLeft], fallback.railPadLeft),
    cardMinWidth: numberAt(value, [key.CardMinWidth], fallback.cardMinWidth),
    cardWidthPercent: numberAt(value, [key.CardWidthPercent], fallback.cardWidthPercent),
  };
}

export function valueAt(root: unknown, path: readonly PropertyKey[]): unknown {
  let current = root;
  for (const key of path) {
    if (!isRecord(current)) {
      return undefined;
    }
    current = current[key];
  }
  return current;
}

export function isRecord(value: unknown): value is Record<PropertyKey, unknown> {
  return value !== null && Object(value) === value;
}

export function setPortalFrameLayoutValue(
  root: PortalFrameLayout,
  path: readonly PropertyKey[],
  value: unknown
): PortalFrameLayout {
  const next = structuredClone(root) as Record<PropertyKey, unknown>;
  let cursor = next;
  for (const key of path.slice(0, -1)) {
    const existing = cursor[key];
    cursor[key] = isRecord(existing) ? structuredClone(existing) : {};
    cursor = cursor[key] as Record<PropertyKey, unknown>;
  }
  cursor[path[path.length - 1]!] = value;
  return next as PortalFrameLayout;
}

function cloneFrameControls(controls: PictureViewerFrameSurfaceControls): PictureViewerFrameSurfaceControls {
  return structuredClone(controls) as PictureViewerFrameSurfaceControls;
}
