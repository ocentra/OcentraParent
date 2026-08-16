import {
  type GeneratedPortalClipboardText,
  type GeneratedPortalDetailValue,
  type GeneratedPortalDevToolUrl,
  type GeneratedTrackingStatusProofArtifact,
} from './generated-portal-contracts';

type SafeParseResult<T> =
  | { readonly success: true; readonly data: T }
  | { readonly success: false; readonly error: TypeError };

type PortalLiteralSchema<T> = {
  readonly parse: (input: unknown) => T;
  readonly safeParse: (input: unknown) => SafeParseResult<T>;
};

function decodeNonEmptyString<T extends string>(input: unknown, label: string): T {
  if (typeof input !== 'string' || input.length === 0) {
    throw new TypeError(`${label} must be a non-empty Rust-owned protocol string`);
  }
  return input as T;
}

export type PortalDevToolUrl = GeneratedPortalDevToolUrl;
export const PortalDevToolUrlSchema: PortalLiteralSchema<PortalDevToolUrl> = {
  parse(input: unknown): PortalDevToolUrl {
    return decodeNonEmptyString<PortalDevToolUrl>(input, 'PortalDevToolUrl');
  },
  safeParse(input: unknown): SafeParseResult<PortalDevToolUrl> {
    if (typeof input !== 'string' || input.length === 0) {
      return {
        success: false,
        error: new TypeError('PortalDevToolUrl must be a non-empty Rust-owned protocol string'),
      };
    }
    return { success: true, data: input as PortalDevToolUrl };
  },
};
export const decodePortalDevToolUrl = (input: unknown): PortalDevToolUrl =>
  decodeNonEmptyString<PortalDevToolUrl>(input, 'PortalDevToolUrl');

export type PortalDetailValue = GeneratedPortalDetailValue;
export const decodePortalDetailValue = (input: unknown): PortalDetailValue =>
  decodeNonEmptyString<PortalDetailValue>(input, 'PortalDetailValue');

export type PortalClipboardText = GeneratedPortalClipboardText;
export const decodePortalClipboardText = (input: unknown): PortalClipboardText =>
  decodeNonEmptyString<PortalClipboardText>(input, 'PortalClipboardText');

export type TrackingStatusProofArtifact = GeneratedTrackingStatusProofArtifact;
export const decodeTrackingStatusProofArtifact = (input: unknown): TrackingStatusProofArtifact =>
  decodeNonEmptyString<TrackingStatusProofArtifact>(input, 'TrackingStatusProofArtifact');
