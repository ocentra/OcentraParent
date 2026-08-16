/* generated from crates/schema/src/billing_contracts_ts.rs */

type GeneratedParseResult<T> =
  | { readonly success: true; readonly data: T }
  | { readonly success: false; readonly error: TypeError };

function generatedRecord(value: unknown, field: string): Record<string, unknown> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new TypeError(`${field} must be an object`);
  }
  return value as Record<string, unknown>;
}

function generatedString(value: unknown, field: string): string {
  if (typeof value !== 'string' || value.trim().length === 0) {
    throw new TypeError(`${field} must be a non-empty string`);
  }
  return value;
}

function generatedNullableString(value: unknown, field: string): string | null {
  if (value === null) {
    return null;
  }
  return generatedString(value, field);
}

function generatedBoolean(value: unknown, field: string): boolean {
  if (typeof value !== 'boolean') {
    throw new TypeError(`${field} must be a boolean`);
  }
  return value;
}

function generatedNumber(value: unknown, field: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new TypeError(`${field} must be a finite number`);
  }
  return value;
}

function generatedEnum<const T extends readonly string[]>(value: unknown, field: string, allowed: T): T[number] {
  if (typeof value === 'string' && (allowed as readonly string[]).includes(value)) {
    return value as T[number];
  }
  throw new TypeError(`${field} must be one of ${allowed.join(', ')}`);
}

function generatedOptionalString(value: unknown, field: string): string | undefined {
  if (value === undefined || value === null) {
    return undefined;
  }
  return generatedString(value, field);
}

function generatedArray<T>(value: unknown, field: string, decoder: (item: unknown, index: number) => T): T[] {
  if (!Array.isArray(value)) {
    throw new TypeError(`${field} must be an array`);
  }
  return value.map((item, index) => decoder(item, index));
}

function generatedSchema<T>(
  name: string,
  decoder: (value: unknown) => T
): {
  parse(value: unknown): T;
  safeParse(value: unknown): GeneratedParseResult<T>;
} {
  return {
    parse(value: unknown): T {
      const parsed = decoder(value);
      if (parsed === null) {
        throw new TypeError(`${name} is invalid`);
      }
      return parsed;
    },
    safeParse(value: unknown): GeneratedParseResult<T> {
      try {
        return { success: true, data: decoder(value) };
      } catch (error) {
        return {
          success: false,
          error: error instanceof TypeError ? error : new TypeError(`${name} is invalid`),
        };
      }
    },
  };
}

export const BillingHostedReturnRoute = {
  CheckoutSuccess: {
    relativePath: '/family/billing/checkout/success',
  },
  CheckoutCancel: {
    relativePath: '/family/billing/checkout/cancel',
  },
  PortalReturn: {
    relativePath: '/family/billing/manage',
  },
} as const;

export type BillingCheckoutSessionRequest = {
  schemaVersion: 'billing-checkout-portal-boundary';
  requestId: string;
  kind: 'checkout-session-create';
  parentAccountRef?: string;
  familyRef?: string;
  subject?: string;
  planId: string;
  successRoute: { relativePath: string };
  cancelRoute: { relativePath: string };
  abuseGateState: string;
};

export type BillingPortalSessionRequest = {
  schemaVersion: 'billing-checkout-portal-boundary';
  requestId: string;
  kind: 'billing-portal-session-create';
  parentAccountRef?: string;
  familyRef?: string;
  subject?: string;
  returnRoute: { relativePath: string };
  abuseGateState: string;
};

export type BillingHostedSessionRejectionReason =
  | 'auth-required'
  | 'unauthorized-role'
  | 'invalid-plan'
  | 'redirect-not-allowlisted'
  | 'abuse-gate-required'
  | 'provider-unavailable';

export type BillingCheckoutSessionResponse = {
  schemaVersion: 'billing-checkout-portal-boundary';
  requestId: string;
  kind: 'checkout-session-create';
  status: 'accepted' | 'rejected';
  hostedSessionId: string | null;
  hostedUrl: string | null;
  expiresAt: string | null;
  rejectionReason: BillingHostedSessionRejectionReason | null;
  provider: 'stripe';
  ownerSubject: string;
  pendingEntitlementConfirmation: boolean;
};

export type BillingPortalSessionResponse = {
  schemaVersion: 'billing-checkout-portal-boundary';
  requestId: string;
  kind: 'billing-portal-session-create';
  status: 'accepted' | 'rejected';
  hostedSessionId: string | null;
  hostedUrl: string | null;
  expiresAt: string | null;
  rejectionReason: BillingHostedSessionRejectionReason | null;
  provider: 'stripe';
  ownerSubject: string;
  pendingEntitlementConfirmation: boolean;
};

export type BillingReferralInviteState = 'invite-created' | 'qualified-credit-granted' | 'fraud-review';

export type BillingReferralInviteResult = {
  requestId: string;
  status: 'accepted' | 'rejected' | 'manual-review';
  inviteState: BillingReferralInviteState | null;
  referralCode: string;
  rejectionReason:
    | 'self-referral-rejected'
    | 'same-household-rejected'
    | 'same-device-farm-rejected'
    | 'same-payment-method-manual-review'
    | 'fraud-review'
    | null;
  auditReference: string;
};

export type BillingReferralSummary = {
  subject: string;
  referralCode: string | null;
  availableCredits: number;
  activeReferredParents: number;
  pendingInvites: number;
  invites: ReadonlyArray<{
    inviteId: string;
    inviteState: BillingReferralInviteState;
    referralCode: string;
    invitedIdentifier: string;
    auditReference: string;
    updatedAt: string;
  }>;
  auditReference: string;
};

export type BillingAccountRuntimeStatusRow = {
  schemaVersion: 'billing-account-runtime-boundary-proof';
  boundaryId: string;
  parentAccount: {
    parentAccountId: string;
  };
  family: {
    familyId: string;
  };
  accountStatus: 'trialing' | 'active' | 'past-due' | 'backend-unavailable' | 'provider-unavailable' | 'manual-review';
  subscriptionStatus: 'active' | 'grace' | 'past-due';
  source: 'signed-local-snapshot' | 'manual-admin-review';
  backendRuntimeState: 'ready' | 'degraded' | 'unavailable';
  parentVisibleState: 'available' | 'past-due' | 'stale' | 'unavailable' | 'manual-review';
  localSafetyBehavior: 'unchanged' | 'grace-with-local-safety' | 'manual-review-with-local-safety';
  evidenceExportAccess: 'retained';
  childActivityCustody: 'not-included';
  providerSecretCustody: 'not-present';
  providerMode: 'stripe-hosted' | 'manual-invoice';
  nextRenewalAt: string | null;
  manualInvoiceState: {
    visible: boolean;
    invoiceState: 'manual-support-required' | null;
  };
  failureState: {
    failureKind: string;
    parentResolution: string;
    retryAllowed: boolean;
    retryAfter: string | null;
  } | null;
  auditReference: string;
};

export type BillingSupportAdminAccountSummary = {
  parentAccountRef: string;
  familyRef: string;
  parentVisibleState: 'available' | 'grace' | 'manual-review' | 'stale' | 'unavailable';
  subscriptionStatus: 'active' | 'grace' | 'past-due';
  planId: string;
  evidenceExportAccess: 'retained';
  childActivityCustody: 'not-included';
  providerSecretCustody: 'not-present';
  manualRequired: boolean;
  failureKind: string | null;
  auditReference: string;
  updatedAt: string;
};

export type BillingSupportAdminAuditEventSummary = {
  eventId: string;
  eventType: string;
  actorRole: 'parent' | 'support' | 'admin' | 'system';
  parentAccountRef: string;
  familyRef: string;
  auditReference: string;
  createdAt: string;
};

export type BillingSupportAdminDisputeSummary = {
  disputeId: string;
  parentAccountRef: string;
  familyRef: string;
  invoiceId: string;
  disputeState: 'dispute-opened' | 'dispute-won' | 'dispute-lost';
  entitlementEffect: 'manual-review-required' | 'grace-paid-access' | 'revoke-paid-access';
  manualRequired: boolean;
  auditReference: string;
  updatedAt: string;
};

export type BillingSupportAdminInvoiceSummary = {
  invoiceId: string;
  invoiceNumber: string;
  parentAccountRef: string;
  familyRef: string;
  planId: string;
  currency: 'USD';
  subtotalCents: number;
  taxCents: number;
  totalCents: number;
  invoiceVisibility: 'customer-portal-hosted' | 'manual-support-required';
  paymentState: 'paid' | 'grace' | 'unpaid' | 'refunded';
  provider: 'stripe' | 'manual-invoice';
  hostedUrl: string | null;
  periodStart: string;
  periodEnd: string;
  updatedAt: string;
  auditReference: string;
  manualRequired: boolean;
};

export type BillingSupportAdminReferralSummary = {
  referralCode: string;
  ownerSubject: string;
  creditedFamilies: number;
  invitedFamilies: number;
  abuseReviewState: 'review-required' | 'clear';
  auditReference: string;
  updatedAt: string;
};

export type BillingSupportAdminAccountsResponse = {
  status: 'ok';
  actorRole: 'support' | 'admin';
  resultCount: number;
  manualActionsPending: number;
  nonClaims: ReadonlyArray<string>;
  results: ReadonlyArray<BillingSupportAdminAccountSummary>;
};

export type BillingSupportAdminAuditEventsResponse = {
  status: 'ok';
  actorRole: 'support' | 'admin';
  resultCount: number;
  results: ReadonlyArray<BillingSupportAdminAuditEventSummary>;
};

export type BillingSupportAdminDisputesResponse = {
  status: 'ok';
  actorRole: 'support' | 'admin';
  resultCount: number;
  results: ReadonlyArray<BillingSupportAdminDisputeSummary>;
};

export type BillingSupportAdminInvoicesResponse = {
  status: 'ok';
  actorRole: 'support' | 'admin';
  resultCount: number;
  results: ReadonlyArray<BillingSupportAdminInvoiceSummary>;
};

export type BillingSupportAdminReferralsResponse = {
  status: 'ok';
  actorRole: 'support' | 'admin';
  resultCount: number;
  results: ReadonlyArray<BillingSupportAdminReferralSummary>;
};

export type BillingSupportAdminReconciliationSummary = {
  requestId: string;
  status: 'accepted';
  queued: boolean;
  driftFamiliesVisible: number;
  retryBacklogVisible: number;
  deadLetterVisible: number;
  auditReference: string;
};

export type BillingSupportAdminRefundResult = {
  requestId: string;
  status: 'accepted' | 'rejected';
  invoiceId: string | null;
  refundState: 'refund-requested' | 'refund-settled' | 'manual-review-required';
  amountCents: number | null;
  auditReference: string;
  rejectionReason: 'invoice-not-found' | null;
};

function decodeHostedReturnRoute(value: unknown): { relativePath: string } {
  const record = generatedRecord(value, 'BillingHostedReturnRoute');
  const relativePath = generatedString(record.relativePath, 'BillingHostedReturnRoute.relativePath');
  if (
    relativePath !== BillingHostedReturnRoute.CheckoutSuccess.relativePath &&
    relativePath !== BillingHostedReturnRoute.CheckoutCancel.relativePath &&
    relativePath !== BillingHostedReturnRoute.PortalReturn.relativePath
  ) {
    throw new TypeError('BillingHostedReturnRoute.relativePath must be a Rust-owned billing path');
  }
  return { relativePath };
}

function decodeBillingCheckoutSessionRequest(value: unknown): BillingCheckoutSessionRequest {
  const record = generatedRecord(value, 'BillingCheckoutSessionRequest');
  return {
    schemaVersion: generatedEnum(record.schemaVersion, 'BillingCheckoutSessionRequest.schemaVersion', [
      'billing-checkout-portal-boundary',
    ]),
    requestId: generatedString(record.requestId, 'BillingCheckoutSessionRequest.requestId'),
    kind: generatedEnum(record.kind, 'BillingCheckoutSessionRequest.kind', ['checkout-session-create']),
    parentAccountRef: generatedOptionalString(
      record.parentAccountRef,
      'BillingCheckoutSessionRequest.parentAccountRef'
    ),
    familyRef: generatedOptionalString(record.familyRef, 'BillingCheckoutSessionRequest.familyRef'),
    subject: generatedOptionalString(record.subject, 'BillingCheckoutSessionRequest.subject'),
    planId: generatedString(record.planId, 'BillingCheckoutSessionRequest.planId'),
    successRoute: decodeHostedReturnRoute(record.successRoute),
    cancelRoute: decodeHostedReturnRoute(record.cancelRoute),
    abuseGateState: generatedString(record.abuseGateState, 'BillingCheckoutSessionRequest.abuseGateState'),
  };
}

function decodeBillingPortalSessionRequest(value: unknown): BillingPortalSessionRequest {
  const record = generatedRecord(value, 'BillingPortalSessionRequest');
  return {
    schemaVersion: generatedEnum(record.schemaVersion, 'BillingPortalSessionRequest.schemaVersion', [
      'billing-checkout-portal-boundary',
    ]),
    requestId: generatedString(record.requestId, 'BillingPortalSessionRequest.requestId'),
    kind: generatedEnum(record.kind, 'BillingPortalSessionRequest.kind', ['billing-portal-session-create']),
    parentAccountRef: generatedOptionalString(record.parentAccountRef, 'BillingPortalSessionRequest.parentAccountRef'),
    familyRef: generatedOptionalString(record.familyRef, 'BillingPortalSessionRequest.familyRef'),
    subject: generatedOptionalString(record.subject, 'BillingPortalSessionRequest.subject'),
    returnRoute: decodeHostedReturnRoute(record.returnRoute),
    abuseGateState: generatedString(record.abuseGateState, 'BillingPortalSessionRequest.abuseGateState'),
  };
}

function decodeBillingCheckoutSessionResponse(value: unknown): BillingCheckoutSessionResponse {
  const record = generatedRecord(value, 'BillingCheckoutSessionResponse');
  return {
    schemaVersion: generatedEnum(record.schemaVersion, 'BillingCheckoutSessionResponse.schemaVersion', [
      'billing-checkout-portal-boundary',
    ]),
    requestId: generatedString(record.requestId, 'BillingCheckoutSessionResponse.requestId'),
    kind: generatedEnum(record.kind, 'BillingCheckoutSessionResponse.kind', ['checkout-session-create']),
    status: generatedEnum(record.status, 'BillingCheckoutSessionResponse.status', ['accepted', 'rejected']),
    hostedSessionId: generatedNullableString(record.hostedSessionId, 'BillingCheckoutSessionResponse.hostedSessionId'),
    hostedUrl: generatedNullableString(record.hostedUrl, 'BillingCheckoutSessionResponse.hostedUrl'),
    expiresAt: generatedNullableString(record.expiresAt, 'BillingCheckoutSessionResponse.expiresAt'),
    rejectionReason:
      record.rejectionReason === null
        ? null
        : generatedEnum(record.rejectionReason, 'BillingCheckoutSessionResponse.rejectionReason', [
            'auth-required',
            'unauthorized-role',
            'invalid-plan',
            'redirect-not-allowlisted',
            'abuse-gate-required',
            'provider-unavailable',
          ]),
    provider: generatedEnum(record.provider, 'BillingCheckoutSessionResponse.provider', ['stripe']),
    ownerSubject: generatedString(record.ownerSubject, 'BillingCheckoutSessionResponse.ownerSubject'),
    pendingEntitlementConfirmation: generatedBoolean(
      record.pendingEntitlementConfirmation,
      'BillingCheckoutSessionResponse.pendingEntitlementConfirmation'
    ),
  };
}

function decodeBillingPortalSessionResponse(value: unknown): BillingPortalSessionResponse {
  const record = generatedRecord(value, 'BillingPortalSessionResponse');
  return {
    schemaVersion: generatedEnum(record.schemaVersion, 'BillingPortalSessionResponse.schemaVersion', [
      'billing-checkout-portal-boundary',
    ]),
    requestId: generatedString(record.requestId, 'BillingPortalSessionResponse.requestId'),
    kind: generatedEnum(record.kind, 'BillingPortalSessionResponse.kind', ['billing-portal-session-create']),
    status: generatedEnum(record.status, 'BillingPortalSessionResponse.status', ['accepted', 'rejected']),
    hostedSessionId: generatedNullableString(record.hostedSessionId, 'BillingPortalSessionResponse.hostedSessionId'),
    hostedUrl: generatedNullableString(record.hostedUrl, 'BillingPortalSessionResponse.hostedUrl'),
    expiresAt: generatedNullableString(record.expiresAt, 'BillingPortalSessionResponse.expiresAt'),
    rejectionReason:
      record.rejectionReason === null
        ? null
        : generatedEnum(record.rejectionReason, 'BillingPortalSessionResponse.rejectionReason', [
            'auth-required',
            'unauthorized-role',
            'invalid-plan',
            'redirect-not-allowlisted',
            'abuse-gate-required',
            'provider-unavailable',
          ]),
    provider: generatedEnum(record.provider, 'BillingPortalSessionResponse.provider', ['stripe']),
    ownerSubject: generatedString(record.ownerSubject, 'BillingPortalSessionResponse.ownerSubject'),
    pendingEntitlementConfirmation: generatedBoolean(
      record.pendingEntitlementConfirmation,
      'BillingPortalSessionResponse.pendingEntitlementConfirmation'
    ),
  };
}

function decodeBillingReferralInviteResult(value: unknown): BillingReferralInviteResult {
  const record = generatedRecord(value, 'BillingReferralInviteResult');
  return {
    requestId: generatedString(record.requestId, 'BillingReferralInviteResult.requestId'),
    status: generatedEnum(record.status, 'BillingReferralInviteResult.status', [
      'accepted',
      'rejected',
      'manual-review',
    ]),
    inviteState:
      record.inviteState === null
        ? null
        : generatedEnum(record.inviteState, 'BillingReferralInviteResult.inviteState', [
            'invite-created',
            'qualified-credit-granted',
            'fraud-review',
          ]),
    referralCode: generatedString(record.referralCode, 'BillingReferralInviteResult.referralCode'),
    rejectionReason:
      record.rejectionReason === null
        ? null
        : generatedEnum(record.rejectionReason, 'BillingReferralInviteResult.rejectionReason', [
            'self-referral-rejected',
            'same-household-rejected',
            'same-device-farm-rejected',
            'same-payment-method-manual-review',
            'fraud-review',
          ]),
    auditReference: generatedString(record.auditReference, 'BillingReferralInviteResult.auditReference'),
  };
}

function decodeReferralInviteRow(value: unknown, field: string): BillingReferralSummary['invites'][number] {
  const record = generatedRecord(value, field);
  return {
    inviteId: generatedString(record.inviteId, `${field}.inviteId`),
    inviteState: generatedEnum(record.inviteState, `${field}.inviteState`, [
      'invite-created',
      'qualified-credit-granted',
      'fraud-review',
    ]),
    referralCode: generatedString(record.referralCode, `${field}.referralCode`),
    invitedIdentifier: generatedString(record.invitedIdentifier, `${field}.invitedIdentifier`),
    auditReference: generatedString(record.auditReference, `${field}.auditReference`),
    updatedAt: generatedString(record.updatedAt, `${field}.updatedAt`),
  };
}

function decodeBillingReferralSummary(value: unknown): BillingReferralSummary {
  const record = generatedRecord(value, 'BillingReferralSummary');
  return {
    subject: generatedString(record.subject, 'BillingReferralSummary.subject'),
    referralCode: generatedNullableString(record.referralCode, 'BillingReferralSummary.referralCode'),
    availableCredits: generatedNumber(record.availableCredits, 'BillingReferralSummary.availableCredits'),
    activeReferredParents: generatedNumber(
      record.activeReferredParents,
      'BillingReferralSummary.activeReferredParents'
    ),
    pendingInvites: generatedNumber(record.pendingInvites, 'BillingReferralSummary.pendingInvites'),
    invites: generatedArray(record.invites, 'BillingReferralSummary.invites', (item, index) =>
      decodeReferralInviteRow(item, `BillingReferralSummary.invites[${index}]`)
    ),
    auditReference: generatedString(record.auditReference, 'BillingReferralSummary.auditReference'),
  };
}

function decodeBillingManualInvoiceState(value: unknown): BillingAccountRuntimeStatusRow['manualInvoiceState'] {
  const record = generatedRecord(value, 'BillingAccountRuntimeStatusRow.manualInvoiceState');
  return {
    visible: generatedBoolean(record.visible, 'BillingAccountRuntimeStatusRow.manualInvoiceState.visible'),
    invoiceState:
      record.invoiceState === null
        ? null
        : generatedEnum(record.invoiceState, 'BillingAccountRuntimeStatusRow.manualInvoiceState.invoiceState', [
            'manual-support-required',
          ]),
  };
}

function decodeBillingAccountRuntimeStatusRow(value: unknown): BillingAccountRuntimeStatusRow {
  const record = generatedRecord(value, 'BillingAccountRuntimeStatusRow');
  return {
    schemaVersion: generatedEnum(record.schemaVersion, 'BillingAccountRuntimeStatusRow.schemaVersion', [
      'billing-account-runtime-boundary-proof',
    ]),
    boundaryId: generatedString(record.boundaryId, 'BillingAccountRuntimeStatusRow.boundaryId'),
    parentAccount: {
      parentAccountId: generatedString(
        generatedRecord(record.parentAccount, 'BillingAccountRuntimeStatusRow.parentAccount').parentAccountId,
        'BillingAccountRuntimeStatusRow.parentAccount.parentAccountId'
      ),
    },
    family: {
      familyId: generatedString(
        generatedRecord(record.family, 'BillingAccountRuntimeStatusRow.family').familyId,
        'BillingAccountRuntimeStatusRow.family.familyId'
      ),
    },
    accountStatus: generatedEnum(record.accountStatus, 'BillingAccountRuntimeStatusRow.accountStatus', [
      'trialing',
      'active',
      'past-due',
      'backend-unavailable',
      'provider-unavailable',
      'manual-review',
    ]),
    subscriptionStatus: generatedEnum(record.subscriptionStatus, 'BillingAccountRuntimeStatusRow.subscriptionStatus', [
      'active',
      'grace',
      'past-due',
    ]),
    source: generatedEnum(record.source, 'BillingAccountRuntimeStatusRow.source', [
      'signed-local-snapshot',
      'manual-admin-review',
    ]),
    backendRuntimeState: generatedEnum(
      record.backendRuntimeState,
      'BillingAccountRuntimeStatusRow.backendRuntimeState',
      ['ready', 'degraded', 'unavailable']
    ),
    parentVisibleState: generatedEnum(record.parentVisibleState, 'BillingAccountRuntimeStatusRow.parentVisibleState', [
      'available',
      'past-due',
      'stale',
      'unavailable',
      'manual-review',
    ]),
    localSafetyBehavior: generatedEnum(
      record.localSafetyBehavior,
      'BillingAccountRuntimeStatusRow.localSafetyBehavior',
      ['unchanged', 'grace-with-local-safety', 'manual-review-with-local-safety']
    ),
    evidenceExportAccess: generatedEnum(
      record.evidenceExportAccess,
      'BillingAccountRuntimeStatusRow.evidenceExportAccess',
      ['retained']
    ),
    childActivityCustody: generatedEnum(
      record.childActivityCustody,
      'BillingAccountRuntimeStatusRow.childActivityCustody',
      ['not-included']
    ),
    providerSecretCustody: generatedEnum(
      record.providerSecretCustody,
      'BillingAccountRuntimeStatusRow.providerSecretCustody',
      ['not-present']
    ),
    providerMode: generatedEnum(record.providerMode, 'BillingAccountRuntimeStatusRow.providerMode', [
      'stripe-hosted',
      'manual-invoice',
    ]),
    nextRenewalAt: generatedNullableString(record.nextRenewalAt, 'BillingAccountRuntimeStatusRow.nextRenewalAt'),
    manualInvoiceState: decodeBillingManualInvoiceState(record.manualInvoiceState),
    failureState:
      record.failureState === null
        ? null
        : (() => {
            const failureState = generatedRecord(record.failureState, 'BillingAccountRuntimeStatusRow.failureState');
            return {
              failureKind: generatedString(
                failureState.failureKind,
                'BillingAccountRuntimeStatusRow.failureState.failureKind'
              ),
              parentResolution: generatedString(
                failureState.parentResolution,
                'BillingAccountRuntimeStatusRow.failureState.parentResolution'
              ),
              retryAllowed: generatedBoolean(
                failureState.retryAllowed,
                'BillingAccountRuntimeStatusRow.failureState.retryAllowed'
              ),
              retryAfter: generatedNullableString(
                failureState.retryAfter,
                'BillingAccountRuntimeStatusRow.failureState.retryAfter'
              ),
            };
          })(),
    auditReference: generatedString(record.auditReference, 'BillingAccountRuntimeStatusRow.auditReference'),
  };
}

function decodeBillingSupportAdminAccountSummary(value: unknown): BillingSupportAdminAccountSummary {
  const record = generatedRecord(value, 'BillingSupportAdminAccountSummary');
  return {
    parentAccountRef: generatedString(record.parentAccountRef, 'BillingSupportAdminAccountSummary.parentAccountRef'),
    familyRef: generatedString(record.familyRef, 'BillingSupportAdminAccountSummary.familyRef'),
    parentVisibleState: generatedEnum(
      record.parentVisibleState,
      'BillingSupportAdminAccountSummary.parentVisibleState',
      ['available', 'grace', 'manual-review', 'stale', 'unavailable']
    ),
    subscriptionStatus: generatedEnum(
      record.subscriptionStatus,
      'BillingSupportAdminAccountSummary.subscriptionStatus',
      ['active', 'grace', 'past-due']
    ),
    planId: generatedString(record.planId, 'BillingSupportAdminAccountSummary.planId'),
    evidenceExportAccess: generatedEnum(
      record.evidenceExportAccess,
      'BillingSupportAdminAccountSummary.evidenceExportAccess',
      ['retained']
    ),
    childActivityCustody: generatedEnum(
      record.childActivityCustody,
      'BillingSupportAdminAccountSummary.childActivityCustody',
      ['not-included']
    ),
    providerSecretCustody: generatedEnum(
      record.providerSecretCustody,
      'BillingSupportAdminAccountSummary.providerSecretCustody',
      ['not-present']
    ),
    manualRequired: generatedBoolean(record.manualRequired, 'BillingSupportAdminAccountSummary.manualRequired'),
    failureKind: generatedNullableString(record.failureKind, 'BillingSupportAdminAccountSummary.failureKind'),
    auditReference: generatedString(record.auditReference, 'BillingSupportAdminAccountSummary.auditReference'),
    updatedAt: generatedString(record.updatedAt, 'BillingSupportAdminAccountSummary.updatedAt'),
  };
}

function decodeBillingSupportAdminAuditEventSummary(value: unknown): BillingSupportAdminAuditEventSummary {
  const record = generatedRecord(value, 'BillingSupportAdminAuditEventSummary');
  return {
    eventId: generatedString(record.eventId, 'BillingSupportAdminAuditEventSummary.eventId'),
    eventType: generatedString(record.eventType, 'BillingSupportAdminAuditEventSummary.eventType'),
    actorRole: generatedEnum(record.actorRole, 'BillingSupportAdminAuditEventSummary.actorRole', [
      'parent',
      'support',
      'admin',
      'system',
    ]),
    parentAccountRef: generatedString(record.parentAccountRef, 'BillingSupportAdminAuditEventSummary.parentAccountRef'),
    familyRef: generatedString(record.familyRef, 'BillingSupportAdminAuditEventSummary.familyRef'),
    auditReference: generatedString(record.auditReference, 'BillingSupportAdminAuditEventSummary.auditReference'),
    createdAt: generatedString(record.createdAt, 'BillingSupportAdminAuditEventSummary.createdAt'),
  };
}

function decodeBillingSupportAdminDisputeSummary(value: unknown): BillingSupportAdminDisputeSummary {
  const record = generatedRecord(value, 'BillingSupportAdminDisputeSummary');
  return {
    disputeId: generatedString(record.disputeId, 'BillingSupportAdminDisputeSummary.disputeId'),
    parentAccountRef: generatedString(record.parentAccountRef, 'BillingSupportAdminDisputeSummary.parentAccountRef'),
    familyRef: generatedString(record.familyRef, 'BillingSupportAdminDisputeSummary.familyRef'),
    invoiceId: generatedString(record.invoiceId, 'BillingSupportAdminDisputeSummary.invoiceId'),
    disputeState: generatedEnum(record.disputeState, 'BillingSupportAdminDisputeSummary.disputeState', [
      'dispute-opened',
      'dispute-won',
      'dispute-lost',
    ]),
    entitlementEffect: generatedEnum(record.entitlementEffect, 'BillingSupportAdminDisputeSummary.entitlementEffect', [
      'manual-review-required',
      'grace-paid-access',
      'revoke-paid-access',
    ]),
    manualRequired: generatedBoolean(record.manualRequired, 'BillingSupportAdminDisputeSummary.manualRequired'),
    auditReference: generatedString(record.auditReference, 'BillingSupportAdminDisputeSummary.auditReference'),
    updatedAt: generatedString(record.updatedAt, 'BillingSupportAdminDisputeSummary.updatedAt'),
  };
}

function decodeBillingSupportAdminInvoiceSummary(value: unknown): BillingSupportAdminInvoiceSummary {
  const record = generatedRecord(value, 'BillingSupportAdminInvoiceSummary');
  return {
    invoiceId: generatedString(record.invoiceId, 'BillingSupportAdminInvoiceSummary.invoiceId'),
    invoiceNumber: generatedString(record.invoiceNumber, 'BillingSupportAdminInvoiceSummary.invoiceNumber'),
    parentAccountRef: generatedString(record.parentAccountRef, 'BillingSupportAdminInvoiceSummary.parentAccountRef'),
    familyRef: generatedString(record.familyRef, 'BillingSupportAdminInvoiceSummary.familyRef'),
    planId: generatedString(record.planId, 'BillingSupportAdminInvoiceSummary.planId'),
    currency: generatedEnum(record.currency, 'BillingSupportAdminInvoiceSummary.currency', ['USD']),
    subtotalCents: generatedNumber(record.subtotalCents, 'BillingSupportAdminInvoiceSummary.subtotalCents'),
    taxCents: generatedNumber(record.taxCents, 'BillingSupportAdminInvoiceSummary.taxCents'),
    totalCents: generatedNumber(record.totalCents, 'BillingSupportAdminInvoiceSummary.totalCents'),
    invoiceVisibility: generatedEnum(record.invoiceVisibility, 'BillingSupportAdminInvoiceSummary.invoiceVisibility', [
      'customer-portal-hosted',
      'manual-support-required',
    ]),
    paymentState: generatedEnum(record.paymentState, 'BillingSupportAdminInvoiceSummary.paymentState', [
      'paid',
      'grace',
      'unpaid',
      'refunded',
    ]),
    provider: generatedEnum(record.provider, 'BillingSupportAdminInvoiceSummary.provider', [
      'stripe',
      'manual-invoice',
    ]),
    hostedUrl: generatedNullableString(record.hostedUrl, 'BillingSupportAdminInvoiceSummary.hostedUrl'),
    periodStart: generatedString(record.periodStart, 'BillingSupportAdminInvoiceSummary.periodStart'),
    periodEnd: generatedString(record.periodEnd, 'BillingSupportAdminInvoiceSummary.periodEnd'),
    updatedAt: generatedString(record.updatedAt, 'BillingSupportAdminInvoiceSummary.updatedAt'),
    auditReference: generatedString(record.auditReference, 'BillingSupportAdminInvoiceSummary.auditReference'),
    manualRequired: generatedBoolean(record.manualRequired, 'BillingSupportAdminInvoiceSummary.manualRequired'),
  };
}

function decodeBillingSupportAdminReferralSummary(value: unknown): BillingSupportAdminReferralSummary {
  const record = generatedRecord(value, 'BillingSupportAdminReferralSummary');
  return {
    referralCode: generatedString(record.referralCode, 'BillingSupportAdminReferralSummary.referralCode'),
    ownerSubject: generatedString(record.ownerSubject, 'BillingSupportAdminReferralSummary.ownerSubject'),
    creditedFamilies: generatedNumber(record.creditedFamilies, 'BillingSupportAdminReferralSummary.creditedFamilies'),
    invitedFamilies: generatedNumber(record.invitedFamilies, 'BillingSupportAdminReferralSummary.invitedFamilies'),
    abuseReviewState: generatedEnum(record.abuseReviewState, 'BillingSupportAdminReferralSummary.abuseReviewState', [
      'review-required',
      'clear',
    ]),
    auditReference: generatedString(record.auditReference, 'BillingSupportAdminReferralSummary.auditReference'),
    updatedAt: generatedString(record.updatedAt, 'BillingSupportAdminReferralSummary.updatedAt'),
  };
}

function decodeBillingSupportAdminAccountsResponse(value: unknown): BillingSupportAdminAccountsResponse {
  const record = generatedRecord(value, 'BillingSupportAdminAccountsResponse');
  return {
    status: generatedEnum(record.status, 'BillingSupportAdminAccountsResponse.status', ['ok']),
    actorRole: generatedEnum(record.actorRole, 'BillingSupportAdminAccountsResponse.actorRole', ['support', 'admin']),
    resultCount: generatedNumber(record.resultCount, 'BillingSupportAdminAccountsResponse.resultCount'),
    manualActionsPending: generatedNumber(
      record.manualActionsPending,
      'BillingSupportAdminAccountsResponse.manualActionsPending'
    ),
    nonClaims: generatedArray(record.nonClaims, 'BillingSupportAdminAccountsResponse.nonClaims', (item, index) =>
      generatedString(item, `BillingSupportAdminAccountsResponse.nonClaims[${index}]`)
    ),
    results: generatedArray(record.results, 'BillingSupportAdminAccountsResponse.results', (item) =>
      decodeBillingSupportAdminAccountSummary(item)
    ),
  };
}

function decodeBillingSupportAdminAuditEventsResponse(value: unknown): BillingSupportAdminAuditEventsResponse {
  const record = generatedRecord(value, 'BillingSupportAdminAuditEventsResponse');
  return {
    status: generatedEnum(record.status, 'BillingSupportAdminAuditEventsResponse.status', ['ok']),
    actorRole: generatedEnum(record.actorRole, 'BillingSupportAdminAuditEventsResponse.actorRole', [
      'support',
      'admin',
    ]),
    resultCount: generatedNumber(record.resultCount, 'BillingSupportAdminAuditEventsResponse.resultCount'),
    results: generatedArray(record.results, 'BillingSupportAdminAuditEventsResponse.results', (item) =>
      decodeBillingSupportAdminAuditEventSummary(item)
    ),
  };
}

function decodeBillingSupportAdminDisputesResponse(value: unknown): BillingSupportAdminDisputesResponse {
  const record = generatedRecord(value, 'BillingSupportAdminDisputesResponse');
  return {
    status: generatedEnum(record.status, 'BillingSupportAdminDisputesResponse.status', ['ok']),
    actorRole: generatedEnum(record.actorRole, 'BillingSupportAdminDisputesResponse.actorRole', ['support', 'admin']),
    resultCount: generatedNumber(record.resultCount, 'BillingSupportAdminDisputesResponse.resultCount'),
    results: generatedArray(record.results, 'BillingSupportAdminDisputesResponse.results', (item) =>
      decodeBillingSupportAdminDisputeSummary(item)
    ),
  };
}

function decodeBillingSupportAdminInvoicesResponse(value: unknown): BillingSupportAdminInvoicesResponse {
  const record = generatedRecord(value, 'BillingSupportAdminInvoicesResponse');
  return {
    status: generatedEnum(record.status, 'BillingSupportAdminInvoicesResponse.status', ['ok']),
    actorRole: generatedEnum(record.actorRole, 'BillingSupportAdminInvoicesResponse.actorRole', ['support', 'admin']),
    resultCount: generatedNumber(record.resultCount, 'BillingSupportAdminInvoicesResponse.resultCount'),
    results: generatedArray(record.results, 'BillingSupportAdminInvoicesResponse.results', (item) =>
      decodeBillingSupportAdminInvoiceSummary(item)
    ),
  };
}

function decodeBillingSupportAdminReferralsResponse(value: unknown): BillingSupportAdminReferralsResponse {
  const record = generatedRecord(value, 'BillingSupportAdminReferralsResponse');
  return {
    status: generatedEnum(record.status, 'BillingSupportAdminReferralsResponse.status', ['ok']),
    actorRole: generatedEnum(record.actorRole, 'BillingSupportAdminReferralsResponse.actorRole', ['support', 'admin']),
    resultCount: generatedNumber(record.resultCount, 'BillingSupportAdminReferralsResponse.resultCount'),
    results: generatedArray(record.results, 'BillingSupportAdminReferralsResponse.results', (item) =>
      decodeBillingSupportAdminReferralSummary(item)
    ),
  };
}

function decodeBillingSupportAdminReconciliationSummary(value: unknown): BillingSupportAdminReconciliationSummary {
  const record = generatedRecord(value, 'BillingSupportAdminReconciliationSummary');
  return {
    requestId: generatedString(record.requestId, 'BillingSupportAdminReconciliationSummary.requestId'),
    status: generatedEnum(record.status, 'BillingSupportAdminReconciliationSummary.status', ['accepted']),
    queued: generatedBoolean(record.queued, 'BillingSupportAdminReconciliationSummary.queued'),
    driftFamiliesVisible: generatedNumber(
      record.driftFamiliesVisible,
      'BillingSupportAdminReconciliationSummary.driftFamiliesVisible'
    ),
    retryBacklogVisible: generatedNumber(
      record.retryBacklogVisible,
      'BillingSupportAdminReconciliationSummary.retryBacklogVisible'
    ),
    deadLetterVisible: generatedNumber(
      record.deadLetterVisible,
      'BillingSupportAdminReconciliationSummary.deadLetterVisible'
    ),
    auditReference: generatedString(record.auditReference, 'BillingSupportAdminReconciliationSummary.auditReference'),
  };
}

function decodeBillingSupportAdminRefundResult(value: unknown): BillingSupportAdminRefundResult {
  const record = generatedRecord(value, 'BillingSupportAdminRefundResult');
  return {
    requestId: generatedString(record.requestId, 'BillingSupportAdminRefundResult.requestId'),
    status: generatedEnum(record.status, 'BillingSupportAdminRefundResult.status', ['accepted', 'rejected']),
    invoiceId: generatedNullableString(record.invoiceId, 'BillingSupportAdminRefundResult.invoiceId'),
    refundState: generatedEnum(record.refundState, 'BillingSupportAdminRefundResult.refundState', [
      'refund-requested',
      'refund-settled',
      'manual-review-required',
    ]),
    amountCents:
      record.amountCents === null
        ? null
        : generatedNumber(record.amountCents, 'BillingSupportAdminRefundResult.amountCents'),
    auditReference: generatedString(record.auditReference, 'BillingSupportAdminRefundResult.auditReference'),
    rejectionReason:
      record.rejectionReason === null
        ? null
        : generatedEnum(record.rejectionReason, 'BillingSupportAdminRefundResult.rejectionReason', [
            'invoice-not-found',
          ]),
  };
}

export const BillingCheckoutSessionRequestSchema = generatedSchema(
  'BillingCheckoutSessionRequest',
  decodeBillingCheckoutSessionRequest
);
export const BillingPortalSessionRequestSchema = generatedSchema(
  'BillingPortalSessionRequest',
  decodeBillingPortalSessionRequest
);
export const BillingCheckoutSessionResponseSchema = generatedSchema(
  'BillingCheckoutSessionResponse',
  decodeBillingCheckoutSessionResponse
);
export const BillingPortalSessionResponseSchema = generatedSchema(
  'BillingPortalSessionResponse',
  decodeBillingPortalSessionResponse
);
export const BillingReferralInviteResultSchema = generatedSchema(
  'BillingReferralInviteResult',
  decodeBillingReferralInviteResult
);
export const BillingReferralSummarySchema = generatedSchema('BillingReferralSummary', decodeBillingReferralSummary);
export const BillingAccountRuntimeStatusRowSchema = generatedSchema(
  'BillingAccountRuntimeStatusRow',
  decodeBillingAccountRuntimeStatusRow
);
export const BillingSupportAdminAccountSummarySchema = generatedSchema(
  'BillingSupportAdminAccountSummary',
  decodeBillingSupportAdminAccountSummary
);
export const BillingSupportAdminAuditEventSummarySchema = generatedSchema(
  'BillingSupportAdminAuditEventSummary',
  decodeBillingSupportAdminAuditEventSummary
);
export const BillingSupportAdminDisputeSummarySchema = generatedSchema(
  'BillingSupportAdminDisputeSummary',
  decodeBillingSupportAdminDisputeSummary
);
export const BillingSupportAdminInvoiceSummarySchema = generatedSchema(
  'BillingSupportAdminInvoiceSummary',
  decodeBillingSupportAdminInvoiceSummary
);
export const BillingSupportAdminReferralSummarySchema = generatedSchema(
  'BillingSupportAdminReferralSummary',
  decodeBillingSupportAdminReferralSummary
);
export const BillingSupportAdminAccountsResponseSchema = generatedSchema(
  'BillingSupportAdminAccountsResponse',
  decodeBillingSupportAdminAccountsResponse
);
export const BillingSupportAdminAuditEventsResponseSchema = generatedSchema(
  'BillingSupportAdminAuditEventsResponse',
  decodeBillingSupportAdminAuditEventsResponse
);
export const BillingSupportAdminDisputesResponseSchema = generatedSchema(
  'BillingSupportAdminDisputesResponse',
  decodeBillingSupportAdminDisputesResponse
);
export const BillingSupportAdminInvoicesResponseSchema = generatedSchema(
  'BillingSupportAdminInvoicesResponse',
  decodeBillingSupportAdminInvoicesResponse
);
export const BillingSupportAdminReferralsResponseSchema = generatedSchema(
  'BillingSupportAdminReferralsResponse',
  decodeBillingSupportAdminReferralsResponse
);
export const BillingSupportAdminReconciliationSummarySchema = generatedSchema(
  'BillingSupportAdminReconciliationSummary',
  decodeBillingSupportAdminReconciliationSummary
);
export const BillingSupportAdminRefundResultSchema = generatedSchema(
  'BillingSupportAdminRefundResult',
  decodeBillingSupportAdminRefundResult
);
