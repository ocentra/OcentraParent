import {
  GeneratedPortalAgentCommand,
  type GeneratedPortalAgentCommandEnvelope,
  type GeneratedPortalAgentCommandName,
  GeneratedPortalAgentEvent,
  type GeneratedPortalAgentEventName,
  GeneratedPortalAgentLanHouseholdActionDeviceKindField,
  GeneratedPortalAgentLanHouseholdActionKind,
  type GeneratedPortalAgentLanHouseholdActionKind as GeneratedPortalAgentLanHouseholdActionKindValue,
  type GeneratedPortalAgentLanHouseholdDeviceKind,
  GeneratedPortalAgentLanHouseholdDeviceKindValues,
  GeneratedPortalAgentLanIntentKind,
  type GeneratedPortalAgentLanIntentKind as GeneratedPortalAgentLanIntentKindValue,
  GeneratedPortalAgentLanParentAuthority,
  type GeneratedPortalAgentLanParentAuthority as GeneratedPortalAgentLanParentAuthorityValue,
  type GeneratedPortalAgentMessageTarget,
  type GeneratedPortalAgentPeer,
  GeneratedPortalAgentPeerDefaults,
  GeneratedPortalAgentProtocolDelimiter,
  type GeneratedPortalAgentProtocolDelimiter as GeneratedPortalAgentProtocolDelimiterValue,
  GeneratedPortalAgentProtocolField,
  type GeneratedPortalAgentProtocolFieldName,
  type GeneratedPortalAgentProtocolPayload,
  type GeneratedPortalAgentProtocolPayloadValue,
  GeneratedPortalAgentProtocolRuntime,
  GeneratedPortalAgentTargetDefaults,
  GeneratedPortalConnectionState,
  type GeneratedPortalConnectionState as GeneratedPortalConnectionStateValue,
  type GeneratedPortalClipboardText,
  type GeneratedPortalDetailValue,
  type GeneratedPortalRouteEventPayloadRecord,
  type GeneratedPortalRouteEventSnapshot,
  GeneratedPortalRoute,
  type GeneratedPortalRoute as GeneratedPortalRouteValue,
  type GeneratedPortalRouteHashPath as GeneratedPortalRouteHashPathValue,
  GeneratedPortalRouteHashPrefix,
  type GeneratedPortalRouteHashQueryPath as GeneratedPortalRouteHashQueryPathValue,
  GeneratedPortalRouteHashQuerySeparator,
  GeneratedPortalRouteLiteral,
  type GeneratedTrackingStatusProofArtifact,
  decodeGeneratedPortalAgentCommandEnvelope,
} from './generated-portal-contracts';

type SafeParseResult<T> =
  | { readonly success: true; readonly data: T }
  | { readonly success: false; readonly error: TypeError };

type PortalLiteralSchema<T> = {
  readonly parse: (input: unknown) => T;
  readonly safeParse: (input: unknown) => SafeParseResult<T>;
};

function parseLiteral<const T extends string>(input: unknown, label: string, allowed: Set<string>): T {
  if (typeof input !== 'string' || !allowed.has(input)) {
    throw new TypeError(`${label} must be a Rust-owned protocol literal`);
  }
  return input as T;
}

function safeParseLiteral<const T extends string>(
  input: unknown,
  label: string,
  allowed: Set<string>
): SafeParseResult<T> {
  if (typeof input !== 'string' || !allowed.has(input)) {
    return { success: false, error: new TypeError(`${label} must be a Rust-owned protocol literal`) };
  }
  return { success: true, data: input as T };
}

function literalSchema<const T extends string>(label: string, allowedValues: readonly T[]): PortalLiteralSchema<T> {
  const allowed = new Set<string>(allowedValues);
  return {
    parse: (input: unknown) => parseLiteral<T>(input, label, allowed),
    safeParse: (input: unknown) => safeParseLiteral<T>(input, label, allowed),
  };
}

function decodeNonEmptyString<T extends string>(input: unknown, label: string): T {
  if (typeof input !== 'string' || input.length === 0) {
    throw new TypeError(`${label} must be a non-empty Rust-owned protocol string`);
  }
  return input as T;
}

const PortalRouteValues = Object.values(GeneratedPortalRouteLiteral) as readonly GeneratedPortalRouteValue[];

export const PortalRouteLiteral = GeneratedPortalRouteLiteral;
export type PortalRoute = GeneratedPortalRouteValue;
export const PortalRoute = GeneratedPortalRoute;
export const PortalRouteSchema = literalSchema<PortalRoute>('PortalRoute', PortalRouteValues);

export const PortalRouteHashPrefix = GeneratedPortalRouteHashPrefix;
export const PortalRouteHashQuerySeparator = GeneratedPortalRouteHashQuerySeparator;
export type PortalRouteHashPath = GeneratedPortalRouteHashPathValue;
export type PortalRouteHashQueryPath = GeneratedPortalRouteHashQueryPathValue;

export type PortalConnectionState = GeneratedPortalConnectionStateValue;
export const PortalConnectionState = GeneratedPortalConnectionState;

export const PortalAgentProtocolRuntime = GeneratedPortalAgentProtocolRuntime;
export const PortalAgentProtocolField = GeneratedPortalAgentProtocolField;
export type PortalAgentProtocolFieldName = GeneratedPortalAgentProtocolFieldName;
export type PortalAgentProtocolPayload = GeneratedPortalAgentProtocolPayload;
export type PortalAgentProtocolPayloadValue = GeneratedPortalAgentProtocolPayloadValue;
export const PortalAgentProtocolDelimiter = GeneratedPortalAgentProtocolDelimiter;
export type PortalAgentProtocolDelimiterValue = GeneratedPortalAgentProtocolDelimiterValue;
export const PortalAgentCommand = GeneratedPortalAgentCommand;
export type PortalAgentCommandName = GeneratedPortalAgentCommandName;
export const PortalAgentEvent = GeneratedPortalAgentEvent;
export type PortalAgentEventName = GeneratedPortalAgentEventName;
export const PortalAgentPeerDefaults = GeneratedPortalAgentPeerDefaults;
export type PortalAgentPeer = GeneratedPortalAgentPeer;
export const PortalAgentTargetDefaults = GeneratedPortalAgentTargetDefaults;
export type PortalAgentMessageTarget = GeneratedPortalAgentMessageTarget;
export type PortalAgentCommandEnvelope = GeneratedPortalAgentCommandEnvelope;
export const decodePortalAgentCommandEnvelope = decodeGeneratedPortalAgentCommandEnvelope;
export const PortalAgentLanHouseholdActionKind = GeneratedPortalAgentLanHouseholdActionKind;
export type PortalAgentLanHouseholdActionKind = GeneratedPortalAgentLanHouseholdActionKindValue;
export const PortalAgentLanIntentKind = GeneratedPortalAgentLanIntentKind;
export type PortalAgentLanIntentKind = GeneratedPortalAgentLanIntentKindValue;
export const PortalAgentLanParentAuthority = GeneratedPortalAgentLanParentAuthority;
export type PortalAgentLanParentAuthority = GeneratedPortalAgentLanParentAuthorityValue;
export const PortalAgentLanHouseholdDeviceKindValues = GeneratedPortalAgentLanHouseholdDeviceKindValues;
export type PortalAgentLanHouseholdDeviceKind = GeneratedPortalAgentLanHouseholdDeviceKind;
export const PortalAgentLanHouseholdActionDeviceKindField = GeneratedPortalAgentLanHouseholdActionDeviceKindField;

export type PortalRouteEventPayloadRecord = GeneratedPortalRouteEventPayloadRecord;
export type PortalRouteEventSnapshot = GeneratedPortalRouteEventSnapshot;
export type PortalRouteEventRecord = Omit<GeneratedPortalRouteEventSnapshot, 'payload'> & {
  readonly payload?: PortalRouteEventPayloadRecord;
};

export type PortalDetailValue = GeneratedPortalDetailValue;
export const decodePortalDetailValue = (input: unknown): PortalDetailValue =>
  decodeNonEmptyString<PortalDetailValue>(input, 'PortalDetailValue');

export type PortalClipboardText = GeneratedPortalClipboardText;
export const decodePortalClipboardText = (input: unknown): PortalClipboardText =>
  decodeNonEmptyString<PortalClipboardText>(input, 'PortalClipboardText');

export type TrackingStatusProofArtifact = GeneratedTrackingStatusProofArtifact;
export const decodeTrackingStatusProofArtifact = (input: unknown): TrackingStatusProofArtifact =>
  decodeNonEmptyString<TrackingStatusProofArtifact>(input, 'TrackingStatusProofArtifact');
