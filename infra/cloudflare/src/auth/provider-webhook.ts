import type { Env } from '../env.js';

export const PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS = {
  unavailable: 'provider-webhook-verification-unavailable',
  unsupported: 'unsupported-provider-webhook',
  stripeSecretMissing: 'stripe-webhook-secret-missing',
  razorpayVerifierUnavailable: 'razorpay-provider-verifier-unavailable',
  paypalVerifierUnavailable: 'paypal-provider-verifier-unavailable',
  appleVerifierUnavailable: 'apple-provider-verifier-unavailable',
  googleVerifierUnavailable: 'google-provider-verifier-unavailable',
} as const;

export type ProviderWebhookName = 'stripe' | 'razorpay' | 'paypal' | 'apple' | 'google';

export type ProviderWebhookUnavailableBlocker =
  (typeof PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS)[keyof typeof PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS];

type ProviderWebhookVerificationResult =
  | { status: 'verified' }
  | { status: 'missing-credential'; headerName: string }
  | {
      status: 'rejected';
      reason: 'invalid-stripe-signature';
    }
  | { status: 'unavailable'; blocker: ProviderWebhookUnavailableBlocker };

export function resolveProviderWebhookName(value: string): ProviderWebhookName | null {
  switch (value) {
    case 'stripe':
    case 'razorpay':
    case 'paypal':
    case 'apple':
    case 'google':
      return value;
    default:
      return null;
  }
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

  return { timestamp, signatures };
}

function safeEqualHex(left: string, right: string): boolean {
  if (left.length !== right.length) return false;
  let diff = 0;
  for (let index = 0; index < left.length; index += 1) {
    diff |= left.charCodeAt(index) ^ right.charCodeAt(index);
  }
  return diff === 0;
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes, (value) => value.toString(16).padStart(2, '0')).join('');
}

export async function verifyStripeWebhookSignature(
  payload: string,
  signatureHeader: string,
  secret: string,
  timestampToleranceSeconds: string | undefined
): Promise<boolean> {
  const parsed = parseStripeSignatureHeader(signatureHeader);
  if (!parsed) return false;
  if (
    timestampToleranceSeconds === undefined ||
    !/^\d+$/.test(timestampToleranceSeconds) ||
    Number(timestampToleranceSeconds) <= 0 ||
    Number(timestampToleranceSeconds) > 86_400
  ) {
    return false;
  }
  const timestampSeconds = Number(parsed.timestamp);
  const nowSeconds = Math.floor(Date.now() / 1000);
  if (
    !Number.isSafeInteger(timestampSeconds) ||
    Math.abs(nowSeconds - timestampSeconds) > Number(timestampToleranceSeconds)
  ) {
    return false;
  }

  const key = await crypto.subtle.importKey(
    'raw',
    new TextEncoder().encode(secret),
    { name: 'HMAC', hash: 'SHA-256' },
    false,
    ['sign']
  );
  const signed = await crypto.subtle.sign('HMAC', key, new TextEncoder().encode(`${parsed.timestamp}.${payload}`));
  const actualSignature = bytesToHex(new Uint8Array(signed));
  return parsed.signatures.some((expected) => safeEqualHex(expected, actualSignature));
}

export async function verifyProviderWebhook(
  provider: ProviderWebhookName,
  request: Request,
  env: Env
): Promise<ProviderWebhookVerificationResult> {
  if (provider !== 'stripe') {
    return {
      status: 'unavailable',
      blocker:
        provider === 'razorpay'
          ? PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.razorpayVerifierUnavailable
          : provider === 'paypal'
            ? PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.paypalVerifierUnavailable
            : provider === 'apple'
              ? PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.appleVerifierUnavailable
              : PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.googleVerifierUnavailable,
    };
  }

  const signatureHeader = request.headers.get('stripe-signature');
  if (!signatureHeader) return { status: 'missing-credential', headerName: 'stripe-signature' };
  if (!env.STRIPE_WEBHOOK_SECRET) {
    return { status: 'unavailable', blocker: PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.stripeSecretMissing };
  }
  const body = await request.text();
  return (await verifyStripeWebhookSignature(
    body,
    signatureHeader,
    env.STRIPE_WEBHOOK_SECRET,
    env.STRIPE_WEBHOOK_TOLERANCE_SECONDS
  ))
    ? { status: 'verified' }
    : { status: 'rejected', reason: 'invalid-stripe-signature' };
}
