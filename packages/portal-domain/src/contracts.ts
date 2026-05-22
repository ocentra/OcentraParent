import { type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
export { PortalCommandButtons, PortalOverviewCommands } from './commands';
export { PortalDetails, PortalReadableValues } from './details';
export { PortalDiagnostics } from './diagnostics';
export {
  PortalAuthChrome,
  type PortalAuthAutoComplete,
  type PortalAuthInputType,
  type PortalAuthMode,
} from './auth-chrome';
export { PortalAssets, PortalExternalLinks, PortalUnifiedChrome } from './unified-chrome';
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
    ControlCardGlyph: 'control-card-glyph',
    ControlCardHeader: 'control-card-header',
    ControlCardStatus: 'control-card-status',
    ControlCardTip: 'control-card-tip',
    ControlCardTipBody: 'control-card-tip-body',
    ControlCardTipTitle: 'control-card-tip-title',
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
    RouteLinkDescription: 'route-link-description',
    RouteLinkLabel: 'route-link-label',
    Routes: 'routes',
    Shell: 'shell',
    SettingsThemePanel: 'settings-theme-panel',
    SidebarActions: 'sidebar-actions',
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
