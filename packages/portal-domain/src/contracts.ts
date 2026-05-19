import { AgentCommand, AgentProtocolDefaults } from '@ocentra-parent/agent-protocol-domain/contracts';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyPortalText = Schema.String.pipe(Schema.minLength(1));

export const PortalRouteSchema = withParser(Schema.Literal('overview', 'commands', 'events'));
export type PortalRoute = Infer<typeof PortalRouteSchema>;

export const PortalDetailValueSchema = withParser(NonEmptyPortalText.pipe(Schema.brand('PortalDetailValue')));
export type PortalDetailValue = Infer<typeof PortalDetailValueSchema>;
export type PortalDisplayText = DisplayText;

export const decodePortalDetailValue = PortalDetailValueSchema.parse;

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
  },
  Classes: {
    ClassNameSeparator: ' ',
    CommandGrid: 'command-grid',
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

export const PortalFormatting = {
  EventDetailSeparator: ' | ',
  CorrelationPrefix: 'correlation ',
} as const;

export const PortalEnvironment = {
  AgentWebSocketUrl: 'VITE_AGENT_WS_URL',
} as const;

export const PortalText = {
  Resolve: resolvePortalDevText,
} as const;
export const PortalTextToken = PortalDevTextToken;

export const PortalDetails = {
  Transport: decodeDisplayText('Transport'),
  State: decodeDisplayText('State'),
  Events: decodeDisplayText('Events'),
  Device: decodeDisplayText('Device'),
  Host: decodeDisplayText('Host'),
  Platform: decodeDisplayText('Platform'),
  Version: decodeDisplayText('Version'),
  Schema: decodeDisplayText('Schema'),
} as const;

export const PortalCommandButtons = [
  {
    label: resolvePortalDevText(PortalDevTextToken.CheckHealth),
    command: AgentCommand.HealthCheck,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetLogSnapshot),
    command: AgentCommand.LogSnapshotGet,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.EchoPortalPing),
    command: AgentCommand.DevEcho,
    payload: {
      [AgentProtocolDefaults.Field.Message]: resolvePortalDevText(PortalDevTextToken.EchoPortalPing),
    },
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetWatcherStatus),
    command: AgentCommand.WatchStatusGet,
    payload: {},
  },
] as const;
