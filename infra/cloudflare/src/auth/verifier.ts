import type { Env } from '../env.js';
import type { AuthState } from '../routes.js';

export interface VerifiedIdentity {
  subject: string;
  state: AuthState;
  role: 'public' | 'parent' | 'support' | 'admin' | 'internal' | 'provider-webhook';
  trustedDevice: boolean;
}

export type AuthResult = { ok: true; identity: VerifiedIdentity } | { ok: false; response: Response };

export const INTERNAL_SECRET_HEADER = 'x-ocentra-internal-secret';

function json(status: number, body: unknown): Response {
  return new Response(JSON.stringify(body, null, 2), {
    status,
    headers: {
      'content-type': 'application/json; charset=utf-8',
    },
  });
}

function missingHeader(headerName: string, state: AuthState): AuthResult {
  return {
    ok: false,
    response: json(401, {
      error: 'authentication-required',
      authState: state,
      missingHeader: headerName,
    }),
  };
}

function forbidden(reason: string, state: AuthState): AuthResult {
  return {
    ok: false,
    response: json(403, {
      error: 'forbidden',
      authState: state,
      reason,
    }),
  };
}

export function signatureHeaderName(pathname: string): string {
  if (pathname.endsWith('/stripe')) {
    return 'stripe-signature';
  }
  if (pathname.endsWith('/paypal')) {
    return 'paypal-transmission-id';
  }
  if (pathname.endsWith('/razorpay')) {
    return 'x-razorpay-signature';
  }
  if (pathname.endsWith('/apple')) {
    return 'authorization';
  }
  return 'x-goog-signature';
}

function parseBearerToken(headerValue: string | null): string | null {
  if (!headerValue) {
    return null;
  }
  const [scheme, value] = headerValue.split(/\s+/, 2);
  if (scheme?.toLowerCase() !== 'bearer' || !value) {
    return null;
  }
  return value.trim();
}

function normalizeSubject(token: string): string {
  const sanitized = token.replace(/[^A-Za-z0-9:_-]/g, '-').slice(0, 64);
  if (sanitized.length === 0) {
    return 'parent:unknown';
  }
  if (
    sanitized.startsWith('parent:') ||
    sanitized.startsWith('guardian:') ||
    sanitized.startsWith('child:') ||
    sanitized.startsWith('member:')
  ) {
    return sanitized;
  }
  return `parent:${sanitized}`;
}

function parseStripeSignatureHeader(signatureHeader: string): {
  timestamp: string;
  signatures: ReadonlyArray<string>;
} | null {
  const parts = signatureHeader.split(',').map((entry) => entry.trim());
  const timestamp = parts.find((entry) => entry.startsWith('t='))?.slice(2);
  const signatures = parts
    .filter((entry) => entry.startsWith('v1='))
    .map((entry) => entry.slice(3))
    .filter((entry) => /^[a-f0-9]{64}$/i.test(entry));

  if (!timestamp || !/^\d+$/.test(timestamp) || signatures.length === 0) {
    return null;
  }

  return {
    timestamp,
    signatures,
  };
}

function safeEqualHex(left: string, right: string): boolean {
  if (left.length !== right.length) {
    return false;
  }
  let diff = 0;
  for (let index = 0; index < left.length; index += 1) {
    diff |= left.charCodeAt(index) ^ right.charCodeAt(index);
  }
  return diff === 0;
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes, (value) => value.toString(16).padStart(2, '0')).join('');
}

export function hasWebhookSignatureSyntax(pathname: string, signatureValue: string | null): boolean {
  if (!signatureValue || signatureValue.trim().length === 0) {
    return false;
  }
  if (pathname.endsWith('/stripe')) {
    return parseStripeSignatureHeader(signatureValue) !== null;
  }
  return true;
}

export async function verifyStripeWebhookSignature(
  payload: string,
  signatureHeader: string,
  secret: string
): Promise<boolean> {
  const parsed = parseStripeSignatureHeader(signatureHeader);
  if (!parsed) {
    return false;
  }

  const key = await crypto.subtle.importKey(
    'raw',
    new TextEncoder().encode(secret),
    {
      name: 'HMAC',
      hash: 'SHA-256',
    },
    false,
    ['sign']
  );
  const signed = await crypto.subtle.sign('HMAC', key, new TextEncoder().encode(`${parsed.timestamp}.${payload}`));
  const actualSignature = bytesToHex(new Uint8Array(signed));
  return parsed.signatures.some((expected) => safeEqualHex(expected, actualSignature));
}

export async function verifyAuthState(authState: AuthState, request: Request, env: Env): Promise<AuthResult> {
  if (authState === 'public') {
    return {
      ok: true,
      identity: {
        subject: 'public',
        state: authState,
        role: 'public',
        trustedDevice: false,
      },
    };
  }

  if (authState === 'internal-queue-only') {
    if (request.headers.get('x-ocentra-internal-call') !== 'true') {
      return forbidden('missing-internal-queue-signal', authState);
    }
    if (
      env.INTERNAL_QUEUE_SHARED_SECRET &&
      request.headers.get(INTERNAL_SECRET_HEADER) !== env.INTERNAL_QUEUE_SHARED_SECRET
    ) {
      return forbidden('internal-queue-secret-mismatch', authState);
    }
    return {
      ok: true,
      identity: {
        subject: 'internal-queue',
        state: authState,
        role: 'internal',
        trustedDevice: false,
      },
    };
  }

  if (authState === 'provider-webhook-signature-required') {
    const headerName = signatureHeaderName(new URL(request.url).pathname);
    const headerValue = request.headers.get(headerName);
    if (!headerValue) {
      return missingHeader(headerName, authState);
    }
    if (!hasWebhookSignatureSyntax(new URL(request.url).pathname, headerValue)) {
      return forbidden('invalid-provider-webhook-signature-header', authState);
    }
    return {
      ok: true,
      identity: {
        subject: 'provider-webhook',
        state: authState,
        role: 'provider-webhook',
        trustedDevice: false,
      },
    };
  }

  const token = parseBearerToken(request.headers.get('authorization'));
  if (!token) {
    return missingHeader('authorization', authState);
  }

  const trustedDevice = request.headers.get('x-ocentra-trusted-device') === 'true';
  if (authState === 'trusted-parent-device-required' && !trustedDevice) {
    return forbidden('trusted-parent-device-required', authState);
  }

  if (authState === 'admin-required') {
    if (request.headers.get('x-ocentra-role') !== 'admin') {
      return forbidden('admin-role-required', authState);
    }
    return {
      ok: true,
      identity: {
        subject: normalizeSubject(token),
        state: authState,
        role: 'admin',
        trustedDevice,
      },
    };
  }

  if (authState === 'support-required') {
    const roleHeader = request.headers.get('x-ocentra-role');
    if (roleHeader !== 'support' && roleHeader !== 'admin') {
      return forbidden('support-role-required', authState);
    }
    return {
      ok: true,
      identity: {
        subject: normalizeSubject(token),
        state: authState,
        role: roleHeader === 'admin' ? 'admin' : 'support',
        trustedDevice,
      },
    };
  }

  return {
    ok: true,
    identity: {
      subject: normalizeSubject(token),
      state: authState,
      role: 'parent',
      trustedDevice,
    },
  };
}
