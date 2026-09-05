export type LocalArtifactProviderErrorCode =
  | 'already-exists'
  | 'atomic-mutation-failure'
  | 'canonicalization-failure'
  | 'containment-failure'
  | 'durability-failure'
  | 'invalid-arguments'
  | 'io'
  | 'link-or-reparse'
  | 'lock-conflict'
  | 'not-found'
  | 'ownership-changed'
  | 'protocol-frame'
  | 'protocol-limit'
  | 'provider-authority'
  | 'provider-start'
  | 'provider-timeout'
  | 'provider-unavailable'
  | 'recovery-uncertainty'
  | 'root-identity-changed'
  | 'size-limit'
  | 'unsupported-provider';

const ErrorCodes: ReadonlySet<string> = new Set<LocalArtifactProviderErrorCode>([
  'already-exists',
  'atomic-mutation-failure',
  'canonicalization-failure',
  'containment-failure',
  'durability-failure',
  'invalid-arguments',
  'io',
  'link-or-reparse',
  'lock-conflict',
  'not-found',
  'ownership-changed',
  'protocol-frame',
  'protocol-limit',
  'provider-authority',
  'provider-start',
  'provider-timeout',
  'provider-unavailable',
  'recovery-uncertainty',
  'root-identity-changed',
  'size-limit',
  'unsupported-provider',
]);

export function isLocalArtifactProviderErrorCode(value: unknown): value is LocalArtifactProviderErrorCode {
  return typeof value === 'string' && ErrorCodes.has(value);
}

export class LocalArtifactProviderError extends Error {
  readonly code: LocalArtifactProviderErrorCode;

  constructor(code: LocalArtifactProviderErrorCode, message: string) {
    super(message);
    this.name = 'LocalArtifactProviderError';
    this.code = code;
  }
}
