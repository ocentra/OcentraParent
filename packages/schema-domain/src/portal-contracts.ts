/* thin adapter over Rust-generated portal contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  GeneratedPortalConnectionState,
  type GeneratedPortalConnectionState as GeneratedPortalConnectionStateValue,
  type GeneratedPortalClipboardText,
  type GeneratedPortalDetailValue,
  type GeneratedPortalDevToolUrl,
  type GeneratedPortalRouteEventPayloadRecord,
  type GeneratedPortalRouteEventRole,
  type GeneratedPortalRouteEventSnapshot,
  GeneratedPortalRoute,
  type GeneratedPortalRoute as GeneratedPortalRouteValue,
  type GeneratedPortalRouteHashPath as GeneratedPortalRouteHashPathValue,
  GeneratedPortalRouteHashPrefix,
  type GeneratedPortalRouteHashQueryPath as GeneratedPortalRouteHashQueryPathValue,
  GeneratedPortalRouteHashQuerySeparator,
  GeneratedPortalRouteLiteral,
  type GeneratedTrackingStatusProofArtifact,
} from './generated/portal-contracts';

export const PortalRouteLiteral = GeneratedPortalRouteLiteral;

export const PortalRouteSchema = withParser(
  Schema.Literal(
    ...(Object.values(GeneratedPortalRouteLiteral) as [GeneratedPortalRouteValue, ...Array<GeneratedPortalRouteValue>])
  )
);
export type PortalRoute = Infer<typeof PortalRouteSchema> & GeneratedPortalRouteValue;

export const PortalRoute = {
  Overview: PortalRouteSchema.parse(GeneratedPortalRoute.Overview),
  Assistant: PortalRouteSchema.parse(GeneratedPortalRoute.Assistant),
  Start: PortalRouteSchema.parse(GeneratedPortalRoute.Start),
  Activity: PortalRouteSchema.parse(GeneratedPortalRoute.Activity),
  Browser: PortalRouteSchema.parse(GeneratedPortalRoute.Browser),
  BrowserSettings: PortalRouteSchema.parse(GeneratedPortalRoute.BrowserSettings),
  Policy: PortalRouteSchema.parse(GeneratedPortalRoute.Policy),
  PolicyApps: PortalRouteSchema.parse(GeneratedPortalRoute.PolicyApps),
  PolicyGames: PortalRouteSchema.parse(GeneratedPortalRoute.PolicyGames),
  PolicyScreen: PortalRouteSchema.parse(GeneratedPortalRoute.PolicyScreen),
  PolicyNetwork: PortalRouteSchema.parse(GeneratedPortalRoute.PolicyNetwork),
  PolicyTracking: PortalRouteSchema.parse(GeneratedPortalRoute.PolicyTracking),
  PolicyRemoteScreen: PortalRouteSchema.parse(GeneratedPortalRoute.PolicyRemoteScreen),
  RuleManagement: PortalRouteSchema.parse(GeneratedPortalRoute.RuleManagement),
  Schedules: PortalRouteSchema.parse(GeneratedPortalRoute.Schedules),
  Approvals: PortalRouteSchema.parse(GeneratedPortalRoute.Approvals),
  Enforcement: PortalRouteSchema.parse(GeneratedPortalRoute.Enforcement),
  PrivacyDesign: PortalRouteSchema.parse(GeneratedPortalRoute.PrivacyDesign),
  Memory: PortalRouteSchema.parse(GeneratedPortalRoute.Memory),
  MemorySettings: PortalRouteSchema.parse(GeneratedPortalRoute.MemorySettings),
  AiGuide: PortalRouteSchema.parse(GeneratedPortalRoute.AiGuide),
  AiRuntime: PortalRouteSchema.parse(GeneratedPortalRoute.AiRuntime),
  ApiProviders: PortalRouteSchema.parse(GeneratedPortalRoute.ApiProviders),
  ReportsGuide: PortalRouteSchema.parse(GeneratedPortalRoute.ReportsGuide),
  ScreenAnalysis: PortalRouteSchema.parse(GeneratedPortalRoute.ScreenAnalysis),
  AppGameSessions: PortalRouteSchema.parse(GeneratedPortalRoute.AppGameSessions),
  NetworkActivity: PortalRouteSchema.parse(GeneratedPortalRoute.NetworkActivity),
  Devices: PortalRouteSchema.parse(GeneratedPortalRoute.Devices),
  LanPairing: PortalRouteSchema.parse(GeneratedPortalRoute.LanPairing),
  CapabilityStatus: PortalRouteSchema.parse(GeneratedPortalRoute.CapabilityStatus),
  Notifications: PortalRouteSchema.parse(GeneratedPortalRoute.Notifications),
  NotificationChannels: PortalRouteSchema.parse(GeneratedPortalRoute.NotificationChannels),
  DriveConnections: PortalRouteSchema.parse(GeneratedPortalRoute.DriveConnections),
  ExportRetention: PortalRouteSchema.parse(GeneratedPortalRoute.ExportRetention),
  RemoteAccess: PortalRouteSchema.parse(GeneratedPortalRoute.RemoteAccess),
  ReportCompiler: PortalRouteSchema.parse(GeneratedPortalRoute.ReportCompiler),
  AuditHistory: PortalRouteSchema.parse(GeneratedPortalRoute.AuditHistory),
  Subscription: PortalRouteSchema.parse(GeneratedPortalRoute.Subscription),
  Entitlements: PortalRouteSchema.parse(GeneratedPortalRoute.Entitlements),
  PlatformsInstall: PortalRouteSchema.parse(GeneratedPortalRoute.PlatformsInstall),
  InstallUpdates: PortalRouteSchema.parse(GeneratedPortalRoute.InstallUpdates),
  Diagnostics: PortalRouteSchema.parse(GeneratedPortalRoute.Diagnostics),
  ProofPanels: PortalRouteSchema.parse(GeneratedPortalRoute.ProofPanels),
  SettingsRules: PortalRouteSchema.parse(GeneratedPortalRoute.SettingsRules),
  AppLayout: PortalRouteSchema.parse(GeneratedPortalRoute.AppLayout),
  FrameTuner: PortalRouteSchema.parse(GeneratedPortalRoute.FrameTuner),
  Commands: PortalRouteSchema.parse(GeneratedPortalRoute.Commands),
  Events: PortalRouteSchema.parse(GeneratedPortalRoute.Events),
  Logs: PortalRouteSchema.parse(GeneratedPortalRoute.Logs),
} as const;

export const PortalRouteHashPrefix = GeneratedPortalRouteHashPrefix;
export const PortalRouteHashQuerySeparator = GeneratedPortalRouteHashQuerySeparator;
export type PortalRouteHashPath = GeneratedPortalRouteHashPathValue;
export type PortalRouteHashQueryPath = GeneratedPortalRouteHashQueryPathValue;

export const PortalConnectionStateSchema = withParser(
  Schema.Literal(
    GeneratedPortalConnectionState.Disconnected,
    GeneratedPortalConnectionState.Connecting,
    GeneratedPortalConnectionState.Connected,
    GeneratedPortalConnectionState.Error
  )
);
export type PortalConnectionState = Infer<typeof PortalConnectionStateSchema> & GeneratedPortalConnectionStateValue;
export const PortalConnectionState = {
  Disconnected: PortalConnectionStateSchema.parse(GeneratedPortalConnectionState.Disconnected),
  Connecting: PortalConnectionStateSchema.parse(GeneratedPortalConnectionState.Connecting),
  Connected: PortalConnectionStateSchema.parse(GeneratedPortalConnectionState.Connected),
  Error: PortalConnectionStateSchema.parse(GeneratedPortalConnectionState.Error),
} as const;

export type PortalRouteEventRole = GeneratedPortalRouteEventRole;
export type PortalRouteEventPayloadRecord = GeneratedPortalRouteEventPayloadRecord;
export type PortalRouteEventSnapshot = GeneratedPortalRouteEventSnapshot;
export type PortalRouteEventId = NonNullable<PortalRouteEventSnapshot['eventId']>;
export type PortalRouteEventName = NonNullable<PortalRouteEventSnapshot['event']>;
export type PortalRouteEventSeverity = NonNullable<PortalRouteEventSnapshot['severity']>;

export const PortalDevToolUrlSchema = withParser(brandedNonEmptyStringSchema('PortalDevToolUrl'));
export type PortalDevToolUrl = Infer<typeof PortalDevToolUrlSchema> & GeneratedPortalDevToolUrl;

export const PortalDetailValueSchema = withParser(brandedNonEmptyStringSchema('PortalDetailValue'));
export type PortalDetailValue = Infer<typeof PortalDetailValueSchema> & GeneratedPortalDetailValue;
export const decodePortalDetailValue = PortalDetailValueSchema.parse;

export const PortalClipboardTextSchema = withParser(brandedNonEmptyStringSchema('PortalClipboardText'));
export type PortalClipboardText = Infer<typeof PortalClipboardTextSchema> & GeneratedPortalClipboardText;
export const decodePortalClipboardText = PortalClipboardTextSchema.parse;

export const TrackingStatusProofArtifactSchema = withParser(brandedNonEmptyStringSchema('TrackingStatusProofArtifact'));
export type TrackingStatusProofArtifact = Infer<typeof TrackingStatusProofArtifactSchema> &
  GeneratedTrackingStatusProofArtifact;
export const decodeTrackingStatusProofArtifact = TrackingStatusProofArtifactSchema.parse;
