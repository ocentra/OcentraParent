import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';

export const PortalDevTextToken = {
  AppTitle: decodeTextTokenId('portal.dev.appTitle'),
  Subtitle: decodeTextTokenId('portal.dev.subtitle'),
  Reconnect: decodeTextTokenId('portal.dev.reconnect'),
  Overview: decodeTextTokenId('portal.dev.route.overview'),
  Commands: decodeTextTokenId('portal.dev.route.commands'),
  Events: decodeTextTokenId('portal.dev.route.events'),
  Connected: decodeTextTokenId('portal.dev.connected'),
  Unavailable: decodeTextTokenId('portal.dev.unavailable'),
  AgentCommands: decodeTextTokenId('portal.dev.agentCommands'),
  AgentEvents: decodeTextTokenId('portal.dev.agentEvents'),
  CommandResult: decodeTextTokenId('portal.dev.commandResult'),
  CopyResult: decodeTextTokenId('portal.dev.copyResult'),
  CopiedResult: decodeTextTokenId('portal.dev.copiedResult'),
  CopyResultFailed: decodeTextTokenId('portal.dev.copyResultFailed'),
  NoCommandResult: decodeTextTokenId('portal.dev.noCommandResult'),
  LatestSnapshot: decodeTextTokenId('portal.dev.latestSnapshot'),
  CheckHealth: decodeTextTokenId('portal.dev.command.checkHealth'),
  GetLogSnapshot: decodeTextTokenId('portal.dev.command.getLogSnapshot'),
  EchoPortalPing: decodeTextTokenId('portal.dev.command.echoPortalPing'),
  GetWatcherStatus: decodeTextTokenId('portal.dev.command.getWatcherStatus'),
  RootMissing: decodeTextTokenId('portal.dev.rootMissing'),
} as const;

export type PortalDevTextTokenValue = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

export const PortalDevText: Record<PortalDevTextTokenValue, DisplayText> = {
  [PortalDevTextToken.AppTitle]: decodeDisplayText('Ocentra Parent'),
  [PortalDevTextToken.Subtitle]: decodeDisplayText('Local agent WebSocket command and event scaffold'),
  [PortalDevTextToken.Reconnect]: decodeDisplayText('Reconnect'),
  [PortalDevTextToken.Overview]: decodeDisplayText('overview'),
  [PortalDevTextToken.Commands]: decodeDisplayText('commands'),
  [PortalDevTextToken.Events]: decodeDisplayText('events'),
  [PortalDevTextToken.Connected]: decodeDisplayText('Agent WebSocket connected'),
  [PortalDevTextToken.Unavailable]: decodeDisplayText('Agent WebSocket unavailable'),
  [PortalDevTextToken.AgentCommands]: decodeDisplayText('Agent commands'),
  [PortalDevTextToken.AgentEvents]: decodeDisplayText('Agent events'),
  [PortalDevTextToken.CommandResult]: decodeDisplayText('Command result'),
  [PortalDevTextToken.CopyResult]: decodeDisplayText('Copy result'),
  [PortalDevTextToken.CopiedResult]: decodeDisplayText('Copied'),
  [PortalDevTextToken.CopyResultFailed]: decodeDisplayText('Copy failed'),
  [PortalDevTextToken.NoCommandResult]: decodeDisplayText('Run a command to see the latest response.'),
  [PortalDevTextToken.LatestSnapshot]: decodeDisplayText('Latest agent snapshot'),
  [PortalDevTextToken.CheckHealth]: decodeDisplayText('Check health'),
  [PortalDevTextToken.GetLogSnapshot]: decodeDisplayText('Get log snapshot'),
  [PortalDevTextToken.EchoPortalPing]: decodeDisplayText('Echo portal ping'),
  [PortalDevTextToken.GetWatcherStatus]: decodeDisplayText('Get watcher status'),
  [PortalDevTextToken.RootMissing]: decodeDisplayText('Portal root element is missing.'),
};

const MissingPortalDevTextTokenMessage = decodeDisplayText('Missing portal dev text token.');

export function resolvePortalDevText(token: PortalDevTextTokenValue): DisplayText {
  const text = PortalDevText[token];
  if (text === undefined) {
    throw new Error(MissingPortalDevTextTokenMessage);
  }
  return text;
}
