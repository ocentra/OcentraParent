import { type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
export { PortalCommandButtons, PortalOverviewCommands } from './commands';
export { PortalDetails } from './details';
export { PortalDiagnostics } from './diagnostics';

const NonEmptyPortalText = Schema.String.pipe(Schema.minLength(1));

export const PortalRouteSchema = withParser(Schema.Literal('overview', 'commands', 'events'));
export type PortalRoute = Infer<typeof PortalRouteSchema>;

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

export const PortalRoute = {
  Overview: PortalRouteSchema.parse('overview'),
  Commands: PortalRouteSchema.parse('commands'),
  Events: PortalRouteSchema.parse('events'),
} as const;

export const PortalRoutes = [PortalRoute.Overview, PortalRoute.Commands, PortalRoute.Events] as const;

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
    Anchor: 'a',
    Button: 'button',
    Code: 'code',
    DefinitionDescription: 'dd',
    DefinitionList: 'dl',
    DefinitionTerm: 'dt',
    Division: 'div',
    Header: 'header',
    HeadingOne: 'h1',
    HeadingTwo: 'h2',
    ListItem: 'li',
    Navigation: 'nav',
    OrderedList: 'ol',
    Paragraph: 'p',
    Section: 'section',
    Strong: 'strong',
    Span: 'span',
    TextArea: 'textarea',
  },
  Classes: {
    ClipboardBuffer: 'clipboard-buffer',
    ClassNameSeparator: ' ',
    CommandGrid: 'command-grid',
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
    Routes: 'routes',
    Shell: 'shell',
    State: 'state',
    Summary: 'summary',
  },
  Attributes: {
    AriaCurrent: 'aria-current',
    Page: 'page',
    ReadOnly: 'readonly',
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
  },
} as const;

export const PortalClipboard = {
  CommandCopy: 'copy',
} as const;

export const PortalFormatting = {
  EventDetailSeparator: ' | ',
  CorrelationPrefix: 'correlation ',
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
