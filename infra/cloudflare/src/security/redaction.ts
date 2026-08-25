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
] as const;
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
