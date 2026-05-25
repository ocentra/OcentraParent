// @ts-nocheck
import {
  useEffect,
  useId,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent,
  type MouseEvent,
  type ReactElement,
  type ReactNode,
  type WheelEvent,
} from 'react';
import { getPlaceholderImageUrl, placeholderImageCount } from '@ocentra/app-assets/placeholders';
import {
  bannerParentPortalAiImageUrl,
  bannerParentPortalBrowserImageUrl,
  bannerParentPortalOverviewImageUrl,
} from '@ocentra/app-assets/banners';
import { normalizeParentPortalSvgControls, type ParentPortalSvgControls } from './ParentPortalSvgSurfaceControls';
import {
  normalizeParentPortalContent,
  type ParentPortalControlArea,
  type ParentPortalGuideNote,
  type ParentPortalGuideTopic,
  type ParentPortalIconName,
  type ParentPortalNavGroup,
  type ParentPortalNavItem,
  type ParentPortalContentData,
  type ParentPortalQuickControl,
  type ParentPortalTabDetail,
  type ParentPortalTabId,
  type ParentPortalTone,
  type PartialParentPortalContentData,
} from './ParentPortalSvgContent';
import {
  createGoldenFrameVariantConfig,
  createGoldenFrameFrameOnlySvgDataUri,
} from './ParentPortalGoldenFrameForeignObject';
import {
  getPictureViewerAnchoredFrame,
  getPictureViewerFrameGroupTransform,
  getPictureViewerFrameTransform,
  normalizePictureViewerFrameControls,
  pictureViewerDarkenHex,
  pictureViewerFrameSegmentThickness,
  pictureViewerFrameSegments,
  type PictureViewerFrameControls,
  type PictureViewerFrameSegment,
} from '../../Common/PictureViewerFrame/PictureViewerFrameControls';
import {
  AccountProfileIcon,
  AiGuideIdeaIcon,
  ActivityNetworkIcon,
  AiMemoryCircuitIcon,
  AiMemorySetBrainIcon,
  AiSetupSearchIcon,
  AlertNotificationBellIcon,
  ApiKeysChipIcon,
  AuditCloudLogsIcon,
  BrowserStackIcon,
  DataPrivacyServerShieldIcon,
  DevicesMultiScreenIcon,
  DrivesCloudIcon,
  EnforcementOfficerIcon,
  ExportRetentionIcon,
  GuideBookIcon,
  LanNetworkMonitorsIcon,
  ManageFileSettingsIcon,
  OverviewListIcon,
  PolicyShieldDocumentIcon,
  PortalGatewayIcon,
  QuickGlanceGlasses,
  ReportDocumentIcon,
  RemoteAccessMonitorsIcon,
  RulesGavelDocumentIcon,
  ScheduleCalendarClockIcon,
  StartDataAnalysisIcon,
  UpdatesSyncDocumentIcon,
  WebGlobeIcon,
} from '../../Common/NavSvgIcons';
import './ParentPortalSvgSurface.css';

type IconProps = {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  color?: string;
  strokeWidth?: number;
};

type IconComponent = (props: IconProps) => ReactElement;
type Tone = ParentPortalTone;
type DetailMode = 'row' | 'control' | 'season';
type ParentPortalFocusSection = 'highlights' | 'table';
type ParentPortalTopCardItem =
  | {
      kind: 'row';
      key: string;
      row: DisplayRow;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    }
  | {
      kind: 'control';
      key: string;
      control: ControlArea | QuickControl;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    }
  | {
      kind: 'guide';
      key: string;
      topic: ParentPortalGuideTopic;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    };

export type ParentPortalMode = 'parentOverview' | 'parentManage' | 'parentGuide';

export type ParentPortalRow = {
  label: string;
  order: number;
  signalScore: number;
  readyCount?: number;
  gapCount?: number;
  primaryArea?: string;
  trend?: string;
  tone?: Tone;
};

type ParentPortalSvgSurfaceProps = {
  pageMode: ParentPortalMode;
  controlCode: number;
  seasonId: string;
  lastUpdated: string;
  parentPortalRows: ParentPortalRow[];
  userEntry: ParentPortalRow | null;
  nearbyAbove: ParentPortalRow[];
  nearbyBelow: ParentPortalRow[];
  controlId?: string;
  loading?: boolean;
  error?: string | null;
  controls?: Partial<ParentPortalSvgControls> | null;
  content?: PartialParentPortalContentData | null;
  initialNavLabel?: string;
  initialSelectedControlId?: string;
  onRefreshParentPortal: (controlCode: number) => void;
  onMatchmaking: () => void;
  onNavigate?: (routePath: string) => void;
};

type NavItem = Omit<ParentPortalNavItem, 'icon'> & {
  icon: IconComponent;
  imageUrl: string;
};

type NavGroup = ParentPortalNavGroup & {
  items: NavItem[];
};

type TabDetail = ParentPortalTabDetail;

type DisplayRow = {
  id: string;
  order: number;
  label: string;
  signal: string;
  signals: string;
  readyCount: string;
  readiness: string;
  primaryArea: string;
  trend: string;
  tone: Tone;
};

type ParentPortalTableVariant = 'statusRows' | 'controls' | 'ai' | 'routines' | 'support' | 'ownership';

type ControlArea = ParentPortalControlArea;

type QuickControl = Omit<ParentPortalQuickControl, 'icon'> & {
  icon: IconComponent;
};

type SelectableControl = ParentPortalControlArea | ParentPortalQuickControl;

type ControlCategorySummary = {
  id: string;
  label: string;
  detail: string;
  count: number;
  tone: Tone;
  sampleControl: QuickControl;
  subcategories: ControlSubcategorySummary[];
};

type ControlSubcategorySummary = {
  id: string;
  label: string;
  count: number;
  tone: Tone;
  sampleControl: QuickControl;
};

const PARENT_PORTAL_RESPONSIVE_MIN_LEFT_W = 210;
const PARENT_PORTAL_RESPONSIVE_MIN_RIGHT_W = 250;
const PARENT_PORTAL_RESPONSIVE_MIN_MAIN_W = 560;
const PARENT_PORTAL_RESPONSIVE_COMPACT_SURFACE_W = 1600;
const PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_W = 8192;
const PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_H = 2800;
const PARENT_PORTAL_TOP_CAROUSEL_MAX_VISIBLE = 5;
const PARENT_PORTAL_SIDE_HANDLE_W = 15;
const PARENT_PORTAL_SIDE_HANDLE_OVERLAP = 1;
const PARENT_PORTAL_CATEGORY_LABELS = [
  'Browser',
  'Policy',
  'Activity',
  'Privacy',
  'Memory',
  'AI',
  'Devices',
  'Support',
] as const;

function clampValue(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

function wrapIndex(value: number, length: number): number {
  if (length <= 0) {
    return 0;
  }
  return ((value % length) + length) % length;
}

function fitSingleLineTextSize(text: string, width: number, min: number, max: number, factor = 0.56): number {
  if (!text) return max;
  return clampValue(width / Math.max(1, text.length * factor), min, max);
}

function truncateTextForWidth(text: string, width: number, fontSize: number, factor = 0.56): string {
  const maxChars = Math.max(1, Math.floor(width / Math.max(1, fontSize * factor)));
  if (text.length <= maxChars) return text;
  if (maxChars <= 3) return text.slice(0, maxChars);
  return `${text.slice(0, maxChars - 3).trimEnd()}...`;
}

function compactControlStatLabel(value: string): string {
  const text = value.trim().replace(/\s+/g, ' ');
  if (!text) return '';
  if (/^\d+(?:\.\d+)?\s*[kmb]$/i.test(text) || /^[+-]?\d+(?:\.\d+)?%$/.test(text)) return text;
  const range = text.match(/\b\d+\s*[-–]\s*\d+\b/);
  if (range) return range[0].replace(/\s+/g, '');
  const number = text.match(/\b\d+\b/);
  return number ? number[0] : text;
}

function minimumParentPortalCanvasWidth(cfg: ParentPortalSvgControls): number {
  return Math.max(
    cfg.canvas.width,
    Math.ceil(
      cfg.layout.outerPad * 2 +
        PARENT_PORTAL_RESPONSIVE_MIN_LEFT_W +
        PARENT_PORTAL_RESPONSIVE_MIN_RIGHT_W +
        PARENT_PORTAL_RESPONSIVE_MIN_MAIN_W +
        cfg.layout.gap * 2
    )
  );
}

function parentPortalCanvasWidthForSurface(
  cfg: ParentPortalSvgControls,
  surfaceSize: { width: number; height: number }
): number {
  if (surfaceSize.width <= 0 || surfaceSize.height <= 0) return cfg.canvas.width;
  const minimumWidth = minimumParentPortalCanvasWidth(cfg);
  const ratioWidth = Math.round(cfg.canvas.height * (surfaceSize.width / surfaceSize.height));
  return Math.max(minimumWidth, Math.min(PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_W, ratioWidth));
}

function compactParentPortalCanvasWidth(cfg: ParentPortalSvgControls): number {
  return Math.ceil(cfg.layout.outerPad * 2 + cfg.layout.leftW + cfg.layout.gap + PARENT_PORTAL_RESPONSIVE_MIN_MAIN_W);
}

function parentPortalCanvasSizeForSurface(
  cfg: ParentPortalSvgControls,
  surfaceSize: { width: number; height: number }
): { width: number; height: number } {
  if (surfaceSize.width <= 0 || surfaceSize.height <= 0) {
    return cfg.canvas;
  }

  if (surfaceSize.width >= PARENT_PORTAL_RESPONSIVE_COMPACT_SURFACE_W) {
    const width = parentPortalCanvasWidthForSurface(cfg, surfaceSize);
    const aspectHeight = Math.round(width * (surfaceSize.height / surfaceSize.width));
    return {
      width,
      height: clampValue(aspectHeight, cfg.canvas.height, PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_H),
    };
  }

  const compactWidth = compactParentPortalCanvasWidth(cfg);
  const aspectWidth = Math.round(cfg.canvas.height * (surfaceSize.width / surfaceSize.height));
  const width = clampValue(aspectWidth, compactWidth, cfg.canvas.width);
  const aspectHeight = Math.round(width * (surfaceSize.height / surfaceSize.width));

  return {
    width,
    height: clampValue(aspectHeight, cfg.canvas.height, PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_H),
  };
}

function responsiveParentPortalColumnWidths(
  canvasWidth: number,
  cfg: ParentPortalSvgControls
): { leftW: number; mainW: number; rightW: number } {
  const availableW = canvasWidth - cfg.layout.outerPad * 2 - cfg.layout.gap;
  const leftW = clampValue(cfg.layout.leftW, PARENT_PORTAL_RESPONSIVE_MIN_LEFT_W, cfg.layout.leftW);
  const mainW = Math.max(PARENT_PORTAL_RESPONSIVE_MIN_MAIN_W, availableW - leftW);
  return { leftW, mainW, rightW: 0 };
}

const iconByName: Record<ParentPortalIconName, IconComponent> = {
  'quick-glance': QuickGlanceGlasses,
  overview: OverviewListIcon,
  start: StartDataAnalysisIcon,
  guide: GuideBookIcon,
  manage: ManageFileSettingsIcon,
  policy: PolicyShieldDocumentIcon,
  browser: BrowserStackIcon,
  web: WebGlobeIcon,
  schedule: ScheduleCalendarClockIcon,
  alerts: AlertNotificationBellIcon,
  report: ReportDocumentIcon,
  rules: RulesGavelDocumentIcon,
  updates: UpdatesSyncDocumentIcon,
  activity: ActivityNetworkIcon,
  portal: PortalGatewayIcon,
  privacy: DataPrivacyServerShieldIcon,
  lan: LanNetworkMonitorsIcon,
  devices: DevicesMultiScreenIcon,
  remote: RemoteAccessMonitorsIcon,
  'ai-setup': AiSetupSearchIcon,
  'ai-guide': AiGuideIdeaIcon,
  'ai-memory-set': AiMemorySetBrainIcon,
  api: ApiKeysChipIcon,
  export: ExportRetentionIcon,
  drives: DrivesCloudIcon,
  audit: AuditCloudLogsIcon,
  'ai-memory': AiMemoryCircuitIcon,
  account: AccountProfileIcon,
  enforcement: EnforcementOfficerIcon,
};

function iconForName(icon: ParentPortalIconName): IconComponent {
  return iconByName[icon] ?? OverviewListIcon;
}

function iconForNavItem(item: ParentPortalNavItem): IconComponent {
  return iconForName(item.icon);
}

function toneColor(tone: Tone, cfg: ParentPortalSvgControls): string {
  return cfg.colors[tone];
}

function colorAlpha(color: string, alphaHex: string): string {
  return color.startsWith('#') ? `${color}${alphaHex}` : color;
}

function assetKey(value?: string): string {
  return (value ?? '')
    .trim()
    .toLowerCase()
    .replace(/&/g, 'and')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

function hashString(value: string): number {
  return Array.from(value).reduce((hash, char) => (hash * 31 + char.charCodeAt(0)) >>> 0, 0);
}

function rowAvatarImageUrl(label: string): string {
  const imageIndex = placeholderImageCount > 0 ? hashString(`row:${label}`) % placeholderImageCount : 0;
  return getPlaceholderImageUrl(imageIndex);
}

function parentPortalControlImageUrl(value?: string): string | null {
  const key = assetKey(value);
  if (!key) return null;
  if (key.includes('ai-benchmark') || key.includes('model')) return bannerParentPortalAiImageUrl;
  if (key.includes('parent-overview') || key.includes('quick-access') || key.includes('hub'))
    return bannerParentPortalOverviewImageUrl;
  if (
    key.includes('all-controls') ||
    key.includes('catalog') ||
    key.includes('browser') ||
    key.includes('web') ||
    key.includes('device') ||
    key.includes('lan')
  ) {
    return bannerParentPortalBrowserImageUrl;
  }
  return bannerParentPortalOverviewImageUrl;
}

function parentPortalControlArtworkUrl(control: Pick<ControlArea | QuickControl, 'id' | 'name'>): string {
  const key = control.id || control.name;
  const imageIndex = placeholderImageCount > 0 ? hashString(key) % placeholderImageCount : 0;
  return getPlaceholderImageUrl(imageIndex);
}

function parentPortalControlCategoryImageUrl(category: ControlCategorySummary): string | null {
  return category.count > 0 ? parentPortalControlArtworkUrl(category.sampleControl) : null;
}

function navItemImageUrl(item: ParentPortalNavItem): string {
  const key = assetKey(item.label);
  if (key.includes('overview')) return bannerParentPortalOverviewImageUrl;
  if (key.includes('overall') || key.includes('global') || key.includes('family'))
    return bannerParentPortalOverviewImageUrl;
  if (key.includes('ai')) return bannerParentPortalAiImageUrl;
  if (
    key.includes('category') ||
    key.includes('control') ||
    key.includes('device') ||
    key.includes('browser') ||
    key.includes('web')
  ) {
    return bannerParentPortalBrowserImageUrl;
  }
  return bannerParentPortalOverviewImageUrl;
}

function cutRectPath(x: number, y: number, w: number, h: number, cut: number) {
  const c = Math.min(cut, w / 2, h / 2);
  return [
    `M ${x + c} ${y}`,
    `H ${x + w - c}`,
    `L ${x + w} ${y + c}`,
    `V ${y + h - c}`,
    `L ${x + w - c} ${y + h}`,
    `H ${x + c}`,
    `L ${x} ${y + h - c}`,
    `V ${y + c}`,
    'Z',
  ].join(' ');
}

function bottomCutRectPath(x: number, y: number, w: number, h: number, cut: number) {
  const c = Math.min(cut, w / 2, h / 2);
  return [
    `M ${x} ${y}`,
    `H ${x + w}`,
    `V ${y + h - c}`,
    `L ${x + w - c} ${y + h}`,
    `H ${x + c}`,
    `L ${x} ${y + h - c}`,
    'Z',
  ].join(' ');
}

function hexPath(cx: number, cy: number, radius: number) {
  return (
    Array.from({ length: 6 }, (_, index) => {
      const angle = Math.PI / 6 + (index * Math.PI) / 3;
      const x = cx + Math.cos(angle) * radius;
      const y = cy + Math.sin(angle) * radius;
      return `${index === 0 ? 'M' : 'L'} ${x.toFixed(1)} ${y.toFixed(1)}`;
    }).join(' ') + ' Z'
  );
}

type ParentPortalRect = { x: number; y: number; w: number; h: number };

function parentPortalFrameRects(
  x: number,
  y: number,
  w: number,
  h: number,
  footerH = 38,
  headerH = 48
): { body: ParentPortalRect; footer: ParentPortalRect; headerH: number; footerH: number } {
  const inset = 18;
  return {
    body: {
      x: x + inset,
      y: y + headerH + 10,
      w: Math.max(1, w - inset * 2),
      h: Math.max(1, h - headerH - footerH - 18),
    },
    footer: {
      x: x + inset,
      y: y + h - footerH,
      w: Math.max(1, w - inset * 2),
      h: footerH,
    },
    headerH,
    footerH,
  };
}

function formatRouteScope(value?: string): string {
  if (!value) return 'All controls';
  return value
    .split(/[/:_-]/)
    .filter(Boolean)
    .slice(0, 3)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function initialTabForPageMode(pageMode: ParentPortalMode, content: ParentPortalContentData): ParentPortalTabId {
  return content.modes[pageMode]?.defaultTab ?? 'overall';
}

function initialNavLabelForTab(navItems: NavItem[], tab: ParentPortalTabId): string {
  return navItems.find((item) => item.tabId === tab)?.label ?? navItems[0]?.label ?? '';
}

function groupedParentPortalNavItems(navGroups: ParentPortalNavGroup[], navItems: NavItem[]): NavGroup[] {
  const groupIds = new Set(navGroups.map((group) => group.id));
  const groups = navGroups.map((group) => ({
    ...group,
    items: navItems.filter((item) => item.groupId === group.id),
  }));
  const ungroupedItems = navItems.filter((item) => !item.groupId || !groupIds.has(item.groupId));
  if (ungroupedItems.length === 0) return groups.filter((group) => group.items.length > 0);
  return [
    ...groups.filter((group) => group.items.length > 0),
    {
      id: 'menu',
      label: 'MENU',
      detail: 'Navigation',
      items: ungroupedItems,
    },
  ];
}

function navSectionsForGroup(group: NavGroup): Array<{ id: string; label: string; items: NavItem[] }> {
  const sections: Array<{ id: string; label: string; items: NavItem[] }> = [];
  const sectionByLabel = new Map<string, { id: string; label: string; items: NavItem[] }>();
  let unsectioned: { id: string; label: string; items: NavItem[] } | null = null;
  for (const item of group.items) {
    const label = item.sectionLabel ?? '';
    if (!label) {
      if (!unsectioned) {
        unsectioned = { id: `${group.id}:items`, label: '', items: [] };
        sections.push(unsectioned);
      }
      unsectioned.items.push(item);
      continue;
    }
    let section = sectionByLabel.get(label);
    if (!section) {
      section = { id: navSectionId(group.id, label), label, items: [] };
      sectionByLabel.set(label, section);
      sections.push(section);
    }
    section.items.push(item);
  }
  return sections;
}

function navSectionIcon(section: { label: string; items: NavItem[] }): IconComponent {
  if (assetKey(section.label).includes('polic')) return PolicyShieldDocumentIcon;
  if (assetKey(section.label).includes('activity')) return ActivityNetworkIcon;
  if (assetKey(section.label).includes('portal')) return PortalGatewayIcon;
  if (assetKey(section.label).includes('data-privacy')) return DataPrivacyServerShieldIcon;
  if (assetKey(section.label).includes('ai-memory')) return AiMemoryCircuitIcon;
  if (assetKey(section.label).includes('account')) return AccountProfileIcon;
  if (assetKey(section.label).includes('remote')) return RemoteAccessMonitorsIcon;
  if (assetKey(section.label).includes('lan')) return LanNetworkMonitorsIcon;
  if (assetKey(section.label).includes('device')) return DevicesMultiScreenIcon;
  return section.items[0]?.icon ?? OverviewListIcon;
}

function navGroupIdForNavLabel(navGroups: NavGroup[], navLabel: string): string {
  return navGroups.find((group) => group.items.some((item) => item.label === navLabel))?.id ?? '';
}

function initialOpenNavGroupIds(navGroups: NavGroup[], navLabel: string): Record<string, boolean> {
  const activeGroupId = navGroupIdForNavLabel(navGroups, navLabel);
  return Object.fromEntries(
    navGroups.map((group, index) => [group.id, group.id === activeGroupId || (!activeGroupId && index === 0)])
  );
}

function navSectionId(groupId: string, sectionLabel: string): string {
  return `${groupId}:${sectionLabel}`;
}

function navSectionIdsForGroups(navGroups: NavGroup[]): string[] {
  return navGroups.flatMap((group) => {
    const labels = new Set(group.items.map((item) => item.sectionLabel).filter(Boolean));
    return Array.from(labels, (label) => navSectionId(group.id, label as string));
  });
}

function navSectionIdForNavLabel(navGroups: NavGroup[], navLabel: string): string | null {
  for (const group of navGroups) {
    const item = group.items.find((entry) => entry.label === navLabel);
    if (item?.sectionLabel) return navSectionId(group.id, item.sectionLabel);
  }
  return null;
}

function initialOpenNavSectionIds(navGroups: NavGroup[], navLabel: string): Record<string, boolean> {
  const activeSectionId = navSectionIdForNavLabel(navGroups, navLabel);
  return Object.fromEntries(
    navGroups.flatMap((group) => {
      const labels = Array.from(new Set(group.items.map((item) => item.sectionLabel).filter(Boolean)));
      return labels.map((label, index) => {
        const sectionId = navSectionId(group.id, label as string);
        return [sectionId, sectionId === activeSectionId || index === 0];
      });
    })
  );
}

function ensureOpenNavGroupIds(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  navLabel: string
): Record<string, boolean> {
  const activeGroupId = navGroupIdForNavLabel(navGroups, navLabel);
  return Object.fromEntries(
    navGroups.map((group) => [group.id, activeGroupId ? group.id === activeGroupId : Boolean(current[group.id])])
  );
}

function ensureOpenNavSectionIds(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  navLabel: string
): Record<string, boolean> {
  const activeSectionId = navSectionIdForNavLabel(navGroups, navLabel);
  if (!activeSectionId) return current;
  return Object.fromEntries(
    navSectionIdsForGroups(navGroups).map((sectionId) => [sectionId, sectionId === activeSectionId])
  );
}

function toggleOpenNavGroupId(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  groupId: string
): Record<string, boolean> {
  const nextOpen = !current[groupId];
  return Object.fromEntries(
    navGroups.map((group) => [
      group.id,
      group.id === groupId ? nextOpen : nextOpen ? false : Boolean(current[group.id]),
    ])
  );
}

function toggleOpenNavSectionId(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  sectionId: string
): Record<string, boolean> {
  const nextOpen = !current[sectionId];
  return Object.fromEntries(
    navSectionIdsForGroups(navGroups).map((entryId) => [
      entryId,
      entryId === sectionId ? nextOpen : nextOpen ? false : Boolean(current[entryId]),
    ])
  );
}

function normalizeSelectionId(value?: string): string {
  return (value ?? '').trim().toLowerCase();
}

function findSelectedControl(
  content: ParentPortalContentData,
  selectedControlId: string
): SelectableControl | undefined {
  const normalizedId = normalizeSelectionId(selectedControlId);
  return [...content.controlAreas, ...content.quickControls.filter(isParentPortalControlEntry)].find(
    (control) => normalizeSelectionId(control.id) === normalizedId
  );
}

function initialControlIdForPageMode(
  pageMode: ParentPortalMode,
  content: ParentPortalContentData,
  controlId?: string
): string {
  const routeId = normalizeSelectionId(controlId);
  const contentControls = content.quickControls.filter(isParentPortalControlEntry);
  const routeControl = [...content.controlAreas, ...contentControls].find(
    (control) => normalizeSelectionId(control.id) === routeId
  );
  if (routeControl) return routeControl.id;
  return content.modes[pageMode]?.selectedControlId ?? content.controlAreas[0]?.id ?? contentControls[0]?.id ?? '';
}

function isParentPortalControlEntry(control: Pick<ParentPortalQuickControl, 'id' | 'name' | 'routePath'>): boolean {
  const id = normalizeSelectionId(control.id);
  const name = normalizeSelectionId(control.name);
  if (id === 'parent-portal-hub' || id === 'quick-access' || id === 'all-controls') return false;
  if (name === 'quick-access' || name === 'all-controls') return false;
  return true;
}

function isHashRoutePath(routePath?: string): routePath is string {
  return typeof routePath === 'string' && routePath.startsWith('#/');
}

function rowSourceForPageMode(
  content: ParentPortalContentData,
  pageMode: ParentPortalMode,
  parentPortalRows: ParentPortalRow[]
): ParentPortalRow[] {
  const source = content.modes[pageMode]?.rowSource ?? 'api';
  if (source === 'aiBenchmarkRows') return content.aiBenchmarkRows;
  if (source === 'fallbackRows') return content.fallbackRows;
  return parentPortalRows.length > 0 ? parentPortalRows : content.fallbackRows;
}

function toDisplayRows(
  rows: ParentPortalRow[],
  pageMode: ParentPortalMode,
  selectedControlName: string,
  controlId?: string
): DisplayRow[] {
  const primaryArea = pageMode === 'parentManage' ? selectedControlName || formatRouteScope(controlId) : 'Mixed';
  return rows.map((row, index) => {
    const readyCount = row.readyCount ?? Math.max(0, Math.round(row.signalScore * 0.32));
    const gapCount = row.gapCount ?? Math.max(1, Math.round(readyCount * 0.62));
    const readiness =
      readyCount + gapCount > 0 ? `${Math.round((readyCount / (readyCount + gapCount)) * 1000) / 10}%` : '-';
    const tones: Tone[] = ['purple', 'red', 'cyan', 'gold', 'purple', 'red', 'cyan'];
    return {
      id: row.label || `row-${row.order}`,
      order: row.order,
      label: row.label || `Item ${row.order}`,
      signal: row.signalScore.toLocaleString(),
      signals: (readyCount + gapCount).toLocaleString(),
      readyCount: readyCount.toLocaleString(),
      readiness,
      primaryArea: row.primaryArea ?? primaryArea,
      trend: row.trend ?? (index % 3 === 0 ? '+2' : index % 3 === 1 ? '+1' : '-'),
      tone: row.tone ?? tones[index % tones.length],
    };
  });
}

function tableVariantForContext(activeNavLabel: string, activeTab: ParentPortalTabId): ParentPortalTableVariant {
  const key = assetKey(activeNavLabel);
  if (key.includes('ai') || activeTab === 'aiStatus') return 'ai';
  if (key.includes('friend') || key.includes('guild') || activeTab === 'support') return 'support';
  if (key.includes('per-game') || key.includes('per-category') || activeTab === 'controls') return 'controls';
  if (key.includes('tournament') || key.includes('season') || key.includes('reward') || activeTab === 'routines')
    return 'routines';
  return 'statusRows';
}

function tableTitleForVariant(
  variant: ParentPortalTableVariant,
  activeNavLabel: string,
  selectedControlName: string
): string {
  const key = assetKey(activeNavLabel);
  if (key.includes('overview') || key.includes('today')) return 'TODAY CONTROL SNAPSHOT';
  if (variant === 'controls') return `${selectedControlName.toUpperCase()} CONTROL DETAIL`;
  if (variant === 'ai') return 'LOCAL AI AND MEMORY READINESS';
  if (variant === 'routines') return 'DEVICE ROUTINE AND APPROVALS';
  if (variant === 'support') return 'SUPPORT EXPORTS AND DRIVE CONNECTIONS';
  if (variant === 'ownership') return 'PARENT OWNERSHIP DETAIL';
  return 'PARENT CONTROL DETAIL';
}

function parentPortalScopeKey(value?: string): string {
  return (value ?? '').toLowerCase().replace(/[^a-z0-9]/g, '');
}

function rowsForControlScope(rows: DisplayRow[], selectedControlName: string, selectedControlId: string): DisplayRow[] {
  const controlKeys = new Set(
    [
      parentPortalScopeKey(selectedControlName),
      parentPortalScopeKey(selectedControlId),
      parentPortalScopeKey(selectedControlName.replace(/^three card/i, '3 card')),
      parentPortalScopeKey(selectedControlName.replace(/^3 card/i, 'three card')),
    ].filter(Boolean)
  );
  const matched = rows.filter((row) => controlKeys.has(parentPortalScopeKey(row.primaryArea)));
  const source = matched.length > 0 ? matched : rows.slice(0, Math.min(10, rows.length));
  return source.map((row, index) => ({
    ...row,
    order: index + 1,
    primaryArea: selectedControlName || row.primaryArea,
  }));
}

function rowsForCategoryScope(rows: DisplayRow[], selectedCategoryLabel: string): DisplayRow[] {
  const categoryKey = assetKey(selectedCategoryLabel);
  const matched = rows.filter(
    (row) => assetKey(controlCategoryLabel({ id: row.primaryArea, name: row.primaryArea })) === categoryKey
  );
  const source = matched.length > 0 ? matched : rows.slice(0, Math.min(10, rows.length));
  return source.map((row, index) => ({
    ...row,
    order: index + 1,
    primaryArea: selectedCategoryLabel || row.primaryArea,
  }));
}

function rowTopCard(row: DisplayRow): ParentPortalTopCardItem {
  return {
    kind: 'row',
    key: `row:${row.id}`,
    row,
    title: row.label,
    subtitle: `Order ${row.order} / ${row.primaryArea}`,
    value: row.signal,
    detail: `${row.readiness} ready`,
    tone: row.tone,
  };
}

function titleCaseControlName(value: string): string {
  return value
    .toLowerCase()
    .split(' ')
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function controlCategoryLabel(
  control: Pick<ControlArea | QuickControl, 'id' | 'name'> & { category?: string }
): string {
  if (control.category) return control.category;
  const key = assetKey(`${control.id} ${control.name}`);
  if (key.includes('browser') || key.includes('web')) return 'Browser';
  if (key.includes('policy') || key.includes('rule')) return 'Policy';
  if (key.includes('activity') || key.includes('evidence')) return 'Activity';
  if (key.includes('privacy') || key.includes('private')) return 'Privacy';
  if (key.includes('memory')) return 'Memory';
  if (key.includes('ai')) return 'AI';
  if (key.includes('device') || key.includes('notification') || key.includes('setting')) return 'Devices';
  if (key.includes('support') || key.includes('export') || key.includes('drive')) return 'Support';
  return 'Controls';
}

function taxonomyLeafLabel(value: string): string {
  const parts = value
    .split('/')
    .map((part) => part.trim())
    .filter(Boolean);
  return parts[parts.length - 1] ?? value;
}

function controlSubcategoryLabel(control: { detail?: string; subcategory?: string | null }): string {
  const value = control.subcategory?.trim() || control.detail?.trim();
  return value ? taxonomyLeafLabel(value) : 'General';
}

function controlTopCard(
  control: ControlArea | QuickControl,
  statsControl?: ControlArea,
  index = 0
): ParentPortalTopCardItem {
  const title = control.name === control.name.toUpperCase() ? titleCaseControlName(control.name) : control.name;
  const value = 'matches' in control ? control.matches : (statsControl?.matches ?? control.detail);
  const detail = 'growth' in control ? control.growth : (statsControl?.growth ?? control.subcategory ?? 'View control');
  return {
    kind: 'control',
    key: `control:${normalizeSelectionId(control.id)}:${assetKey(control.name)}:${index}`,
    control,
    title,
    subtitle: controlCategoryLabel(control),
    value,
    detail,
    tone: control.tone,
  };
}

function guideTopCard(topic: ParentPortalGuideTopic): ParentPortalTopCardItem {
  return {
    kind: 'guide',
    key: `guide:${normalizeSelectionId(topic.id)}`,
    topic,
    title: topic.title,
    subtitle: topic.subtitle,
    value: topic.category,
    detail: topic.detail,
    tone: topic.tone,
  };
}

function buildControlCategorySummaries(controls: QuickControl[]): ControlCategorySummary[] {
  const summaries = new Map<string, ControlCategorySummary>();
  const subcategoryMaps = new Map<string, Map<string, ControlSubcategorySummary>>();
  const fallbackControl = controls[0];
  const tones: Tone[] = ['gold', 'cyan', 'purple', 'red', 'muted'];
  for (const control of controls) {
    const label = controlCategoryLabel(control);
    const id = assetKey(label);
    const subcategoryLabel = controlSubcategoryLabel(control);
    const subcategoryId = assetKey(subcategoryLabel);
    const existing = summaries.get(id);
    if (existing) {
      existing.count += 1;
      const subcategories = subcategoryMaps.get(id) ?? new Map<string, ControlSubcategorySummary>();
      const existingSubcategory = subcategories.get(subcategoryId);
      if (existingSubcategory) {
        existingSubcategory.count += 1;
      } else {
        subcategories.set(subcategoryId, {
          id: subcategoryId,
          label: subcategoryLabel,
          count: 1,
          tone: control.tone,
          sampleControl: control,
        });
      }
      subcategoryMaps.set(id, subcategories);
      existing.subcategories = Array.from(subcategories.values()).sort(
        (a, b) => b.count - a.count || a.label.localeCompare(b.label)
      );
      continue;
    }
    const firstSubcategory: ControlSubcategorySummary = {
      id: subcategoryId,
      label: subcategoryLabel,
      count: 1,
      tone: control.tone,
      sampleControl: control,
    };
    subcategoryMaps.set(id, new Map([[subcategoryId, firstSubcategory]]));
    summaries.set(id, {
      id,
      label,
      detail: subcategoryLabel,
      count: 1,
      tone: control.tone,
      sampleControl: control,
      subcategories: [firstSubcategory],
    });
  }
  PARENT_PORTAL_CATEGORY_LABELS.forEach((label, index) => {
    const id = assetKey(label);
    if (summaries.has(id) || !fallbackControl) return;
    summaries.set(id, {
      id,
      label,
      detail: 'Catalog scope',
      count: 0,
      tone: tones[index % tones.length],
      sampleControl: fallbackControl,
      subcategories: [],
    });
  });
  return Array.from(summaries.values()).sort((a, b) => {
    if (a.count !== b.count) return b.count - a.count;
    const aKnown = PARENT_PORTAL_CATEGORY_LABELS.indexOf(a.label as (typeof PARENT_PORTAL_CATEGORY_LABELS)[number]);
    const bKnown = PARENT_PORTAL_CATEGORY_LABELS.indexOf(b.label as (typeof PARENT_PORTAL_CATEGORY_LABELS)[number]);
    if (aKnown >= 0 && bKnown >= 0) return aKnown - bKnown;
    if (aKnown >= 0) return -1;
    if (bKnown >= 0) return 1;
    return a.label.localeCompare(b.label);
  });
}

function detailForNav(activeNavLabel: string, detail: TabDetail): TabDetail {
  const key = assetKey(activeNavLabel);
  if (!key.includes('overview') && !key.includes('today')) return detail;
  return {
    ...detail,
    eyebrow: 'Family command',
    title: 'Today',
    summary: 'Snapshot of child-device connection, evidence readiness, browser controls, and setup gaps.',
    primary: 'Current device state',
    secondary: 'Controls, evidence, privacy, and setup',
    action: 'Review today',
    tone: 'cyan',
  };
}

function SurfacePanel({
  x,
  y,
  w,
  h,
  tone = 'cyan',
  frame = 'default',
  selected = false,
  disabled = false,
  frameCornerThicknessScale = 1,
  onClick,
  ariaLabel,
  children,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  tone?: Tone;
  frame?: 'default' | 'deckSide';
  selected?: boolean;
  disabled?: boolean;
  frameCornerThicknessScale?: number;
  onClick?: () => void;
  ariaLabel?: string;
  children?: ReactNode;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const interactive = Boolean(onClick) && !disabled;
  const active = selected || hovered;
  const fill = selected ? cfg.colors.selectedFill : hovered ? `${color}20` : cfg.colors.panelFill;
  const handleClick = (event: MouseEvent<SVGGElement>) => {
    if (!interactive) return;
    event.stopPropagation();
    onClick?.();
  };
  const handleKeyDown = (event: KeyboardEvent<SVGGElement>) => {
    if (!interactive || (event.key !== 'Enter' && event.key !== ' ')) return;
    event.preventDefault();
    event.stopPropagation();
    onClick?.();
  };
  return (
    <g
      className={interactive ? 'parent-portal-svg-clickable' : undefined}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      onMouseEnter={() => interactive && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      role={interactive ? 'button' : undefined}
      tabIndex={interactive ? 0 : undefined}
      aria-label={ariaLabel}
      aria-disabled={disabled || undefined}
    >
      {frame === 'deckSide' ? (
        <>
          <path d={cutRectPath(x + 5, y + 5, w - 10, h - 10, 18)} fill="#020b14" stroke="none" pointerEvents="none" />
          <ParentPortalSidePanelFrame
            x={x}
            y={y}
            w={w}
            h={h}
            tone={tone}
            active={active}
            cornerThicknessScale={frameCornerThicknessScale}
            cfg={cfg}
          />
        </>
      ) : (
        <>
          {hovered && !selected ? (
            <path
              d={cutRectPath(
                x - cfg.chrome.hoverPad,
                y - cfg.chrome.hoverPad,
                w + cfg.chrome.hoverPad * 2,
                h + cfg.chrome.hoverPad * 2,
                cfg.chrome.panelCut
              )}
              fill="none"
              stroke={color}
              strokeWidth={2.2}
              opacity={cfg.chrome.glowOpacity}
              filter="url(#parentPortalGlow)"
            />
          ) : null}
          <path
            d={cutRectPath(x, y, w, h, active ? cfg.chrome.panelCut + 2 : cfg.chrome.panelCut)}
            fill={fill}
            stroke={color}
            strokeWidth={active ? cfg.chrome.panelStrokeWidth + 0.5 : cfg.chrome.panelStrokeWidth}
            opacity={disabled ? 0.48 : 0.97}
          />
          <path
            d={cutRectPath(
              x + cfg.chrome.panelInnerInset,
              y + cfg.chrome.panelInnerInset,
              w - cfg.chrome.panelInnerInset * 2,
              h - cfg.chrome.panelInnerInset * 2,
              Math.max(4, cfg.chrome.panelCut - 4)
            )}
            fill="none"
            stroke={color}
            strokeWidth={0.7}
            opacity={active ? 0.52 : 0.25}
          />
          {selected ? (
            <path
              d={cutRectPath(x, y, w, h, cfg.chrome.panelCut + 2)}
              fill="none"
              stroke={color}
              strokeWidth={5}
              opacity={0.16}
              filter="url(#parentPortalGlow)"
            />
          ) : null}
        </>
      )}
      {children}
    </g>
  );
}

function ParentPortalFrameSideHandle({
  x,
  y,
  side,
  disabled = false,
  height = 132,
  width = 22,
  onClick,
  cfg,
}: {
  x: number;
  y: number;
  side: 'left' | 'right';
  disabled?: boolean;
  height?: number;
  width?: number;
  onClick: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const handleW = width;
  const handleH = height;
  const color = hovered && !disabled ? '#23ff98' : cfg.colors.cyan;
  const compact = handleH <= 64;
  const visualH = compact ? 40 : handleH;
  const visualY = y + (handleH - visualH) / 2;
  const tipInset = Math.max(5, Math.min(7, handleW * 0.32));
  const arrowInset = compact ? 4.2 : Math.max(5, Math.min(6, handleW * 0.32));
  const arrowHalfH = compact ? 4.8 : Math.max(7, Math.min(10, handleH * 0.18));
  const bodyPath =
    side === 'left'
      ? `M ${x + tipInset} ${visualY} H ${x + handleW} V ${visualY + visualH} H ${x + tipInset} L ${x} ${visualY + visualH - tipInset} V ${visualY + tipInset} Z`
      : `M ${x} ${visualY} H ${x + handleW - tipInset} L ${x + handleW} ${visualY + tipInset} V ${visualY + visualH - tipInset} L ${x + handleW - tipInset} ${visualY + visualH} H ${x} Z`;
  const arrowTipX = side === 'left' ? x + tipInset : x + handleW - tipInset;
  const arrowBackX = side === 'left' ? x + tipInset + arrowInset : x + handleW - tipInset - arrowInset;
  const arrowY = visualY + visualH / 2;
  const arrowPath = `M ${arrowTipX} ${arrowY} L ${arrowBackX} ${arrowY - arrowHalfH} V ${arrowY + arrowHalfH} Z`;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={disabled ? -1 : 0}
      aria-disabled={disabled}
      aria-label={side === 'left' ? 'Previous parent portal carousel page' : 'Next parent portal carousel page'}
      opacity={disabled ? 0.72 : 1}
      onClick={(event) => {
        event.stopPropagation();
        if (!disabled) onClick();
      }}
      onKeyDown={(event) => {
        if (disabled || (event.key !== 'Enter' && event.key !== ' ')) return;
        event.preventDefault();
        event.stopPropagation();
        onClick();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {hovered && !disabled ? (
        <path d={bodyPath} fill={color} opacity={0.24} filter="url(#parentPortalGreenGlow)" pointerEvents="none" />
      ) : null}
      <path
        d={bodyPath}
        fill={hovered && !disabled ? 'rgba(13, 89, 48, 0.9)' : 'rgba(8, 47, 75, 0.9)'}
        stroke={color}
        strokeWidth={hovered && !disabled ? 2 : 1.35}
      />
      <path
        d={bodyPath}
        fill="url(#parentPortalFrameShine)"
        opacity={hovered && !disabled ? 0.95 : 0.76}
        pointerEvents="none"
      />
      <path
        d={arrowPath}
        fill="#001522"
        opacity={0.8}
        transform={`translate(${side === 'left' ? 1 : -1} 0)`}
        pointerEvents="none"
      />
      <path d={arrowPath} fill={hovered && !disabled ? '#c9ffd8' : '#ecfbff'} pointerEvents="none" />
      <rect x={x - 6} y={y - 6} width={handleW + 12} height={handleH + 12} fill="transparent" />
    </g>
  );
}

function ParentPortalFrameDots({
  x,
  y,
  page,
  pageCount,
  onPageChange,
  cfg,
}: {
  x: number;
  y: number;
  page: number;
  pageCount: number;
  onPageChange: (page: number) => void;
  cfg: ParentPortalSvgControls;
}) {
  const visibleCount = Math.min(pageCount, 7);
  const start =
    pageCount <= visibleCount ? 0 : clampValue(page - Math.floor(visibleCount / 2), 0, pageCount - visibleCount);
  const slots = Array.from({ length: visibleCount }, (_, index) => start + index);
  const inactiveW = 15;
  const activeW = 34;
  const gap = 8;
  const totalW =
    slots.reduce((sum, slot) => sum + (slot === page ? activeW : inactiveW), 0) + Math.max(0, slots.length - 1) * gap;
  let cursor = x - totalW / 2;
  return (
    <g>
      {slots.map((slot) => {
        const active = slot === page;
        const dotW = active ? activeW : inactiveW;
        const dotX = cursor;
        cursor += dotW + gap;
        return (
          <g
            key={slot}
            className="parent-portal-svg-clickable"
            role="button"
            tabIndex={0}
            aria-label={`Open carousel page ${slot + 1}`}
            onClick={(event) => {
              event.stopPropagation();
              onPageChange(slot);
            }}
            onKeyDown={(event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              event.stopPropagation();
              onPageChange(slot);
            }}
          >
            <rect x={dotX - 5} y={y - 10} width={dotW + 10} height={22} fill="transparent" />
            <rect
              x={dotX}
              y={y - 4}
              width={dotW}
              height={8}
              rx={4}
              fill={active ? 'url(#parentPortalFooterActivePill)' : 'rgba(100, 216, 255, 0.08)'}
              stroke={active ? '#ffe187' : cfg.colors.cyan}
              strokeWidth={active ? 1.5 : 1.1}
              strokeOpacity={active ? 0.95 : 0.58}
              filter={active ? 'url(#parentPortalGlow)' : undefined}
            />
          </g>
        );
      })}
    </g>
  );
}

function ParentPortalHeaderAction({
  x,
  y,
  w,
  h,
  label,
  tone = 'cyan',
  active = false,
  onClick,
  ariaLabel,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  tone?: Tone;
  active?: boolean;
  onClick: () => void;
  ariaLabel?: string;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const lit = active || hovered;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={ariaLabel ?? label}
      onClick={(event) => {
        event.stopPropagation();
        onClick();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onClick();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {lit ? (
        <rect
          x={x - 3}
          y={y - 3}
          width={w + 6}
          height={h + 6}
          fill="none"
          stroke={color}
          strokeWidth={1.4}
          opacity={0.24}
          filter="url(#parentPortalGlow)"
        />
      ) : null}
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={3}
        fill={lit ? `${color}24` : 'rgba(4, 18, 31, 0.88)'}
        stroke={color}
        strokeWidth={lit ? 1.35 : 0.9}
        strokeOpacity={lit ? 0.95 : 0.64}
      />
      <rect
        x={x + 2}
        y={y + 2}
        width={w - 4}
        height={Math.max(1, h * 0.36)}
        rx={2}
        fill="#ffffff"
        opacity={lit ? 0.12 : 0.06}
      />
      <text
        x={x + w / 2}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={fitSingleLineTextSize(label, w - 16, 8.5, 11.5, 0.58)}
        fontWeight={900}
        fill={cfg.colors.bodyText}
      >
        {label}
      </text>
    </g>
  );
}

function ParentPortalSectionFrame({
  x,
  y,
  w,
  h,
  title,
  subtitle,
  count,
  tone = 'cyan',
  headerSlot,
  headerRight,
  footer,
  footerH,
  headerH,
  innerStrokeOpacity = 0.6,
  bodyStrokeOpacity = 0.72,
  bodyFill = 'rgba(7, 30, 48, 0.38)',
  footerLineOpacity = 0.42,
  showSideHandles = false,
  sideDisabled = false,
  onPrevious,
  onNext,
  onWheel,
  selected = false,
  onSelect,
  ariaLabel,
  cfg,
  children,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  title: string;
  subtitle?: string;
  count?: string;
  tone?: Tone;
  headerSlot?: ReactNode;
  headerRight?: ReactNode;
  footer?: (rect: ParentPortalRect) => ReactNode;
  footerH?: number;
  headerH?: number;
  innerStrokeOpacity?: number;
  bodyStrokeOpacity?: number;
  bodyFill?: string;
  footerLineOpacity?: number;
  showSideHandles?: boolean;
  sideDisabled?: boolean;
  onPrevious?: () => void;
  onNext?: () => void;
  onWheel?: (event: WheelEvent<SVGGElement>) => void;
  selected?: boolean;
  onSelect?: () => void;
  ariaLabel?: string;
  cfg: ParentPortalSvgControls;
  children: (rect: ParentPortalRect) => ReactNode;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const { body, footer: footerRect, headerH: resolvedHeaderH } = parentPortalFrameRects(x, y, w, h, footerH, headerH);
  const cut = Math.max(6, Math.min(cfg.chrome.panelCut, 12));
  const interactive = Boolean(onSelect);
  const active = selected || hovered;
  const countW = count ? 58 : 0;
  const titleX = x + 22 + countW;
  const titleW = Math.max(190, Math.min(w * 0.42, title.length * 11 + 68));
  const titleY = y + 11;
  const titleH = 31;
  const countPath = count ? cutRectPath(x + 22, titleY - 3, countW, titleH + 3, 5) : '';
  const titlePath = cutRectPath(titleX, titleY, titleW, titleH, 5);
  const sideHandleW = PARENT_PORTAL_SIDE_HANDLE_W;
  const sideHandleH = Math.max(72, Math.min(128, body.h - 28));
  const sideHandleY = body.y + Math.max(12, (body.h - sideHandleH) / 2);
  const leftHandleX = x - sideHandleW + PARENT_PORTAL_SIDE_HANDLE_OVERLAP;
  const rightHandleX = x + w - PARENT_PORTAL_SIDE_HANDLE_OVERLAP;
  const handleSelect = (event: MouseEvent<SVGGElement>) => {
    if (!interactive) return;
    event.stopPropagation();
    onSelect?.();
  };
  const handleKeyDown = (event: KeyboardEvent<SVGGElement>) => {
    if (!interactive || (event.key !== 'Enter' && event.key !== ' ')) return;
    event.preventDefault();
    event.stopPropagation();
    onSelect?.();
  };
  return (
    <g
      className={interactive ? 'parent-portal-svg-clickable' : undefined}
      role={interactive ? 'button' : undefined}
      tabIndex={interactive ? 0 : undefined}
      aria-label={ariaLabel}
      aria-pressed={interactive ? selected : undefined}
      onClick={handleSelect}
      onKeyDown={handleKeyDown}
      onWheel={onWheel}
      onMouseEnter={() => interactive && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {interactive ? (
        <rect x={x - 8} y={y - 8} width={w + 16} height={h + 16} fill="transparent" pointerEvents="all" />
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, cut)}
        fill="none"
        stroke={color}
        strokeWidth={active ? 5.2 : 3.1}
        opacity={active ? 0.35 : 0.24}
        filter="url(#parentPortalGlow)"
        pointerEvents="none"
      />
      {active ? (
        <path
          d={cutRectPath(x - 4, y - 4, w + 8, h + 8, cut + 2)}
          fill="none"
          stroke={color}
          strokeWidth={2}
          opacity={selected ? 0.42 : 0.26}
          filter="url(#parentPortalGlow)"
          pointerEvents="none"
        />
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, cut)}
        fill="url(#parentPortalFrameFill)"
        stroke={color}
        strokeWidth={active ? 2 : 1.5}
      />
      <path
        d={cutRectPath(x + 6, y + 6, w - 12, h - 12, Math.max(4, cut - 4))}
        fill="url(#parentPortalFrameGlass)"
        stroke={cfg.colors.cyan}
        strokeWidth={active ? 1.2 : 0.85}
        strokeOpacity={selected ? Math.max(innerStrokeOpacity, 0.46) : innerStrokeOpacity}
      />
      <path
        d={cutRectPath(x + 8, y + 8, w - 16, Math.min(62, resolvedHeaderH + 10), Math.max(4, cut - 5))}
        fill="url(#parentPortalFrameShine)"
        opacity={active ? 0.56 : 0.42}
        pointerEvents="none"
      />
      {headerSlot ? (
        headerSlot
      ) : (
        <>
          {count ? (
            <>
              <path
                d={countPath}
                fill={colorAlpha(color, 'd8')}
                stroke="#8df5ff"
                strokeWidth={1.15}
                filter="url(#parentPortalGlow)"
              />
              <text
                x={x + 22 + countW / 2}
                y={titleY + 20}
                textAnchor="middle"
                fontSize={15}
                fontWeight={950}
                fill="#02121a"
                stroke="#d9fbff"
                strokeWidth={0.6}
                paintOrder="stroke fill"
              >
                {count}
              </text>
            </>
          ) : null}
          <path
            d={titlePath}
            fill={tone === 'purple' ? 'rgba(50, 27, 92, 0.78)' : 'rgba(8, 43, 62, 0.74)'}
            stroke={color}
            strokeWidth={1.1}
          />
          <path
            d={`M ${titleX + 12} ${titleY + 6} H ${titleX + titleW - 12}`}
            stroke="#ffffff"
            strokeWidth={1.1}
            opacity={0.13}
          />
          <text x={titleX + 18} y={titleY + 21} fontSize={13.5} fontWeight={950} fill={cfg.colors.bodyText}>
            {title}
          </text>
          <line
            x1={titleX + titleW + 12}
            y1={y + resolvedHeaderH - 8}
            x2={x + w - 20}
            y2={y + resolvedHeaderH - 8}
            stroke={cfg.colors.cyan}
            strokeWidth={1.1}
            opacity={0.38}
          />
          {subtitle && !count ? (
            <text x={x + 24} y={y + resolvedHeaderH - 12} fontSize={10.2} fontWeight={820} fill={cfg.colors.mutedText}>
              {subtitle}
            </text>
          ) : null}
          {headerRight}
        </>
      )}
      <path
        d={cutRectPath(body.x, body.y, body.w, body.h, 7)}
        fill={bodyFill}
        stroke={color}
        strokeWidth={0.9}
        strokeOpacity={bodyStrokeOpacity}
      />
      {children(body)}
      {footerLineOpacity > 0 ? (
        <line
          x1={footerRect.x + 12}
          y1={footerRect.y + 3}
          x2={footerRect.x + footerRect.w - 12}
          y2={footerRect.y + 3}
          stroke={cfg.colors.cyan}
          strokeWidth={1.1}
          opacity={footerLineOpacity}
        />
      ) : null}
      {footer?.(footerRect)}
      {showSideHandles && onPrevious ? (
        <ParentPortalFrameSideHandle
          x={leftHandleX}
          y={sideHandleY}
          side="left"
          height={sideHandleH}
          width={sideHandleW}
          disabled={sideDisabled}
          onClick={onPrevious}
          cfg={cfg}
        />
      ) : null}
      {showSideHandles && onNext ? (
        <ParentPortalFrameSideHandle
          x={rightHandleX}
          y={sideHandleY}
          side="right"
          height={sideHandleH}
          width={sideHandleW}
          disabled={sideDisabled}
          onClick={onNext}
          cfg={cfg}
        />
      ) : null}
    </g>
  );
}

function ArtworkSlot({
  x,
  y,
  w,
  h,
  label,
  imageUrl = null,
  tone = 'cyan',
  compact = false,
  shape = 'rect',
  imageFit = 'meet',
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  imageUrl?: string | null;
  tone?: Tone;
  compact?: boolean;
  shape?: 'rect' | 'hex' | 'circle';
  imageFit?: 'meet' | 'slice';
  cfg: ParentPortalSvgControls;
}) {
  const rawId = useId();
  const color = toneColor(tone, cfg);
  const cx = x + w / 2;
  const cy = y + h / 2;
  const inset = Math.min(8, w * 0.18, h * 0.18);
  const radius = Math.min(w, h) / 2 - 1;
  const clipId = `parent-portal-art-${rawId.replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const primaryText = compact ? 'MISS' : 'MISSING';
  const primaryFontSize = compact ? Math.min(7, Math.max(4.5, w * 0.24)) : Math.min(11, Math.max(7.5, w * 0.09));
  const secondaryFontSize = Math.min(8.5, Math.max(6.5, w * 0.06));
  const fill = imageUrl ? 'rgba(3, 13, 24, 0.82)' : 'rgba(48, 12, 23, 0.72)';
  const renderShape = (shapeFill: string, shapeStroke: string, dash = '') => {
    if (shape === 'circle') {
      return (
        <circle
          cx={cx}
          cy={cy}
          r={radius}
          fill={shapeFill}
          stroke={shapeStroke}
          strokeWidth={1.1}
          strokeDasharray={dash}
          opacity={0.95}
        />
      );
    }
    if (shape === 'hex') {
      return (
        <path
          d={hexPath(cx, cy, radius)}
          fill={shapeFill}
          stroke={shapeStroke}
          strokeWidth={1.1}
          strokeDasharray={dash}
          opacity={0.95}
        />
      );
    }
    return (
      <path
        d={cutRectPath(x, y, w, h, 4)}
        fill={shapeFill}
        stroke={shapeStroke}
        strokeWidth={1.1}
        strokeDasharray={dash}
        opacity={0.95}
      />
    );
  };
  return (
    <g pointerEvents="none">
      {renderShape(fill, color, imageUrl ? '' : '4 3')}
      {imageUrl ? (
        <>
          <clipPath id={clipId}>
            {shape === 'circle' ? (
              <circle cx={cx} cy={cy} r={radius - 1} />
            ) : shape === 'hex' ? (
              <path d={hexPath(cx, cy, radius - 1)} />
            ) : (
              <path d={cutRectPath(x + 1, y + 1, w - 2, h - 2, 4)} />
            )}
          </clipPath>
          <image
            href={imageUrl}
            x={x + 2}
            y={y + 2}
            width={w - 4}
            height={h - 4}
            preserveAspectRatio={`xMidYMid ${imageFit}`}
            clipPath={`url(#${clipId})`}
            opacity={0.98}
          />
          {renderShape('none', color)}
        </>
      ) : (
        <>
          <title>{`Missing ${label} image`}</title>
          <line
            x1={x + inset}
            y1={y + inset}
            x2={x + w - inset}
            y2={y + h - inset}
            stroke={color}
            strokeOpacity={0.55}
            strokeWidth={0.8}
          />
          <line
            x1={x + w - inset}
            y1={y + inset}
            x2={x + inset}
            y2={y + h - inset}
            stroke={color}
            strokeOpacity={0.55}
            strokeWidth={0.8}
          />
          <text
            x={cx}
            y={compact ? cy + primaryFontSize * 0.35 : cy - 1}
            textAnchor="middle"
            fontSize={primaryFontSize}
            fontWeight={950}
            fill={color}
          >
            {primaryText}
          </text>
          {compact ? null : (
            <text
              x={cx}
              y={cy + 13}
              textAnchor="middle"
              fontSize={secondaryFontSize}
              fontWeight={900}
              fill={cfg.colors.mutedText}
            >
              {label}
            </text>
          )}
        </>
      )}
    </g>
  );
}

function NavRow({
  item,
  active,
  x,
  w,
  y,
  rowH,
  iconSize,
  nested = false,
  branchColor,
  onSelect,
  cfg,
}: {
  item: NavItem;
  active: boolean;
  x: number;
  w: number;
  y: number;
  rowH: number;
  iconSize: number;
  nested?: boolean;
  branchColor?: string;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = active ? cfg.colors.bodyText : '#d8eaff';
  const rowX = x + 14;
  const rowW = w - 28;
  const lit = active || hovered;
  const branchAccent = branchColor ?? cfg.colors.cyan;
  const accent = active ? cfg.colors.cyan : branchAccent;
  const slotX = x + 22;
  const slotY = y + (rowH - iconSize) / 2;
  const slotW = Math.max(32, iconSize - 2);
  const textX = slotX + slotW + 8;
  const labelW = Math.max(48, x + w - 28 - textX);
  const labelSize = 10.9;
  const arrowTop = y + 7;
  const arrowBottom = y + rowH - 7;
  const arrowMid = y + rowH / 2;
  return (
    <g
      className="parent-portal-svg-clickable"
      onClick={(event) => {
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      role="button"
      tabIndex={0}
      aria-label={`Open ${item.label}`}
    >
      <rect x={rowX - 6} y={y - 4} width={rowW + 28} height={rowH + 8} fill="transparent" pointerEvents="all" />
      {nested ? (
        <>
          <path
            d={`M ${rowX - 8} ${y + 7} V ${y + rowH - 7}`}
            stroke={branchAccent}
            strokeWidth={active ? 2.2 : 1.4}
            strokeLinecap="round"
            opacity={active ? 0.72 : 0.5}
            filter={active ? 'url(#parentPortalGlow)' : undefined}
            pointerEvents="none"
          />
          <path
            d={`M ${rowX - 8} ${y + rowH / 2} H ${rowX - 1}`}
            stroke={branchAccent}
            strokeWidth={active ? 2 : 1.2}
            strokeLinecap="round"
            opacity={active ? 0.66 : 0.48}
            pointerEvents="none"
          />
        </>
      ) : null}
      {lit ? (
        <>
          <path
            d={cutRectPath(rowX - 3, y - 3, rowW + 6, rowH + 6, 11)}
            fill="none"
            stroke={accent}
            strokeWidth={active ? 2.2 : 2}
            opacity={active ? 0.42 : 0.34}
            filter="url(#parentPortalGlow)"
            pointerEvents="none"
          />
          {active ? (
            <path
              d={`M ${rowX + rowW - 8} ${arrowTop} L ${rowX + rowW + 18} ${arrowMid} L ${rowX + rowW - 8} ${arrowBottom} Z`}
              fill={accent}
              opacity={0.82}
              filter="url(#parentPortalGlow)"
              pointerEvents="none"
            />
          ) : null}
        </>
      ) : null}
      <path
        d={cutRectPath(rowX, y, rowW, rowH, 8)}
        fill={
          active
            ? 'url(#parentPortalActiveBlue)'
            : hovered
              ? colorAlpha(branchAccent, '18')
              : colorAlpha(branchAccent, '08')
        }
        fillOpacity={1}
        stroke={lit ? accent : 'transparent'}
        strokeWidth={lit ? 1.6 : 0}
        pointerEvents="none"
      />
      {lit ? (
        <path
          d={cutRectPath(rowX + 3, y + 3, rowW - 6, rowH - 6, 6)}
          fill="none"
          stroke={accent}
          strokeWidth={1}
          opacity={active ? 0.68 : 0.42}
          pointerEvents="none"
        />
      ) : null}
      <NavIconSlot
        Icon={item.icon}
        x={slotX}
        y={slotY + 3}
        size={slotW}
        color={accent}
        lit={lit}
        selected={active}
        cfg={cfg}
      />
      <text
        x={textX}
        y={y + rowH * 0.64}
        fontSize={labelSize}
        fontWeight={940}
        fill={color}
        stroke="#03121f"
        strokeWidth={0.7}
        strokeOpacity={0.78}
        paintOrder="stroke"
        pointerEvents="none"
      >
        {truncateTextForWidth(item.label, labelW, labelSize, 0.54)}
      </text>
    </g>
  );
}

function NavIconSlot({
  Icon,
  x,
  y,
  size,
  color,
  lit,
  selected = false,
  cfg,
}: {
  Icon: IconComponent;
  x: number;
  y: number;
  size: number;
  color: string;
  lit: boolean;
  selected?: boolean;
  cfg: ParentPortalSvgControls;
}) {
  const iconSize = Math.max(24, size - 2);
  const iconX = x + (size - iconSize) / 2;
  const iconY = y + (size - iconSize) / 2;
  return (
    <g pointerEvents="none">
      <Icon
        x={iconX}
        y={iconY}
        width={iconSize}
        height={iconSize}
        color={selected || lit ? cfg.colors.bodyText : color}
        strokeWidth={selected ? 2.35 : 2.05}
      />
    </g>
  );
}

function FoldoutTriangleIndicator({
  x,
  y,
  size,
  open,
  hovered,
  accent,
  glowFilter,
  cfg,
}: {
  x: number;
  y: number;
  size: number;
  open: boolean;
  hovered: boolean;
  accent: string;
  glowFilter: string;
  cfg: ParentPortalSvgControls;
}) {
  const lit = open || hovered;
  const indicatorColor = accent;
  const cx = x + size / 2;
  const cy = y + size / 2;
  const triangleInset = Math.max(5, size * 0.28);
  const trianglePath = open
    ? `M ${cx - triangleInset} ${cy - triangleInset * 0.4} L ${cx + triangleInset} ${cy - triangleInset * 0.4} L ${cx} ${cy + triangleInset * 0.78} Z`
    : `M ${cx - triangleInset * 0.38} ${cy - triangleInset} L ${cx - triangleInset * 0.38} ${cy + triangleInset} L ${cx + triangleInset * 0.78} ${cy} Z`;
  return (
    <g pointerEvents="none">
      {lit ? (
        <path
          d={cutRectPath(x - 2, y - 2, size + 4, size + 4, 5)}
          fill="none"
          stroke={indicatorColor}
          strokeWidth={open ? 1.45 : 1.15}
          opacity={open ? 0.44 : 0.3}
          filter={glowFilter}
        />
      ) : null}
      <path
        d={cutRectPath(x, y, size, size, 4)}
        fill={colorAlpha(indicatorColor, open ? '26' : hovered ? '1e' : '10')}
        stroke={indicatorColor}
        strokeWidth={lit ? 1.15 : 0.85}
        strokeOpacity={lit ? 0.9 : 0.58}
      />
      <path
        d={trianglePath}
        fill={indicatorColor}
        stroke={cfg.colors.bodyText}
        strokeWidth={open ? 0.9 : 0.7}
        strokeOpacity={open ? 0.55 : 0.32}
        strokeLinejoin="round"
        filter={lit ? glowFilter : undefined}
      />
    </g>
  );
}

function NavSectionHeader({
  label,
  icon: Icon,
  open,
  x,
  w,
  y,
  h,
  accentColor,
  glowFilter,
  onToggle,
  cfg,
}: {
  label: string;
  icon: IconComponent;
  open: boolean;
  x: number;
  w: number;
  y: number;
  h: number;
  accentColor?: string;
  glowFilter?: string;
  onToggle: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const lit = open || hovered;
  const baseAccent = accentColor ?? cfg.colors.cyan;
  const activeGlowFilter = glowFilter ?? 'url(#parentPortalGlow)';
  const accent = baseAccent;
  const rowX = x + 24;
  const rowW = w - 40;
  const panelY = y;
  const panelH = h;
  const slotSize = Math.max(28, Math.min(32, h - 6));
  const slotX = rowX + 8;
  const slotY = panelY + (panelH - slotSize) / 2;
  const indicatorSize = 20;
  const indicatorX = rowX + rowW - indicatorSize - 10;
  const indicatorY = panelY + (panelH - indicatorSize) / 2;
  const textX = slotX + slotSize + 10;
  const labelW = Math.max(40, indicatorX - textX - 10);
  const labelSize = 11.8;
  return (
    <g
      className="parent-portal-svg-clickable"
      onClick={(event) => {
        event.stopPropagation();
        onToggle();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onToggle();
      }}
      role="button"
      tabIndex={0}
      aria-label={`${open ? 'Collapse' : 'Expand'} ${label}`}
      aria-expanded={open}
    >
      <rect x={rowX - 7} y={panelY - 4} width={rowW + 14} height={panelH + 8} fill="transparent" pointerEvents="all" />
      <path
        d={`M ${x + 18} ${panelY + panelH / 2} H ${rowX - 8}`}
        stroke={accent}
        strokeWidth={1}
        opacity={lit ? 0.56 : 0.38}
        pointerEvents="none"
      />
      {lit ? (
        <path
          d={cutRectPath(rowX - 3, panelY - 3, rowW + 6, panelH + 6, 10)}
          fill="none"
          stroke={accent}
          strokeWidth={open ? 1.9 : 1.5}
          opacity={open ? 0.34 : 0.25}
          filter={activeGlowFilter}
          pointerEvents="none"
        />
      ) : null}
      {open ? (
        <path
          d={cutRectPath(rowX - 1, panelY + 6, 5, panelH - 12, 2)}
          fill={accent}
          opacity={0.84}
          filter={activeGlowFilter}
          pointerEvents="none"
        />
      ) : null}
      <path
        d={cutRectPath(rowX, panelY, rowW, panelH, 6)}
        fill={open ? colorAlpha(baseAccent, '19') : lit ? colorAlpha(baseAccent, '14') : colorAlpha(baseAccent, '08')}
        stroke={accent}
        strokeWidth={lit ? 1.25 : 0.85}
        strokeOpacity={lit ? 0.84 : 0.46}
        pointerEvents="none"
      />
      {lit ? (
        <path
          d={cutRectPath(rowX + 3, panelY + 3, rowW - 6, panelH - 6, 5)}
          fill="none"
          stroke={accent}
          strokeWidth={0.9}
          opacity={open ? 0.52 : 0.38}
          pointerEvents="none"
        />
      ) : null}
      <NavIconSlot Icon={Icon} x={slotX} y={slotY} size={slotSize} color={accent} lit={lit} selected={open} cfg={cfg} />
      <text
        x={textX}
        y={panelY + panelH * 0.61}
        fontSize={labelSize}
        fontWeight={940}
        fill={cfg.colors.bodyText}
        stroke="#03121f"
        strokeWidth={0.8}
        strokeOpacity={0.78}
        paintOrder="stroke"
        pointerEvents="none"
      >
        {truncateTextForWidth(label, labelW, labelSize, 0.54)}
      </text>
      <FoldoutTriangleIndicator
        x={indicatorX}
        y={indicatorY}
        size={indicatorSize}
        open={open}
        hovered={hovered}
        accent={baseAccent}
        glowFilter={activeGlowFilter}
        cfg={cfg}
      />
    </g>
  );
}

function navGroupThemeColor(groupId: string, cfg: ParentPortalSvgControls): string {
  if (groupId === 'guide') return '#5ecfff';
  if (groupId === 'manage') return '#4ff2d2';
  return cfg.colors.cyan;
}

function NavGroupHeader({
  group,
  open,
  x,
  w,
  y,
  h,
  onToggle,
  cfg,
}: {
  group: NavGroup;
  open: boolean;
  x: number;
  w: number;
  y: number;
  h: number;
  onToggle: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const rowX = x + 14;
  const rowW = w - 28;
  const lit = open || hovered;
  const accent = navGroupThemeColor(group.id, cfg);
  const glowFilter = 'url(#parentPortalGlow)';
  const Icon =
    group.id === 'quickGlance'
      ? QuickGlanceGlasses
      : group.id === 'guide'
        ? GuideBookIcon
        : group.id === 'manage'
          ? ManageFileSettingsIcon
          : (group.items[0]?.icon ?? OverviewListIcon);
  const slotSize = Math.max(32, Math.min(38, h - 6));
  const slotX = rowX + 8;
  const slotY = y + (h - slotSize) / 2;
  const indicatorSize = 26;
  const indicatorX = rowX + rowW - indicatorSize - 10;
  const indicatorY = y + (h - indicatorSize) / 2;
  const textX = slotX + slotSize + 10;
  const labelW = Math.max(46, indicatorX - textX - 10);
  const labelSize = 13.8;
  return (
    <g
      className="parent-portal-svg-clickable"
      onClick={(event) => {
        event.stopPropagation();
        onToggle();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onToggle();
      }}
      role="button"
      tabIndex={0}
      aria-label={`${open ? 'Collapse' : 'Expand'} ${group.label}`}
      aria-expanded={open}
    >
      <rect x={rowX - 6} y={y - 4} width={rowW + 12} height={h + 8} fill="transparent" pointerEvents="all" />
      {lit ? (
        <path
          d={cutRectPath(rowX - 3, y - 3, rowW + 6, h + 6, 11)}
          fill="none"
          stroke={accent}
          strokeWidth={open ? 2 : 1.6}
          opacity={open ? 0.35 : 0.23}
          filter={glowFilter}
          pointerEvents="none"
        />
      ) : null}
      {open ? (
        <path
          d={cutRectPath(rowX - 1, y + 6, 5, h - 12, 2)}
          fill={accent}
          opacity={0.78}
          filter={glowFilter}
          pointerEvents="none"
        />
      ) : null}
      <path
        d={cutRectPath(rowX, y, rowW, h, 8)}
        fill={lit ? colorAlpha(accent, open ? '22' : '16') : colorAlpha(accent, '0a')}
        fillOpacity={1}
        stroke={accent}
        strokeWidth={lit ? 1.45 : 0.8}
        strokeOpacity={lit ? 0.86 : 0.48}
        pointerEvents="none"
      />
      {lit ? (
        <path
          d={cutRectPath(rowX + 3, y + 3, rowW - 6, h - 6, 6)}
          fill="none"
          stroke={accent}
          strokeWidth={0.95}
          opacity={open ? 0.58 : 0.38}
          pointerEvents="none"
        />
      ) : null}
      <NavIconSlot Icon={Icon} x={slotX} y={slotY} size={slotSize} color={accent} lit={lit} selected={open} cfg={cfg} />
      <text
        x={textX}
        y={y + h * 0.62}
        fontSize={labelSize}
        fontWeight={940}
        fill={cfg.colors.bodyText}
        stroke="#03121f"
        strokeWidth={0.8}
        strokeOpacity={0.78}
        paintOrder="stroke"
        pointerEvents="none"
      >
        {truncateTextForWidth(group.label, labelW, labelSize, 0.54)}
      </text>
      <FoldoutTriangleIndicator
        x={indicatorX}
        y={indicatorY}
        size={indicatorSize}
        open={open}
        hovered={hovered}
        accent={accent}
        glowFilter={glowFilter}
        cfg={cfg}
      />
    </g>
  );
}

function NavPanel({
  activeNavLabel,
  navGroups,
  openGroupIds,
  onNavGroupToggle,
  onNavItemSelect,
  cfg,
}: {
  activeNavLabel: string;
  navGroups: NavGroup[];
  openGroupIds: Record<string, boolean>;
  onNavGroupToggle: (groupId: string) => void;
  onNavItemSelect: (item: NavItem) => void;
  cfg: ParentPortalSvgControls;
}) {
  const { outerPad, leftW, topY } = cfg.layout;
  const navItems = navGroups.flatMap((group) => group.items);
  const rawNavClipId = useId();
  const navClipId = `parent-portal-nav-clip-${rawNavClipId.replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const [navScroll, setNavScroll] = useState(0);
  const [openSectionIds, setOpenSectionIds] = useState(() => initialOpenNavSectionIds(navGroups, activeNavLabel));
  const groupH = 46;
  const sectionH = 42;
  const rowH = 38;
  const rowStep = navItems.length > 8 ? 40 : 42;
  const groupGap = 6;
  const sectionGap = 5;
  const iconSize = 38;
  const sideTopY = Math.max(0, topY - 14);
  const rowTop = sideTopY + 28;
  const navH = Math.max(300, cfg.canvas.height - sideTopY - 2);
  const navViewportY = rowTop;
  const navViewportH = Math.max(72, sideTopY + navH - rowTop - 22);
  const navContentH = navGroups.reduce((height, group) => {
    if (!openGroupIds[group.id]) return height + groupH + groupGap;
    const itemHeight = navSectionsForGroup(group).reduce((nextHeight, section) => {
      const sectionDelta = section.label ? sectionH : 0;
      const sectionOpen = !section.label || Boolean(openSectionIds[section.id]);
      return (
        nextHeight +
        sectionDelta +
        (sectionOpen ? section.items.length * rowStep : 0) +
        (section.label ? sectionGap : 0)
      );
    }, 0);
    return height + groupH + itemHeight + groupGap;
  }, 0);
  let activeNavRowY: number | null = null;
  let scanY = rowTop;
  for (const group of navGroups) {
    scanY += groupH;
    if (openGroupIds[group.id]) {
      for (const section of navSectionsForGroup(group)) {
        if (section.label) {
          scanY += sectionH;
        }
        if (section.label && !openSectionIds[section.id]) continue;
        for (const item of section.items) {
          if (item.label === activeNavLabel) {
            activeNavRowY = scanY;
          }
          scanY += rowStep;
        }
        if (section.label) scanY += sectionGap;
      }
    }
    scanY += groupGap;
  }
  const maxNavScroll = Math.max(0, navContentH - navViewportH);
  const safeNavScroll = clampValue(navScroll, 0, maxNavScroll);
  useEffect(() => {
    if (safeNavScroll !== navScroll) setNavScroll(safeNavScroll);
  }, [navScroll, safeNavScroll]);
  useEffect(() => {
    setOpenSectionIds((current) => ensureOpenNavSectionIds(current, navGroups, activeNavLabel));
  }, [activeNavLabel, navGroups]);
  useEffect(() => {
    if (activeNavRowY === null || maxNavScroll <= 0) return;
    const topPad = 6;
    const visibleTop = navViewportY + topPad;
    const visibleBottom = navViewportY + navViewportH - topPad;
    setNavScroll((value) => {
      const activeTop = activeNavRowY - value;
      const activeBottom = activeTop + rowH;
      if (activeTop < visibleTop) {
        return clampValue(activeNavRowY - visibleTop, 0, maxNavScroll);
      }
      if (activeBottom > visibleBottom) {
        return clampValue(activeNavRowY + rowH - visibleBottom, 0, maxNavScroll);
      }
      return value;
    });
  }, [activeNavLabel, activeNavRowY, maxNavScroll, navViewportH, navViewportY, rowH]);
  const handleNavWheel = (event: WheelEvent<SVGGElement>) => {
    if (maxNavScroll <= 0) return;
    event.stopPropagation();
    event.preventDefault();
    setNavScroll((value) => clampValue(value + event.deltaY * 0.72, 0, maxNavScroll));
  };
  const thumbH = maxNavScroll > 0 ? clampValue((navViewportH / navContentH) * navViewportH, 46, navViewportH) : 0;
  const thumbY =
    maxNavScroll > 0
      ? navViewportY + (safeNavScroll / maxNavScroll) * Math.max(0, navViewportH - thumbH)
      : navViewportY;
  const toggleNavSection = (sectionId: string) => {
    setOpenSectionIds((current) => toggleOpenNavSectionId(current, navGroups, sectionId));
  };
  let cursorY = rowTop;
  return (
    <g>
      <SurfacePanel x={outerPad} y={sideTopY} w={leftW} h={navH} tone="cyan" frame="deckSide" cfg={cfg}>
        <defs>
          <clipPath id={navClipId}>
            <rect x={outerPad - 8} y={navViewportY - 2} width={leftW + 46} height={navViewportH + 4} />
          </clipPath>
        </defs>
        <g onWheel={handleNavWheel}>
          <rect
            x={outerPad + 12}
            y={navViewportY - 2}
            width={leftW - 24}
            height={navViewportH + 4}
            fill="transparent"
            pointerEvents={maxNavScroll > 0 ? 'all' : 'none'}
          />
          <g clipPath={`url(#${navClipId})`}>
            <g transform={`translate(0 ${-safeNavScroll})`}>
              {navGroups.map((group) => {
                const groupY = cursorY;
                cursorY += groupH;
                const childStartY = cursorY;
                const open = Boolean(openGroupIds[group.id]);
                const groupAccent = navGroupThemeColor(group.id, cfg);
                const groupGlowFilter = 'url(#parentPortalGlow)';
                const rows = open
                  ? navSectionsForGroup(group).flatMap((section) => {
                      const sectionRows: ReactNode[] = [];
                      const sectionOpen = !section.label || Boolean(openSectionIds[section.id]);
                      if (section.label) {
                        const sectionY = cursorY;
                        cursorY += sectionH;
                        sectionRows.push(
                          <NavSectionHeader
                            key={section.id}
                            label={section.label}
                            icon={navSectionIcon(section)}
                            open={sectionOpen}
                            x={outerPad + 8}
                            w={leftW - 8}
                            y={sectionY}
                            h={sectionH}
                            accentColor={groupAccent}
                            glowFilter={groupGlowFilter}
                            onToggle={() => toggleNavSection(section.id)}
                            cfg={cfg}
                          />
                        );
                      }
                      if (!sectionOpen) return sectionRows;
                      for (const item of section.items) {
                        const itemY = cursorY;
                        cursorY += rowStep;
                        const nested = Boolean(section.label);
                        const rowInset = nested ? 30 : 8;
                        sectionRows.push(
                          <NavRow
                            key={item.label}
                            item={item}
                            active={item.label === activeNavLabel}
                            x={outerPad + rowInset}
                            w={leftW - rowInset}
                            y={itemY}
                            rowH={rowH}
                            iconSize={iconSize}
                            nested={nested}
                            branchColor={groupAccent}
                            onSelect={() => onNavItemSelect(item)}
                            cfg={cfg}
                          />
                        );
                      }
                      if (section.label) cursorY += sectionGap;
                      return sectionRows;
                    })
                  : null;
                const childEndY = cursorY;
                const childRailH = Math.max(0, childEndY - childStartY - 2);
                cursorY += groupGap;
                return (
                  <g key={group.id}>
                    {open && childRailH > 0 ? (
                      <>
                        <path
                          d={cutRectPath(outerPad + 22, childStartY + 3, leftW - 35, childRailH, 9)}
                          fill="rgba(2, 12, 20, 0.34)"
                          stroke={groupAccent}
                          strokeWidth={0.85}
                          strokeOpacity={0.38}
                          pointerEvents="none"
                        />
                        <path
                          d={`M ${outerPad + 28} ${childStartY + 9} V ${childStartY + childRailH - 6}`}
                          stroke={groupAccent}
                          strokeWidth={1.45}
                          strokeLinecap="round"
                          opacity={0.58}
                          pointerEvents="none"
                        />
                      </>
                    ) : null}
                    <NavGroupHeader
                      group={group}
                      open={open}
                      x={outerPad}
                      w={leftW}
                      y={groupY}
                      h={groupH}
                      onToggle={() => onNavGroupToggle(group.id)}
                      cfg={cfg}
                    />
                    {rows}
                  </g>
                );
              })}
            </g>
          </g>
        </g>
        {maxNavScroll > 0 ? (
          <g pointerEvents="none">
            <path
              d={`M ${outerPad + leftW - 10} ${navViewportY + 8} V ${navViewportY + navViewportH - 8}`}
              stroke={cfg.colors.cyan}
              strokeWidth={1.4}
              strokeLinecap="round"
              opacity={0.28}
            />
            <path
              d={`M ${outerPad + leftW - 10} ${thumbY} V ${thumbY + thumbH}`}
              stroke={cfg.colors.cyan}
              strokeWidth={3.4}
              strokeLinecap="round"
              opacity={0.82}
              filter="url(#parentPortalGlow)"
            />
          </g>
        ) : null}
      </SurfacePanel>
    </g>
  );
}

type ManageControlOption = {
  readonly label: string;
  readonly detail: string;
  readonly enabled: boolean;
  readonly tone: Tone;
};

type ManageControlAction = {
  readonly label: string;
  readonly detail: string;
  readonly tone: Tone;
};

type ManageControlSpec = {
  readonly title: string;
  readonly devices: readonly string[];
  readonly modes: readonly ManageControlAction[];
  readonly options: readonly ManageControlOption[];
  readonly actions: readonly ManageControlAction[];
  readonly status: readonly ManageControlAction[];
};

type ManageLaneId = 'portal' | 'childPolicy' | 'deviceOps';
type ManageScopeId = 'global' | 'perDevice';

type ManageTargetSelection = {
  readonly scope: ManageScopeId;
  readonly device: string;
  readonly browser: string;
};

type ManageTargetChoice = {
  readonly label: string;
  readonly detail: string;
  readonly tone: Tone;
  readonly scope?: ManageScopeId;
};

const MANAGE_LANES: readonly {
  readonly id: ManageLaneId;
  readonly label: string;
  readonly detail: string;
  readonly tone: Tone;
}[] = [
  { id: 'portal', label: 'PORTAL', detail: '', tone: 'cyan' },
  { id: 'childPolicy', label: 'POLICIES', detail: '', tone: 'gold' },
  { id: 'deviceOps', label: 'DEVICE TOOLS', detail: '', tone: 'purple' },
];

const MANAGE_ROUTE_KEYS = new Set([
  'browser-settings',
  'rule-management',
  'schedules',
  'approvals',
  'enforcement',
  'report-settings',
  'screen-analysis',
  'app-game-sessions',
  'network-activity',
  'memory-settings',
  'ai-runtime',
  'api-providers',
  'remote-access',
  'subscription',
  'platforms-install',
  'devices',
  'lan-pairing',
  'capability-status',
  'notifications',
  'notification-channels',
  'drive-connections',
  'export-retention',
  'report-compiler',
  'audit-history',
  'entitlements',
  'install-updates',
  'diagnostics',
  'settings-rules',
]);

function isManageQuickControl(control: ControlArea | QuickControl): boolean {
  const routeKey = assetKey(control.routePath);
  return MANAGE_ROUTE_KEYS.has(routeKey);
}

function guideRoutePathForManageKey(activeNavLabel: string, selectedControlName: string): string {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  if (
    key.includes('support') ||
    key.includes('diagnostic') ||
    key.includes('subscription') ||
    key.includes('entitlement') ||
    key.includes('platform') ||
    key.includes('update') ||
    key.includes('install') ||
    key.includes('setting')
  )
    return '#/start';
  if (key.includes('device') || key.includes('lan') || key.includes('capability')) return '#/start';
  if (
    key.includes('browser') ||
    key.includes('rule') ||
    key.includes('policy') ||
    key.includes('schedule') ||
    key.includes('approval') ||
    key.includes('enforce')
  )
    return '#/policy';
  if (key.includes('drive') || key.includes('export') || key.includes('private') || key.includes('remote'))
    return '#/privacy-design';
  if (key.includes('ai') || key.includes('api') || key.includes('memory')) return '#/ai-runtime';
  if (
    key.includes('report') ||
    key.includes('screen') ||
    key.includes('apps-games') ||
    key.includes('network') ||
    key.includes('alert') ||
    key.includes('notification') ||
    key.includes('channel') ||
    key.includes('audit')
  )
    return '#/report-settings';
  return '#/start';
}

function manageLaneForKey(activeNavLabel: string, selectedControlName: string): ManageLaneId {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  if (
    key.includes('channel') ||
    key.includes('drive') ||
    key.includes('api-key') ||
    key.includes('api-providers') ||
    key.includes('notification') ||
    key.includes('alert') ||
    key.includes('subscription') ||
    key.includes('entitlement') ||
    key.includes('export') ||
    key.includes('retention') ||
    key.includes('support') ||
    key.includes('diagnostic') ||
    key.includes('audit') ||
    key.includes('family-setting') ||
    key.includes('settings-rules')
  ) {
    return 'portal';
  }
  if (
    key.includes('lan') ||
    key.includes('capability') ||
    key.includes('remote') ||
    key.includes('platform') ||
    key.includes('update') ||
    key.includes('install') ||
    key.includes('device-pairing') ||
    key.includes('devices')
  ) {
    return 'deviceOps';
  }
  return 'childPolicy';
}

function manageLaneForControl(control: ControlArea | QuickControl): ManageLaneId {
  return manageLaneForKey(
    control.name,
    `${control.category ?? ''} ${control.subcategory ?? ''} ${control.routePath ?? ''}`
  );
}

function manageScopeForLane(lane: ManageLaneId): ManageScopeId {
  return lane === 'portal' ? 'global' : 'perDevice';
}

function isBrowserManageKey(activeNavLabel: string, selectedControlName: string): boolean {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  return key.includes('browser') || key.includes('web');
}

function manageScopeChoicesForLane(lane: ManageLaneId): readonly ManageTargetChoice[] {
  if (lane === 'portal') {
    return [{ label: 'Parent profile', detail: 'Portal setting only.', tone: 'cyan', scope: 'global' }];
  }
  if (lane === 'deviceOps') {
    return [
      { label: 'All devices', detail: 'Apply to every paired child device.', tone: 'cyan', scope: 'global' },
      { label: 'Selected device', detail: 'Send only to one child device.', tone: 'gold', scope: 'perDevice' },
    ];
  }
  return [
    { label: 'Family default', detail: 'Base rule for all children.', tone: 'cyan', scope: 'global' },
    { label: 'Child override', detail: 'Override one child device.', tone: 'gold', scope: 'perDevice' },
  ];
}

function manageBrowserTargetsForKey(
  activeNavLabel: string,
  selectedControlName: string
): readonly ManageTargetChoice[] {
  if (!isBrowserManageKey(activeNavLabel, selectedControlName)) return [];
  return [
    { label: 'Chrome', detail: 'Chrome browser policy target.', tone: 'cyan' },
    { label: 'Edge', detail: 'Edge browser policy target.', tone: 'gold' },
    { label: 'Firefox', detail: 'Firefox browser policy target.', tone: 'purple' },
    { label: 'All browsers', detail: 'Every detected browser family.', tone: 'red' },
  ];
}

function scheduleOptionsForManageKey(
  activeNavLabel: string,
  selectedControlName: string
): readonly ManageControlAction[] {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  if (
    key.includes('browser') ||
    key.includes('app') ||
    key.includes('game') ||
    key.includes('network') ||
    key.includes('alert') ||
    key.includes('rule') ||
    key.includes('schedule')
  ) {
    return [
      { label: 'Always', detail: 'Applies all day.', tone: 'cyan' },
      { label: 'School', detail: 'School-hour rule.', tone: 'gold' },
      { label: 'Homework', detail: 'Homework window.', tone: 'purple' },
      { label: 'Bedtime', detail: 'Sleep cutoff.', tone: 'red' },
      { label: 'Weekend', detail: 'Weekend limits.', tone: 'cyan' },
      { label: 'Custom', detail: 'Parent-defined window.', tone: 'gold' },
    ];
  }
  return [];
}

function manageControlSpecFor(activeNavLabel: string, selectedControlName: string): ManageControlSpec | null {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  const devices = ['Aarav laptop', 'Mina tablet', 'Family default'];
  const baseStatus = [
    { label: 'Scope', detail: 'Per child device', tone: 'cyan' as Tone },
    { label: 'Custody', detail: 'Local first', tone: 'gold' as Tone },
    { label: 'Audit', detail: 'Every change recorded', tone: 'purple' as Tone },
  ];

  if (key.includes('browser')) {
    return {
      title: 'Browser Setup',
      devices,
      modes: [
        { label: 'Advisory', detail: 'Show risk without blocking.', tone: 'cyan' },
        { label: 'Ask first', detail: 'Ask parent before risky browsing.', tone: 'gold' },
        { label: 'Block', detail: 'Block unsupported browser paths.', tone: 'red' },
      ],
      options: [
        {
          label: 'Require managed browser',
          detail: 'Prefer browser paths with visible evidence.',
          enabled: true,
          tone: 'cyan',
        },
        {
          label: 'Flag unsupported browsers',
          detail: 'Show installed browsers that cannot be controlled yet.',
          enabled: true,
          tone: 'gold',
        },
        {
          label: 'Explain before block',
          detail: 'Tell the child why a page or browser path is blocked.',
          enabled: true,
          tone: 'purple',
        },
        {
          label: 'Allow parent override',
          detail: 'Parent can approve a timed exception.',
          enabled: false,
          tone: 'cyan',
        },
      ],
      actions: [
        { label: 'Scan browsers', detail: 'Refresh supported and unsupported browser state.', tone: 'cyan' },
        { label: 'Apply policy', detail: 'Save this browser rule for the selected device.', tone: 'gold' },
        { label: 'Open web activity', detail: 'Review URL evidence and browser family.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('rule') || key.includes('policy')) {
    return {
      title: 'Rule Setup',
      devices,
      modes: [
        { label: 'Allow', detail: 'Let it pass and record the decision.', tone: 'cyan' },
        { label: 'Ask', detail: 'Pause and ask the parent.', tone: 'gold' },
        { label: 'Block', detail: 'Stop the activity after policy review.', tone: 'red' },
      ],
      options: [
        { label: 'School hours', detail: 'Use school-day defaults for apps and sites.', enabled: true, tone: 'cyan' },
        { label: 'Bedtime quiet', detail: 'Reduce distractions during sleep windows.', enabled: true, tone: 'purple' },
        {
          label: 'New apps ask first',
          detail: 'Require approval for newly detected apps.',
          enabled: true,
          tone: 'gold',
        },
        {
          label: 'Mature content review',
          detail: 'Review risky categories before allow.',
          enabled: false,
          tone: 'red',
        },
      ],
      actions: [
        { label: 'Preview decision', detail: 'See what the rule would do before applying.', tone: 'cyan' },
        { label: 'Save rule', detail: 'Keep the rule in family policy.', tone: 'gold' },
        { label: 'View audit', detail: 'Open recent rule changes and parent approvals.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('schedule')) {
    return {
      title: 'Schedules',
      devices,
      modes: [
        { label: 'School day', detail: 'Use weekday learning windows.', tone: 'cyan' },
        { label: 'Weekend', detail: 'Use weekend family limits.', tone: 'gold' },
        { label: 'Temporary', detail: 'Grant or reduce time today only.', tone: 'purple' },
      ],
      options: [
        { label: 'Homework window', detail: 'Focus mode during homework time.', enabled: true, tone: 'cyan' },
        { label: 'Sleep lock', detail: 'Use bedtime cutoff and morning resume.', enabled: true, tone: 'purple' },
        { label: 'Game budget', detail: 'Cap game time separately from school tools.', enabled: false, tone: 'gold' },
        { label: 'Parent extension', detail: 'Allow one-time time grants.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Edit week', detail: 'Open weekly schedule controls.', tone: 'cyan' },
        { label: 'Grant time', detail: 'Create a temporary time exception.', tone: 'gold' },
        { label: 'Pause today', detail: 'Pause this schedule for the selected child.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('approval')) {
    return {
      title: 'Approvals',
      devices,
      modes: [
        { label: 'Ask parent', detail: 'Require an explicit parent answer.', tone: 'gold' },
        { label: 'Auto explain', detail: 'Give an explanation without changing rule.', tone: 'cyan' },
        { label: 'Auto deny', detail: 'Deny known restricted requests.', tone: 'red' },
      ],
      options: [
        { label: 'Require reason', detail: 'Child must explain why they need access.', enabled: true, tone: 'cyan' },
        { label: 'Timed approval', detail: 'Approvals expire automatically.', enabled: true, tone: 'gold' },
        { label: 'Notify parent', detail: 'Send parent a minimal alert.', enabled: true, tone: 'purple' },
        { label: 'Remember answer', detail: 'Reuse this approval rule next time.', enabled: false, tone: 'cyan' },
      ],
      actions: [
        { label: 'Open queue', detail: 'Review pending child requests.', tone: 'gold' },
        { label: 'Approve once', detail: 'Grant a one-time exception.', tone: 'cyan' },
        { label: 'Set default', detail: 'Choose the default answer for this class.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('enforce')) {
    return {
      title: 'Enforcement',
      devices,
      modes: [
        { label: 'Observe', detail: 'Record would-enforce outcomes.', tone: 'cyan' },
        { label: 'Dry run', detail: 'Validate without executing adapters.', tone: 'gold' },
        { label: 'Enforce', detail: 'Apply supported adapter actions.', tone: 'red' },
      ],
      options: [
        { label: 'Require evidence ref', detail: 'Never act from AI text alone.', enabled: true, tone: 'cyan' },
        { label: 'Timer rollback', detail: 'Expire temporary controls automatically.', enabled: true, tone: 'gold' },
        { label: 'Capability gate', detail: 'Unavailable adapters stay honest.', enabled: true, tone: 'purple' },
        { label: 'Parent override', detail: 'Parent can cancel or supersede action.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Test dry run', detail: 'Run policy preview without adapter action.', tone: 'cyan' },
        { label: 'Apply timed block', detail: 'Create a typed temporary block.', tone: 'red' },
        { label: 'Rollback', detail: 'Cancel or expire the active control.', tone: 'gold' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('report')) {
    return {
      title: key.includes('build') ? 'Report Compiler' : 'Report Setup',
      devices,
      modes: [
        { label: 'Daily', detail: 'Short day summary.', tone: 'cyan' },
        { label: 'Weekly', detail: 'Patterns and changes.', tone: 'purple' },
        { label: 'Monthly', detail: 'Longer family review.', tone: 'gold' },
      ],
      options: [
        { label: 'Activity summary', detail: 'Apps, sites, focus, and screen analysis.', enabled: true, tone: 'cyan' },
        {
          label: 'Policy decisions',
          detail: 'Include blocks, asks, approvals, and overrides.',
          enabled: true,
          tone: 'gold',
        },
        { label: 'AI citations', detail: 'Use cited local memory in explanations.', enabled: true, tone: 'purple' },
        { label: 'Export copy', detail: 'Write a parent-owned report export.', enabled: false, tone: 'cyan' },
      ],
      actions: [
        { label: 'Compile now', detail: 'Build a stateless report from selected evidence.', tone: 'cyan' },
        { label: 'Schedule report', detail: 'Set report time and cadence.', tone: 'gold' },
        { label: 'Open reports', detail: 'Review generated report history.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('capability')) {
    return {
      title: 'Capability Status',
      devices: ['Aarav laptop', 'Mina tablet', 'Family default'],
      modes: [
        { label: 'Detect', detail: 'Refresh available child-device capabilities.', tone: 'cyan' },
        { label: 'Degrade', detail: 'Use safer reduced behavior when a capability is partial.', tone: 'gold' },
        { label: 'Disable', detail: 'Keep unsupported controls unavailable.', tone: 'red' },
      ],
      options: [
        {
          label: 'Browser control',
          detail: 'Supported, degraded, or unavailable browser management.',
          enabled: true,
          tone: 'cyan',
        },
        { label: 'App control', detail: 'Supported app and process policy actions.', enabled: true, tone: 'gold' },
        { label: 'Screen summary', detail: 'Local screen analysis readiness.', enabled: false, tone: 'purple' },
        { label: 'Network metadata', detail: 'Domain and flow visibility state.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Refresh status', detail: 'Query the selected child device.', tone: 'cyan' },
        { label: 'Show gaps', detail: 'List controls that cannot run on this device.', tone: 'gold' },
        { label: 'Open install', detail: 'Go to install and update controls.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('lan-pairing') || key.includes('lan pairing')) {
    return {
      title: 'LAN Pairing',
      devices: ['New child device', 'Aarav laptop', 'Family tablet'],
      modes: [
        { label: 'Discover', detail: 'Find child agents on the same LAN.', tone: 'cyan' },
        { label: 'Pair', detail: 'Require challenge and trusted proof.', tone: 'gold' },
        { label: 'Revoke', detail: 'Remove device trust before new commands.', tone: 'red' },
      ],
      options: [
        {
          label: 'Require pairing proof',
          detail: 'Reject anonymous or stale LAN attempts.',
          enabled: true,
          tone: 'cyan',
        },
        { label: 'Origin check', detail: 'Only accept allowed parent origins.', enabled: true, tone: 'gold' },
        {
          label: 'Replay protection',
          detail: 'Reject stale or reused pairing attempts.',
          enabled: true,
          tone: 'purple',
        },
        { label: 'Fail closed', detail: 'Unpaired devices receive no control intents.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Start pairing', detail: 'Begin a trusted local pairing challenge.', tone: 'gold' },
        { label: 'Select child', detail: 'Make this device the active control target.', tone: 'cyan' },
        { label: 'Revoke trust', detail: 'Stop accepting control intents from this device.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('device')) {
    return {
      title: 'Devices',
      devices: ['Aarav laptop', 'Mina tablet', 'New child device'],
      modes: [
        { label: 'Selected', detail: 'Use this device as the active target.', tone: 'cyan' },
        { label: 'Pair', detail: 'Add a child device.', tone: 'gold' },
        { label: 'Suspend', detail: 'Stop sending commands to this device.', tone: 'red' },
      ],
      options: [
        { label: 'Show offline', detail: 'Keep stale devices visible but marked.', enabled: true, tone: 'cyan' },
        {
          label: 'Per-device overrides',
          detail: 'Allow this child to differ from family defaults.',
          enabled: true,
          tone: 'gold',
        },
        {
          label: 'Require parent session',
          detail: 'Protect device changes behind parent login.',
          enabled: true,
          tone: 'red',
        },
        { label: 'Capability badges', detail: 'Show supported, degraded, unavailable.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Pair device', detail: 'Start local pairing.', tone: 'gold' },
        { label: 'Set active', detail: 'Use selected child for controls.', tone: 'cyan' },
        { label: 'Open capability', detail: 'Review what this device supports.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('screen')) {
    return {
      title: 'Screen Analysis',
      devices,
      modes: [
        { label: 'Off', detail: 'Do not run screen summaries.', tone: 'cyan' },
        { label: 'Summary', detail: 'Create parent-readable local summaries.', tone: 'gold' },
        { label: 'Ask first', detail: 'Require parent approval before enabling.', tone: 'purple' },
      ],
      options: [
        {
          label: 'Local summarizer',
          detail: 'Summaries run on the child device when available.',
          enabled: false,
          tone: 'cyan',
        },
        { label: 'No screenshots in alerts', detail: 'Alerts never send raw screenshots.', enabled: true, tone: 'red' },
        { label: 'Evidence citations', detail: 'Show why a summary was produced.', enabled: true, tone: 'gold' },
        { label: 'Sensitive redaction', detail: 'Avoid raw private evidence exposure.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Check support', detail: 'Verify local screen analysis capability.', tone: 'cyan' },
        { label: 'Set privacy', detail: 'Choose summary and evidence level.', tone: 'gold' },
        { label: 'Open guide', detail: 'Explain screen analysis to the parent.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('apps-games')) {
    return {
      title: 'Apps And Games',
      devices,
      modes: [
        { label: 'Observe', detail: 'Record app sessions without blocking.', tone: 'cyan' },
        { label: 'Limit', detail: 'Apply time budgets and schedules.', tone: 'gold' },
        { label: 'Block', detail: 'Block selected apps when supported.', tone: 'red' },
      ],
      options: [
        { label: 'Known apps', detail: 'Show named apps and game sessions.', enabled: true, tone: 'cyan' },
        { label: 'New app asks', detail: 'Ask parent before unknown apps get time.', enabled: true, tone: 'gold' },
        { label: 'Game budget', detail: 'Separate games from school tools.', enabled: false, tone: 'purple' },
        {
          label: 'Suspicious app flag',
          detail: 'Surface apps that do not match normal use.',
          enabled: true,
          tone: 'red',
        },
      ],
      actions: [
        { label: 'Choose apps', detail: 'Open app allow, ask, block list.', tone: 'cyan' },
        { label: 'Set budget', detail: 'Set time limit and schedule.', tone: 'gold' },
        { label: 'Review sessions', detail: 'Open recent app timeline.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('network')) {
    return {
      title: 'Network Activity',
      devices,
      modes: [
        { label: 'Metadata', detail: 'Keep domain and flow metadata.', tone: 'cyan' },
        { label: 'Review', detail: 'Ask parent on risky destinations.', tone: 'gold' },
        { label: 'Block', detail: 'Block destinations when adapters support it.', tone: 'red' },
      ],
      options: [
        { label: 'Domain list', detail: 'Show domains, not raw packet contents.', enabled: true, tone: 'cyan' },
        { label: 'No payload capture', detail: 'Never collect private payload bodies.', enabled: true, tone: 'red' },
        { label: 'School allowlist', detail: 'Permit known learning destinations.', enabled: false, tone: 'gold' },
        { label: 'Unsupported warning', detail: 'Show when OS control is unavailable.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Open domains', detail: 'Review recent domains.', tone: 'cyan' },
        { label: 'Edit allowlist', detail: 'Choose trusted domains.', tone: 'gold' },
        { label: 'Export summary', detail: 'Save parent-owned network summary.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('channel')) {
    return {
      title: 'Notification Channels',
      devices: ['Parent profile', 'Family portal', 'Emergency only'],
      modes: [
        { label: 'Portal', detail: 'Keep notices inside the parent portal.', tone: 'cyan' },
        { label: 'Verified', detail: 'Use verified parent-owned destinations.', tone: 'gold' },
        { label: 'Muted', detail: 'Pause external delivery.', tone: 'purple' },
      ],
      options: [
        { label: 'Email', detail: 'Send minimal parent alerts to verified email.', enabled: false, tone: 'cyan' },
        { label: 'SMS', detail: 'Send high-priority parent alerts by SMS.', enabled: false, tone: 'gold' },
        { label: 'WhatsApp', detail: 'Use parent-owned WhatsApp when configured.', enabled: false, tone: 'purple' },
        { label: 'Quiet hours', detail: 'Hold low priority alerts overnight.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Verify channel', detail: 'Confirm parent-owned destination.', tone: 'gold' },
        { label: 'Send test', detail: 'Send a minimal test alert.', tone: 'cyan' },
        { label: 'Mute all', detail: 'Pause external channels.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('alert') || key.includes('notification')) {
    return {
      title: 'Alerts',
      devices: ['Parent profile', 'All child devices', 'Selected child'],
      modes: [
        { label: 'Portal only', detail: 'Keep alerts inside the portal.', tone: 'cyan' },
        { label: 'Priority', detail: 'Send only high-priority events out.', tone: 'gold' },
        { label: 'External', detail: 'Use selected verified channels.', tone: 'purple' },
      ],
      options: [
        { label: 'Policy alerts', detail: 'Notify on blocked or ask-parent decisions.', enabled: true, tone: 'gold' },
        { label: 'Device offline', detail: 'Warn when child device goes stale.', enabled: true, tone: 'cyan' },
        { label: 'Approval requests', detail: 'Notify parent when child asks.', enabled: true, tone: 'purple' },
        { label: 'No raw evidence', detail: 'Never send raw local evidence in alerts.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Test alert', detail: 'Send a minimal test notification.', tone: 'cyan' },
        { label: 'Choose channels', detail: 'Open verified channel setup.', tone: 'gold' },
        { label: 'Set quiet hours', detail: 'Choose when alerts pause.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('drive')) {
    return {
      title: 'Drives',
      devices: ['Parent Drive', 'Local only', 'Support bundle'],
      modes: [
        { label: 'Disconnected', detail: 'No drive export destination.', tone: 'cyan' },
        { label: 'Connect', detail: 'Use parent-owned storage.', tone: 'gold' },
        { label: 'Review', detail: 'Preview before any export.', tone: 'purple' },
      ],
      options: [
        { label: 'Google Drive', detail: 'Connect parent-owned Drive export.', enabled: false, tone: 'cyan' },
        { label: 'OneDrive', detail: 'Use a parent-owned OneDrive target.', enabled: false, tone: 'purple' },
        { label: 'Report exports', detail: 'Allow selected reports to export.', enabled: true, tone: 'gold' },
        { label: 'Support bundle review', detail: 'Preview bundle before sharing.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Connect drive', detail: 'Start parent-owned drive connection.', tone: 'cyan' },
        { label: 'Test export', detail: 'Write a minimal test file.', tone: 'gold' },
        { label: 'Disconnect', detail: 'Remove this storage target.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('export') || key.includes('retention') || key.includes('private')) {
    return {
      title: 'Export Delete Retention',
      devices,
      modes: [
        { label: 'Keep local', detail: 'No export destination.', tone: 'cyan' },
        { label: 'Export copy', detail: 'Write selected data to parent storage.', tone: 'gold' },
        { label: 'Delete class', detail: 'Delete selected local data class.', tone: 'red' },
      ],
      options: [
        { label: 'Activity summaries', detail: 'Include daily and weekly reports.', enabled: true, tone: 'cyan' },
        { label: 'Audit log', detail: 'Include rule, pairing, and export records.', enabled: true, tone: 'gold' },
        { label: 'Retention window', detail: 'Choose what local data expires.', enabled: true, tone: 'purple' },
        {
          label: 'Raw evidence excluded',
          detail: 'Do not export raw private evidence by default.',
          enabled: true,
          tone: 'red',
        },
      ],
      actions: [
        { label: 'Export report', detail: 'Write selected report to storage.', tone: 'gold' },
        { label: 'Preview data', detail: 'Show selected data classes.', tone: 'cyan' },
        { label: 'Delete selected', detail: 'Delete selected local data class.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('api')) {
    return {
      title: 'API Providers',
      devices: ['Parent portal', 'No child key', 'Family account'],
      modes: [
        { label: 'Disabled', detail: 'Do not use external AI providers.', tone: 'cyan' },
        { label: 'Ask parent', detail: 'Require parent choice per provider.', tone: 'gold' },
        { label: 'Enabled', detail: 'Use configured provider limits.', tone: 'purple' },
      ],
      options: [
        { label: 'OpenAI key', detail: 'Parent-owned provider key.', enabled: false, tone: 'cyan' },
        { label: 'Per-device limits', detail: 'Limit which child devices can call APIs.', enabled: true, tone: 'gold' },
        { label: 'No raw evidence', detail: 'Do not send raw evidence to providers.', enabled: true, tone: 'red' },
        { label: 'Cost guard', detail: 'Use budget and request limits.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Add provider', detail: 'Add parent-owned API provider.', tone: 'gold' },
        { label: 'Test key', detail: 'Validate provider connection.', tone: 'cyan' },
        { label: 'Disable API', detail: 'Turn off external providers.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('memory')) {
    return {
      title: 'Memory Setup',
      devices,
      modes: [
        { label: 'Off', detail: 'Do not store cited memory.', tone: 'cyan' },
        { label: 'Cited', detail: 'Store only cited parent-approved context.', tone: 'gold' },
        { label: 'Review', detail: 'Require parent review before reuse.', tone: 'purple' },
      ],
      options: [
        { label: 'Cited answers', detail: 'Show sources for activity explanations.', enabled: true, tone: 'purple' },
        { label: 'Parent review', detail: 'Parent can review and revoke memory.', enabled: true, tone: 'cyan' },
        { label: 'Per-device memory', detail: 'Keep child-device context separated.', enabled: true, tone: 'gold' },
        { label: 'Export/delete', detail: 'Parent controls memory export and deletion.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Review memory', detail: 'Open cited memory controls.', tone: 'purple' },
        { label: 'Export memory', detail: 'Save parent-owned memory copy.', tone: 'gold' },
        { label: 'Clear memory', detail: 'Delete selected memory records.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('ai')) {
    return {
      title: 'AI Setup',
      devices,
      modes: [
        { label: 'Local AI', detail: 'Use local model when available.', tone: 'cyan' },
        { label: 'Local hub', detail: 'Queue to stronger local family machine.', tone: 'gold' },
        { label: 'API opt-in', detail: 'Use external provider only by parent choice.', tone: 'purple' },
      ],
      options: [
        { label: 'Cited answers', detail: 'Show sources for activity explanations.', enabled: true, tone: 'purple' },
        { label: 'Per-device model', detail: 'Choose model by child device capability.', enabled: true, tone: 'cyan' },
        {
          label: 'External provider keys',
          detail: 'Parent supplies and controls API keys.',
          enabled: false,
          tone: 'gold',
        },
        { label: 'Memory review', detail: 'Review, revoke, and export cited memory.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Choose model', detail: 'Set local or provider model for this device.', tone: 'cyan' },
        { label: 'Add API key', detail: 'Connect an optional parent-owned provider.', tone: 'gold' },
        { label: 'Review memory', detail: 'Open cited memory controls.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('entitlement')) {
    return {
      title: 'Entitlements',
      devices: ['Family plan', 'This child', 'All child devices'],
      modes: [
        { label: 'Included', detail: 'Feature is included in this plan.', tone: 'cyan' },
        { label: 'Limited', detail: 'Feature has usage or device limits.', tone: 'gold' },
        { label: 'Locked', detail: 'Feature is unavailable for this plan.', tone: 'red' },
      ],
      options: [
        {
          label: 'Local safety controls',
          detail: 'Safety-critical local controls stay visible.',
          enabled: true,
          tone: 'cyan',
        },
        { label: 'Device count', detail: 'Show paired child-device entitlement use.', enabled: true, tone: 'gold' },
        { label: 'Remote features', detail: 'Show remote and external-service gates.', enabled: false, tone: 'purple' },
        {
          label: 'Grace fallback',
          detail: 'Keep honest degraded state when billing lapses.',
          enabled: true,
          tone: 'red',
        },
      ],
      actions: [
        { label: 'Refresh gates', detail: 'Recheck current feature state.', tone: 'cyan' },
        { label: 'View plan', detail: 'Open subscription plan controls.', tone: 'gold' },
        { label: 'Apply code', detail: 'Redeem family or support entitlement.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('subscription')) {
    return {
      title: 'Subscription',
      devices: ['Family plan', 'Parent portal', 'All child devices'],
      modes: [
        { label: 'Trial', detail: 'Temporary access window.', tone: 'cyan' },
        { label: 'Paid', detail: 'Plan-backed controls.', tone: 'gold' },
        { label: 'Grace', detail: 'Safe degraded access.', tone: 'purple' },
      ],
      options: [
        { label: 'Device limit', detail: 'Show paired child-device count.', enabled: true, tone: 'cyan' },
        { label: 'Plan comparison', detail: 'Show what each plan includes.', enabled: true, tone: 'gold' },
        { label: 'Billing portal', detail: 'Open subscription management.', enabled: false, tone: 'purple' },
        { label: 'Grace mode', detail: 'Keep safety-critical local controls visible.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Change plan', detail: 'Open subscription choices.', tone: 'gold' },
        { label: 'Apply code', detail: 'Redeem family or support entitlement.', tone: 'cyan' },
        { label: 'View limits', detail: 'Review plan and device limits.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('support') || key.includes('diagnostic')) {
    return {
      title: 'Support',
      devices: ['Parent portal', 'Selected child', 'All devices'],
      modes: [
        { label: 'Self check', detail: 'Run local health checks.', tone: 'cyan' },
        { label: 'Bundle', detail: 'Create parent-reviewed support bundle.', tone: 'gold' },
        { label: 'Contact', detail: 'Prepare support request.', tone: 'purple' },
      ],
      options: [
        { label: 'Include versions', detail: 'Attach app and service versions.', enabled: true, tone: 'cyan' },
        {
          label: 'Include capability',
          detail: 'Attach supported/degraded/unavailable states.',
          enabled: true,
          tone: 'gold',
        },
        {
          label: 'Exclude evidence',
          detail: 'Do not include raw child evidence by default.',
          enabled: true,
          tone: 'red',
        },
        { label: 'Parent review', detail: 'Parent sees bundle before sharing.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Run diagnostics', detail: 'Check portal and child service health.', tone: 'cyan' },
        { label: 'Build bundle', detail: 'Create parent-reviewed support bundle.', tone: 'gold' },
        { label: 'Open support', detail: 'Open help and contact options.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('update') || key.includes('install')) {
    return {
      title: 'Install Updates',
      devices: ['Parent desktop', 'Child Windows', 'Mobile preview'],
      modes: [
        { label: 'Stable', detail: 'Use stable update channel.', tone: 'cyan' },
        { label: 'Preview', detail: 'Use preview builds when selected.', tone: 'gold' },
        { label: 'Rollback', detail: 'Return to last working build.', tone: 'red' },
      ],
      options: [
        { label: 'Auto check', detail: 'Check for updates at startup.', enabled: true, tone: 'cyan' },
        { label: 'Ask before install', detail: 'Parent approves update install.', enabled: true, tone: 'gold' },
        { label: 'Rollback point', detail: 'Keep last working installer state.', enabled: true, tone: 'purple' },
        { label: 'Mobile preview', detail: 'Show mobile app status separately.', enabled: false, tone: 'red' },
      ],
      actions: [
        { label: 'Check update', detail: 'Check available versions.', tone: 'cyan' },
        { label: 'Install now', detail: 'Apply approved update.', tone: 'gold' },
        { label: 'Rollback app', detail: 'Use last known good install.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('platform')) {
    return {
      title: 'Platforms',
      devices: ['Parent desktop', 'Child Windows', 'Mobile preview'],
      modes: [
        { label: 'Desktop', detail: 'Tauri parent and child desktop apps.', tone: 'cyan' },
        { label: 'Mobile', detail: 'Mobile parent app target.', tone: 'gold' },
        { label: 'Unsupported', detail: 'Show honest platform gaps.', tone: 'red' },
      ],
      options: [
        { label: 'Parent desktop', detail: 'Tauri parent portal target.', enabled: true, tone: 'cyan' },
        { label: 'Child service', detail: 'Headless child-device agent target.', enabled: true, tone: 'gold' },
        { label: 'Mobile app', detail: 'Mobile parent controls target.', enabled: false, tone: 'purple' },
        { label: 'Honest limits', detail: 'Show unavailable platform capabilities.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Open install', detail: 'Open installation checklist.', tone: 'cyan' },
        { label: 'Check platform', detail: 'Verify current platform support.', tone: 'gold' },
        { label: 'Open guide', detail: 'Explain platform targets.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('remote')) {
    return {
      title: 'Remote Access',
      devices,
      modes: [
        { label: 'Home LAN', detail: 'Local-only by default.', tone: 'cyan' },
        { label: 'Parent relay', detail: 'Minimal remote control path.', tone: 'gold' },
        { label: 'Drive read', detail: 'Read exported reports only.', tone: 'purple' },
      ],
      options: [
        {
          label: 'Pair before remote',
          detail: 'Remote access requires trusted device setup.',
          enabled: true,
          tone: 'cyan',
        },
        { label: 'No raw evidence', detail: 'Remote summaries avoid raw evidence blobs.', enabled: true, tone: 'red' },
        { label: 'Parent-owned exports', detail: 'Use parent storage for report copies.', enabled: true, tone: 'gold' },
        { label: 'Emergency revoke', detail: 'Stop remote sessions immediately.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Enable remote', detail: 'Choose a remote access path.', tone: 'gold' },
        { label: 'Revoke remote', detail: 'Stop remote access for this device.', tone: 'red' },
        { label: 'Open exports', detail: 'Use parent-owned report storage.', tone: 'cyan' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('audit')) {
    return {
      title: 'Audit History',
      devices,
      modes: [
        { label: 'Today', detail: 'Recent decisions.', tone: 'cyan' },
        { label: 'By device', detail: 'Filter to child device.', tone: 'gold' },
        { label: 'By area', detail: 'Rules, exports, AI, pairing.', tone: 'purple' },
      ],
      options: [
        { label: 'Rule changes', detail: 'Show policy edits and approvals.', enabled: true, tone: 'gold' },
        { label: 'Export events', detail: 'Show data export/delete records.', enabled: true, tone: 'cyan' },
        {
          label: 'Pairing attempts',
          detail: 'Show accepted and rejected LAN attempts.',
          enabled: true,
          tone: 'purple',
        },
        { label: 'Failures', detail: 'Show degraded and unavailable outcomes.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Filter audit', detail: 'Choose device, area, and time range.', tone: 'cyan' },
        { label: 'Export audit', detail: 'Save parent-owned audit report.', tone: 'gold' },
        { label: 'Open event', detail: 'Inspect selected decision details.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('setting')) {
    return {
      title: 'Family Settings',
      devices: ['Family default', 'This child', 'Parent profile'],
      modes: [
        { label: 'Family', detail: 'Apply to all child devices.', tone: 'cyan' },
        { label: 'Child', detail: 'Override for one child.', tone: 'gold' },
        { label: 'Parent', detail: 'Parent identity and preferences.', tone: 'purple' },
      ],
      options: [
        { label: 'Your House Your Rule', detail: 'Use family-defined controls first.', enabled: true, tone: 'gold' },
        { label: 'Light and dark', detail: 'Theme follows parent choice.', enabled: true, tone: 'cyan' },
        { label: 'Require parent session', detail: 'Protect changes behind parent login.', enabled: true, tone: 'red' },
        { label: 'Child profile defaults', detail: 'Create defaults for new devices.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Edit family', detail: 'Open family defaults.', tone: 'cyan' },
        { label: 'Add child', detail: 'Create a child profile.', tone: 'gold' },
        { label: 'Parent login', detail: 'Protect this console session.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  return null;
}

function ManageTargetPanel({
  x,
  y,
  w,
  h,
  activeNavLabel,
  selectedControlName,
  spec,
  lane,
  targetSelection,
  onTargetChange,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  activeNavLabel: string;
  selectedControlName: string;
  spec: ManageControlSpec;
  lane: ManageLaneId;
  targetSelection: ManageTargetSelection;
  onTargetChange: (selection: ManageTargetSelection) => void;
  cfg: ParentPortalSvgControls;
}) {
  const scopeChoices = manageScopeChoicesForLane(lane);
  const browserChoices = manageBrowserTargetsForKey(activeNavLabel, selectedControlName);
  const color = toneColor(MANAGE_LANES.find((item) => item.id === lane)?.tone ?? 'cyan', cfg);
  const deviceChoices = spec.devices.slice(0, 3);
  const browserRowVisible = browserChoices.length > 0;
  const rowGap = browserRowVisible ? 43 : 52;
  const scopeY = y + 24;
  const deviceY = scopeY + rowGap;
  const browserY = deviceY + rowGap;
  const scopeCount = Math.max(1, scopeChoices.length);
  const deviceCount = Math.max(1, deviceChoices.length);
  const browserCount = Math.max(1, Math.min(browserChoices.length, 4));
  const scopeW = (w - 26 - (scopeCount - 1) * 8) / scopeCount;
  const deviceW = (w - 26 - (deviceCount - 1) * 8) / deviceCount;
  const browserW = (w - 26 - (browserCount - 1) * 8) / browserCount;
  const globalSummary = lane === 'portal' ? 'Parent profile' : lane === 'deviceOps' ? 'All devices' : 'Family default';
  const summary = browserRowVisible
    ? `${targetSelection.scope === 'global' ? globalSummary : targetSelection.device} / ${targetSelection.browser}`
    : targetSelection.scope === 'global'
      ? globalSummary
      : targetSelection.device;

  return (
    <g>
      <text x={x + 12} y={y + 12} fontSize={10.5} fontWeight={950} fill={color}>
        SCOPE
      </text>
      {scopeChoices.map((choice, index) => (
        <ManagePill
          key={`${selectedControlName}:scope:${choice.label}`}
          x={x + 12 + index * (scopeW + 8)}
          y={scopeY}
          w={scopeW}
          h={28}
          label={choice.label}
          selected={targetSelection.scope === choice.scope}
          tone={choice.tone}
          onSelect={() => {
            if (!choice.scope) return;
            onTargetChange({ ...targetSelection, scope: choice.scope });
          }}
          cfg={cfg}
        />
      ))}

      <text x={x + 12} y={deviceY - 10} fontSize={10.5} fontWeight={950} fill={cfg.colors.gold}>
        CHILD / DEVICE
      </text>
      {deviceChoices.map((choice, index) => (
        <ManagePill
          key={`${selectedControlName}:device-target:${choice}`}
          x={x + 12 + index * (deviceW + 8)}
          y={deviceY}
          w={deviceW}
          h={28}
          label={choice}
          selected={targetSelection.device === choice}
          tone={index === 1 ? 'gold' : index === 2 ? 'purple' : 'cyan'}
          onSelect={() =>
            onTargetChange({ ...targetSelection, device: choice, scope: lane === 'portal' ? 'global' : 'perDevice' })
          }
          cfg={cfg}
        />
      ))}

      {browserRowVisible ? (
        <>
          <text x={x + 12} y={browserY - 10} fontSize={10.5} fontWeight={950} fill={cfg.colors.purple}>
            BROWSER TARGET
          </text>
          {browserChoices.slice(0, 4).map((choice, index) => (
            <ManagePill
              key={`${selectedControlName}:browser-target:${choice.label}`}
              x={x + 12 + index * (browserW + 8)}
              y={browserY}
              w={browserW}
              h={28}
              label={choice.label}
              selected={targetSelection.browser === choice.label}
              tone={choice.tone}
              onSelect={() => onTargetChange({ ...targetSelection, browser: choice.label })}
              cfg={cfg}
            />
          ))}
        </>
      ) : null}

      <text x={x + w - 12} y={y + h - 10} textAnchor="end" fontSize={11.5} fontWeight={900} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(`Editing: ${summary}`, Math.max(180, w * 0.52), 11.5, 0.58)}
      </text>
    </g>
  );
}

function ManageControlPanel({
  x,
  y,
  w,
  h,
  activeNavLabel,
  selectedControlName,
  spec,
  themeTone,
  targetSelection,
  guideRoutePath,
  onNavigate,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  activeNavLabel: string;
  selectedControlName: string;
  spec: ManageControlSpec;
  themeTone: Tone;
  targetSelection: ManageTargetSelection;
  guideRoutePath: string;
  onNavigate?: (routePath: string) => void;
  cfg: ParentPortalSvgControls;
}) {
  const lane = manageLaneForKey(activeNavLabel, selectedControlName);
  const [mode, setMode] = useState(spec.modes[0]?.label ?? '');
  const [schedule, setSchedule] = useState('Always');
  const [enabled, setEnabled] = useState(
    () => new Set(spec.options.filter((option) => option.enabled).map((option) => option.label))
  );
  const [lastAction, setLastAction] = useState('Ready');
  const [syncStatus, setSyncStatus] = useState('Local draft');
  const specKey = `${lane}:${spec.title}:${spec.options.map((option) => option.label).join('|')}`;
  useEffect(() => {
    setMode(spec.modes[0]?.label ?? '');
    setSchedule('Always');
    setEnabled(new Set(spec.options.filter((option) => option.enabled).map((option) => option.label)));
    setLastAction('Ready');
    setSyncStatus('Local draft');
  }, [specKey]);
  const compact = w < 560;
  const activeModeTone = spec.modes.find((item) => item.label === mode)?.tone ?? themeTone;
  const color = toneColor(themeTone, cfg);
  const activeModeColor = toneColor(activeModeTone, cfg);
  const titleSize = fitSingleLineTextSize(spec.title, w - 90, 17, 26, 0.58);
  const headerH = compact ? 68 : 52;
  const leftW = compact ? w : Math.max(420, Math.round(w * 0.62));
  const rightW = compact ? w : w - leftW - 18;
  const controlY = y + headerH;
  const compactBodyH = Math.max(1, h - headerH - 12);
  const compactActionMinH = 190;
  const editorH = compact
    ? clampValue(Math.round(compactBodyH * 0.7), 300, Math.max(300, compactBodyH - compactActionMinH - 8))
    : Math.max(280, h - headerH - 10);
  const actionX = compact ? x : x + leftW + 18;
  const actionY = compact ? controlY + editorH + 8 : controlY;
  const actionH = compact ? Math.max(compactActionMinH, compactBodyH - editorH - 8) : Math.max(280, h - headerH - 10);
  const optionCount = compact ? 4 : Math.min(spec.options.length, 6);
  const optionRows = spec.options.slice(0, optionCount);
  const actionRows = spec.actions.slice(0, compact ? 2 : 3);
  const schedules = scheduleOptionsForManageKey(activeNavLabel, selectedControlName);
  const optionColumnCount = compact ? 1 : 2;
  const optionRowsUsed = Math.max(1, Math.ceil(optionRows.length / optionColumnCount));
  const scheduleY = controlY + 118 + optionRowsUsed * 40 + 38;
  const isPortalLane = lane === 'portal';
  const isDeviceOpsLane = lane === 'deviceOps';
  const overrideMode = targetSelection.scope;
  const globalTargetLabel = isPortalLane ? 'Parent profile' : isDeviceOpsLane ? 'All devices' : 'Family default';
  const device = targetSelection.scope === 'global' ? globalTargetLabel : targetSelection.device;
  const browserTarget = targetSelection.browser;
  const targetLabel = isPortalLane ? 'Parent profile' : device;
  const selectionLabel = isBrowserManageKey(activeNavLabel, selectedControlName)
    ? `${targetLabel} / ${browserTarget}`
    : targetLabel;
  const targetLabelUpper = targetLabel.toUpperCase();
  const applyHeaderLabel = isPortalLane ? 'APPLY' : `APPLY TO ${targetLabelUpper}`;
  const applyLabel = isPortalLane ? 'Save portal' : isDeviceOpsLane ? `Send to ${targetLabel}` : `Sync ${targetLabel}`;
  const inheritanceLabel = isPortalLane ? 'Portal global' : isDeviceOpsLane ? 'Choose device' : 'Family default';
  const overrideLabel = isPortalLane
    ? 'Parent profile'
    : isDeviceOpsLane
      ? `Command ${targetLabel}`
      : `Override ${targetLabel}`;
  const controlsActive = isPortalLane || isDeviceOpsLane || overrideMode === 'perDevice';

  return (
    <g>
      <text x={x} y={y + 24} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
        {spec.title.toUpperCase()}
      </text>
      <g
        className="parent-portal-svg-clickable"
        role="button"
        tabIndex={0}
        aria-label={`Open ${spec.title} guide`}
        onClick={(event) => {
          event.stopPropagation();
          onNavigate?.(guideRoutePath);
        }}
      >
        <title>{`Open ${spec.title} guide`}</title>
        <path
          d={cutRectPath(x + w - 58, y + 4, 56, 26, 8)}
          fill={colorAlpha(cfg.colors.cyan, '20')}
          stroke={cfg.colors.cyan}
          strokeWidth={0.9}
        />
        <text x={x + w - 30} y={y + 22} textAnchor="middle" fontSize={11} fontWeight={950} fill={cfg.colors.bodyText}>
          I
        </text>
      </g>
      <path d={`M ${x} ${y + 39} H ${x + w}`} stroke={color} strokeWidth={1.1} opacity={0.55} />
      <text x={x} y={y + 50} fontSize={10.5} fontWeight={850} fill={cfg.colors.mutedText}>
        {truncateTextForWidth(
          `${MANAGE_LANES.find((item) => item.id === lane)?.label ?? 'MANAGE'} / ${selectionLabel} / ${
            overrideMode === 'global' ? inheritanceLabel : overrideLabel
          }`,
          w - 80,
          11.5,
          0.58
        )}
      </text>

      <SurfacePanel x={x} y={controlY} w={leftW} h={editorH} tone={themeTone} cfg={cfg}>
        <text x={x + 18} y={controlY + 28} fontSize={10} fontWeight={950} fill={activeModeColor}>
          {isDeviceOpsLane ? 'COMMAND MODE' : 'SETTING MODE'}
        </text>
        {spec.modes.map((item, index) => (
          <ManageModeButton
            key={`${spec.title}:mode:${item.label}`}
            x={x + 18 + index * ((leftW - 42) / Math.min(3, spec.modes.length))}
            y={controlY + 44}
            w={(leftW - 52) / Math.min(3, spec.modes.length)}
            h={36}
            item={item}
            selected={mode === item.label}
            onSelect={() => {
              setMode(item.label);
              setLastAction(`${item.label} selected`);
              setSyncStatus('Draft changed');
            }}
            cfg={cfg}
          />
        ))}

        <text x={x + 18} y={controlY + 102} fontSize={10} fontWeight={950} fill={cfg.colors.cyan}>
          SETTING CHOICES
        </text>
        {optionRows.map((option, index) => {
          const optionW = compact ? leftW - 36 : (leftW - 48) / 2;
          const optionX = x + 18 + (compact ? 0 : (index % 2) * (optionW + 12));
          const optionY = controlY + 118 + Math.floor(index / optionColumnCount) * 40;
          const controlOption = { ...option, detail: '' };
          return (
            <ManageToggle
              key={`${spec.title}:option:${option.label}`}
              x={optionX}
              y={optionY}
              w={optionW}
              h={34}
              option={controlOption}
              selected={enabled.has(option.label)}
              disabled={!controlsActive}
              onToggle={() => {
                if (!controlsActive) return;
                setEnabled((current) => {
                  const next = new Set(current);
                  if (next.has(option.label)) next.delete(option.label);
                  else next.add(option.label);
                  return next;
                });
                setLastAction(`${option.label} changed`);
                setSyncStatus('Draft changed');
              }}
              cfg={cfg}
            />
          );
        })}

        {!compact && schedules.length && scheduleY + 44 < controlY + editorH ? (
          <>
            <text x={x + 18} y={scheduleY} fontSize={10} fontWeight={950} fill={cfg.colors.gold}>
              WHEN THIS APPLIES
            </text>
            {schedules.slice(0, compact ? 3 : 6).map((item, index) => {
              const chipW = compact ? (leftW - 48) / 3 : (leftW - 68) / 6;
              return (
                <ManagePill
                  key={`${spec.title}:schedule:${item.label}`}
                  x={x + 18 + index * (chipW + 6)}
                  y={scheduleY + 16}
                  w={chipW}
                  h={26}
                  label={item.label}
                  selected={schedule === item.label}
                  tone={item.tone}
                  onSelect={() => {
                    if (!controlsActive) return;
                    setSchedule(item.label);
                    setLastAction(`${item.label} schedule`);
                    setSyncStatus('Draft changed');
                  }}
                  cfg={cfg}
                />
              );
            })}
          </>
        ) : null}
      </SurfacePanel>

      <SurfacePanel x={actionX} y={actionY} w={rightW} h={actionH} tone={themeTone} cfg={cfg}>
        <text x={actionX + 18} y={actionY + 27} fontSize={10} fontWeight={950} fill={color}>
          {truncateTextForWidth(applyHeaderLabel, rightW - 36, 10, 0.58)}
        </text>
        <ManageActionButton
          x={actionX + 18}
          y={actionY + 43}
          w={rightW - 36}
          h={38}
          action={{ label: 'Validate Draft', detail: '', tone: 'cyan' }}
          onSelect={() => {
            setLastAction('Draft validated');
            setSyncStatus('Validated locally');
          }}
          cfg={cfg}
        />
        <ManageActionButton
          x={actionX + 18}
          y={actionY + 89}
          w={rightW - 36}
          h={38}
          action={{ label: applyLabel, detail: '', tone: 'gold' }}
          onSelect={() => {
            setLastAction(applyLabel);
            setSyncStatus(isPortalLane ? 'Saved in portal draft' : `Pending sync to ${targetLabel}`);
          }}
          cfg={cfg}
        />
        <ManageActionButton
          x={actionX + 18}
          y={actionY + 135}
          w={rightW - 36}
          h={38}
          action={{ label: 'Revert', detail: '', tone: 'red' }}
          onSelect={() => {
            setEnabled(new Set(spec.options.filter((option) => option.enabled).map((option) => option.label)));
            setMode(spec.modes[0]?.label ?? '');
            setSchedule('Always');
            setLastAction('Reverted');
            setSyncStatus('Local draft');
          }}
          cfg={cfg}
        />
        {!compact ? (
          <>
            <text x={actionX + 18} y={actionY + 195} fontSize={10} fontWeight={950} fill={cfg.colors.purple}>
              SHORTCUTS
            </text>
            {actionRows.map((action, index) => (
              <ManageActionButton
                key={`${spec.title}:action:${action.label}`}
                x={actionX + 18}
                y={actionY + 211 + index * 44}
                w={rightW - 36}
                h={34}
                action={{ ...action, detail: '' }}
                onSelect={() => {
                  setLastAction(action.label);
                  setSyncStatus(
                    action.label.toLowerCase().includes('open') ? 'Navigated locally' : 'Pending Rust sync'
                  );
                }}
                cfg={cfg}
              />
            ))}
            <path
              d={`M ${actionX + 18} ${actionY + actionH - 82} H ${actionX + rightW - 18}`}
              stroke={cfg.colors.panelStroke}
              strokeWidth={0.8}
              opacity={0.65}
            />
            <text x={actionX + 18} y={actionY + actionH - 58} fontSize={10} fontWeight={950} fill={cfg.colors.cyan}>
              CURRENT SELECTION
            </text>
            <text x={actionX + 18} y={actionY + actionH - 36} fontSize={12} fontWeight={900} fill={cfg.colors.bodyText}>
              {truncateTextForWidth(`${selectionLabel} / ${mode}`, rightW - 36, 12, 0.58)}
            </text>
            <text
              x={actionX + 18}
              y={actionY + actionH - 16}
              fontSize={10.5}
              fontWeight={760}
              fill={cfg.colors.mutedText}
            >
              {truncateTextForWidth(`${syncStatus}: ${lastAction}`, rightW - 36, 10.5, 0.58)}
            </text>
          </>
        ) : null}
      </SurfacePanel>
    </g>
  );
}

function ManagePill({ x, y, w, h, label, selected, tone, onSelect, cfg }) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={`Select ${label}`}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onClick={onSelect}
    >
      <title>{label}</title>
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={selected ? colorAlpha(color, '34') : hovered ? colorAlpha(color, '22') : colorAlpha(color, '08')}
        stroke={selected || hovered ? color : cfg.colors.panelStroke}
        strokeWidth={selected ? 1.25 : hovered ? 1 : 0.7}
        filter={selected || hovered ? 'url(#parentPortalGlow)' : undefined}
      />
      <text
        x={x + w / 2}
        y={y + 19}
        textAnchor="middle"
        fontSize={10.5}
        fontWeight={900}
        fill={selected ? cfg.colors.bodyText : cfg.colors.mutedText}
      >
        {truncateTextForWidth(label, w - 12, 10.5, 0.58)}
      </text>
    </g>
  );
}

function ManageModeButton({ x, y, w, h, item, selected, onSelect, cfg }) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(item.tone, cfg);
  const titleText = item.detail ? `${item.label}: ${item.detail}` : item.label;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={`Use ${item.label}`}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onClick={onSelect}
    >
      <title>{titleText}</title>
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={selected ? colorAlpha(color, '30') : hovered ? colorAlpha(color, '18') : colorAlpha(color, '08')}
        stroke={selected || hovered ? color : cfg.colors.panelStroke}
        strokeWidth={selected ? 1.25 : hovered ? 1 : 0.7}
        filter={selected || hovered ? 'url(#parentPortalGlow)' : undefined}
      />
      <text x={x + 10} y={y + h / 2 + 4} fontSize={12} fontWeight={950} fill={selected ? cfg.colors.bodyText : color}>
        {truncateTextForWidth(item.label, w - 20, 12, 0.58)}
      </text>
    </g>
  );
}

function ManageToggle({ x, y, w, h, option, selected, disabled = false, onToggle, cfg }) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(option.tone, cfg);
  return (
    <g
      className={disabled ? undefined : 'parent-portal-svg-clickable'}
      role="checkbox"
      tabIndex={disabled ? undefined : 0}
      aria-label={option.label}
      aria-checked={selected}
      aria-disabled={disabled || undefined}
      opacity={disabled ? 0.46 : 1}
      onMouseEnter={disabled ? undefined : () => setHovered(true)}
      onMouseLeave={disabled ? undefined : () => setHovered(false)}
      onClick={disabled ? undefined : onToggle}
    >
      <title>{option.detail ? `${option.label}: ${option.detail}` : option.label}</title>
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={selected ? colorAlpha(color, '22') : hovered ? colorAlpha(color, '14') : colorAlpha(color, '08')}
        stroke={selected || hovered ? color : cfg.colors.panelStroke}
        strokeWidth={selected ? 1.05 : hovered ? 0.95 : 0.7}
        filter={selected || hovered ? 'url(#parentPortalGlow)' : undefined}
      />
      <path
        d={cutRectPath(x + 8, y + 9, 24, 24, 5)}
        fill={selected ? colorAlpha(color, '4c') : 'transparent'}
        stroke={color}
        strokeWidth={1}
      />
      {selected ? (
        <path
          d={`M ${x + 14} ${y + 22} l 5 5 l 9 -13`}
          fill="none"
          stroke={cfg.colors.bodyText}
          strokeWidth={2}
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      ) : null}
      <text x={x + 42} y={y + 18} fontSize={10.5} fontWeight={930} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(option.label, w - 50, 10.5, 0.58)}
      </text>
    </g>
  );
}

function ManageActionButton({ x, y, w, h, action, onSelect, cfg }) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(action.tone, cfg);
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={action.label}
      onClick={onSelect}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      <title>{action.detail ? `${action.label}: ${action.detail}` : action.label}</title>
      {hovered ? (
        <path
          d={cutRectPath(x - 4, y - 4, w + 8, h + 8, 10)}
          fill="none"
          stroke={color}
          strokeWidth={1.4}
          opacity={0.5}
          filter="url(#parentPortalGlow)"
        />
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={hovered ? colorAlpha(color, '32') : colorAlpha(color, '18')}
        stroke={hovered ? color : cfg.colors.panelStroke}
        strokeWidth={hovered ? 1.2 : 0.8}
      />
      <text x={x + 14} y={y + h / 2 + 4} fontSize={11.5} fontWeight={950} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(action.label.toUpperCase(), w - 28, 11.5, 0.58)}
      </text>
    </g>
  );
}

function ParentPortalDetailPanel({
  x,
  y,
  w,
  h,
  activeNavLabel,
  activeNavGroupId,
  detail,
  rows,
  selectedControlName,
  themeTone,
  guideTopic,
  guidePage,
  onGuidePageChange,
  quickPanelMode,
  onQuickPanelModeChange,
  onGuideNoteSelect,
  manageTargetSelection,
  onNavigate,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  activeNavLabel: string;
  activeNavGroupId: string;
  detail: TabDetail;
  rows: DisplayRow[];
  selectedControlName: string;
  themeTone?: Tone;
  guideTopic?: ParentPortalGuideTopic | null;
  guidePage: number;
  onGuidePageChange: (page: number) => void;
  quickPanelMode: 'read' | 'action';
  onQuickPanelModeChange: (mode: 'read' | 'action') => void;
  onGuideNoteSelect: (note: ParentPortalGuideNote) => void;
  manageTargetSelection?: ManageTargetSelection;
  onNavigate?: (routePath: string) => void;
  cfg: ParentPortalSvgControls;
}) {
  if (guideTopic) {
    return (
      <GuideTopicDetailPanel
        x={x}
        y={y}
        w={w}
        h={h}
        topic={guideTopic}
        page={guidePage}
        onPageChange={onGuidePageChange}
        quickPanelMode={quickPanelMode}
        onQuickPanelModeChange={onQuickPanelModeChange}
        onNoteSelect={onGuideNoteSelect}
        cfg={cfg}
      />
    );
  }
  const manageSpec = activeNavGroupId === 'manage' ? manageControlSpecFor(activeNavLabel, selectedControlName) : null;
  if (manageSpec) {
    return (
      <ManageControlPanel
        x={x}
        y={y}
        w={w}
        h={h}
        activeNavLabel={activeNavLabel}
        selectedControlName={selectedControlName}
        spec={manageSpec}
        themeTone={themeTone ?? detail.tone}
        targetSelection={
          manageTargetSelection ?? {
            scope: manageScopeForLane(manageLaneForKey(activeNavLabel, selectedControlName)),
            device: manageSpec.devices[0] ?? 'Family default',
            browser: manageBrowserTargetsForKey(activeNavLabel, selectedControlName)[0]?.label ?? 'All targets',
          }
        }
        guideRoutePath={guideRoutePathForManageKey(activeNavLabel, manageSpec.title)}
        onNavigate={onNavigate}
        cfg={cfg}
      />
    );
  }
  const color = toneColor(detail.tone, cfg);
  const cardGap = 12;
  const usableH = Math.max(120, h);
  const title = activeNavLabel || detail.title;
  const bodyLines = wrapCardText(detail.summary, w - 40, 12, 2);
  const featureCards = [
    {
      label: 'WHAT PARENTS CONTROL',
      value: detail.primary,
      body: detail.secondary,
      tone: detail.tone,
    },
    {
      label: 'CURRENT AREA',
      value: selectedControlName,
      body: 'Open this area per child device, then wire real service state as each adapter lands.',
      tone: 'cyan',
    },
    {
      label: 'DATA CUSTODY',
      value: 'LOCAL FIRST',
      body: 'No cloud sharing by default. Drive exports and support bundles are parent opt-in.',
      tone: 'gold',
    },
    ...rows.slice(0, 3).map((row) => ({
      label: row.primaryArea.toUpperCase(),
      value: row.label,
      body: `${row.trend} / ${row.readiness}`,
      tone: row.tone,
    })),
  ];
  const visibleCards = featureCards.slice(0, usableH < 300 ? 3 : 6);
  const columnCount = w > 1220 ? 3 : w > 760 ? 2 : 1;
  const rowCount = Math.max(1, Math.ceil(visibleCards.length / columnCount));
  const headerH = bodyLines.length > 1 ? 78 : 62;
  const cardW = (w - cardGap * Math.max(0, columnCount - 1)) / columnCount;
  const cardH = clampValue((usableH - headerH - cardGap * Math.max(0, rowCount - 1)) / rowCount, 74, 118);
  const titleSize = fitSingleLineTextSize(title, w - 40, 16, 26, 0.58);
  return (
    <g>
      <text x={x} y={y + 24} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
        {title}
      </text>
      <path d={`M ${x} ${y + 39} H ${x + w}`} stroke={color} strokeWidth={1.1} opacity={0.5} />
      {bodyLines.map((line, index) => (
        <text
          key={`${line}:${index}`}
          x={x}
          y={y + 58 + index * 17}
          fontSize={12}
          fontWeight={760}
          fill={cfg.colors.mutedText}
        >
          {line}
        </text>
      ))}
      {visibleCards.map((card, index) => {
        const col = index % columnCount;
        const row = Math.floor(index / columnCount);
        const cardX = x + col * (cardW + cardGap);
        const cardY = y + headerH + row * (cardH + cardGap);
        const cardColor = toneColor(card.tone, cfg);
        const valueSize = fitSingleLineTextSize(card.value, cardW - 34, 12, 17, 0.58);
        const cardBodyLines = wrapCardText(card.body, cardW - 34, 10.5, cardH > 92 ? 2 : 1);
        return (
          <SurfacePanel
            key={`${card.label}:${index}`}
            x={cardX}
            y={cardY}
            w={cardW}
            h={cardH}
            tone={card.tone}
            cfg={cfg}
          >
            <text x={cardX + 16} y={cardY + 25} fontSize={9.8} fontWeight={900} fill={cardColor}>
              {card.label}
            </text>
            <text x={cardX + 16} y={cardY + 49} fontSize={valueSize} fontWeight={950} fill={cfg.colors.bodyText}>
              {truncateTextForWidth(card.value, cardW - 34, valueSize, 0.58)}
            </text>
            {cardBodyLines.map((line, lineIndex) => (
              <text
                key={`${line}:${lineIndex}`}
                x={cardX + 16}
                y={cardY + 70 + lineIndex * 15}
                fontSize={10.5}
                fontWeight={720}
                fill={cfg.colors.mutedText}
              >
                {line}
              </text>
            ))}
          </SurfacePanel>
        );
      })}
    </g>
  );
}

function GuideQuickTab({
  x,
  y,
  w,
  h,
  label,
  active,
  tone,
  onClick,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  active: boolean;
  tone: Tone;
  onClick: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const lit = active || hovered;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={`Show ${label}`}
      aria-pressed={active}
      onClick={(event) => {
        event.stopPropagation();
        onClick();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onClick();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <ClickableCardHoverChrome x={x} y={y} w={w} h={h} color={color} active={active} hovered={hovered} arrow={false} />
      <path
        d={cutRectPath(x, y, w, h, 7)}
        fill={lit ? colorAlpha(color, active ? '42' : '24') : 'rgba(5, 19, 32, 0.86)'}
        stroke={lit ? color : cfg.colors.panelStroke}
        strokeWidth={active ? 1.25 : 0.85}
      />
      <text
        x={x + w / 2}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={fitSingleLineTextSize(label, w - 12, 8.8, 10.8, 0.58)}
        fontWeight={950}
        fill={lit ? cfg.colors.bodyText : cfg.colors.mutedText}
      >
        {label}
      </text>
    </g>
  );
}

function GuideNoteCard({
  x,
  y,
  w,
  h,
  note,
  mode,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  note: ParentPortalGuideNote;
  mode: 'read' | 'action';
  onSelect: (note: ParentPortalGuideNote) => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const noteColor = toneColor(note.tone, cfg);
  const actionable =
    mode === 'action' ||
    Boolean(note.targetRoutePath || note.targetNavLabel || note.targetTopicId || typeof note.targetPage === 'number');
  const lit = actionable && (hovered || mode === 'action');
  const labelSize = fitSingleLineTextSize(note.label, w - 20, 9.5, 12.5, 0.58);
  const noteLines = wrapCardText(note.body, w - 24, 10.2, h > 66 ? 3 : 2);
  return (
    <g
      className={actionable ? 'parent-portal-svg-clickable' : undefined}
      role={actionable ? 'button' : undefined}
      tabIndex={actionable ? 0 : undefined}
      aria-label={actionable ? `Open ${note.label}` : undefined}
      onClick={(event) => {
        if (!actionable) return;
        event.stopPropagation();
        onSelect(note);
      }}
      onKeyDown={(event) => {
        if (!actionable || (event.key !== 'Enter' && event.key !== ' ')) return;
        event.preventDefault();
        event.stopPropagation();
        onSelect(note);
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      {lit ? (
        <>
          <path
            d={cutRectPath(x - 4, y - 4, w + 8, h + 8, 10)}
            fill="none"
            stroke={noteColor}
            strokeWidth={hovered ? 1.7 : 1.2}
            opacity={hovered ? 0.42 : 0.28}
            filter="url(#parentPortalGlow)"
          />
          {hovered ? (
            <path
              d={`M ${x + w - 10} ${y + 15} L ${x + w + 8} ${y + h / 2} L ${x + w - 10} ${y + h - 15} Z`}
              fill={noteColor}
              opacity={0.64}
              filter="url(#parentPortalGlow)"
            />
          ) : null}
        </>
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={lit ? colorAlpha(noteColor, hovered ? '2f' : '24') : colorAlpha(noteColor, '16')}
        stroke={lit ? noteColor : cfg.colors.panelStroke}
        strokeWidth={lit ? 1.16 : 0.78}
        strokeOpacity={lit ? 0.9 : 0.62}
      />
      <text x={x + 12} y={y + 19} fontSize={labelSize} fontWeight={950} fill={lit ? cfg.colors.bodyText : noteColor}>
        {truncateTextForWidth(note.label.toUpperCase(), w - (actionable ? 72 : 24), labelSize, 0.58)}
      </text>
      {actionable ? (
        <text x={x + w - 12} y={y + 19} textAnchor="end" fontSize={8.4} fontWeight={950} fill="#fff3b2">
          OPEN
        </text>
      ) : null}
      {noteLines.map((line, lineIndex) => (
        <text
          key={`${note.label}:note:${lineIndex}`}
          x={x + 12}
          y={y + 38 + lineIndex * 13}
          fontSize={10.2}
          fontWeight={720}
          fill={cfg.colors.mutedText}
        >
          {line}
        </text>
      ))}
    </g>
  );
}

function GuideTopicDetailPanel({
  x,
  y,
  w,
  h,
  topic,
  page,
  onPageChange,
  quickPanelMode,
  onQuickPanelModeChange,
  onNoteSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  topic: ParentPortalGuideTopic;
  page: number;
  onPageChange: (page: number) => void;
  quickPanelMode: 'read' | 'action';
  onQuickPanelModeChange: (mode: 'read' | 'action') => void;
  onNoteSelect: (note: ParentPortalGuideNote) => void;
  cfg: ParentPortalSvgControls;
}) {
  const color = toneColor(topic.tone, cfg);
  const pageCount = Math.max(1, topic.pages.length);
  const safePage = clampValue(page, 0, pageCount - 1);
  const currentPage = topic.pages[safePage] ??
    topic.pages[0] ?? {
      eyebrow: topic.category,
      title: topic.title,
      body: topic.detail,
      steps: [topic.subtitle],
    };
  const compact = w < 760;
  const gap = compact ? 10 : 14;
  const sideW = compact ? w : clampValue(w * 0.25, 235, 330);
  const mainW = compact ? w : Math.max(280, w - sideW - gap);
  const mainH = compact ? Math.max(210, h * 0.62) : h;
  const sideX = compact ? x : x + mainW + gap;
  const sideY = compact ? y + mainH + gap : y;
  const sideH = compact ? Math.max(160, h - mainH - gap) : h;
  const titleSize = fitSingleLineTextSize(topic.title, mainW - 36, 17, 25, 0.58);
  const subtitleLines = wrapCardText(topic.subtitle, mainW - 36, 12, 2);
  const bodyLines = wrapCardText(currentPage.body, mainW - 44, 12.3, compact ? 4 : 5);
  const stepStartY = y + 168 + bodyLines.length * 17;
  const stepGap = compact ? 42 : 46;
  const maxSteps = Math.max(2, Math.min(currentPage.steps.length, Math.floor((y + mainH - 44 - stepStartY) / stepGap)));
  const visibleSteps = currentPage.steps.slice(0, maxSteps);
  const quickNotes = quickPanelMode === 'action' ? topic.actions : topic.tips;
  const noteGap = 9;
  const noteHeaderH = 58;
  const noteH =
    quickNotes.length > 0 ? Math.max(54, Math.min(82, (sideH - noteHeaderH - 28) / quickNotes.length - 1)) : 56;
  const visibleNotes = quickNotes.slice(0, Math.max(1, Math.floor((sideH - noteHeaderH - 14) / (noteH + noteGap))));
  return (
    <g>
      <SurfacePanel x={x} y={y} w={mainW} h={mainH} tone={topic.tone} cfg={cfg}>
        <text x={x + 18} y={y + 28} fontSize={9.8} fontWeight={950} fill={color}>
          {currentPage.eyebrow}
        </text>
        <text x={x + 18} y={y + 58} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
          {truncateTextForWidth(topic.title, mainW - 36, titleSize, 0.58)}
        </text>
        {subtitleLines.map((line, index) => (
          <text
            key={`${topic.id}:subtitle:${index}`}
            x={x + 18}
            y={y + 79 + index * 16}
            fontSize={12}
            fontWeight={760}
            fill={cfg.colors.mutedText}
          >
            {line}
          </text>
        ))}
        <path d={`M ${x + 18} ${y + 104} H ${x + mainW - 18}`} stroke={color} strokeWidth={0.9} opacity={0.45} />
        <text x={x + 18} y={y + 127} fontSize={15} fontWeight={950} fill={cfg.colors.bodyText}>
          {truncateTextForWidth(currentPage.title, mainW - 36, 15, 0.58)}
        </text>
        {bodyLines.map((line, index) => (
          <text
            key={`${topic.id}:body:${index}`}
            x={x + 18}
            y={y + 151 + index * 17}
            fontSize={12.3}
            fontWeight={720}
            fill={cfg.colors.mutedText}
          >
            {line}
          </text>
        ))}
        {visibleSteps.map((step, index) => {
          const stepY = stepStartY + index * stepGap;
          const stepLines = wrapCardText(step, mainW - 76, 11.5, 2);
          return (
            <g key={`${topic.id}:step:${index}`}>
              <path
                d={cutRectPath(x + 18, stepY - 14, 32, 28, 7)}
                fill={colorAlpha(color, '2a')}
                stroke={color}
                strokeWidth={0.85}
              />
              <text x={x + 34} y={stepY + 5} textAnchor="middle" fontSize={11} fontWeight={950} fill={color}>
                {index + 1}
              </text>
              {stepLines.map((line, lineIndex) => (
                <text
                  key={`${topic.id}:step:${index}:${lineIndex}`}
                  x={x + 62}
                  y={stepY + lineIndex * 15}
                  fontSize={11.5}
                  fontWeight={760}
                  fill={lineIndex === 0 ? cfg.colors.bodyText : cfg.colors.mutedText}
                >
                  {line}
                </text>
              ))}
            </g>
          );
        })}
        {pageCount > 1 ? (
          <g>
            {topic.pages.map((item, index) => {
              const pillW = 30;
              const pillX = x + mainW - 18 - (pageCount - index) * (pillW + 6);
              const selected = index === safePage;
              return (
                <g
                  key={`${topic.id}:page:${item.title}`}
                  className="parent-portal-svg-clickable"
                  role="button"
                  tabIndex={0}
                  aria-label={`Show guide page ${index + 1}`}
                  aria-pressed={selected}
                  onClick={(event) => {
                    event.stopPropagation();
                    onPageChange(index);
                  }}
                  onKeyDown={(event) => {
                    if (event.key !== 'Enter' && event.key !== ' ') return;
                    event.preventDefault();
                    event.stopPropagation();
                    onPageChange(index);
                  }}
                >
                  <path
                    d={cutRectPath(pillX, y + mainH - 30, pillW, 18, 5)}
                    fill={selected ? colorAlpha(color, '44') : 'rgba(3, 12, 22, 0.82)'}
                    stroke={selected ? '#ffe187' : color}
                    strokeWidth={selected ? 1.2 : 0.75}
                  />
                  <text
                    x={pillX + pillW / 2}
                    y={y + mainH - 17}
                    textAnchor="middle"
                    fontSize={9}
                    fontWeight={950}
                    fill={selected ? '#fff3b2' : cfg.colors.mutedText}
                  >
                    {index + 1}
                  </text>
                </g>
              );
            })}
          </g>
        ) : null}
      </SurfacePanel>
      <SurfacePanel x={sideX} y={sideY} w={sideW} h={sideH} tone={topic.tone} cfg={cfg}>
        <GuideQuickTab
          x={sideX + 14}
          y={sideY + 14}
          w={(sideW - 34) / 2}
          h={28}
          label="QUICK READ"
          active={quickPanelMode === 'read'}
          tone={topic.tone}
          onClick={() => onQuickPanelModeChange('read')}
          cfg={cfg}
        />
        <GuideQuickTab
          x={sideX + 20 + (sideW - 34) / 2}
          y={sideY + 14}
          w={(sideW - 34) / 2}
          h={28}
          label="QUICK ACTION"
          active={quickPanelMode === 'action'}
          tone="gold"
          onClick={() => onQuickPanelModeChange('action')}
          cfg={cfg}
        />
        <path
          d={`M ${sideX + 16} ${sideY + 50} H ${sideX + sideW - 16}`}
          stroke={color}
          strokeWidth={0.85}
          opacity={0.42}
        />
        {visibleNotes.map((note, index) => (
          <GuideNoteCard
            key={`${topic.id}:note:${quickPanelMode}:${note.label}:${index}`}
            x={sideX + 14}
            y={sideY + noteHeaderH + index * (noteH + noteGap)}
            w={sideW - 28}
            h={noteH}
            note={note}
            mode={quickPanelMode}
            onSelect={onNoteSelect}
            cfg={cfg}
          />
        ))}
      </SurfacePanel>
    </g>
  );
}

function GuideOverviewDashboard({
  x,
  y,
  w,
  h,
  topics,
  selectedTopicId,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  topics: ParentPortalGuideTopic[];
  selectedTopicId: string;
  onSelect: (topic: ParentPortalGuideTopic) => void;
  cfg: ParentPortalSvgControls;
}) {
  const setupTopic = topics.find((topic) => normalizeSelectionId(topic.id) === 'setup-overall') ?? topics[0];
  const cards = topics.filter((topic) => topic !== setupTopic);
  const columns = w > 1220 ? 4 : w > 900 ? 3 : w > 340 ? 2 : 1;
  const denseTopicMap = columns === 1 || cards.length > 10;
  const introSplit = w > 720;
  const introH = h > 390 ? (denseTopicMap ? 78 : 92) : denseTopicMap ? 58 : 70;
  const gap = denseTopicMap ? 7 : 12;
  const rows = Math.max(1, Math.ceil(cards.length / columns));
  const cardW = (w - gap * Math.max(0, columns - 1)) / columns;
  const rawCardH = (h - introH - gap * Math.max(0, rows - 1)) / rows;
  const cardH = clampValue(rawCardH, columns === 1 ? 34 : denseTopicMap ? 58 : 84, denseTopicMap ? 104 : 130);
  const setupColor = setupTopic ? toneColor(setupTopic.tone, cfg) : cfg.colors.cyan;
  const introTitleSize = fitSingleLineTextSize(setupTopic?.title ?? '', introSplit ? w * 0.42 : w - 36, 15, 20, 0.58);
  const [hoveredTopicId, setHoveredTopicId] = useState<string | null>(null);
  return (
    <g>
      {setupTopic ? (
        <g
          className="parent-portal-svg-clickable"
          role="button"
          tabIndex={0}
          aria-label={`Open ${setupTopic.title}`}
          onClick={(event) => {
            event.stopPropagation();
            onSelect(setupTopic);
          }}
          onKeyDown={(event) => {
            if (event.key !== 'Enter' && event.key !== ' ') return;
            event.preventDefault();
            event.stopPropagation();
            onSelect(setupTopic);
          }}
          onMouseEnter={() => setHoveredTopicId(setupTopic.id)}
          onMouseLeave={() => setHoveredTopicId(null)}
          onFocus={() => setHoveredTopicId(setupTopic.id)}
          onBlur={() => setHoveredTopicId(null)}
        >
          <ClickableCardHoverChrome
            x={x}
            y={y}
            w={w}
            h={introH - 10}
            color={setupColor}
            active={normalizeSelectionId(setupTopic.id) === normalizeSelectionId(selectedTopicId)}
            hovered={hoveredTopicId === setupTopic.id}
          />
          <path
            d={cutRectPath(x, y, w, introH - 10, 12)}
            fill={colorAlpha(setupColor, '1f')}
            stroke={setupColor}
            strokeWidth={1.1}
            strokeOpacity={0.72}
            filter="url(#parentPortalGlow)"
          />
          <text x={x + 18} y={y + 28} fontSize={10} fontWeight={950} fill={setupColor}>
            FIRST SETUP
          </text>
          <text
            x={x + 18}
            y={y + (introH < 68 ? 50 : 56)}
            fontSize={introTitleSize}
            fontWeight={950}
            fill={cfg.colors.bodyText}
          >
            {truncateTextForWidth(setupTopic.title, introSplit ? w * 0.42 : w - 36, introTitleSize, 0.58)}
          </text>
          {introSplit ? (
            <>
              <text
                x={x + Math.max(300, w * 0.38)}
                y={y + 35}
                fontSize={12.4}
                fontWeight={760}
                fill={cfg.colors.mutedText}
              >
                {truncateTextForWidth(setupTopic.subtitle, Math.max(220, w * 0.5), 12.4, 0.58)}
              </text>
              <text
                x={x + Math.max(300, w * 0.38)}
                y={y + 58}
                fontSize={11.6}
                fontWeight={720}
                fill={cfg.colors.mutedText}
              >
                {truncateTextForWidth(setupTopic.detail, Math.max(220, w * 0.5), 11.6, 0.58)}
              </text>
            </>
          ) : null}
        </g>
      ) : null}
      {cards.map((topic, index) => {
        const col = index % columns;
        const row = Math.floor(index / columns);
        const cardX = x + col * (cardW + gap);
        const cardY = y + introH + row * (cardH + gap);
        const color = toneColor(topic.tone, cfg);
        const selected = normalizeSelectionId(topic.id) === normalizeSelectionId(selectedTopicId);
        const compactCard = cardH < 70 || cardW < 220;
        const denseCard = cardH < 58;
        const badgeY = denseCard ? cardY + 5 : cardY + 14;
        const badgeH = denseCard ? 24 : 28;
        const titleSize = fitSingleLineTextSize(
          topic.title,
          cardW - 62,
          compactCard ? 9.8 : 12,
          compactCard ? 13.5 : 16,
          0.58
        );
        const subtitleLines = compactCard ? [] : wrapCardText(topic.subtitle, cardW - 34, 10.6, cardH > 105 ? 2 : 1);
        const detailSize = compactCard ? 8.8 : 9.6;
        const titleY = denseCard ? cardY + cardH / 2 + 4 : cardY + (compactCard ? 31 : 32);
        return (
          <g
            key={topic.id}
            className="parent-portal-svg-clickable"
            role="button"
            tabIndex={0}
            aria-label={`Open ${topic.title}`}
            aria-pressed={selected}
            onClick={(event) => {
              event.stopPropagation();
              onSelect(topic);
            }}
            onKeyDown={(event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              event.stopPropagation();
              onSelect(topic);
            }}
            onMouseEnter={() => setHoveredTopicId(topic.id)}
            onMouseLeave={() => setHoveredTopicId(null)}
            onFocus={() => setHoveredTopicId(topic.id)}
            onBlur={() => setHoveredTopicId(null)}
          >
            <ClickableCardHoverChrome
              x={cardX}
              y={cardY}
              w={cardW}
              h={cardH}
              color={color}
              active={selected}
              hovered={hoveredTopicId === topic.id}
            />
            <path
              d={cutRectPath(cardX, cardY, cardW, cardH, 10)}
              fill={selected ? colorAlpha(color, '2e') : 'rgba(5, 18, 31, 0.86)'}
              stroke={selected ? '#ffe187' : color}
              strokeWidth={selected ? 1.7 : 0.95}
              strokeOpacity={selected ? 0.92 : 0.62}
              filter={selected ? 'url(#parentPortalGoldGlow)' : undefined}
            />
            <path
              d={cutRectPath(cardX + 12, badgeY, 34, badgeH, 7)}
              fill={colorAlpha(color, '2a')}
              stroke={color}
              strokeWidth={0.8}
            />
            <text
              x={cardX + 29}
              y={badgeY + badgeH / 2 + 4}
              textAnchor="middle"
              fontSize={denseCard ? 10 : 11}
              fontWeight={950}
              fill={color}
            >
              {topic.rank}
            </text>
            <text x={cardX + 56} y={titleY} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
              {truncateTextForWidth(topic.title, cardW - 70, titleSize, 0.58)}
            </text>
            {subtitleLines.map((line, lineIndex) => (
              <text
                key={`${topic.id}:dashboard-subtitle:${lineIndex}`}
                x={cardX + 16}
                y={cardY + 62 + lineIndex * 15}
                fontSize={10.6}
                fontWeight={720}
                fill={cfg.colors.mutedText}
              >
                {line}
              </text>
            ))}
            {!compactCard ? (
              <path
                d={`M ${cardX + 16} ${cardY + cardH - 25} H ${cardX + cardW - 16}`}
                stroke={color}
                strokeWidth={0.8}
                opacity={0.36}
              />
            ) : null}
            {!denseCard ? (
              <text
                x={cardX + 16}
                y={cardY + cardH - (compactCard ? 8 : 10)}
                fontSize={detailSize}
                fontWeight={900}
                fill={color}
              >
                {truncateTextForWidth(topic.detail.toUpperCase(), cardW - 32, detailSize, 0.58)}
              </text>
            ) : null}
          </g>
        );
      })}
    </g>
  );
}

function wrapCardText(text: string, width: number, fontSize: number, maxLines: number): string[] {
  const words = text.trim().split(/\s+/).filter(Boolean);
  const lines: string[] = [];
  let current = '';
  for (const word of words) {
    const next = current ? `${current} ${word}` : word;
    if (next.length * fontSize * 0.55 <= width || !current) {
      current = next;
      continue;
    }
    lines.push(current);
    current = word;
    if (lines.length >= maxLines) break;
  }
  if (current && lines.length < maxLines) lines.push(current);
  return lines.map((line, index) =>
    index === maxLines - 1 ? truncateTextForWidth(line, width, fontSize, 0.55) : line
  );
}

const PARENT_PORTAL_TOP_CARD_MIN_W = 300;
const PARENT_PORTAL_CONTROL_CARD_MIN_W = 245;

function rowFrameTone(): 'gold' | 'silver' | 'bronze' | 'blue' | 'red' {
  return 'blue';
}

function ParentPortalPictureViewerFrameLines({
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
              key={`deck-preview-frame-glow-${segment.id}`}
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
              key={`deck-preview-frame-outline-${segment.id}`}
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
          key={`deck-preview-frame-${segment.id}`}
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

const SIDE_PANEL_CORNER_SEGMENT_IDS = [
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

function scaleSidePanelCornerSegments(frame: PictureViewerFrameControls, scale: number): PictureViewerFrameControls {
  if (scale === 1) {
    return frame;
  }
  const segmentThicknesses = { ...frame.segmentThicknesses };
  for (const segmentId of SIDE_PANEL_CORNER_SEGMENT_IDS) {
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

function ParentPortalSidePanelFrame({
  x,
  y,
  w,
  h,
  tone,
  active,
  cornerThicknessScale = 1,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  tone: Tone;
  active: boolean;
  cornerThicknessScale?: number;
  cfg: ParentPortalSvgControls;
}) {
  const color = toneColor(tone, cfg);
  const tall = h > w * 1.08;
  const baseViewBoxW = tall ? 1200 : 1600;
  const frameScale = Math.max(0.01, w / baseViewBoxW);
  const minViewBoxH = tall ? 1200 : 420;
  const viewBox = useMemo(
    () => ({ w: baseViewBoxW, h: Math.max(minViewBoxH, h / frameScale) }),
    [baseViewBoxW, frameScale, h, minViewBoxH]
  );
  const frameControls = useMemo(() => {
    const base = normalizePictureViewerFrameControls({
      orientation: tall ? 'portrait' : 'landscape',
      viewBox,
      frameGroup: { inset: 2, offsetX: 0, offsetY: 0 },
      outerAnchor: { sideInset: 24, topInset: 24, bottomInset: 24 },
      innerAnchor: { sideInset: 54, topInset: 58, bottomInset: 58 },
    });
    return {
      ...base,
      navArrows: {
        ...base.navArrows,
        enabled: false,
      },
      outerFrame: scaleSidePanelCornerSegments(
        {
          ...base.outerFrame,
          color,
          glowColor: color,
          glowEnabled: true,
          glowOpacity: active ? 0.34 : 0.2,
          glowBlur: active ? 18 : 11,
          glowWidthBoost: active ? 7 : 4,
          outlineOpacity: 1,
          outlineWidthBoost: 2,
          topRise: 0,
          cornerCut: tall ? 86 : 62,
          topStepWidth: tall ? 420 : 520,
          topStepInset: 0,
          bottomTabWidth: tall ? 420 : 520,
          bottomTabDepth: 0,
          bottomTabInset: 0,
          bottomTabDirection: 'down' as const,
          opacity: 1,
        },
        cornerThicknessScale
      ),
      innerFrame: scaleSidePanelCornerSegments(
        {
          ...base.innerFrame,
          color,
          glowColor: color,
          glowEnabled: active,
          glowOpacity: active ? 0.2 : 0,
          glowBlur: 8,
          glowWidthBoost: 4,
          outlineOpacity: 0.62,
          outlineWidthBoost: 1.4,
          topRise: 0,
          cornerCut: tall ? 64 : 42,
          bottomTabDepth: 0,
          bottomTabWidth: tall ? 340 : 420,
          opacity: active ? 0.88 : 0.58,
        },
        cornerThicknessScale
      ),
    };
  }, [active, color, cornerThicknessScale, tall, viewBox]);
  const rawId = useId().replace(/[^a-zA-Z0-9_-]/g, '');
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
  const outerGlowId = `parentPortalSidePanelFrameOuterGlow-${rawId}`;
  const innerGlowId = `parentPortalSidePanelFrameInnerGlow-${rawId}`;

  return (
    <g transform={`translate(${x} ${y}) scale(${frameScale})`} pointerEvents="none">
      <defs>
        <filter id={outerGlowId} x="-40%" y="-40%" width="180%" height="180%">
          <feGaussianBlur stdDeviation={outerFrame.glowBlur} />
        </filter>
        <filter id={innerGlowId} x="-40%" y="-40%" width="180%" height="180%">
          <feGaussianBlur stdDeviation={innerFrame.glowBlur} />
        </filter>
      </defs>
      <g transform={getPictureViewerFrameGroupTransform(frameControls)}>
        <g transform={getPictureViewerFrameTransform(frameControls)}>
          <ParentPortalPictureViewerFrameLines frame={outerFrame} segments={outerSegments} filterId={outerGlowId} />
          <ParentPortalPictureViewerFrameLines frame={innerFrame} segments={innerSegments} filterId={innerGlowId} />
        </g>
      </g>
    </g>
  );
}

function ParentPortalTopCarouselCard({
  item,
  x,
  y,
  w,
  h,
  selected,
  onSelect,
  onHoverChange,
  controlHoverAnchor = 'center',
  cfg,
}: {
  item: ParentPortalTopCardItem;
  x: number;
  y: number;
  w: number;
  h: number;
  selected: boolean;
  onSelect: () => void;
  onHoverChange?: (item: ParentPortalTopCardItem | null) => void;
  controlHoverAnchor?: 'center' | 'up';
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const rawControlClipId = useId();
  const controlClipId = `parent-control-card-${rawControlClipId.replace(/:/g, '')}`;
  const color = toneColor(item.tone, cfg);
  const active = selected || hovered;
  const rowFrameHref = useMemo(() => {
    if (item.kind !== 'row') return '';
    const frameConfig = createGoldenFrameVariantConfig({
      rank: String(item.row.order),
      name: item.row.label,
      statName: 'Global',
      statValue: item.row.signal,
      tone: rowFrameTone(),
    });
    return createGoldenFrameFrameOnlySvgDataUri(frameConfig);
  }, [item]);
  const compactControlCard = item.kind === 'control' && !hovered;
  const controlW = item.kind === 'control' ? (hovered ? Math.min(w + 28, w * 1.08) : w) : w;
  const controlH =
    item.kind === 'control' ? (hovered ? clampValue(h * 1.82, 86, 112) : clampValue(h * 0.62, 38, 44)) : h;
  const controlX = x + (w - controlW) / 2;
  const controlY =
    item.kind === 'control' && hovered && controlHoverAnchor === 'up' ? y + h - controlH - 4 : y + (h - controlH) / 2;
  const controlPad = Math.max(10, Math.min(14, controlW * 0.045));
  const controlBannerX = controlX + controlPad;
  const controlBannerY = controlY + 18;
  const controlBannerW = controlW - controlPad * 2;
  const controlBannerH = Math.max(78, Math.min(controlH * 0.68, controlH - 58));
  const controlBodyY = controlBannerY + controlBannerH + 18;
  const controlImageUrl = item.kind === 'control' ? parentPortalControlArtworkUrl(item.control) : null;
  const controlCategoryText = item.kind === 'control' ? controlSubcategoryLabel(item.control) : '';
  const controlTitleW = Math.max(58, controlW - controlPad * 2 - 4);
  const controlTitleSize = fitSingleLineTextSize(item.title, controlTitleW, 13.5, 20, 0.56);
  const controlTitleBaseline = Math.min(controlY + controlH - 16, controlBodyY + controlTitleSize * 0.42);
  const controlCategorySize = fitSingleLineTextSize(controlCategoryText, controlBannerW - 20, 7.5, 10.5, 0.56);
  const controlTitleText = truncateTextForWidth(item.title, controlTitleW, controlTitleSize, 0.56);
  const controlCategoryDisplayText = truncateTextForWidth(
    controlCategoryText,
    controlBannerW - 34,
    controlCategorySize,
    0.56
  );
  const compactControlImageSize = Math.min(31, Math.max(23, controlH - 16));
  const compactControlImageX = controlX + 10;
  const compactControlImageY = controlY + (controlH - compactControlImageSize) / 2;
  const compactControlChipValue = item.kind === 'control' ? compactControlStatLabel(item.value) : '';
  const compactControlChipText = /\d|%/.test(compactControlChipValue) ? compactControlChipValue : '';
  const compactControlChipW = compactControlChipText
    ? Math.max(38, Math.min(54, compactControlChipText.length * 6.2 + 20))
    : 0;
  const compactControlTitleX = compactControlImageX + compactControlImageSize + 14;
  const compactControlTitleW = Math.max(46, controlX + controlW - compactControlTitleX - compactControlChipW - 18);
  const compactControlTitleSize = fitSingleLineTextSize(item.title, compactControlTitleW, 11.8, 14.2, 0.58);
  const rowFrameScale = Math.min(w / 1536, h / 864);
  const rowFrameW = 1536 * rowFrameScale;
  const rowFrameH = 864 * rowFrameScale;
  const rowFrameX = x + (w - rowFrameW) / 2;
  const rowFrameY = y + (h - rowFrameH) / 2;
  const rowHoverScale = hovered ? 1.075 : selected ? 1.012 : 1;
  const rowScaleCx = x + w / 2;
  const rowScaleCy = y + h / 2;
  const scaleFromCenter = (value: number, center: number) => center + (value - center) * rowHoverScale;
  const rowDrawFrameX = scaleFromCenter(rowFrameX, rowScaleCx);
  const rowDrawFrameY = scaleFromCenter(rowFrameY, rowScaleCy);
  const rowDrawFrameW = rowFrameW * rowHoverScale;
  const rowDrawFrameH = rowFrameH * rowHoverScale;
  const rowHoverBoxX = rowDrawFrameX + rowDrawFrameW * 0.035;
  const rowHoverBoxY = rowDrawFrameY + rowDrawFrameH * 0.13;
  const rowHoverBoxW = rowDrawFrameW * 0.93;
  const rowHoverBoxH = rowDrawFrameH * 0.72;
  const guidePad = Math.max(12, Math.min(18, w * 0.045));
  const guideRankW = 42;
  const guideTitleX = x + guidePad + guideRankW + 10;
  const guideTitleW = Math.max(60, w - guidePad * 2 - guideRankW - 12);
  const guideTitleSize = fitSingleLineTextSize(item.title, guideTitleW, 13, 18, 0.56);
  const guideSubtitleLines = wrapCardText(item.subtitle, guideTitleW, 10.6, 2);
  const guideDetailLines = wrapCardText(item.detail, w - guidePad * 2, 10.2, h > 102 ? 2 : 1);
  const rowHoverStrokeWidth = hovered ? 2.4 : selected ? 1.35 : 1.5;
  const rowHoverOuterOpacity = hovered ? 0.72 : selected ? 0.34 : 0.38;
  const rowHoverInnerOpacity = hovered ? 0.82 : selected ? 0.48 : 0.52;
  const rowHoverFill = hovered
    ? colorAlpha(color, '2c')
    : selected
      ? 'rgba(255, 210, 59, 0.10)'
      : colorAlpha(color, '16');
  const hitX = item.kind === 'control' ? Math.min(x - 4, controlX - 8) : x - 4;
  const hitY = item.kind === 'control' ? Math.min(y - 4, controlY - 8) : y - 4;
  const hitW = item.kind === 'control' ? Math.max(x + w + 4, controlX + controlW + 8) - hitX : w + 8;
  const hitH = item.kind === 'control' ? Math.max(y + h + 4, controlY + controlH + 8) - hitY : h + 8;
  const showHoverState = () => {
    setHovered(true);
    if (item.kind === 'control') onHoverChange?.(item);
  };
  const clearHoverState = () => {
    setHovered(false);
    if (item.kind === 'control') onHoverChange?.(null);
  };
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={
        item.kind === 'control'
          ? `Show ${item.title} controls`
          : item.kind === 'guide'
            ? `Show ${item.title} guide`
            : `Show ${item.title} control row`
      }
      aria-pressed={selected}
      onClick={(event) => {
        event.stopPropagation();
        onSelect();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={showHoverState}
      onMouseOver={showHoverState}
      onMouseMove={showHoverState}
      onPointerEnter={showHoverState}
      onPointerMove={showHoverState}
      onMouseLeave={clearHoverState}
      onPointerLeave={clearHoverState}
    >
      <rect x={hitX} y={hitY} width={hitW} height={hitH} fill="transparent" pointerEvents="all" />
      {item.kind === 'control' && active && !compactControlCard ? (
        <path
          d={cutRectPath(x, y, w, h, 14)}
          fill={colorAlpha(color, selected ? '24' : '12')}
          stroke={selected ? '#ffe187' : color}
          strokeWidth={selected ? 2.1 : 1.4}
          opacity={selected ? 0.82 : 0.5}
          filter={selected ? 'url(#parentPortalGoldGlow)' : 'url(#parentPortalGlow)'}
          pointerEvents="none"
        />
      ) : null}
      {item.kind === 'row' ? (
        <>
          {active ? (
            <>
              <path
                d={cutRectPath(rowHoverBoxX - 8, rowHoverBoxY - 8, rowHoverBoxW + 16, rowHoverBoxH + 16, 13)}
                fill="none"
                stroke={selected ? '#ffe187' : color}
                strokeWidth={rowHoverStrokeWidth}
                opacity={rowHoverOuterOpacity}
                filter={selected ? 'url(#parentPortalGoldGlow)' : 'url(#parentPortalGlow)'}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(rowHoverBoxX, rowHoverBoxY, rowHoverBoxW, rowHoverBoxH, 10)}
                fill={rowHoverFill}
                stroke={selected ? '#ffe187' : color}
                strokeWidth={hovered ? 1.6 : 1.1}
                strokeOpacity={rowHoverInnerOpacity}
                pointerEvents="none"
              />
            </>
          ) : null}
          <image
            href={rowFrameHref}
            xlinkHref={rowFrameHref}
            x={rowDrawFrameX}
            y={rowDrawFrameY}
            width={rowDrawFrameW}
            height={rowDrawFrameH}
            preserveAspectRatio="xMidYMid meet"
            filter={active ? (selected ? 'url(#parentPortalGoldGlow)' : 'url(#parentPortalGlow)') : undefined}
            pointerEvents="none"
          />
          <text
            x={rowDrawFrameX + rowDrawFrameW * 0.13}
            y={rowDrawFrameY + rowDrawFrameH * 0.36}
            fontSize={Math.max(12, rowDrawFrameH * 0.1)}
            fontWeight={950}
            fill={cfg.colors.bodyText}
            pointerEvents="none"
          >
            {item.title}
          </text>
          <text
            x={rowDrawFrameX + rowDrawFrameW * 0.13}
            y={rowDrawFrameY + rowDrawFrameH * 0.52}
            fontSize={Math.max(8, rowDrawFrameH * 0.064)}
            fontWeight={820}
            fill={cfg.colors.mutedText}
            pointerEvents="none"
          >
            {item.row.primaryArea}
          </text>
          <text
            x={rowDrawFrameX + rowDrawFrameW * 0.72}
            y={rowDrawFrameY + rowDrawFrameH * 0.42}
            textAnchor="middle"
            fontSize={Math.max(11, rowDrawFrameH * 0.082)}
            fontWeight={950}
            fill={color}
            pointerEvents="none"
          >
            {item.row.trend}
          </text>
          <text
            x={rowDrawFrameX + rowDrawFrameW * 0.72}
            y={rowDrawFrameY + rowDrawFrameH * 0.56}
            textAnchor="middle"
            fontSize={Math.max(8, rowDrawFrameH * 0.056)}
            fontWeight={760}
            fill={cfg.colors.mutedText}
            pointerEvents="none"
          >
            {item.detail}
          </text>
        </>
      ) : item.kind === 'guide' ? (
        <>
          <path
            d={cutRectPath(x, y, w, h, 12)}
            fill={active ? colorAlpha(color, selected ? '2e' : '20') : 'rgba(5, 17, 30, 0.88)'}
            stroke={selected ? '#ffe187' : color}
            strokeWidth={selected ? 2 : hovered ? 1.55 : 1.05}
            strokeOpacity={selected ? 0.92 : hovered ? 0.8 : 0.56}
            filter={selected ? 'url(#parentPortalGoldGlow)' : hovered ? 'url(#parentPortalGlow)' : undefined}
            pointerEvents="none"
          />
          <path
            d={cutRectPath(x + 5, y + 5, w - 10, h - 10, 10)}
            fill="url(#parentPortalFrameGlass)"
            stroke={color}
            strokeWidth={0.75}
            strokeOpacity={active ? 0.48 : 0.26}
            pointerEvents="none"
          />
          <path
            d={bottomCutRectPath(x + w * 0.34, y - 5, w * 0.32, 12, 5)}
            fill={color}
            fillOpacity={active ? 0.42 : 0.24}
            stroke={selected ? '#ffe187' : color}
            strokeWidth={1}
            filter="url(#parentPortalGlow)"
            pointerEvents="none"
          />
          <path
            d={cutRectPath(x + guidePad, y + 17, guideRankW, 30, 7)}
            fill={colorAlpha(color, selected ? '44' : '28')}
            stroke={selected ? '#ffe187' : color}
            strokeWidth={selected ? 1.2 : 0.8}
            pointerEvents="none"
          />
          <text
            x={x + guidePad + guideRankW / 2}
            y={y + 37}
            textAnchor="middle"
            fontSize={13}
            fontWeight={950}
            fill={selected ? '#fff3b2' : color}
            pointerEvents="none"
          >
            {item.topic.rank}
          </text>
          <text
            x={guideTitleX}
            y={y + 32}
            fontSize={guideTitleSize}
            fontWeight={950}
            fill={cfg.colors.bodyText}
            pointerEvents="none"
          >
            {truncateTextForWidth(item.title, guideTitleW, guideTitleSize, 0.56)}
          </text>
          {guideSubtitleLines.map((line, index) => (
            <text
              key={`${item.key}:subtitle:${index}`}
              x={guideTitleX}
              y={y + 51 + index * 14}
              fontSize={10.5}
              fontWeight={760}
              fill={cfg.colors.mutedText}
              pointerEvents="none"
            >
              {line}
            </text>
          ))}
          <path
            d={`M ${x + guidePad} ${y + h - 42} H ${x + w - guidePad}`}
            stroke={color}
            strokeWidth={0.8}
            opacity={active ? 0.5 : 0.28}
            pointerEvents="none"
          />
          {guideDetailLines.map((line, index) => (
            <text
              key={`${item.key}:detail:${index}`}
              x={x + guidePad}
              y={y + h - 25 + index * 13}
              fontSize={10.2}
              fontWeight={800}
              fill={index === 0 ? color : cfg.colors.mutedText}
              pointerEvents="none"
            >
              {line}
            </text>
          ))}
        </>
      ) : (
        <>
          {compactControlCard ? (
            <>
              <path
                d={cutRectPath(controlX, controlY, controlW, controlH, 8)}
                fill={selected ? colorAlpha(color, '30') : 'rgba(4, 16, 28, 0.82)'}
                stroke={selected ? '#ffe187' : color}
                strokeWidth={selected ? 1.65 : 0.95}
                strokeOpacity={selected ? 0.96 : 0.7}
                filter={selected ? 'url(#parentPortalGoldGlow)' : undefined}
                pointerEvents="none"
              />
              <ArtworkSlot
                x={compactControlImageX}
                y={compactControlImageY}
                w={compactControlImageSize}
                h={compactControlImageSize}
                label={`${item.title} image`}
                imageUrl={controlImageUrl}
                tone={item.tone}
                compact
                shape="rect"
                imageFit="slice"
                cfg={cfg}
              />
              <line
                x1={compactControlImageX + compactControlImageSize + 7}
                y1={controlY + 8}
                x2={compactControlImageX + compactControlImageSize + 7}
                y2={controlY + controlH - 8}
                stroke={color}
                strokeWidth={0.85}
                opacity={selected ? 0.64 : 0.42}
              />
              <text
                x={compactControlTitleX}
                y={controlY + controlH / 2 + compactControlTitleSize * 0.34}
                fontSize={compactControlTitleSize}
                fontWeight={950}
                fill={cfg.colors.bodyText}
              >
                {truncateTextForWidth(item.title, compactControlTitleW, compactControlTitleSize, 0.58)}
              </text>
              {compactControlChipText ? (
                <>
                  <path
                    d={cutRectPath(
                      controlX + controlW - compactControlChipW - 12,
                      controlY + (controlH - 24) / 2,
                      compactControlChipW,
                      24,
                      5
                    )}
                    fill={colorAlpha(color, selected ? '34' : '18')}
                    stroke={color}
                    strokeWidth={0.8}
                    strokeOpacity={0.72}
                    pointerEvents="none"
                  />
                  <text
                    x={controlX + controlW - 12 - compactControlChipW / 2}
                    y={controlY + controlH / 2 + 4}
                    textAnchor="middle"
                    fontSize={10.2}
                    fontWeight={950}
                    fill={color}
                  >
                    {compactControlChipText}
                  </text>
                </>
              ) : null}
            </>
          ) : (
            <>
              {active ? (
                <path
                  d={cutRectPath(controlX - 5, controlY - 5, controlW + 10, controlH + 10, 16)}
                  fill="none"
                  stroke={selected ? '#ffe187' : color}
                  strokeWidth={hovered ? 2.3 : 1.6}
                  opacity={hovered ? 0.48 : 0.34}
                  filter={selected ? 'url(#parentPortalGoldGlow)' : 'url(#parentPortalGlow)'}
                  pointerEvents="none"
                />
              ) : null}
              <path
                d={cutRectPath(controlX, controlY, controlW, controlH, 15)}
                fill={active ? colorAlpha(color, selected ? '24' : '18') : 'rgba(6, 18, 31, 0.95)'}
                stroke={active ? (selected ? '#ffe187' : color) : cfg.colors.panelStroke}
                strokeWidth={selected ? 2 : hovered ? 1.65 : 1.05}
                strokeOpacity={active ? 0.94 : 0.68}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(controlX + 4, controlY + 4, controlW - 8, controlH - 8, 13)}
                fill="url(#parentPortalFrameGlass)"
                stroke={color}
                strokeWidth={active ? 1.1 : 0.7}
                opacity={active ? 0.56 : 0.36}
                pointerEvents="none"
              />
              <path
                d={bottomCutRectPath(controlX + controlW * 0.32, controlY - 6, controlW * 0.36, 13, 5)}
                fill="#8ceeff"
                fillOpacity={0.34}
                stroke="#9eefff"
                strokeWidth={1.1}
                filter="url(#parentPortalGlow)"
                pointerEvents="none"
              />
              <path
                d={bottomCutRectPath(controlX + controlW * 0.34, controlY - 3, controlW * 0.32, 7, 3)}
                fill="#d9fbff"
                fillOpacity={0.48}
                pointerEvents="none"
              />
              <defs>
                <clipPath id={controlClipId}>
                  <path d={cutRectPath(controlBannerX, controlBannerY, controlBannerW, controlBannerH, 11)} />
                </clipPath>
              </defs>
              {controlImageUrl ? (
                <image
                  href={controlImageUrl}
                  x={controlBannerX}
                  y={controlBannerY}
                  width={controlBannerW}
                  height={controlBannerH}
                  preserveAspectRatio="xMidYMid slice"
                  clipPath={`url(#${controlClipId})`}
                  opacity={0.96}
                  pointerEvents="none"
                />
              ) : (
                <ArtworkSlot
                  x={controlBannerX}
                  y={controlBannerY}
                  w={controlBannerW}
                  h={controlBannerH}
                  label={`${item.title} image`}
                  imageUrl={null}
                  tone={item.tone}
                  shape="rect"
                  cfg={cfg}
                />
              )}
              <path
                d={cutRectPath(controlBannerX, controlBannerY, controlBannerW, controlBannerH, 11)}
                fill="url(#parentPortalCardBannerShade)"
                clipPath={`url(#${controlClipId})`}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(controlBannerX, controlBannerY, controlBannerW, controlBannerH, 11)}
                fill="none"
                stroke={color}
                strokeWidth={active ? 1.2 : 0.85}
                strokeOpacity={active ? 0.86 : 0.56}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(
                  controlBannerX + 8,
                  controlBannerY + 7,
                  Math.min(controlBannerW - 16, controlCategoryDisplayText.length * 5.8 + 28),
                  20,
                  6
                )}
                fill="rgba(4, 11, 24, 0.72)"
                stroke="rgba(255,255,255,0.16)"
                strokeWidth={0.7}
                pointerEvents="none"
              />
              <text
                x={controlBannerX + 18}
                y={controlBannerY + 21.5}
                fontSize={controlCategorySize}
                fontWeight={900}
                fill="#e6f9ff"
              >
                {controlCategoryDisplayText}
              </text>
              <text
                x={controlX + controlPad + 2}
                y={controlTitleBaseline}
                fontSize={controlTitleSize}
                fontWeight={950}
                fill={cfg.colors.bodyText}
              >
                {controlTitleText}
              </text>
            </>
          )}
        </>
      )}
    </g>
  );
}

function ParentPortalTopCarousel({
  x,
  y,
  w,
  h,
  items,
  page,
  selectedKey,
  onSelect,
  onHoverChange,
  minCardW = PARENT_PORTAL_TOP_CARD_MIN_W,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  items: ParentPortalTopCardItem[];
  page: number;
  selectedKey: string;
  onSelect: (item: ParentPortalTopCardItem) => void;
  onHoverChange?: (item: ParentPortalTopCardItem | null) => void;
  minCardW?: number;
  cfg: ParentPortalSvgControls;
}) {
  const allRowItems = items.length > 0 && items.every((item) => item.kind === 'row');
  const gap = allRowItems ? 10 : 16;
  const visibleCount = Math.max(
    1,
    Math.min(items.length || 1, PARENT_PORTAL_TOP_CAROUSEL_MAX_VISIBLE, Math.floor((w + gap) / (minCardW + gap)))
  );
  const pageCount = Math.max(1, Math.ceil(items.length / visibleCount));
  const safePage = wrapIndex(page, pageCount);
  const visibleItems = items.slice(safePage * visibleCount, safePage * visibleCount + visibleCount);
  const stretchedCardW =
    visibleItems.length > 0 ? (w - gap * Math.max(0, visibleItems.length - 1)) / visibleItems.length : w;
  const rowMaxCardW = visibleItems.length <= 3 ? 370 : 340;
  const cardW = allRowItems ? Math.min(stretchedCardW, rowMaxCardW) : stretchedCardW;
  const trackW = visibleItems.length > 0 ? visibleItems.length * cardW + Math.max(0, visibleItems.length - 1) * gap : 0;
  const startX = x + Math.max(0, (w - trackW) / 2);
  return (
    <g>
      {visibleItems.map((item, index) => {
        const selected = item.key === selectedKey || item.key.startsWith(`${selectedKey}:`);
        return (
          <ParentPortalTopCarouselCard
            key={item.key}
            item={item}
            x={startX + index * (cardW + gap)}
            y={y}
            w={cardW}
            h={h}
            selected={selected}
            onSelect={() => onSelect(item)}
            onHoverChange={onHoverChange}
            cfg={cfg}
          />
        );
      })}
      {items.length === 0 ? (
        <path
          d={cutRectPath(x, y, w, h, 8)}
          fill="rgba(6, 20, 34, 0.7)"
          stroke={cfg.colors.cyan}
          strokeWidth={0.8}
          strokeOpacity={0.5}
        />
      ) : null}
    </g>
  );
}

function ControlCategoryCard({
  category,
  x,
  y,
  w,
  h,
  selected,
  onSelect,
  cfg,
}: {
  category: ControlCategorySummary;
  x: number;
  y: number;
  w: number;
  h: number;
  selected: boolean;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const active = selected || hovered;
  const color = toneColor(category.tone, cfg);
  const cx = x + w / 2;
  const cy = y + h / 2;
  const imageSize = Math.min(31, Math.max(23, h - 16));
  const imageX = x + 10;
  const imageY = y + (h - imageSize) / 2;
  const titleX = imageX + imageSize + 14;
  const countText = category.count > 0 ? String(category.count) : '-';
  const countW = 39;
  const titleW = Math.max(48, x + w - titleX - countW - 18);
  const titleSize = fitSingleLineTextSize(category.label, titleW, 11.8, 14.2, 0.58);
  const countX = x + w - countW - 12;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={`Show ${category.label} controls`}
      aria-pressed={selected}
      transform={hovered ? `translate(${cx} ${cy}) scale(1.018) translate(${-cx} ${-cy})` : undefined}
      onClick={(event) => {
        event.stopPropagation();
        onSelect();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <rect x={x - 3} y={y - 3} width={w + 6} height={h + 6} fill="transparent" pointerEvents="all" />
      <ClickableCardHoverChrome x={x} y={y} w={w} h={h} color={color} active={selected} hovered={hovered} />
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={selected ? colorAlpha(color, '30') : hovered ? colorAlpha(color, '22') : 'rgba(4, 16, 28, 0.78)'}
        stroke={selected ? '#ffe187' : color}
        strokeWidth={selected ? 1.65 : hovered ? 1.35 : 0.95}
        strokeOpacity={active ? 0.96 : 0.68}
        filter={active ? (selected ? 'url(#parentPortalGoldGlow)' : 'url(#parentPortalGlow)') : undefined}
        pointerEvents="none"
      />
      <ArtworkSlot
        x={imageX}
        y={imageY}
        w={imageSize}
        h={imageSize}
        label={`${category.label} artwork`}
        imageUrl={parentPortalControlCategoryImageUrl(category)}
        tone={category.tone}
        compact
        shape="rect"
        imageFit="slice"
        cfg={cfg}
      />
      <line
        x1={imageX + imageSize + 7}
        y1={y + 8}
        x2={imageX + imageSize + 7}
        y2={y + h - 8}
        stroke={color}
        strokeWidth={0.85}
        opacity={active ? 0.64 : 0.42}
      />
      <text
        x={titleX}
        y={y + h / 2 + titleSize * 0.34}
        fontSize={titleSize}
        fontWeight={950}
        fill={cfg.colors.bodyText}
      >
        {category.label}
      </text>
      <path
        d={cutRectPath(countX, y + (h - 24) / 2, countW, 24, 5)}
        fill={colorAlpha(color, selected ? '34' : '18')}
        stroke={color}
        strokeWidth={0.8}
        strokeOpacity={0.72}
        pointerEvents="none"
      />
      <text x={countX + countW / 2} y={y + h / 2 + 4} textAnchor="middle" fontSize={10.2} fontWeight={950} fill={color}>
        {countText}
      </text>
    </g>
  );
}

function ControlCategoryGrid({
  x,
  y,
  w,
  h,
  categories,
  selectedCategoryId,
  page,
  pageCount,
  handleLeftX,
  handleRightX,
  onPageChange,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  categories: ControlCategorySummary[];
  selectedCategoryId: string;
  page: number;
  pageCount: number;
  handleLeftX?: number;
  handleRightX?: number;
  onPageChange: (page: number) => void;
  onSelect: (category: ControlCategorySummary) => void;
  cfg: ParentPortalSvgControls;
}) {
  if (categories.length === 0) return null;
  const gap = 10;
  const handleW = PARENT_PORTAL_SIDE_HANDLE_W;
  const handleH = Math.max(40, h - 10);
  const handleY = y + (h - handleH) / 2;
  const trackX = x;
  const trackW = Math.max(1, w);
  const cardW = (trackW - gap * Math.max(0, categories.length - 1)) / categories.length;
  return (
    <g>
      <ParentPortalFrameSideHandle
        x={handleLeftX ?? x}
        y={handleY}
        side="left"
        height={handleH}
        width={handleW}
        disabled={pageCount <= 1}
        onClick={() => onPageChange(wrapIndex(page - 1, pageCount))}
        cfg={cfg}
      />
      <ParentPortalFrameSideHandle
        x={handleRightX ?? x + w - handleW}
        y={handleY}
        side="right"
        height={handleH}
        width={handleW}
        disabled={pageCount <= 1}
        onClick={() => onPageChange(wrapIndex(page + 1, pageCount))}
        cfg={cfg}
      />
      {categories.map((category, index) => {
        const cardX = trackX + index * (cardW + gap);
        return (
          <ControlCategoryCard
            key={category.id}
            category={category}
            x={cardX}
            y={y}
            w={cardW}
            h={h}
            selected={category.id === selectedCategoryId}
            onSelect={() => onSelect(category)}
            cfg={cfg}
          />
        );
      })}
    </g>
  );
}

function ControlSubcategoryGrid({
  x,
  y,
  w,
  h,
  category,
  selectedSubcategoryId,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  category: ControlCategorySummary;
  selectedSubcategoryId: string | null;
  onSelect: (subcategory: ControlSubcategorySummary) => void;
  cfg: ParentPortalSvgControls;
}) {
  const subcategories = category.subcategories;
  if (subcategories.length === 0) return null;
  const categoryColor = toneColor(category.tone, cfg);
  return (
    <foreignObject x={x} y={y} width={w} height={h}>
      <div
        className="parent-portal-subcategory-grid"
        style={
          {
            '--parent-portal-subcategory-accent': categoryColor,
            '--parent-portal-subcategory-text': cfg.colors.bodyText,
            '--parent-portal-subcategory-muted': cfg.colors.mutedText,
          } as CSSProperties
        }
        onClick={(event) => event.stopPropagation()}
        onWheel={(event) => event.stopPropagation()}
      >
        {subcategories.map((subcategory) => {
          const selected = selectedSubcategoryId === subcategory.id;
          const color = toneColor(subcategory.tone, cfg);
          const style = {
            '--parent-portal-subcategory-color': color,
            '--parent-portal-subcategory-fill': selected ? colorAlpha(color, '30') : 'rgba(5, 19, 32, 0.9)',
            '--parent-portal-subcategory-border': selected ? '#ffe187' : color,
          } as CSSProperties;
          return (
            <button
              key={subcategory.id}
              type="button"
              className={`parent-portal-subcategory-grid__item${selected ? ' parent-portal-subcategory-grid__item--selected' : ''}`}
              style={style}
              aria-label={`Filter ${category.label} by ${subcategory.label}`}
              aria-pressed={selected}
              onClick={(event) => {
                event.stopPropagation();
                onSelect(subcategory);
              }}
            >
              <span className="parent-portal-subcategory-grid__dot" />
              <span className="parent-portal-subcategory-grid__divider" />
              <span className="parent-portal-subcategory-grid__label">{subcategory.label}</span>
              <span className="parent-portal-subcategory-grid__count">{subcategory.count}</span>
            </button>
          );
        })}
      </div>
    </foreignObject>
  );
}

function ClickableCardHoverChrome({
  x,
  y,
  w,
  h,
  color,
  active,
  hovered,
  arrow = true,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  color: string;
  active: boolean;
  hovered: boolean;
  arrow?: boolean;
}) {
  if (!active && !hovered) return null;
  return (
    <>
      <path
        d={cutRectPath(x - 4, y - 4, w + 8, h + 8, 10)}
        fill="none"
        stroke={color}
        strokeWidth={active ? 1.9 : 1.45}
        opacity={active ? 0.42 : 0.32}
        filter="url(#parentPortalGlow)"
        pointerEvents="none"
      />
      {arrow ? (
        <path
          d={`M ${x + w - 10} ${y + 14} L ${x + w + 10} ${y + h / 2} L ${x + w - 10} ${y + h - 14} Z`}
          fill={color}
          opacity={active ? 0.72 : 0.56}
          filter="url(#parentPortalGlow)"
          pointerEvents="none"
        />
      ) : null}
    </>
  );
}

function MainBoard({
  activeNavLabel,
  activeNavGroupId,
  activeTab,
  rows,
  tabs,
  tabDetails,
  controlAreas,
  quickControls,
  guideTopics,
  season,
  uiCopy,
  selectedControlId,
  selectedRowId,
  selectedRow,
  selectedControlName,
  rowsPerPage,
  detailMode,
  onTabChange,
  onRowSelect,
  onPageChange,
  onDetailClose,
  onControlSelect,
  onNavigate,
  onSelectNavLabel,
  cfg,
  mainX,
  mainW,
  mainY,
  mainH,
}: {
  activeNavLabel: string;
  activeNavGroupId: string;
  activeTab: ParentPortalTabId;
  rows: DisplayRow[];
  tabs: ParentPortalContentData['tabs'];
  tabDetails: ParentPortalContentData['tabDetails'];
  controlAreas: ControlArea[];
  quickControls: QuickControl[];
  guideTopics: ParentPortalContentData['guideTopics'];
  season: ParentPortalContentData['season'];
  uiCopy: ParentPortalContentData['uiCopy'];
  selectedControlId: string;
  selectedRowId: string;
  selectedRow: DisplayRow;
  selectedControlName: string;
  rowsPerPage: number;
  detailMode: DetailMode | null;
  onTabChange: (tab: ParentPortalTabId) => void;
  onRowSelect: (rowId: string) => void;
  onPageChange: (page: number) => void;
  onDetailClose: () => void;
  onControlSelect: (controlId: string) => void;
  onNavigate?: (routePath: string) => void;
  onSelectNavLabel: (navLabel: string) => void;
  cfg: ParentPortalSvgControls;
  mainX: number;
  mainW: number;
  mainY: number;
  mainH: number;
}) {
  const activeTabConfig = tabs.find((tab) => tab.id === activeTab) ?? tabs[0];
  const detail = detailForNav(activeNavLabel, tabDetails[activeTab]);
  const rankingTitle = activeTab === 'overall' ? 'PARENT CONTROL SNAPSHOT' : activeTabConfig.title;
  const activeNavKey = assetKey(activeNavLabel);
  const guideOverviewMode = activeNavKey.includes('start-here');
  const guideEligible = activeNavGroupId === 'guide';
  const guideTopicPool = useMemo(
    () =>
      guideEligible
        ? guideOverviewMode
          ? guideTopics
          : guideTopics.filter((topic) => assetKey(topic.navLabel) === activeNavKey)
        : [],
    [activeNavKey, guideEligible, guideOverviewMode, guideTopics]
  );
  const guideMode = guideTopicPool.length > 0;
  const [selectedGuideTopicId, setSelectedGuideTopicId] = useState(() => guideTopicPool[0]?.id ?? '');
  const [guidePage, setGuidePage] = useState(0);
  const [guideQuickPanelMode, setGuideQuickPanelMode] = useState<'read' | 'action'>('read');
  const [guideDashboardDrilldown, setGuideDashboardDrilldown] = useState(false);
  const selectedGuideTopic =
    guideTopicPool.find((topic) => normalizeSelectionId(topic.id) === normalizeSelectionId(selectedGuideTopicId)) ??
    guideTopicPool[0] ??
    null;
  const guideDashboardMode = guideOverviewMode && !guideDashboardDrilldown;
  useEffect(() => {
    const firstTopic = guideTopicPool[0];
    if (!firstTopic) return;
    const selectedStillVisible = guideTopicPool.some(
      (topic) => normalizeSelectionId(topic.id) === normalizeSelectionId(selectedGuideTopicId)
    );
    if (selectedStillVisible) return;
    setSelectedGuideTopicId(firstTopic.id);
    setGuidePage(0);
  }, [guideTopicPool, selectedGuideTopicId]);
  useEffect(() => {
    setGuideDashboardDrilldown(false);
    setGuidePage(0);
    setGuideQuickPanelMode('read');
  }, [activeNavKey]);
  useEffect(() => {
    setGuideQuickPanelMode('read');
  }, [selectedGuideTopicId]);
  const guideTopicById = useMemo(
    () => new Map(guideTopics.map((topic) => [normalizeSelectionId(topic.id), topic])),
    [guideTopics]
  );
  const handleGuideNoteSelect = (note: ParentPortalGuideNote) => {
    if (isHashRoutePath(note.targetRoutePath)) {
      if (note.targetNavLabel) onSelectNavLabel(note.targetNavLabel);
      onNavigate?.(note.targetRoutePath);
      return;
    }
    if (note.targetTopicId) {
      const targetTopic = guideTopicById.get(normalizeSelectionId(note.targetTopicId));
      if (targetTopic) {
        onSelectNavLabel(note.targetNavLabel ?? targetTopic.navLabel);
        setSelectedGuideTopicId(targetTopic.id);
        setGuidePage(typeof note.targetPage === 'number' ? note.targetPage : 0);
        setGuideDashboardDrilldown(true);
        setGuideQuickPanelMode('read');
        return;
      }
    }
    if (note.targetNavLabel) {
      onSelectNavLabel(note.targetNavLabel);
    }
    if (typeof note.targetPage === 'number') {
      setGuidePage(clampValue(note.targetPage, 0, Math.max(0, (selectedGuideTopic?.pages.length ?? 1) - 1)));
      setGuideDashboardDrilldown(true);
    }
  };
  const focusedSectionTitle = activeNavKey.includes('overview')
    ? 'PARENT OVERVIEW'
    : activeNavKey.includes('today')
      ? 'TODAY'
      : activeNavKey.includes('global') || activeNavKey.includes('overall')
        ? rankingTitle
        : guideDashboardMode
          ? 'START HERE'
          : guideMode
            ? 'GUIDE TOPICS'
            : activeNavLabel || rankingTitle;
  const tableVariant = tableVariantForContext(activeNavLabel, activeTab);
  const baseTableTitle = tableTitleForVariant(tableVariant, activeNavLabel, selectedControlName);
  const isOverviewContext = activeNavKey.includes('overview') || activeNavKey.includes('today');
  const sortedRows = useMemo(() => [...rows].sort((a, b) => a.order - b.order), [rows]);
  const controlScopedRows = useMemo(
    () => rowsForControlScope(sortedRows, selectedControlName, selectedControlId),
    [selectedControlId, selectedControlName, sortedRows]
  );
  const controlCategories = useMemo(() => buildControlCategorySummaries(quickControls), [quickControls]);
  const selectedQuickControl = useMemo(
    () =>
      quickControls.find((control) => normalizeSelectionId(control.id) === normalizeSelectionId(selectedControlId)) ??
      quickControls[0],
    [quickControls, selectedControlId]
  );
  const [selectedCategoryIdOverride, setSelectedCategoryIdOverride] = useState<string | null>(null);
  const [selectedSubcategoryIdOverride, setSelectedSubcategoryIdOverride] = useState<string | null>(null);
  const [expandedCategoryId, setExpandedCategoryId] = useState<string | null>(null);
  const [hoveredTopControlKey, setHoveredControlAreaKey] = useState<string | null>(null);
  const manageLane = selectedQuickControl ? manageLaneForControl(selectedQuickControl) : 'childPolicy';
  const selectedControlCategoryId =
    selectedCategoryIdOverride ??
    (selectedQuickControl ? assetKey(controlCategoryLabel(selectedQuickControl)) : (controlCategories[0]?.id ?? ''));
  const selectedCategory =
    controlCategories.find((category) => category.id === selectedControlCategoryId) ?? controlCategories[0] ?? null;
  const selectedCategoryLabel =
    selectedCategory?.label ?? (selectedQuickControl ? controlCategoryLabel(selectedQuickControl) : 'Controls');
  const selectedSubcategoryId = selectedCategory?.subcategories.some(
    (subcategory) => subcategory.id === selectedSubcategoryIdOverride
  )
    ? selectedSubcategoryIdOverride
    : null;
  const perCategoryMode = activeNavKey.includes('category');
  const aiBrowserMode = tableVariant === 'ai' && (activeNavKey.includes('game') || activeNavKey.includes('category'));
  const categoryBrowserMode = perCategoryMode || activeNavKey.includes('ai-by-category');
  const manageMode = activeNavGroupId === 'manage';
  const manageThemeTone = manageMode ? (selectedQuickControl?.tone ?? detail.tone) : detail.tone;
  const manageLaneTitle =
    manageLane === 'childPolicy' ? 'Policy' : manageLane === 'deviceOps' ? 'Device Tools' : 'Portal';
  const manageCurrentSpec = useMemo(
    () => (manageMode ? manageControlSpecFor(activeNavLabel, selectedControlName) : null),
    [activeNavLabel, manageMode, selectedControlName]
  );
  const manageBrowserTargets = useMemo(
    () => manageBrowserTargetsForKey(activeNavLabel, selectedControlName),
    [activeNavLabel, selectedControlName]
  );
  const [manageTargetSelection, setManageTargetSelection] = useState<ManageTargetSelection>(() => ({
    scope: 'perDevice',
    device: 'Aarav laptop',
    browser: 'Chrome',
  }));
  useEffect(() => {
    if (!manageMode || !manageCurrentSpec) return;
    setManageTargetSelection({
      scope: manageScopeForLane(manageLane),
      device: manageCurrentSpec.devices[0] ?? 'Family default',
      browser: manageBrowserTargets[0]?.label ?? 'All targets',
    });
  }, [manageBrowserTargets, manageCurrentSpec, manageLane, manageMode]);
  const controlBrowserMode = !guideMode && !manageMode && (tableVariant === 'controls' || aiBrowserMode);
  const expandedControlCategory =
    controlBrowserMode && selectedCategory && expandedCategoryId === selectedCategory.id ? selectedCategory : null;
  const tableTitle =
    guideMode && selectedGuideTopic
      ? `${selectedGuideTopic.title.toUpperCase()} GUIDE`
      : manageMode
        ? manageCurrentSpec
          ? `${manageCurrentSpec.title.toUpperCase()} EDITOR`
          : `${manageLaneTitle.toUpperCase()} SETTINGS`
        : activeNavKey.includes('ai-by-game')
          ? `${selectedControlName.toUpperCase()} AI BENCHMARKS`
          : activeNavKey.includes('ai-by-category')
            ? `${selectedCategoryLabel.toUpperCase()} AI BENCHMARKS`
            : perCategoryMode
              ? `${selectedCategoryLabel.toUpperCase()} CATEGORY DETAIL`
              : baseTableTitle;
  const filteredCategoryControls = quickControls.filter(
    (control) => assetKey(controlCategoryLabel(control)) === selectedControlCategoryId
  );
  const filteredSubcategoryControls = selectedSubcategoryId
    ? filteredCategoryControls.filter((control) => assetKey(controlSubcategoryLabel(control)) === selectedSubcategoryId)
    : filteredCategoryControls;
  const sortedCategoryControls = [...filteredSubcategoryControls].sort(
    (a, b) =>
      (a.controlCode ?? Number.MAX_SAFE_INTEGER) - (b.controlCode ?? Number.MAX_SAFE_INTEGER) ||
      a.name.localeCompare(b.name)
  );
  const controlBrowserPoolBase = controlBrowserMode ? sortedCategoryControls : quickControls;
  const controlBrowserPool =
    activeNavGroupId === 'manage'
      ? quickControls.filter((control) => manageLaneForControl(control) === manageLane)
      : controlBrowserPoolBase;
  const controlAreasById = new Map(controlAreas.map((control) => [normalizeSelectionId(control.id), control]));
  const topItems: ParentPortalTopCardItem[] = guideMode
    ? guideTopicPool.map(guideTopCard)
    : controlBrowserMode
      ? controlBrowserPool.map((control, index) =>
          controlTopCard(control, controlAreasById.get(normalizeSelectionId(control.id)), index)
        )
      : sortedRows.slice(0, isOverviewContext ? 3 : 10).map(rowTopCard);
  const selectedTopKey =
    guideMode && selectedGuideTopic
      ? `guide:${normalizeSelectionId(selectedGuideTopic.id)}`
      : controlBrowserMode
        ? `control:${normalizeSelectionId(selectedControlId)}`
        : `row:${selectedRowId}`;
  const focusContextKey = `${activeNavLabel}:${activeTab}`;
  const [focusState, setFocusState] = useState<{ contextKey: string; section: ParentPortalFocusSection }>(() => ({
    contextKey: focusContextKey,
    section: 'highlights',
  }));
  const focusedSection = focusState.contextKey === focusContextKey ? focusState.section : 'highlights';
  const setFocusedSection = (section: ParentPortalFocusSection) =>
    setFocusState({ contextKey: focusContextKey, section });
  const tableFocused = focusedSection === 'table';
  const showTopSection = !tableFocused;
  const sectionGap = Math.max(8, Math.min(cfg.layout.gap, 14));
  const expandedTopPanelH = Math.max(276, Math.min(mainH - 210, clampValue(mainH * 0.46, 276, 334)));
  const hoverTopPanelH = Math.max(242, Math.min(mainH - 210, clampValue(mainH * 0.4, 242, 292)));
  const compactTopPanelH = Math.max(178, Math.min(mainH - 250, clampValue(mainH * 0.29, 178, 214)));
  const topPanelH = !showTopSection
    ? 0
    : manageMode
      ? clampValue(mainH * 0.23, 178, 228)
      : guideDashboardMode
        ? mainH
        : guideMode
          ? clampValue(mainH * 0.24, 164, 220)
          : controlBrowserMode
            ? hoveredTopControlKey
              ? Math.max(hoverTopPanelH, expandedControlCategory ? expandedTopPanelH : 0)
              : expandedControlCategory
                ? expandedTopPanelH
                : compactTopPanelH
            : clampValue(mainH * 0.39, 260, 294);
  const bottomPanelY = showTopSection ? mainY + topPanelH + sectionGap : mainY;
  const bottomPanelH = showTopSection ? mainH - topPanelH - sectionGap : mainH;
  const selectorHandleGutter = PARENT_PORTAL_SIDE_HANDLE_W;
  const selectorX = mainX + selectorHandleGutter;
  const selectorW = Math.max(320, mainW - selectorHandleGutter * 2);
  const rowHandleReserve = (PARENT_PORTAL_SIDE_HANDLE_W + 8) * 2;
  const selectorInnerW = Math.max(1, selectorW - 36);
  const categoryTrackW = Math.max(1, selectorInnerW - rowHandleReserve);
  const categoryVisibleCount = Math.max(
    1,
    Math.min(controlCategories.length || 1, Math.floor((categoryTrackW + 8) / 178), 6)
  );
  const [categoryPage, setCategoryPage] = useState(0);
  const categoryPageCount = Math.max(1, Math.ceil(controlCategories.length / categoryVisibleCount));
  const safeCategoryPage = wrapIndex(categoryPage, categoryPageCount);
  const visibleControlCategories = controlCategories.slice(
    safeCategoryPage * categoryVisibleCount,
    safeCategoryPage * categoryVisibleCount + categoryVisibleCount
  );
  const tableFrame = parentPortalFrameRects(mainX, bottomPanelY, mainW, bottomPanelH);
  const tableY = bottomPanelY + 47;
  const rowStep = cfg.chrome.rowHeight + cfg.chrome.rowGap;
  const visibleRowCapacity = Math.max(1, Math.floor((tableFrame.body.y + tableFrame.body.h - tableY - 42) / rowStep));
  const effectiveRowsPerPage = Math.max(1, Math.min(rowsPerPage, visibleRowCapacity));
  const categoryScopedRows = rowsForCategoryScope(sortedRows, selectedCategoryLabel);
  const aiControlScopedRows = rowsForControlScope(sortedRows, selectedControlName, selectedControlId);
  const tableRows =
    tableVariant === 'controls'
      ? categoryBrowserMode
        ? categoryScopedRows
        : controlScopedRows
      : tableVariant === 'ai' && activeNavKey.includes('ai-by-category')
        ? categoryScopedRows
        : tableVariant === 'ai' && activeNavKey.includes('ai-by-game')
          ? aiControlScopedRows
          : isOverviewContext
            ? [
                ...sortedRows.filter((row) => row.id === selectedRowId && row.order <= 3),
                ...sortedRows.filter((row) => row.order > 3),
              ]
            : sortedRows;
  const [highlightPage, setHighlightPage] = useState(0);
  const topCardGap = 10;
  const topCardMinW = guideMode
    ? 250
    : controlBrowserMode
      ? PARENT_PORTAL_CONTROL_CARD_MIN_W
      : PARENT_PORTAL_TOP_CARD_MIN_W;
  const topCarouselAvailableW = controlBrowserMode
    ? Math.max(1, selectorInnerW - rowHandleReserve)
    : Math.max(1, selectorW - 56);
  const topCardVisibleCount = Math.max(
    1,
    Math.min(
      topItems.length || 1,
      PARENT_PORTAL_TOP_CAROUSEL_MAX_VISIBLE,
      Math.floor((topCarouselAvailableW + topCardGap) / (topCardMinW + topCardGap))
    )
  );
  const highlightPageCount = Math.max(1, Math.ceil(topItems.length / topCardVisibleCount));
  const safeHighlightPage = wrapIndex(highlightPage, highlightPageCount);
  const shiftHighlightPage = (delta: number) =>
    setHighlightPage((value) => wrapIndex(value + delta, highlightPageCount));
  const framePage = safeHighlightPage;
  const framePageCount = highlightPageCount;
  const shiftFramePage = (delta: number) => {
    shiftHighlightPage(delta);
  };
  const routeControlCategoryToBottom = (category: ControlCategorySummary) => {
    setFocusedSection('highlights');
    setSelectedCategoryIdOverride(category.id);
    setSelectedSubcategoryIdOverride(null);
    setExpandedCategoryId((value) => (value === category.id ? null : category.id));
    setHighlightPage(0);
    const nextRows =
      category.count > 0
        ? rowsForControlScope(sortedRows, category.sampleControl.name, category.sampleControl.id)
        : rowsForCategoryScope(sortedRows, category.label);
    if (nextRows[0]) onRowSelect(nextRows[0].id);
    if (category.count > 0) onControlSelect(category.sampleControl.id);
    onPageChange(1);
  };
  const routeControlSubcategoryToBottom = (subcategory: ControlSubcategorySummary) => {
    setFocusedSection('highlights');
    setSelectedSubcategoryIdOverride(subcategory.id);
    setExpandedCategoryId(null);
    setHighlightPage(0);
    const nextRows = rowsForControlScope(sortedRows, subcategory.sampleControl.name, subcategory.sampleControl.id);
    if (nextRows[0]) onRowSelect(nextRows[0].id);
    onControlSelect(subcategory.sampleControl.id);
    onPageChange(1);
  };
  const routeTopItemToBottom = (item: ParentPortalTopCardItem) => {
    setFocusedSection('highlights');
    if (item.kind === 'guide') {
      setSelectedGuideTopicId(item.topic.id);
      setGuidePage(0);
      onPageChange(1);
      return;
    }
    if (item.kind === 'control') {
      const nextCategoryId = assetKey(controlCategoryLabel(item.control));
      setSelectedCategoryIdOverride(nextCategoryId);
      setSelectedSubcategoryIdOverride(assetKey(controlSubcategoryLabel(item.control)));
      setExpandedCategoryId(null);
      const nextCategoryIndex = controlCategories.findIndex((category) => category.id === nextCategoryId);
      if (nextCategoryIndex >= 0) {
        setCategoryPage(Math.floor(nextCategoryIndex / categoryVisibleCount));
      }
      const nextRows = rowsForControlScope(sortedRows, item.control.name, item.control.id);
      if (nextRows[0]) onRowSelect(nextRows[0].id);
      onControlSelect(item.control.id);
      onPageChange(1);
      return;
    }
    onRowSelect(item.row.id);
    const targetIndex = tableRows.findIndex((row) => row.id === item.row.id);
    if (targetIndex >= 0) {
      onPageChange(Math.floor(targetIndex / effectiveRowsPerPage) + 1);
    }
  };
  const handleHighlightsWheel = (event: WheelEvent<SVGGElement>) => {
    const delta = Math.abs(event.deltaX) > Math.abs(event.deltaY) ? event.deltaX : event.deltaY;
    if (framePageCount <= 1 || Math.abs(delta) < 8) return;
    event.preventDefault();
    event.stopPropagation();
    shiftFramePage(delta > 0 ? 1 : -1);
  };
  const tableHeaderButtonY = bottomPanelY + 13;
  const tableHeaderAction = tableFocused ? (
    <ParentPortalHeaderAction
      x={mainX + mainW - 118}
      y={tableHeaderButtonY + 2}
      w={98}
      h={25}
      tone="gold"
      active
      label="TOP"
      onClick={() => setFocusedSection('highlights')}
      ariaLabel="Show highlights section"
      cfg={cfg}
    />
  ) : guideOverviewMode && guideDashboardDrilldown ? (
    <ParentPortalHeaderAction
      x={mainX + mainW - 118}
      y={tableHeaderButtonY + 2}
      w={98}
      h={25}
      tone="cyan"
      active
      label="MAP"
      onClick={() => setGuideDashboardDrilldown(false)}
      ariaLabel="Show guide setup map"
      cfg={cfg}
    />
  ) : null;
  return (
    <g>
      {showTopSection ? (
        <ParentPortalSectionFrame
          x={selectorX}
          y={mainY}
          w={selectorW}
          h={topPanelH}
          title={
            manageMode && manageCurrentSpec ? `${manageCurrentSpec.title.toUpperCase()} TARGET` : focusedSectionTitle
          }
          subtitle={
            manageMode && manageCurrentSpec
              ? 'Choose global, child device, and browser target before editing below'
              : guideDashboardMode
                ? 'Set up parent app, child devices, controls, privacy, alerts, and storage'
                : guideMode && selectedGuideTopic
                  ? `${selectedGuideTopic.subtitle} / ${selectedGuideTopic.detail}`
                  : `${detail.eyebrow} / ${detail.primary}`
          }
          count="1"
          tone={manageMode ? manageThemeTone : 'cyan'}
          headerH={manageMode ? 44 : controlBrowserMode ? 48 : 40}
          footerH={manageMode ? 18 : guideDashboardMode ? 18 : controlBrowserMode ? 22 : 30}
          innerStrokeOpacity={manageMode ? 0.34 : controlBrowserMode ? 0.24 : undefined}
          bodyStrokeOpacity={manageMode || controlBrowserMode ? 0 : undefined}
          bodyFill={manageMode || controlBrowserMode ? 'transparent' : undefined}
          footerLineOpacity={manageMode || controlBrowserMode ? 0 : undefined}
          headerRight={null}
          showSideHandles={!manageMode && !controlBrowserMode && !guideDashboardMode}
          sideDisabled={guideDashboardMode || framePageCount <= 1}
          onPrevious={() => shiftFramePage(-1)}
          onNext={() => shiftFramePage(1)}
          selected={focusedSection === 'highlights'}
          onSelect={() => setFocusedSection('highlights')}
          ariaLabel="Focus parent control selector"
          footer={(footerRect) => (
            <>
              {manageMode || guideDashboardMode ? null : (
                <ParentPortalFrameDots
                  x={footerRect.x + footerRect.w / 2}
                  y={footerRect.y + (controlBrowserMode ? 14 : 18)}
                  page={framePage}
                  pageCount={framePageCount}
                  onPageChange={setHighlightPage}
                  cfg={cfg}
                />
              )}
              <text
                x={footerRect.x + footerRect.w - 22}
                y={footerRect.y + (controlBrowserMode ? 18 : 23)}
                textAnchor="end"
                fontSize={controlBrowserMode ? 9.5 : 10}
                fontWeight={900}
                fill={cfg.colors.mutedText}
              >
                {manageMode
                  ? 'TARGET SELECTOR'
                  : guideDashboardMode
                    ? `GUIDES ${guideTopicPool.length}`
                    : `${guideMode ? 'TOPICS' : controlBrowserMode ? 'AREAS' : isOverviewContext ? 'READY' : 'ITEMS'} ${
                        framePage + 1
                      }/${framePageCount}`}
              </text>
            </>
          )}
          cfg={cfg}
        >
          {(body) => {
            const contentX = body.x + 2;
            const contentY = body.y + (controlBrowserMode ? 2 : -2);
            const contentW = body.w - 4;
            const contentH = body.h + (controlBrowserMode ? -2 : 4);
            const expandedCategory = expandedControlCategory;
            const categoryH = controlBrowserMode
              ? clampValue(
                  contentH * (expandedCategory ? 0.17 : 0.28),
                  expandedCategory ? 34 : 28,
                  expandedCategory ? 42 : 34
                )
              : 0;
            const subcategoryH = expandedCategory ? clampValue(contentH * 0.32, 82, 110) : 0;
            const categoryGap = controlBrowserMode ? 4 : 0;
            const subcategoryGap = expandedCategory ? 4 : 0;
            const subcategoryY = contentY + categoryH + categoryGap;
            const carouselY = subcategoryY + subcategoryH + subcategoryGap;
            const carouselH = expandedCategory
              ? Math.max(64, contentH - categoryH - categoryGap - subcategoryH - subcategoryGap)
              : Math.max(48, contentH - categoryH - categoryGap);
            const rowHandleW = PARENT_PORTAL_SIDE_HANDLE_W;
            const rowHandleLeftX = selectorX - rowHandleW + PARENT_PORTAL_SIDE_HANDLE_OVERLAP;
            const rowHandleRightX = selectorX + selectorW - PARENT_PORTAL_SIDE_HANDLE_OVERLAP;
            const carouselTrackX = contentX;
            const carouselTrackW = contentW;
            const carouselHandleH = controlBrowserMode ? Math.max(44, Math.min(92, carouselH - 8)) : 0;
            const carouselHandleY = carouselY + Math.max(0, (carouselH - carouselHandleH) / 2);
            return (
              <g onWheel={handleHighlightsWheel}>
                <rect x={contentX - 4} y={contentY - 4} width={contentW + 8} height={contentH + 8} fill="transparent" />
                {manageMode && manageCurrentSpec ? (
                  <ManageTargetPanel
                    x={contentX + 8}
                    y={contentY + 10}
                    w={contentW - 16}
                    h={contentH - 12}
                    activeNavLabel={activeNavLabel}
                    selectedControlName={selectedControlName}
                    spec={manageCurrentSpec}
                    lane={manageLane}
                    targetSelection={manageTargetSelection}
                    onTargetChange={setManageTargetSelection}
                    cfg={cfg}
                  />
                ) : guideDashboardMode ? (
                  <GuideOverviewDashboard
                    x={contentX + 10}
                    y={contentY + 10}
                    w={contentW - 20}
                    h={contentH - 18}
                    topics={guideTopicPool}
                    selectedTopicId={selectedGuideTopicId}
                    onSelect={(topic) => {
                      setSelectedGuideTopicId(topic.id);
                      setGuidePage(0);
                      setGuideDashboardDrilldown(true);
                    }}
                    cfg={cfg}
                  />
                ) : controlBrowserMode ? (
                  <ControlCategoryGrid
                    x={contentX}
                    y={contentY}
                    w={contentW}
                    h={categoryH}
                    categories={visibleControlCategories}
                    selectedCategoryId={selectedControlCategoryId}
                    page={safeCategoryPage}
                    pageCount={categoryPageCount}
                    handleLeftX={rowHandleLeftX}
                    handleRightX={rowHandleRightX}
                    onPageChange={setCategoryPage}
                    onSelect={routeControlCategoryToBottom}
                    cfg={cfg}
                  />
                ) : null}
                {!guideDashboardMode && expandedCategory ? (
                  <ControlSubcategoryGrid
                    x={carouselTrackX}
                    y={subcategoryY}
                    w={carouselTrackW}
                    h={subcategoryH}
                    category={expandedCategory}
                    selectedSubcategoryId={selectedSubcategoryId}
                    onSelect={routeControlSubcategoryToBottom}
                    cfg={cfg}
                  />
                ) : null}
                {!guideDashboardMode && controlBrowserMode ? (
                  <>
                    <ParentPortalFrameSideHandle
                      x={rowHandleLeftX}
                      y={carouselHandleY}
                      side="left"
                      height={carouselHandleH}
                      width={rowHandleW}
                      disabled={highlightPageCount <= 1}
                      onClick={() => shiftHighlightPage(-1)}
                      cfg={cfg}
                    />
                    <ParentPortalFrameSideHandle
                      x={rowHandleRightX}
                      y={carouselHandleY}
                      side="right"
                      height={carouselHandleH}
                      width={rowHandleW}
                      disabled={highlightPageCount <= 1}
                      onClick={() => shiftHighlightPage(1)}
                      cfg={cfg}
                    />
                  </>
                ) : null}
                {!guideDashboardMode && !manageMode ? (
                  <ParentPortalTopCarousel
                    x={carouselTrackX}
                    y={controlBrowserMode ? carouselY : contentY}
                    w={carouselTrackW}
                    h={controlBrowserMode ? carouselH : contentH}
                    items={topItems}
                    page={safeHighlightPage}
                    selectedKey={selectedTopKey}
                    onSelect={routeTopItemToBottom}
                    onHoverChange={(item) => setHoveredControlAreaKey(item?.kind === 'control' ? item.key : null)}
                    minCardW={topCardMinW}
                    cfg={cfg}
                  />
                ) : null}
              </g>
            );
          }}
        </ParentPortalSectionFrame>
      ) : null}
      {!guideDashboardMode ? (
        <ParentPortalSectionFrame
          x={mainX}
          y={bottomPanelY}
          w={mainW}
          h={bottomPanelH}
          title={tableTitle}
          count="2"
          tone={manageMode ? manageThemeTone : 'cyan'}
          headerRight={tableHeaderAction}
          bodyStrokeOpacity={0}
          bodyFill="transparent"
          selected={tableFocused}
          onSelect={() => setFocusedSection('table')}
          ariaLabel={tableFocused ? 'Expanded parent detail panel' : 'Expand parent detail panel'}
          cfg={cfg}
        >
          {(body) => (
            <ParentPortalDetailPanel
              x={body.x + 18}
              y={body.y + 18}
              w={body.w - 36}
              h={body.h - 36}
              activeNavLabel={activeNavLabel}
              activeNavGroupId={activeNavGroupId}
              detail={detail}
              rows={tableRows}
              selectedControlName={selectedControlName}
              themeTone={manageThemeTone}
              guideTopic={selectedGuideTopic}
              guidePage={guidePage}
              onGuidePageChange={setGuidePage}
              quickPanelMode={guideQuickPanelMode}
              onQuickPanelModeChange={setGuideQuickPanelMode}
              onGuideNoteSelect={handleGuideNoteSelect}
              manageTargetSelection={manageTargetSelection}
              onNavigate={onNavigate}
              cfg={cfg}
            />
          )}
        </ParentPortalSectionFrame>
      ) : null}
      {detailMode ? (
        <DetailOverlay
          x={mainX + 18}
          y={mainY + 50}
          w={mainW - 36}
          h={mainH - 100}
          mode={detailMode}
          activeTab={activeTab}
          detail={detail}
          season={season}
          controlAreas={controlAreas}
          uiCopy={uiCopy}
          selectedRow={selectedRow}
          selectedControlName={selectedControlName}
          onTabChange={onTabChange}
          onClose={onDetailClose}
          cfg={cfg}
        />
      ) : null}
    </g>
  );
}

function DetailOverlay({
  x,
  y,
  w,
  h,
  mode,
  activeTab,
  detail,
  season,
  controlAreas,
  uiCopy,
  selectedRow,
  selectedControlName,
  onTabChange,
  onClose,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  mode: DetailMode;
  activeTab: ParentPortalTabId;
  detail: TabDetail;
  season: ParentPortalContentData['season'];
  controlAreas: ControlArea[];
  uiCopy: ParentPortalContentData['uiCopy'];
  selectedRow: DisplayRow;
  selectedControlName: string;
  onTabChange: (tab: ParentPortalTabId) => void;
  onClose: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const tone: Tone = mode === 'season' ? 'gold' : mode === 'control' ? 'purple' : selectedRow.tone;
  const color = toneColor(tone, cfg);
  const selectedControl = controlAreas.find(
    (control) => control.name === selectedControlName || control.id === selectedControlName
  );
  const title =
    mode === 'season'
      ? season.detailTitle
      : mode === 'control'
        ? `${selectedControlName.toUpperCase()} DRILLDOWN`
        : `${selectedRow.label.toUpperCase()} PROFILE`;
  const subtitle =
    mode === 'season'
      ? season.detailSubtitle
      : mode === 'control'
        ? `Control detail from ${detail.eyebrow}`
        : `${detail.eyebrow} / ${selectedControlName}`;
  const stats =
    mode === 'season'
      ? season.stats.map((stat) => [stat.label, stat.value] as [string, string])
      : mode === 'control'
        ? [
            ['MATCHES', selectedControl?.matches ?? 'N/A'],
            ['GROWTH', selectedControl?.growth ?? 'N/A'],
            ['AREAS', '100'],
          ]
        : [
            ['SIGNAL', selectedRow.signal],
            ['READINESS', selectedRow.readiness],
            ['TREND', selectedRow.trend],
          ];
  const primaryTab: ParentPortalTabId = mode === 'season' ? 'routines' : mode === 'control' ? 'controls' : activeTab;
  const primaryLabel = mode === 'season' ? 'OPEN EVENTS' : mode === 'control' ? 'OPEN CONTROL' : 'KEEP SELECTED';
  const artworkImageUrl =
    mode === 'season'
      ? bannerParentPortalOverviewImageUrl
      : mode === 'control'
        ? parentPortalControlImageUrl(selectedControl?.id ?? selectedControlName)
        : rowAvatarImageUrl(selectedRow.label);
  return (
    <g role="dialog" aria-label={title}>
      <path
        d={cutRectPath(x - 8, y - 8, w + 16, h + 16, 18)}
        fill="rgba(1, 5, 12, 0.72)"
        stroke={color}
        strokeWidth={1.2}
        opacity={0.98}
      />
      <path
        d={cutRectPath(x, y, w, h, 16)}
        fill="rgba(5, 17, 30, 0.97)"
        stroke={color}
        strokeWidth={1.4}
        filter="url(#parentPortalGlow)"
      />
      <ArtworkSlot
        x={x + 22}
        y={y + 24}
        w={92}
        h={82}
        label={mode === 'season' ? 'SEASON' : mode === 'control' ? 'CONTROL ART' : 'ROW'}
        imageUrl={artworkImageUrl}
        tone={tone}
        shape={mode === 'row' ? 'circle' : 'hex'}
        cfg={cfg}
      />
      <text x={x + 136} y={y + 39} fontSize={22} fontWeight={950} fill={cfg.colors.bodyText}>
        {title}
      </text>
      <text x={x + 136} y={y + 64} fontSize={12} fontWeight={800} fill="#a9c3da">
        {subtitle}
      </text>
      <text x={x + 136} y={y + 91} fontSize={11} fontWeight={900} fill={color}>
        {detail.primary}
      </text>
      <text x={x + 136} y={y + 112} fontSize={10.5} fontWeight={760} fill="#d8eaff">
        {detail.summary}
      </text>
      <ParentPortalHeaderAction
        x={x + w - 208}
        y={y + 26}
        w={126}
        h={30}
        tone={tone}
        active
        label={primaryLabel}
        onClick={() => {
          onTabChange(primaryTab);
          onClose();
        }}
        ariaLabel={primaryLabel}
        cfg={cfg}
      />
      <ParentPortalHeaderAction
        x={x + w - 68}
        y={y + 26}
        w={44}
        h={30}
        tone="muted"
        label="X"
        onClick={onClose}
        ariaLabel="Close parent portal detail"
        cfg={cfg}
      />
      {stats.map(([label, value], index) => {
        const cardW = (w - 62) / 3;
        const cardX = x + 22 + index * (cardW + 9);
        return (
          <SurfacePanel key={label} x={cardX} y={y + 136} w={cardW} h={72} tone={index === 1 ? tone : 'cyan'} cfg={cfg}>
            <text x={cardX + 18} y={y + 162} fontSize={10} fontWeight={900} fill={cfg.colors.mutedText}>
              {label}
            </text>
            <text x={cardX + 18} y={y + 190} fontSize={22} fontWeight={950} fill={cfg.colors.bodyText}>
              {value}
            </text>
          </SurfacePanel>
        );
      })}
      <path
        d={cutRectPath(x + 22, y + 230, w - 44, h - 256, 12)}
        fill="rgba(7, 22, 37, 0.72)"
        stroke={color}
        strokeWidth={0.8}
        strokeOpacity={0.72}
      />
      <text x={x + 44} y={y + 266} fontSize={12} fontWeight={950} fill={cfg.colors.bodyText}>
        {uiCopy.detailSnapshotTitle}
      </text>
      {uiCopy.detailSnapshotLines.slice(0, 2).map((line, index) => (
        <text key={line} x={x + 44} y={y + 291 + index * 27} fontSize={11} fontWeight={760} fill="#b9d2e7">
          {line}
        </text>
      ))}
    </g>
  );
}

function Defs() {
  return (
    <defs>
      <linearGradient id="parentPortalActiveBlue" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#39dfff" stopOpacity="0.74" />
        <stop offset="48%" stopColor="#266aa6" stopOpacity="0.78" />
        <stop offset="100%" stopColor="#273184" stopOpacity="0.9" />
      </linearGradient>
      <radialGradient id="parentPortalHoverShine" cx="18%" cy="0%" r="125%">
        <stop offset="0%" stopColor="#ffffff" stopOpacity="0.16" />
        <stop offset="35%" stopColor="#42e8ff" stopOpacity="0.1" />
        <stop offset="100%" stopColor="#42e8ff" stopOpacity="0" />
      </radialGradient>
      <linearGradient id="parentPortalFrameFill" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#08243a" stopOpacity="0.94" />
        <stop offset="48%" stopColor="#041624" stopOpacity="0.96" />
        <stop offset="100%" stopColor="#061d31" stopOpacity="0.92" />
      </linearGradient>
      <linearGradient id="parentPortalFrameGlass" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#123f59" stopOpacity="0.32" />
        <stop offset="52%" stopColor="#061525" stopOpacity="0.24" />
        <stop offset="100%" stopColor="#0b2445" stopOpacity="0.28" />
      </linearGradient>
      <linearGradient id="parentPortalFrameShine" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" stopColor="#ffffff" stopOpacity="0.14" />
        <stop offset="42%" stopColor="#42e8ff" stopOpacity="0.07" />
        <stop offset="100%" stopColor="#42e8ff" stopOpacity="0" />
      </linearGradient>
      <linearGradient id="parentPortalCardBannerShade" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" stopColor="#07111f" stopOpacity="0.16" />
        <stop offset="52%" stopColor="#06101f" stopOpacity="0.46" />
        <stop offset="100%" stopColor="#061525" stopOpacity="0.92" />
      </linearGradient>
      <linearGradient id="parentPortalFrameCountGold" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" stopColor="#fff7c7" />
        <stop offset="22%" stopColor="#ffd85c" />
        <stop offset="56%" stopColor="#d89010" />
        <stop offset="100%" stopColor="#4a2a00" />
      </linearGradient>
      <linearGradient id="parentPortalFooterActivePill" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#ffe187" />
        <stop offset="100%" stopColor="#b98214" />
      </linearGradient>
      <filter id="parentPortalGlow" x="-35%" y="-35%" width="170%" height="170%">
        <feGaussianBlur stdDeviation={3.5} result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
      <filter id="parentPortalGreenGlow" x="-60%" y="-60%" width="220%" height="220%">
        <feGaussianBlur stdDeviation={5} result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
      <filter id="parentPortalGoldGlow" x="-60%" y="-60%" width="220%" height="220%">
        <feGaussianBlur stdDeviation={4} result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
    </defs>
  );
}

export function ParentPortalSvgSurface({
  pageMode,
  parentPortalRows,
  userEntry,
  controlId,
  loading = false,
  error = null,
  controls,
  content,
  initialNavLabel,
  initialSelectedControlId,
  onRefreshParentPortal,
  onNavigate,
}: ParentPortalSvgSurfaceProps) {
  const mainRef = useRef<HTMLElement | null>(null);
  const baseCfg = useMemo(() => normalizeParentPortalSvgControls(controls), [controls]);
  const pageContent = useMemo(() => normalizeParentPortalContent(content), [content]);
  const tabs = pageContent.tabs;
  const tabDetails = pageContent.tabDetails;
  const navItems = useMemo<NavItem[]>(
    () =>
      pageContent.navItems.map((item) => ({
        ...item,
        icon: iconForNavItem(item),
        imageUrl: navItemImageUrl(item),
      })),
    [pageContent.navItems]
  );
  const navGroups = useMemo(
    () => groupedParentPortalNavItems(pageContent.navGroups, navItems),
    [navItems, pageContent.navGroups]
  );
  const controlAreas = pageContent.controlAreas;
  const quickControls = useMemo<QuickControl[]>(() => {
    const controlEntries = pageContent.quickControls.filter(isParentPortalControlEntry).map((control) => ({
      ...control,
      icon: iconForName(control.icon),
    }));
    if (controlEntries.length > 0) return controlEntries;
    return pageContent.controlAreas.map((control) => ({
      id: control.id,
      name: control.name,
      detail: typeof control.controlCode === 'number' ? `Area ${control.controlCode}` : 'View parent portal',
      icon: ManageFileSettingsIcon,
      tone: control.tone,
      category: control.category,
      subcategory: control.subcategory,
      controlCode: control.controlCode,
      routePath: control.routePath,
    }));
  }, [pageContent.quickControls, pageContent.controlAreas]);
  const [surfaceSize, setSurfaceSize] = useState({ width: 0, height: 0 });
  const canvasSize = useMemo(
    () => parentPortalCanvasSizeForSurface(baseCfg, surfaceSize),
    [baseCfg, surfaceSize.height, surfaceSize.width]
  );
  const columns = useMemo(
    () => responsiveParentPortalColumnWidths(canvasSize.width, baseCfg),
    [baseCfg, canvasSize.width]
  );
  const cfg = useMemo<ParentPortalSvgControls>(
    () => ({
      ...baseCfg,
      canvas: {
        ...baseCfg.canvas,
        width: canvasSize.width,
        height: canvasSize.height,
      },
      layout: {
        ...baseCfg.layout,
        leftW: columns.leftW,
        rightW: columns.rightW,
      },
    }),
    [baseCfg, canvasSize.height, canvasSize.width, columns.leftW, columns.rightW]
  );
  const pageModeTab = initialTabForPageMode(pageMode, pageContent);
  const initialNavItem = navItems.find((item) => item.label === initialNavLabel);
  const initialTab = initialNavItem?.tabId ?? pageModeTab;
  const [activeTab, setActiveTab] = useState<ParentPortalTabId>(initialTab);
  const [activeNavLabel, setActiveNavLabel] = useState(
    () => initialNavItem?.label ?? initialNavLabelForTab(navItems, pageModeTab)
  );
  const [openNavGroupIds, setOpenNavGroupIds] = useState(() =>
    initialOpenNavGroupIds(navGroups, initialNavItem?.label ?? initialNavLabelForTab(navItems, pageModeTab))
  );
  const [selectedControlId, setSelectedControlId] = useState(
    initialSelectedControlId ?? initialControlIdForPageMode(pageMode, pageContent, controlId)
  );
  const selectedControl = findSelectedControl(pageContent, selectedControlId);
  const selectedControlName = selectedControl?.name ?? formatRouteScope(controlId);
  const baseSourceRows = useMemo(
    () => rowSourceForPageMode(pageContent, pageMode, parentPortalRows),
    [parentPortalRows, pageContent, pageMode]
  );
  const sourceRows = activeTab === 'aiStatus' ? pageContent.aiBenchmarkRows : baseSourceRows;
  const rows = useMemo(
    () => toDisplayRows(sourceRows, pageMode, selectedControlName, controlId),
    [controlId, pageMode, selectedControlName, sourceRows]
  );
  const userDisplayRow = useMemo(
    () =>
      userEntry && pageMode !== 'parentGuide'
        ? toDisplayRows([userEntry], pageMode, selectedControlName, controlId)[0]
        : null,
    [controlId, pageMode, selectedControlName, userEntry]
  );
  const [selectedRowId, setSelectedRowId] = useState(userDisplayRow?.id ?? rows[0]?.id ?? '');
  const [, setPage] = useState(1);
  const rowsPerPage = 10;
  const [detailMode, setDetailMode] = useState<DetailMode | null>(null);

  useEffect(() => {
    const nextNavItem = navItems.find((item) => item.label === initialNavLabel);
    const nextTab = nextNavItem?.tabId ?? pageModeTab;
    const nextNavLabel = nextNavItem?.label ?? initialNavLabelForTab(navItems, nextTab);
    setActiveTab(nextTab);
    setActiveNavLabel(nextNavLabel);
    setOpenNavGroupIds((current) => ensureOpenNavGroupIds(current, navGroups, nextNavLabel));
    setSelectedControlId(initialSelectedControlId ?? initialControlIdForPageMode(pageMode, pageContent, controlId));
    setDetailMode(null);
    setPage(1);
  }, [controlId, initialNavLabel, initialSelectedControlId, navGroups, navItems, pageContent, pageMode, pageModeTab]);

  useEffect(() => {
    const target = mainRef.current;
    if (!target || typeof ResizeObserver === 'undefined') return undefined;
    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) return;
      const { width, height } = entry.contentRect;
      setSurfaceSize((current) => {
        const nextWidth = Math.round(width);
        const nextHeight = Math.round(height);
        return current.width === nextWidth && current.height === nextHeight
          ? current
          : { width: nextWidth, height: nextHeight };
      });
    });
    observer.observe(target);
    return () => observer.disconnect();
  }, []);

  const selectedRow =
    rows.find((row) => row.id === selectedRowId) ??
    (activeTab === 'aiStatus' ? null : userDisplayRow) ??
    rows[0] ??
    toDisplayRows(pageContent.fallbackRows, pageMode, selectedControlName, controlId)[0];
  const leftX = cfg.layout.outerPad;
  const sideMainGap = Math.max(1, Math.round(cfg.layout.gap * 0.5));
  const mainX = leftX + cfg.layout.leftW + sideMainGap;
  const rightX = cfg.canvas.width - cfg.layout.outerPad - cfg.layout.rightW;
  const boardY = cfg.layout.topY;
  const mainW = rightX - mainX;
  const mainH = cfg.canvas.height - boardY - 6;
  const activeNavGroupId = navGroupIdForNavLabel(navGroups, activeNavLabel);
  const activateNavLabel = (navLabel: string) => {
    setActiveNavLabel(navLabel);
    setOpenNavGroupIds((current) => ensureOpenNavGroupIds(current, navGroups, navLabel));
  };
  const changeTab = (tab: ParentPortalTabId) => {
    const nextNavLabel = initialNavLabelForTab(navItems, tab);
    setActiveTab(tab);
    activateNavLabel(nextNavLabel);
    setDetailMode(null);
    setPage(1);
  };
  const selectNavItem = (item: NavItem) => {
    setActiveTab(item.tabId);
    activateNavLabel(item.label);
    setDetailMode(null);
    setPage(1);
    if (isHashRoutePath(item.routePath)) {
      onNavigate?.(item.routePath);
    }
  };
  const toggleNavGroup = (groupId: string) => {
    setOpenNavGroupIds((current) => toggleOpenNavGroupId(current, navGroups, groupId));
  };
  const selectRow = (rowId: string) => {
    setSelectedRowId(rowId);
    setDetailMode(null);
  };
  const selectControl = (controlIdValue: string) => {
    setSelectedControlId(controlIdValue);
    const control = findSelectedControl(pageContent, controlIdValue);
    const tab =
      control?.routePath === '#/ai-runtime' || assetKey(control?.category ?? '').includes('ai')
        ? 'aiStatus'
        : control?.routePath === '#/overview'
          ? 'overall'
          : 'controls';
    const nextNavLabel = initialNavLabelForTab(navItems, tab);
    setActiveTab(tab);
    activateNavLabel(nextNavLabel);
    setDetailMode(null);
    setPage(1);
    if (pageMode === 'parentManage' && typeof control?.controlCode === 'number') {
      onRefreshParentPortal(control.controlCode);
    }
    if (isHashRoutePath(control?.routePath)) {
      onNavigate?.(control.routePath);
    }
  };
  return (
    <main ref={mainRef} className="parent-portal-svg-main">
      <svg
        viewBox={`0 0 ${cfg.canvas.width} ${cfg.canvas.height}`}
        className="parent-portal-svg-surface"
        role="img"
        aria-label="Ocentra parent dashboard"
        preserveAspectRatio="xMidYMin meet"
      >
        <Defs />
        <NavPanel
          activeNavLabel={activeNavLabel}
          navGroups={navGroups}
          openGroupIds={openNavGroupIds}
          onNavGroupToggle={toggleNavGroup}
          onNavItemSelect={selectNavItem}
          cfg={cfg}
        />
        <MainBoard
          activeNavLabel={activeNavLabel}
          activeNavGroupId={activeNavGroupId}
          activeTab={activeTab}
          rows={rows}
          tabs={tabs}
          tabDetails={tabDetails}
          controlAreas={controlAreas}
          quickControls={quickControls}
          guideTopics={pageContent.guideTopics}
          season={pageContent.season}
          uiCopy={pageContent.uiCopy}
          selectedControlId={selectedControlId}
          selectedRowId={selectedRow.id}
          selectedRow={selectedRow}
          selectedControlName={selectedControlName}
          rowsPerPage={rowsPerPage}
          detailMode={detailMode}
          onTabChange={changeTab}
          onRowSelect={selectRow}
          onPageChange={setPage}
          onDetailClose={() => setDetailMode(null)}
          onControlSelect={selectControl}
          onNavigate={onNavigate}
          onSelectNavLabel={(navLabel) => {
            const item = navItems.find((entry) => entry.label === navLabel);
            if (item) {
              selectNavItem(item);
              return;
            }
            activateNavLabel(navLabel);
          }}
          cfg={cfg}
          mainX={mainX}
          mainW={mainW}
          mainY={boardY}
          mainH={mainH}
        />
        {loading || error ? (
          <g role={loading ? 'status' : 'alert'}>
            <rect
              x={mainX + 28}
              y={boardY + 120}
              width={mainW - 56}
              height={82}
              rx={6}
              fill="rgba(3, 7, 18, 0.82)"
              stroke={error ? cfg.colors.red : cfg.colors.cyan}
              strokeWidth={1.2}
            />
            <text
              x={mainX + mainW / 2}
              y={boardY + 154}
              textAnchor="middle"
              fontSize={16}
              fontWeight={950}
              fill={cfg.colors.bodyText}
            >
              {loading ? pageContent.uiCopy.loadingTitle : pageContent.uiCopy.errorTitle}
            </text>
            <text
              x={mainX + mainW / 2}
              y={boardY + 180}
              textAnchor="middle"
              fontSize={11}
              fontWeight={760}
              fill={cfg.colors.mutedText}
            >
              {error ?? pageContent.uiCopy.loadingBody}
            </text>
          </g>
        ) : null}
      </svg>
    </main>
  );
}
