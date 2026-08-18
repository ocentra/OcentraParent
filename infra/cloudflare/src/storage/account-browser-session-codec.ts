import type { AccountIdentityProvider } from '@ocentra-parent/schema-domain/account-identity-authority';

const SESSION_TOKEN_PATTERN = /^[A-Za-z0-9_-]{43}\.[A-Za-z0-9_-]{43}$/;
const DIGEST_HEX_PATTERN = /^[a-f0-9]{64}$/;

export interface BrowserSessionSecrets {
  readonly sessionToken: string;
  readonly refreshToken: string;
  readonly csrfToken: string;
}

export interface BrowserSessionRow {
  session_id: string;
  session_token_digest: string;
  refresh_token_digest: string;
  csrf_token_digest: string;
  provider: AccountIdentityProvider;
  provider_subject: string;
  account_id: string;
  authority_session_id: string;
  authority_session_generation: number;
  authority_generation: number;
  issued_at: string;
  expires_at: string;
  refresh_generation: number;
  status: 'active' | 'revoked';
  last_seen_at: string;
  revoked_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface BrowserSessionIdentity {
  readonly sessionId: string;
  readonly provider: AccountIdentityProvider;
  readonly providerSubject: string;
  readonly accountId: string;
  readonly authoritySessionId: string;
  readonly authoritySessionGeneration: number;
  readonly authorityGeneration: number;
  readonly issuedAt: string;
  readonly expiresAt: string;
  readonly refreshGeneration: number;
}

function base64Url(bytes: Uint8Array): string {
  let binary = '';
  for (const value of bytes) binary += String.fromCharCode(value);
  return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/g, '');
}

export function newOpaqueValue(): string {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return base64Url(bytes);
}

export function newSessionId(): string {
  return newOpaqueValue();
}

export function sessionCookieValue(sessionId: string, secret: string): string {
  return `${sessionId}.${secret}`;
}

export function parseSessionCookie(value: string | null): { sessionId: string; secret: string } | null {
  if (value === null || !SESSION_TOKEN_PATTERN.test(value)) return null;
  const separator = value.indexOf('.');
  const sessionId = value.slice(0, separator);
  const secret = value.slice(separator + 1);
  return sessionId.length === 43 && secret.length === 43 ? { sessionId, secret } : null;
}

export async function sha256Hex(value: string): Promise<string> {
  const bytes = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(value));
  return Array.from(new Uint8Array(bytes), (part) => part.toString(16).padStart(2, '0')).join('');
}

export function isDigest(value: string): boolean {
  return DIGEST_HEX_PATTERN.test(value);
}

export function sessionIdentity(row: BrowserSessionRow): BrowserSessionIdentity {
  return {
    sessionId: row.session_id,
    provider: row.provider,
    providerSubject: row.provider_subject,
    accountId: row.account_id,
    authoritySessionId: row.authority_session_id,
    authoritySessionGeneration: row.authority_session_generation,
    authorityGeneration: row.authority_generation,
    issuedAt: row.issued_at,
    expiresAt: row.expires_at,
    refreshGeneration: row.refresh_generation,
  };
}

export function nowIso(nowMs = Date.now()): string {
  return new Date(nowMs).toISOString();
}

export function cookieMaxAge(expiresAt: string, nowMs = Date.now()): number {
  const remaining = Date.parse(expiresAt) - nowMs;
  return Math.max(0, Math.floor(remaining / 1000));
}

export function readCookie(request: Request, name: string): string | null {
  const source = request.headers.get('cookie');
  if (!source) return null;
  for (const part of source.split(';')) {
    const separator = part.indexOf('=');
    if (separator < 0 || part.slice(0, separator).trim() !== name) continue;
    return part.slice(separator + 1).trim() || null;
  }
  return null;
}
