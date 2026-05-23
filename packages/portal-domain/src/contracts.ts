import { type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
export { PortalCommandButtons, PortalOverviewCommands } from './commands';
export { PortalDetails, PortalReadableValues } from './details';
export { PortalDiagnostics } from './diagnostics';
export {
  PortalFrameColorFields,
  PortalFrameChromeNumberFields,
  PortalCarouselContentNumberFields,
  PortalCarouselFrameNumberFields,
  PortalCarouselRailNumberFields,
  PortalFrameContentBooleanFields,
  PortalFrameContentNumberFields,
  PortalFrameGeometryNumberFields,
  PortalGoldenCardBooleanFields,
  PortalGoldenCardContentNumberFields,
  PortalGoldenCardFrameNumberFields,
  PortalFrameInnerEdgeNumberFields,
  PortalFrameInnerGapNumberFields,
  PortalFrameInnerSegmentNumberFields,
  PortalFrameInnerShapeNumberFields,
  PortalFrameOuterEdgeNumberFields,
  PortalFrameOuterGapNumberFields,
  PortalFrameOuterSegmentNumberFields,
  PortalFrameOuterShapeNumberFields,
  PortalFrameShellNumberFields,
  PortalFrameSlotNumberFields,
  PortalFrameTuner,
  PortalFrameTunerFrameSections,
  PortalFrameTunerPanels,
  portalFrameCssNumber,
  portalFrameCssOpacity,
  portalFrameCssPercent,
  portalFrameCssPixel,
  type PortalFrameCssStyle,
  type PortalFrameCssValue,
  type PortalFrameBooleanField,
  type PortalFrameColorField,
  type PortalFrameNumberField,
  type PortalFrameTargetValue,
  type PortalFrameTunerFrameSectionValue,
  type PortalFrameTunerPanelValue,
} from './frame-tuner';
export {
  PortalAuthChrome,
  type PortalAuthAutoComplete,
  type PortalAuthInputType,
  type PortalAuthMode,
} from './auth-chrome';
export { PortalAssets, PortalExternalLinks, PortalUnifiedChrome } from './unified-chrome';
export {
  PARENT_LEADERBOARD_COPY_CONTENT,
  PARENT_LEADERBOARD_COPY_ROUTE,
  PARENT_LEADERBOARD_COPY_ROUTE_CONTEXT,
  PARENT_LEADERBOARD_COPY_ROWS,
  parentLeaderboardCopyRouteContext,
  type ParentLeaderboardCopyContent,
  type ParentLeaderboardCopyIconName,
  type ParentLeaderboardCopyPageMode,
  type ParentLeaderboardCopyRow,
  type ParentLeaderboardCopyRowSource,
  type ParentLeaderboardCopyRouteContext,
  type ParentLeaderboardCopyTabId,
  type ParentLeaderboardCopyTone,
} from './parent-leaderboard-copy-data';
export {
  type ParentLeaderboardCopyNavGroup,
  type ParentLeaderboardCopyNavGroupId,
  type ParentLeaderboardCopyNavItem,
} from './parent-leaderboard-copy-nav';
export {
  PARENT_LEADERBOARD_COPY_GUIDE_TOPICS,
  type ParentLeaderboardCopyGuideNote,
  type ParentLeaderboardCopyGuidePage,
  type ParentLeaderboardCopyGuideTopic,
} from './parent-leaderboard-copy-guides';
export {
  parseActivityMemoryGraphReadModel,
  type PortalActivityMemoryGraphEdge,
  type PortalActivityMemoryGraphNode,
  type PortalActivityMemoryGraphNodeId,
  type PortalActivityMemoryGraphReadModel,
} from './activity-memory-graph';
export {
  PortalRoute,
  PortalRouteDescriptors,
  PortalRouteGroup,
  PortalRoutes,
  PortalRouteSchema,
  PortalSidebarRouteDescriptors,
  type PortalRouteDescriptor,
  type PortalRouteGroupValue,
} from './routes';

const NonEmptyPortalText = Schema.String.pipe(Schema.minLength(1));

export const PortalDetailValueSchema = withParser(NonEmptyPortalText.pipe(Schema.brand('PortalDetailValue')));
export const PortalClipboardTextSchema = withParser(NonEmptyPortalText.pipe(Schema.brand('PortalClipboardText')));
export type PortalDetailValue = Infer<typeof PortalDetailValueSchema>;
export type PortalClipboardText = Infer<typeof PortalClipboardTextSchema>;
export type PortalDisplayText = DisplayText;

export const decodePortalDetailValue = PortalDetailValueSchema.parse;
export const decodePortalClipboardText = PortalClipboardTextSchema.parse;

export const PortalConnectionStateSchema = withParser(
  Schema.Literal('disconnected', 'connecting', 'connected', 'error')
);
export type PortalConnectionState = Infer<typeof PortalConnectionStateSchema>;

export const PortalConnectionState = {
  Disconnected: PortalConnectionStateSchema.parse('disconnected'),
  Connecting: PortalConnectionStateSchema.parse('connecting'),
  Connected: PortalConnectionStateSchema.parse('connected'),
  Error: PortalConnectionStateSchema.parse('error'),
} as const;

export const PortalDom = {
  RootSelector: '#app',
  HashPrefix: '#/',
  EmptyHashRoute: '',
  Tags: {
    Aside: 'aside',
    Anchor: 'a',
    Button: 'button',
    Code: 'code',
    DefinitionDescription: 'dd',
    DefinitionList: 'dl',
    DefinitionTerm: 'dt',
    Details: 'details',
    Division: 'div',
    Header: 'header',
    HeadingOne: 'h1',
    HeadingTwo: 'h2',
    ListItem: 'li',
    Main: 'main',
    Navigation: 'nav',
    OrderedList: 'ol',
    Paragraph: 'p',
    Section: 'section',
    Strong: 'strong',
    SummaryTag: 'summary',
    Span: 'span',
    Image: 'img',
    TextArea: 'textarea',
    UnorderedList: 'ul',
  },
  Classes: {
    AppChrome: 'app-chrome',
    AppContent: 'app-content',
    AppFrame: 'app-frame',
    AppHeader: 'app-header',
    AppMain: 'app-main',
    AppRightRail: 'app-right-rail',
    AppSidebar: 'app-sidebar',
    AppStatus: 'app-status',
    AppStatusBar: 'app-status-bar',
    AppToolbar: 'app-toolbar',
    CapabilityGrid: 'capability-grid',
    CapabilityItem: 'capability-item',
    ClipboardBuffer: 'clipboard-buffer',
    ClassNameSeparator: ' ',
    CommandGrid: 'command-grid',
    ControlCard: 'control-card',
    ControlCardAccentPrimary: 'control-card-accent-primary',
    ControlCardAccentPrivacy: 'control-card-accent-privacy',
    ControlCardAccentWarn: 'control-card-accent-warn',
    ControlCardBody: 'control-card-body',
    ControlCardContent: 'control-card-content',
    ControlCardGlyph: 'control-card-glyph',
    ControlCardGoldenArt: 'control-card-golden-art',
    ControlCardHeader: 'control-card-header',
    ControlCardStatus: 'control-card-status',
    ControlCardTip: 'control-card-tip',
    ControlCardTipBody: 'control-card-tip-body',
    ControlCardTipTitle: 'control-card-tip-title',
    ControlCarouselCount: 'control-carousel-count',
    ControlCarouselFrame: 'control-carousel-frame',
    ControlCarouselHandle: 'control-carousel-handle',
    ControlCarouselHandleLeft: 'control-carousel-handle-left',
    ControlCarouselHandleRight: 'control-carousel-handle-right',
    ControlCarouselIntro: 'control-carousel-intro',
    ControlCarouselLabel: 'control-carousel-label',
    ControlCarouselPager: 'control-carousel-pager',
    ControlCarouselPill: 'control-carousel-pill',
    ControlCarouselPillActive: 'control-carousel-pill-active',
    ControlCarouselRail: 'control-carousel-rail',
    ControlCarouselStage: 'control-carousel-stage',
    ControlCarouselTitle: 'control-carousel-title',
    ControlDeck: 'control-deck',
    ControlDeckHeader: 'control-deck-header',
    ControlDeckIntro: 'control-deck-intro',
    DeviceRuleScopePanel: 'device-rule-scope-panel',
    CommandResultEmpty: 'command-result-empty',
    CommandResultHeader: 'command-result-header',
    CommandResultPanel: 'command-result-panel',
    CommandResultTab: 'command-result-tab',
    CommandResultTabActive: 'command-result-tab-active',
    CopyResultButton: 'copy-result-button',
    Header: 'header',
    Log: 'log',
    LogLevelPrefix: 'log-',
    LogList: 'log-list',
    PageDescription: 'page-description',
    PageHeader: 'page-header',
    PageTitle: 'page-title',
    PanelGrid: 'panel-grid',
    PanelGridWide: 'panel-grid-wide',
    ProductDashboard: 'product-dashboard',
    ProductHeroCopy: 'product-hero-copy',
    ProductMetric: 'product-metric',
    ProductMetricLabel: 'product-metric-label',
    ProductMetricMeta: 'product-metric-meta',
    ProductMetricValue: 'product-metric-value',
    ProductStatusCard: 'product-status-card',
    ProductStatusCardBadge: 'product-status-card-badge',
    ProductStatusCardBody: 'product-status-card-body',
    ProductStatusCardEvidence: 'product-status-card-evidence',
    ProductStatusCardManaged: 'product-status-card-managed',
    ProductStatusCardMedia: 'product-status-card-media',
    ProductStatusCardMeta: 'product-status-card-meta',
    ProductStatusCardMetaItem: 'product-status-card-meta-item',
    ProductStatusCardMetaLabel: 'product-status-card-meta-label',
    ProductStatusCardMetaValue: 'product-status-card-meta-value',
    ProductStatusCardProtection: 'product-status-card-protection',
    ProductBadge: 'product-badge',
    ProductBrand: 'product-brand',
    ProductBrandMark: 'product-brand-mark',
    ProductEyebrow: 'product-eyebrow',
    ProductPanelNote: 'product-panel-note',
    ProductShellHero: 'product-shell-hero',
    ProductSidebarPanel: 'product-sidebar-panel',
    RightRailCard: 'right-rail-card',
    RightRailStrip: 'right-rail-strip',
    RightRailTitle: 'right-rail-title',
    RouteTabs: 'route-tabs',
    RouteGroup: 'route-group',
    RouteGroupLabel: 'route-group-label',
    RouteLink: 'route-link',
    RouteLinkArrow: 'route-link-arrow',
    RouteLinkCopy: 'route-link-copy',
    RouteLinkDescription: 'route-link-description',
    RouteLinkFrame: 'route-link-frame',
    RouteLinkIcon: 'route-link-icon',
    RouteLinkLabel: 'route-link-label',
    Routes: 'routes',
    Shell: 'shell',
    SettingsThemePanel: 'settings-theme-panel',
    SidebarActions: 'sidebar-actions',
    SidebarDeviceFrame: 'sidebar-device-frame',
    SidebarNavFrame: 'sidebar-nav-frame',
    SidebarReconnectButton: 'sidebar-reconnect-button',
    SidebarStatusButton: 'sidebar-status-button',
    State: 'state',
    Summary: 'summary',
    ThemeToggle: 'theme-toggle',
    ThemeToggleButton: 'theme-toggle-button',
    ThemeToggleButtonActive: 'theme-toggle-button-active',
  },
  Attributes: {
    AriaCurrent: 'aria-current',
    AriaHidden: 'aria-hidden',
    AriaSelected: 'aria-selected',
    DataRouteId: 'data-ocentra-parent-route-id',
    DataTheme: 'data-theme',
    Page: 'page',
    True: 'true',
    False: 'false',
    ReadOnly: 'readonly',
    Role: 'role',
    Tab: 'tab',
    TabList: 'tablist',
  },
  Events: {
    Click: 'click',
    Close: 'close',
    Error: 'error',
    HashChange: 'hashchange',
    Message: 'message',
    Open: 'open',
    Storage: 'storage',
  },
  ButtonType: {
    Button: 'button',
    Submit: 'submit',
  },
} as const;

export const PortalTheme = {
  Dark: 'dark',
  Light: 'light',
  LocalStorageKey: 'ocentra-parent-theme',
  MediaDark: '(prefers-color-scheme: dark)',
} as const;
export type PortalThemeValue = (typeof PortalTheme)[keyof Pick<typeof PortalTheme, 'Dark' | 'Light'>];

export const PortalClipboard = {
  CommandCopy: 'copy',
} as const;

export const PortalFormatting = {
  EventDetailSeparator: ' | ',
  CorrelationPrefix: 'correlation ',
  EndpointSeparator: ':',
  GraphEdgeSeparator: ' -> ',
} as const;

export const PortalTiming = {
  CopyFeedbackMs: 1200,
} as const;

export const PortalEnvironment = {
  AgentWebSocketUrl: 'VITE_AGENT_WS_URL',
} as const;

export const PortalText = {
  Resolve: resolvePortalDevText,
} as const;
export const PortalTextToken = PortalDevTextToken;
