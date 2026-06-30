import {
  GeneratedPortalConnectionState,
  type GeneratedPortalConnectionState as GeneratedPortalConnectionStateValue,
  type GeneratedPortalClipboardText,
  type GeneratedPortalDetailValue,
  type GeneratedPortalDevToolUrl,
  type GeneratedPortalRouteEventPayloadRecord as SchemaPortalRouteEventPayloadRecord,
  type GeneratedPortalRouteEventSnapshot as SchemaPortalRouteEventSnapshot,
  GeneratedPortalRoute,
  type GeneratedPortalRoute as GeneratedPortalRouteValue,
  type GeneratedPortalRouteHashPath as GeneratedPortalRouteHashPathValue,
  GeneratedPortalRouteHashPrefix,
  type GeneratedPortalRouteHashQueryPath as GeneratedPortalRouteHashQueryPathValue,
  GeneratedPortalRouteHashQuerySeparator,
  GeneratedPortalRouteLiteral,
  type GeneratedTrackingStatusProofArtifact,
} from '@ocentra-parent/schema-domain/generated/portal-contracts';

type SafeParseResult<T> =
  | { readonly success: true; readonly data: T }
  | { readonly success: false; readonly error: PortalContractParseError };

type ParsedValue<T> = {
  parse(input: unknown): T;
  safeParse(input: unknown): SafeParseResult<T>;
};

class PortalContractParseError extends Error {
  constructor(label: string, input: unknown) {
    super(`${label}: expected a valid non-empty string`);
    this.name = 'PortalContractParseError';
    this.cause = input;
  }
}

function success<T>(data: T): SafeParseResult<T> {
  return { success: true, data };
}

function failure<T>(label: string, input: unknown): SafeParseResult<T> {
  return { success: false, error: new PortalContractParseError(label, input) };
}

function createLiteralParser<const T extends string>(label: string, allowedValues: readonly T[]): ParsedValue<T> {
  const allowed = new Set<string>(allowedValues);
  return {
    parse(input: unknown): T {
      if (typeof input !== 'string' || !allowed.has(input)) {
        throw new PortalContractParseError(label, input);
      }
      return input as T;
    },
    safeParse(input: unknown): SafeParseResult<T> {
      if (typeof input !== 'string' || !allowed.has(input)) {
        return failure<T>(label, input);
      }
      return success(input as T);
    },
  };
}

function createNonEmptyStringParser<T extends string>(label: string): ParsedValue<T> {
  return {
    parse(input: unknown): T {
      if (typeof input !== 'string' || input.length === 0) {
        throw new PortalContractParseError(label, input);
      }
      return input as T;
    },
    safeParse(input: unknown): SafeParseResult<T> {
      if (typeof input !== 'string' || input.length === 0) {
        return failure<T>(label, input);
      }
      return success(input as T);
    },
  };
}

const PortalRouteValues = Object.values(GeneratedPortalRouteLiteral) as readonly GeneratedPortalRouteValue[];

export const PortalRouteLiteral = GeneratedPortalRouteLiteral;
export type PortalRoute = GeneratedPortalRouteValue;
export const PortalRoute = GeneratedPortalRoute;
export const PortalRouteSchema = createLiteralParser<PortalRoute>('PortalRoute', PortalRouteValues);

export const PortalRouteHashPrefix = GeneratedPortalRouteHashPrefix;
export const PortalRouteHashQuerySeparator = GeneratedPortalRouteHashQuerySeparator;
export type PortalRouteHashPath = GeneratedPortalRouteHashPathValue;
export type PortalRouteHashQueryPath = GeneratedPortalRouteHashQueryPathValue;

export type PortalConnectionState = GeneratedPortalConnectionStateValue;
export const PortalConnectionState = GeneratedPortalConnectionState;

export type PortalRouteEventPayloadRecord = SchemaPortalRouteEventPayloadRecord;
export type PortalRouteEventSnapshot = SchemaPortalRouteEventSnapshot;
export interface PortalRouteEventRecord {
  readonly event?: string;
  readonly eventId?: string;
  readonly correlationId?: string;
  readonly sentAt?: string;
  readonly sourcePeerId?: string;
  readonly sourceRole?: PortalRouteEventSnapshot['sourceRole'];
  readonly targetPeerId?: string;
  readonly targetRole?: PortalRouteEventSnapshot['targetRole'];
  readonly severity?: string;
  readonly payload?: PortalRouteEventPayloadRecord;
  readonly snapshot?: unknown;
}
export type PortalRouteEventName = NonNullable<PortalRouteEventRecord['event']>;

export type PortalDevToolUrl = GeneratedPortalDevToolUrl;
export const PortalDevToolUrlSchema = createNonEmptyStringParser<PortalDevToolUrl>('PortalDevToolUrl');

export type PortalDetailValue = GeneratedPortalDetailValue;
export const decodePortalDetailValue = createNonEmptyStringParser<PortalDetailValue>('PortalDetailValue').parse;

export type PortalClipboardText = GeneratedPortalClipboardText;
export const decodePortalClipboardText = createNonEmptyStringParser<PortalClipboardText>('PortalClipboardText').parse;

export type TrackingStatusProofArtifact = GeneratedTrackingStatusProofArtifact;
export const decodeTrackingStatusProofArtifact =
  createNonEmptyStringParser<TrackingStatusProofArtifact>('TrackingStatusProofArtifact').parse;
