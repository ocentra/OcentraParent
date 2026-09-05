const REDACTED = '[redacted]';

const SENSITIVE_FIELD_FRAGMENTS = [
  'authorization',
  'cookie',
  'secret',
  'token',
  'apikey',
  'password',
  'signature',
  'csrf',
  'signingkeyref',
  'serviceaccountref',
  'evidenceref',
  'evidencepath',
  'recoverybundle',
  'supportbundle',
  'childdata',
  'childprofile',
  'childactivity',
  'childdevice',
  'childname',
  'childid',
  'childref',
  'childcontent',
  'childtelemetry',
  'childpolicy',
  'policydetails',
  'policytext',
  'screenshot',
  'urlhistory',
  'browsinghistory',
  'providerpayload',
  'rawproviderpayload',
  'webhookbody',
  'rawwebhookbody',
  'devicesecret',
  'localdevicesecret',
  'sessiontoken',
  'sessionid',
  'razorpaykeyid',
  'paypalclientid',
  'applestorekeyref',
] as const;

export const PROVIDER_METADATA_ALLOWLIST = [
  'planReference',
  'priceReference',
  'familyReference',
  'accountReference',
  'referralCode',
  'referralReference',
  'invoiceReference',
  'receiptReference',
  'sessionReference',
  'checkoutReference',
  'idempotencyReference',
  'region',
  'currency',
  'testLiveMarker',
] as const;

export const PROVIDER_METADATA_DENYLIST = [
  'childName',
  'childId',
  'childReference',
  'childActivity',
  'childContent',
  'childTelemetry',
  'childPolicy',
  'screenshot',
  'urlHistory',
  'browsingHistory',
  'policyDetails',
  'policyText',
  'supportBundle',
  'localDeviceSecret',
  'providerPayload',
  'rawWebhookBody',
] as const;

const PROVIDER_METADATA_VALUE_MAX_LENGTH = 256;
const PROVIDER_METADATA_DENIED_VALUE_PATTERN =
  /\b(?:child(?:name|id|reference|activity|content|telemetry|policy)?|screenshot|url\s*history|browsing\s*history|policy(?:details|text)?|support\s*bundle|local\s*device\s*secret)\b/iu;
const SECRET_VALUE_PATTERNS = [
  /bearer\s+\S+/i,
  /session=[^;\s]+/i,
  /sk_(?:live|test)_[A-Za-z0-9]+/i,
  /whsec_[A-Za-z0-9]+/i,
  /-----BEGIN [A-Z ]+-----/i,
  /(?:^|[^a-z])(?:local[-_]?device[-_]?secret|support[-_]?bundle|recovery[-_]?bundle)(?:[^a-z]|$)/i,
  /(?:[A-Za-z]:\\|\\\\|\/Users\/|\/home\/|\/private\/|\/tmp\/)/,
] as const;

function isObjectRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function normalizeFieldName(fieldName: string): string {
  return fieldName.toLowerCase().replace(/[^a-z0-9]/g, '');
}

const PROVIDER_METADATA_KEYS_BY_NORMALIZED_NAME = new Map(
  PROVIDER_METADATA_ALLOWLIST.map((key) => [normalizeFieldName(key), key] as const)
);

export function isSensitiveFieldName(fieldName: string): boolean {
  const normalized = normalizeFieldName(fieldName);
  return SENSITIVE_FIELD_FRAGMENTS.some((fragment) => normalized.includes(fragment));
}

export function redactStringValue(value: string): string {
  if (SECRET_VALUE_PATTERNS.some((pattern) => pattern.test(value))) {
    return REDACTED;
  }
  return value;
}

export function redactPayload(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map((entry) => redactPayload(entry));
  }

  if (isObjectRecord(value)) {
    const redactedEntries = Object.entries(value).map(([key, entryValue]) => {
      if (isSensitiveFieldName(key)) {
        return [key, REDACTED] as const;
      }
      return [key, redactPayload(entryValue)] as const;
    });
    return Object.fromEntries(redactedEntries);
  }

  if (typeof value === 'string') {
    return redactStringValue(value);
  }

  return value;
}

export function redactHeaders(headers: Headers): Record<string, string> {
  const entries: Array<[string, string]> = [];
  for (const [key, value] of headers.entries()) {
    entries.push([key, isSensitiveFieldName(key) ? REDACTED : redactStringValue(value)]);
  }
  entries.sort(([left], [right]) => left.localeCompare(right));
  return Object.fromEntries(entries);
}

export type ProviderMetadataResult =
  | {
      readonly accepted: true;
      readonly metadata: Readonly<Record<string, string>>;
    }
  | {
      readonly accepted: false;
      readonly reason:
        | 'metadata-must-be-object'
        | 'metadata-key-not-allowed'
        | 'metadata-key-denied'
        | 'metadata-duplicate-key'
        | 'metadata-test-live-mismatch'
        | 'metadata-value-invalid'
        | 'metadata-value-too-large';
    };

export type ProviderTestLiveMode = 'test' | 'live';

/**
 * Provider adapters may send only billing reconciliation references. This
 * boundary is deliberately independent of provider SDKs: an unknown or
 * privacy-sensitive field is rejected rather than silently forwarded.
 */
export function sanitizeProviderMetadata(value: unknown): ProviderMetadataResult {
  if (!isObjectRecord(value)) {
    return { accepted: false, reason: 'metadata-must-be-object' };
  }

  const metadata: Record<string, string> = {};
  for (const [inputKey, inputValue] of Object.entries(value)) {
    const normalizedKey = normalizeFieldName(inputKey);
    if (
      PROVIDER_METADATA_DENYLIST.some((deniedKey) => normalizeFieldName(deniedKey) === normalizedKey) ||
      isSensitiveFieldName(inputKey)
    ) {
      return { accepted: false, reason: 'metadata-key-denied' };
    }

    const canonicalKey = PROVIDER_METADATA_KEYS_BY_NORMALIZED_NAME.get(normalizedKey);
    if (!canonicalKey) {
      return { accepted: false, reason: 'metadata-key-not-allowed' };
    }
    if (Object.prototype.hasOwnProperty.call(metadata, canonicalKey)) {
      return { accepted: false, reason: 'metadata-duplicate-key' };
    }
    if (typeof inputValue !== 'string' || inputValue.trim().length === 0) {
      return { accepted: false, reason: 'metadata-value-invalid' };
    }

    const normalizedValue = inputValue.trim();
    if (normalizedValue.length > PROVIDER_METADATA_VALUE_MAX_LENGTH) {
      return { accepted: false, reason: 'metadata-value-too-large' };
    }
    if (canonicalKey === 'testLiveMarker' && normalizedValue !== 'test' && normalizedValue !== 'live') {
      return { accepted: false, reason: 'metadata-value-invalid' };
    }
    if (
      redactStringValue(normalizedValue) !== normalizedValue ||
      PROVIDER_METADATA_DENIED_VALUE_PATTERN.test(normalizedValue)
    ) {
      return { accepted: false, reason: 'metadata-value-invalid' };
    }
    metadata[canonicalKey] = normalizedValue;
  }

  return { accepted: true, metadata };
}

/**
 * Keeps an explicit provider mode marker bound to the Worker environment. A
 * missing marker remains compatible with providers that expose mode outside
 * metadata; an explicit contradictory marker is never accepted.
 */
export function sanitizeProviderMetadataForMode(
  value: unknown,
  expectedMode: ProviderTestLiveMode
): ProviderMetadataResult {
  const result = sanitizeProviderMetadata(value);
  if (!result.accepted || result.metadata.testLiveMarker === undefined) {
    return result;
  }
  if (result.metadata.testLiveMarker !== expectedMode) {
    return { accepted: false, reason: 'metadata-test-live-mismatch' };
  }
  return result;
}
