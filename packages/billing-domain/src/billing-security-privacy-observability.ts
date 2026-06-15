import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  BillingAbuseProtectedOperationSchema,
  BillingAlertKindSchema,
  BillingBotProtectionModeSchema,
  BillingForbiddenMetadataDataClassSchema,
  BillingLogRedactionClaimSchema,
  BillingLogRedactionStateSchema,
  BillingLogSignalSchema,
  BillingMetadataAllowlistedFieldSchema,
  BillingMetadataAllowlistClaimSchema,
  BillingMetadataDenylistClaimSchema,
  BillingMetadataSurfaceSchema,
  BillingSecretScanStateSchema,
  BillingSecurityPrivacyAuditReferenceSchema,
  BillingSecurityPrivacyBoundaryIdSchema,
  BillingSecurityPrivacyNonClaimSchema,
  BillingSecurityPrivacyObservabilitySchemaVersionSchema,
  type BillingAlertKind,
  type BillingMetadataSurface,
  type BillingSecurityPrivacyNonClaim,
} from './billing-security-privacy-observability-values';

export const BillingMetadataAllowlistRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSecurityPrivacyObservabilitySchemaVersionSchema,
    boundaryId: BillingSecurityPrivacyBoundaryIdSchema,
    metadataSurface: BillingMetadataSurfaceSchema,
    allowedField: BillingMetadataAllowlistedFieldSchema,
    typedReferenceOnly: Schema.Boolean,
    auditReference: BillingSecurityPrivacyAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.typedReferenceOnly ||
        'Expected billing metadata allowlist rows to stay reference-only instead of raw family payloads'
    )
  )
);

export const BillingMetadataDenylistRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSecurityPrivacyObservabilitySchemaVersionSchema,
    metadataSurface: BillingMetadataSurfaceSchema,
    forbiddenClass: BillingForbiddenMetadataDataClassSchema,
    blocked: Schema.Boolean,
    auditReference: BillingSecurityPrivacyAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) => row.blocked || 'Expected forbidden billing metadata classes to be explicitly blocked'
    )
  )
);

export const BillingLogRedactionRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSecurityPrivacyObservabilitySchemaVersionSchema,
    logSignal: BillingLogSignalSchema,
    redactionState: BillingLogRedactionStateSchema,
    paymentIdentifiersRedacted: Schema.Boolean,
    childSafetyDataAbsent: Schema.Boolean,
    auditReference: BillingSecurityPrivacyAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.paymentIdentifiersRedacted ||
        'Expected billing observability logs to redact payment-facing identifiers'
    ),
    Schema.filter(
      (row) =>
        row.childSafetyDataAbsent ||
        'Expected billing observability logs to exclude child activity and safety payloads'
    )
  )
);

export const BillingAbuseProtectionRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSecurityPrivacyObservabilitySchemaVersionSchema,
    operation: BillingAbuseProtectedOperationSchema,
    rateLimitEnabled: Schema.Boolean,
    botProtectionMode: BillingBotProtectionModeSchema,
    payloadSchemaValidated: Schema.Boolean,
    auditReference: BillingSecurityPrivacyAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) => row.rateLimitEnabled || 'Expected billing abuse control rows to keep rate limiting enabled'
    ),
    Schema.filter(
      (row) =>
        row.payloadSchemaValidated ||
        'Expected billing abuse control rows to keep payload schema validation enabled'
    ),
    Schema.filter(
      (row) =>
        row.operation !== 'provider-webhook-ingest'
          ? row.botProtectionMode !== 'not-applicable'
          : row.botProtectionMode === 'not-applicable' ||
            'Expected interactive billing flows to keep bot protection enabled while webhook ingest remains service-to-service'
    )
  )
);

export const BillingWebhookSecurityRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSecurityPrivacyObservabilitySchemaVersionSchema,
    surface: Schema.Literal('provider-webhook-ingest'),
    signatureVerified: Schema.Boolean,
    timestampWithinTolerance: Schema.Boolean,
    replayCacheChecked: Schema.Boolean,
    secretScanState: BillingSecretScanStateSchema,
    auditReference: BillingSecurityPrivacyAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) => row.signatureVerified || 'Expected billing webhooks to require signature verification'
    ),
    Schema.filter(
      (row) =>
        row.timestampWithinTolerance ||
        'Expected billing webhooks to enforce timestamp tolerance before processing'
    ),
    Schema.filter(
      (row) => row.replayCacheChecked || 'Expected billing webhooks to guard against replay delivery'
    ),
    Schema.filter(
      (row) =>
        row.secretScanState === 'clean' ||
        'Expected billing security proof to keep secret scans clean before claiming readiness'
    )
  )
);

export const BillingAlertRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSecurityPrivacyObservabilitySchemaVersionSchema,
    alertKind: BillingAlertKindSchema,
    configured: Schema.Boolean,
    redactedPayloadOnly: Schema.Boolean,
    auditReference: BillingSecurityPrivacyAuditReferenceSchema,
  }).pipe(
    Schema.filter((row) => row.configured || 'Expected billing alerts to be configured for the named risk surface'),
    Schema.filter(
      (row) =>
        row.redactedPayloadOnly ||
        'Expected billing alert payloads to remain redacted and child-safe by default'
    )
  )
);

export const BillingSecurityPrivacyObservabilityProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSecurityPrivacyObservabilitySchemaVersionSchema,
    metadataAllowlistRows: Schema.Array(BillingMetadataAllowlistRowSchema),
    metadataDenylistRows: Schema.Array(BillingMetadataDenylistRowSchema),
    logRedactionRows: Schema.Array(BillingLogRedactionRowSchema),
    abuseProtectionRows: Schema.Array(BillingAbuseProtectionRowSchema),
    webhookSecurityRows: Schema.Array(BillingWebhookSecurityRowSchema),
    alertRows: Schema.Array(BillingAlertRowSchema),
    nonClaims: Schema.Array(BillingSecurityPrivacyNonClaimSchema),
    metadataAllowlistClaim: BillingMetadataAllowlistClaimSchema,
    metadataDenylistClaim: BillingMetadataDenylistClaimSchema,
    logRedactionClaim: BillingLogRedactionClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingSecurityPrivacyObservabilityProofIsHonest(proof) ||
        'Expected billing security proof to keep metadata denylists, redacted logging, abuse controls, and alert coverage explicit'
    )
  )
);

export type BillingMetadataAllowlistRow = Infer<typeof BillingMetadataAllowlistRowSchema>;
export type BillingMetadataDenylistRow = Infer<typeof BillingMetadataDenylistRowSchema>;
export type BillingLogRedactionRow = Infer<typeof BillingLogRedactionRowSchema>;
export type BillingAbuseProtectionRow = Infer<typeof BillingAbuseProtectionRowSchema>;
export type BillingWebhookSecurityRow = Infer<typeof BillingWebhookSecurityRowSchema>;
export type BillingAlertRow = Infer<typeof BillingAlertRowSchema>;
export type BillingSecurityPrivacyObservabilityProof = Infer<
  typeof BillingSecurityPrivacyObservabilityProofSchema
>;

export const decodeBillingSecurityPrivacyObservabilityProof = Schema.decodeUnknownSync(
  BillingSecurityPrivacyObservabilityProofSchema
);

export const BillingSecurityPrivacyObservabilityProofReadModel =
  BillingSecurityPrivacyObservabilityProofSchema.parse({
    schemaVersion: 'billing-security-privacy-observability-proof',
    metadataAllowlistRows: [
      {
        schemaVersion: 'billing-security-privacy-observability-proof',
        boundaryId: 'billing-security-boundary-1',
        metadataSurface: 'checkout-session',
        allowedField: 'parent-account-ref',
        typedReferenceOnly: true,
        auditReference: 'billing-security-audit-allowlist-checkout-account',
      },
      {
        schemaVersion: 'billing-security-privacy-observability-proof',
        boundaryId: 'billing-security-boundary-1',
        metadataSurface: 'checkout-session',
        allowedField: 'family-ref',
        typedReferenceOnly: true,
        auditReference: 'billing-security-audit-allowlist-checkout-family',
      },
      {
        schemaVersion: 'billing-security-privacy-observability-proof',
        boundaryId: 'billing-security-boundary-1',
        metadataSurface: 'checkout-session',
        allowedField: 'plan-ref',
        typedReferenceOnly: true,
        auditReference: 'billing-security-audit-allowlist-checkout-plan',
      },
      {
        schemaVersion: 'billing-security-privacy-observability-proof',
        boundaryId: 'billing-security-boundary-1',
        metadataSurface: 'billing-portal-session',
        allowedField: 'subscription-ref',
        typedReferenceOnly: true,
        auditReference: 'billing-security-audit-allowlist-portal-subscription',
      },
      {
        schemaVersion: 'billing-security-privacy-observability-proof',
        boundaryId: 'billing-security-boundary-1',
        metadataSurface: 'provider-webhook-event',
        allowedField: 'billing-failure-ref',
        typedReferenceOnly: true,
        auditReference: 'billing-security-audit-allowlist-webhook-failure',
      },
    ],
    metadataDenylistRows: [
      denylistRow('checkout-session', 'child-name'),
      denylistRow('checkout-session', 'child-activity'),
      denylistRow('checkout-session', 'location-history'),
      denylistRow('billing-portal-session', 'policy-detail'),
      denylistRow('provider-webhook-event', 'ai-safety-analysis'),
      denylistRow('provider-webhook-event', 'screenshot'),
      denylistRow('support-audit-export', 'screen-analysis'),
    ],
    logRedactionRows: [
      logRedactionRow('checkout-request', 'redacted-identifiers-only'),
      logRedactionRow('portal-session', 'redacted-identifiers-only'),
      logRedactionRow('provider-webhook', 'hashed-or-truncated-identifiers'),
      logRedactionRow('payment-drift-reconciliation', 'hashed-or-truncated-identifiers'),
      logRedactionRow('billing-support-audit', 'redacted-identifiers-only'),
    ],
    abuseProtectionRows: [
      abuseProtectionRow('checkout-session-create', 'turnstile-enforced'),
      abuseProtectionRow('billing-portal-session-create', 'trusted-authenticated-session'),
      abuseProtectionRow('provider-webhook-ingest', 'not-applicable'),
    ],
    webhookSecurityRows: [
      {
        schemaVersion: 'billing-security-privacy-observability-proof',
        surface: 'provider-webhook-ingest',
        signatureVerified: true,
        timestampWithinTolerance: true,
        replayCacheChecked: true,
        secretScanState: 'clean',
        auditReference: 'billing-security-audit-webhook-guard',
      },
    ],
    alertRows: [
      alertRow('webhook-failure'),
      alertRow('payment-drift'),
      alertRow('checkout-abuse'),
      alertRow('fraud-signal'),
      alertRow('secret-exposure'),
    ],
    nonClaims: [
      'no-child-data-in-metadata',
      'no-raw-payment-identifiers-in-logs',
      'no-provider-secret-logs',
      'no-child-activity-custody',
      'no-pci-pan-custody',
    ],
    metadataAllowlistClaim: 'typed-reference-only',
    metadataDenylistClaim: 'child-safety-data-excluded',
    logRedactionClaim: 'redacted-billing-identifiers-only',
    updatedAt: '2026-06-13T12:00:00.000Z',
  });

function billingSecurityPrivacyObservabilityProofIsHonest(proof: {
  readonly metadataAllowlistRows: ReadonlyArray<{
    readonly metadataSurface: BillingMetadataSurface;
    readonly allowedField: string;
    readonly typedReferenceOnly: boolean;
  }>;
  readonly metadataDenylistRows: ReadonlyArray<{
    readonly forbiddenClass: string;
    readonly blocked: boolean;
  }>;
  readonly logRedactionRows: ReadonlyArray<{
    readonly paymentIdentifiersRedacted: boolean;
    readonly childSafetyDataAbsent: boolean;
  }>;
  readonly abuseProtectionRows: ReadonlyArray<{
    readonly operation: string;
    readonly rateLimitEnabled: boolean;
    readonly payloadSchemaValidated: boolean;
    readonly botProtectionMode: string;
  }>;
  readonly webhookSecurityRows: ReadonlyArray<{
    readonly signatureVerified: boolean;
    readonly timestampWithinTolerance: boolean;
    readonly replayCacheChecked: boolean;
    readonly secretScanState: string;
  }>;
  readonly alertRows: ReadonlyArray<{ readonly alertKind: BillingAlertKind; readonly configured: boolean }>;
  readonly nonClaims: ReadonlyArray<BillingSecurityPrivacyNonClaim>;
}): boolean {
  const requiredNonClaims: ReadonlyArray<BillingSecurityPrivacyNonClaim> = [
    'no-child-data-in-metadata',
    'no-raw-payment-identifiers-in-logs',
    'no-provider-secret-logs',
    'no-child-activity-custody',
    'no-pci-pan-custody',
  ];
  const requiredAllowlistFields = ['parent-account-ref', 'family-ref', 'plan-ref', 'subscription-ref'];
  const requiredDenylistClasses = [
    'child-name',
    'child-activity',
    'location-history',
    'screenshot',
    'policy-detail',
    'ai-safety-analysis',
  ];
  const requiredOperations = [
    'checkout-session-create',
    'billing-portal-session-create',
    'provider-webhook-ingest',
  ];
  const requiredAlerts: ReadonlyArray<BillingAlertKind> = [
    'webhook-failure',
    'payment-drift',
    'checkout-abuse',
    'fraud-signal',
    'secret-exposure',
  ];
  return (
    requiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    requiredAllowlistFields.every((field) =>
      proof.metadataAllowlistRows.some((row) => row.allowedField === field && row.typedReferenceOnly)
    ) &&
    requiredDenylistClasses.every((forbiddenClass) =>
      proof.metadataDenylistRows.some((row) => row.forbiddenClass === forbiddenClass && row.blocked)
    ) &&
    proof.logRedactionRows.every((row) => row.paymentIdentifiersRedacted && row.childSafetyDataAbsent) &&
    requiredOperations.every((operation) =>
      proof.abuseProtectionRows.some(
        (row) =>
          row.operation === operation &&
          row.rateLimitEnabled &&
          row.payloadSchemaValidated &&
          (operation === 'provider-webhook-ingest'
            ? row.botProtectionMode === 'not-applicable'
            : row.botProtectionMode !== 'not-applicable')
      )
    ) &&
    proof.webhookSecurityRows.some(
      (row) =>
        row.signatureVerified &&
        row.timestampWithinTolerance &&
        row.replayCacheChecked &&
        row.secretScanState === 'clean'
    ) &&
    requiredAlerts.every((alertKind) =>
      proof.alertRows.some((row) => row.alertKind === alertKind && row.configured)
    )
  );
}

function denylistRow(
  metadataSurface: BillingMetadataSurface,
  forbiddenClass: Infer<typeof BillingForbiddenMetadataDataClassSchema>
) {
  return {
    schemaVersion: 'billing-security-privacy-observability-proof' as const,
    metadataSurface,
    forbiddenClass,
    blocked: true,
    auditReference: `billing-security-audit-denylist-${metadataSurface}-${forbiddenClass}` as const,
  };
}

function logRedactionRow(
  logSignal: Infer<typeof BillingLogSignalSchema>,
  redactionState: Infer<typeof BillingLogRedactionStateSchema>
) {
  return {
    schemaVersion: 'billing-security-privacy-observability-proof' as const,
    logSignal,
    redactionState,
    paymentIdentifiersRedacted: true,
    childSafetyDataAbsent: true,
    auditReference: `billing-security-audit-log-${logSignal}` as const,
  };
}

function abuseProtectionRow(
  operation: Infer<typeof BillingAbuseProtectedOperationSchema>,
  botProtectionMode: Infer<typeof BillingBotProtectionModeSchema>
) {
  return {
    schemaVersion: 'billing-security-privacy-observability-proof' as const,
    operation,
    rateLimitEnabled: true,
    botProtectionMode,
    payloadSchemaValidated: true,
    auditReference: `billing-security-audit-abuse-${operation}` as const,
  };
}

function alertRow(alertKind: BillingAlertKind) {
  return {
    schemaVersion: 'billing-security-privacy-observability-proof' as const,
    alertKind,
    configured: true,
    redactedPayloadOnly: true,
    auditReference: `billing-security-audit-alert-${alertKind}` as const,
  };
}
