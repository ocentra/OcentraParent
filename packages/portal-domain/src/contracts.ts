import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
export {
  decodePortalClipboardText,
  decodePortalDetailValue,
  PortalClipboardTextSchema,
  PortalDetailValueSchema,
  type PortalClipboardText,
  type PortalDetailValue,
} from './detail-values';
export { PortalActivitySurfaceDefaultRequestPayload, PortalCommandButtons, PortalOverviewCommands } from './commands';
export { PortalBrowserInventoryFields, PortalDetails, PortalReadableValues } from './details';
export { PortalDiagnostics } from './diagnostics';
export {
  createAppGameNotificationParentSurfacePanelIntent,
  type AppGameNotificationParentSurfaceDetail,
  type AppGameNotificationParentSurfacePanelIntent,
  type AppGameNotificationParentSurfacePanelRow,
} from './app-game-notification-parent-surface-panel';
export {
  createLocalAiRuntimePanelIntent,
  type LocalAiRuntimePanelCard,
  type LocalAiRuntimePanelDetail,
  type LocalAiRuntimePanelIntent,
} from './local-ai-runtime-panel';
export { createAppGameNotificationParentSurfaceReadModelFromReadiness } from './app-game-notification-parent-surface-live-readiness';
export {
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
} from './frame-tuner';
export { PortalFrameColorFields, PortalFrameTunerFrameSections } from './frame-tuner-lists';
export { PortalAssets, PortalExternalLinks, PortalUnifiedChrome } from './unified-chrome';
export {
  createPortalAppLayoutButtonDraft,
  createPortalAppLayoutFoldoutDraft,
  defaultPortalAppLayoutContentDraft,
  defaultPortalAppLayoutSurfaceContent,
  normalizePortalAppLayoutContentDraft,
  normalizePortalAppLayoutSurfaceContentDraft,
  type PortalAppLayoutButtonDraft,
  type PortalAppLayoutContentAreaKey,
  type PortalAppLayoutContentDraft,
  type PortalAppLayoutFoldoutDraft,
  type PortalAppLayoutSurfaceContentDraft,
  type PortalAppLayoutSurfaceKey,
  type PortalAppLayoutTone,
} from './app-layout';
export {
  PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION,
  PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS,
  type ParentAssistantPortalQuickAction,
  type ParentAssistantPortalQuickActionId,
} from './parent-assistant-chat';
export {
  PARENT_PORTAL_CONTENT,
  PARENT_PORTAL_ROUTE,
  PARENT_PORTAL_ROUTE_CONTEXT,
  PARENT_PORTAL_ROWS,
  parentPortalRouteContext,
  type ParentPortalContent,
  type ParentPortalIconName,
  type ParentPortalPageMode,
  type ParentPortalRow,
  type ParentPortalRowSource,
  type ParentPortalRouteContext,
  type ParentPortalTabId,
  type ParentPortalTone,
} from './parent-portal-data';
export {
  PARENT_PORTAL_SERVICE_STATE,
  resolveParentPortalServiceState,
  type ParentPortalServiceState,
} from './parent-portal-service-state';
export {
  TrackingStatusProofArtifacts,
  TrackingStatusProofArtifactSchema,
  type TrackingStatusProofArtifact,
} from './tracking-status-proof-artifacts';
export {
  PARENT_PORTAL_NAV_LABELS,
  type ParentPortalHashRoutePath,
  type ParentPortalNavGroup,
  type ParentPortalNavGroupId,
  type ParentPortalNavItem,
  type ParentPortalNavLabel,
  type ParentPortalNavSectionLabel,
} from './parent-portal-nav';
export {
  PARENT_PORTAL_GUIDE_QUERY,
  PARENT_PORTAL_GUIDE_TOPICS,
  PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES,
  PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS,
  type ParentPortalGuideNote,
  type ParentPortalGuidePage,
  type ParentPortalGuideTopic,
} from './parent-portal-guides';
export {
  parseActivityMemoryGraphReadModel,
  type PortalActivityMemoryGraphEdge,
  type PortalActivityMemoryGraphNode,
  type PortalActivityMemoryGraphNodeId,
  type PortalActivityMemoryGraphReadModel,
} from './activity-memory-graph';
export {
  createAppGamePolicyReadinessPanelIntent,
  type AppGamePolicyReadinessPanelDetail,
  type AppGamePolicyReadinessPanelIntent,
  type AppGamePolicyReadinessPanelRow,
} from './app-game-policy-readiness-panel';
export {
  createBrowserParentExplanationPanelIntent,
  type BrowserParentExplanationPanelDetail,
  type BrowserParentExplanationPanelIntent,
  type BrowserParentExplanationPanelRow,
} from './browser-parent-explanation-panel';

export {
  createSocialDashboardPanelIntent,
  type SocialDashboardPanelDetail,
  type SocialDashboardPanelIntent,
  type SocialDashboardPanelRow,
} from './social-dashboard-panel';
export {
  createSocialAuditExplanationPanelIntent,
  type SocialAuditExplanationPanelDetail,
  type SocialAuditExplanationPanelIntent,
  type SocialAuditExplanationPanelRow,
} from './social-audit-explanation-panel';
export {
  createSocialAlertReportPanelIntent,
  type SocialAlertReportPanelDetail,
  type SocialAlertReportPanelIntent,
  type SocialAlertReportPanelRow,
} from './social-alert-report-panel';
export {
  createSocialParentNotificationDeliveryPanelIntent,
  type SocialParentNotificationDeliveryPanelDetail,
  type SocialParentNotificationDeliveryPanelIntent,
  type SocialParentNotificationDeliveryPanelRow,
} from './social-parent-notification-delivery-panel';
export {
  createSocialAlertReportParentSurfacePanelIntent,
  type SocialAlertReportParentSurfacePanelDetail,
  type SocialAlertReportParentSurfacePanelIntent,
  type SocialAlertReportParentSurfacePanelRow,
} from './social-alert-report-parent-surface-panel';
export {
  createBrowserSocialProviderReceiptStreamStatusIntent,
  type BrowserSocialProviderReceiptStreamStatusDetail,
  type BrowserSocialProviderReceiptStreamStatusIntent,
} from './browser-social-provider-receipt-stream-status';
export {
  createBrowserSocialProviderReceiptIngestionReadinessStatusIntent,
  type BrowserSocialProviderReceiptIngestionReadinessStatusDetail,
  type BrowserSocialProviderReceiptIngestionReadinessStatusIntent,
} from './browser-social-provider-receipt-ingestion-readiness-status';
export {
  PortalRoute,
  PortalDevToolWindow,
  PortalAiRuntimeRoutes,
  PortalRouteDescriptors,
  PortalAppGameParentSurfaceRoutes,
  PortalBrowserParentSurfaceRoutes,
  PortalNetworkEvidenceDrawerRoutes,
  PortalRouteGroup,
  PortalRoutes,
  PortalRouteSchema,
  PortalScreenSettingsRoutes,
  PortalScreenSummaryRoutes,
  PortalSidebarRouteDescriptors,
  PortalTrackingStatusRoutes,
  PortalDevToolUrlSchema,
  isPortalAiRuntimeRoute,
  isPortalAppGameParentSurfaceRoute,
  isPortalBrowserParentSurfaceRoute,
  isPortalNetworkEvidenceDrawerRoute,
  isPortalScreenSettingsRoute,
  isPortalScreenSummaryRoute,
  isPortalTrackingStatusRoute,
  portalDevToolUrl,
  type PortalDevToolUrl,
  type PortalRouteDescriptor,
  type PortalRouteGroupValue,
} from './routes';
export {
  DEFAULT_PORTAL_BACKGROUND_CONFIG,
  DEFAULT_PORTAL_BACKGROUND_DARK_COLORS,
  DEFAULT_PORTAL_BACKGROUND_LIGHT_COLORS,
  PortalBackgroundRuntime,
  normalizePortalBackgroundConfig,
  portalBackgroundAppRenderConfig,
  portalBackgroundRenderConfig,
  type PortalBackgroundConfig,
  type PortalBackgroundRenderConfig,
  type PortalBackgroundThemeColors,
} from './portal-background';
export {
  PORTAL_BACKGROUND_SVG_HEIGHT,
  PORTAL_BACKGROUND_SVG_WIDTH,
  portalBackgroundSvgContent,
  portalBackgroundSvgMarkup,
} from './portal-background-svg-markup';
export {
  BrowserChildInterventionPageDefaults,
  BrowserChildInterventionPageSamples,
  renderBrowserChildInterventionPage,
  type BrowserChildInterventionPageAction,
  type BrowserChildInterventionPageBackdrop,
  type BrowserChildInterventionPageModel,
  type BrowserChildInterventionPageTheme,
} from './browser-child-intervention-page';
export {
  createSocialChildInterventionPageModels,
  type SocialChildInterventionPageModelOptions,
  type SocialChildInterventionPageModelResult,
  type SocialChildInterventionRequestedUrlResolver,
} from './social-child-intervention-page-model';

export type PortalDisplayText = DisplayText;

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
  HashQuerySeparator: '?',
  BackgroundDevToolHashFlag: 'bg-only=1',
  EmptyHashRoute: '',
  Ids: {
    AppLoading: 'app-loading',
  },
  Runtime: {
    HideAppLoading: '__hideAppLoading',
    Undefined: 'undefined',
  },
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
    AppLoadingHidden: 'hidden',
    AppLoadingHide: 'hide',
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
    TrackingStatusOverlay: 'tracking-status-overlay',
    TrackingStatusOverlayContent: 'tracking-status-overlay-content',
    TrackingStatusOverlayGrid: 'tracking-status-overlay-grid',
    TrackingStatusOverlayHeader: 'tracking-status-overlay-header',
    TrackingStatusOverlayMeta: 'tracking-status-overlay-meta',
  },
  Attributes: {
    AriaCurrent: 'aria-current',
    AriaHidden: 'aria-hidden',
    AriaSelected: 'aria-selected',
    DataRouteId: 'data-ocentra-parent-route-id',
    DataTrackingProof: 'data-ocentra-tracking-proof',
    DataTheme: 'data-theme',
    Page: 'page',
    True: 'true',
    False: 'false',
    ReadOnly: 'readonly',
    Role: 'role',
    Tab: 'tab',
    TabList: 'tablist',
    TrackingProofCitationDetail: 'service-backed-citation-detail',
    TrackingProofChildCheckIn: 'child-check-in',
    TrackingProofChildRuntimeUi: 'child-runtime-ui',
    TrackingProofEvidenceDrawer: 'service-backed-evidence-drawer',
    TrackingProofFamilyDashboard: 'family-dashboard-rollup',
    TrackingProofRetentionSettings: 'retention-settings-ui',
  },
  Events: {
    Click: 'click',
    Close: 'close',
    Error: 'error',
    HashChange: 'hashchange',
    Message: 'message',
    Open: 'open',
    Storage: 'storage',
    TransitionEnd: 'transitionend',
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

export const PortalLanPairingScan = {
  PendingIndicatorMs: 8000,
  Text: {
    HeaderTitle: decodeDisplayText('Local Area Network'),
    Scan: decodeDisplayText('SCAN'),
    Scanning: decodeDisplayText('SCANNING'),
    ScanLocalAreaNetwork: decodeDisplayText('Scan Local Area Network'),
    OpenLocalAreaNetworkGuide: decodeDisplayText('Open Local Area Network guide'),
  },
} as const;

export const PortalTiming = {
  CopyFeedbackMs: 1200,
} as const;

export const PortalEnvironment = {
  AgentWebSocketUrl: 'VITE_AGENT_WS_URL',
  BrowserParentExplanationProofBundle: 'VITE_BROWSER_PARENT_EXPLANATION_PROOF_BUNDLE',
  SocialAuditExplanationProofBundle: 'VITE_SOCIAL_AUDIT_EXPLANATION_PROOF_BUNDLE',
} as const;

const PortalText = {
  Resolve: resolvePortalDevText,
} as const;
const PortalTextToken = PortalDevTextToken;

export { PortalText, PortalTextToken };
