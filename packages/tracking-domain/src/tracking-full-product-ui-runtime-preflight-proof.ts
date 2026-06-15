import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import { type TrackingFullProductUiRuntimeArtifactGateProof } from './tracking-full-product-ui-runtime-artifact-gate-proof';

const TrackingFullProductUiRuntimePreflightTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingFullProductUiRuntimePreflightAreaSchema = Schema.Literal(
  'retention-settings-production-write-result',
  'rendered-child-device-check-in',
  'rendered-child-device-location-consent',
  'child-device-safe-help-response'
);

export const TrackingFullProductUiRuntimePreflightStatusSchema = Schema.Literal('manual-required');

export const TrackingFullProductUiRuntimePreflightRowIdSchema = TrackingFullProductUiRuntimePreflightTextSchema.pipe(
  Schema.brand('TrackingFullProductUiRuntimePreflightRowId')
);

export const TrackingFullProductUiRuntimePreflightPathSchema = TrackingFullProductUiRuntimePreflightTextSchema.pipe(
  Schema.brand('TrackingFullProductUiRuntimePreflightPath')
);

export const TrackingFullProductUiRuntimePreflightCommandSchema = TrackingFullProductUiRuntimePreflightTextSchema.pipe(
  Schema.brand('TrackingFullProductUiRuntimePreflightCommand')
);

export const TrackingFullProductUiRuntimePreflightCriterionSchema =
  TrackingFullProductUiRuntimePreflightTextSchema.pipe(Schema.brand('TrackingFullProductUiRuntimePreflightCriterion'));

const RequiredTrackingFullProductUiRuntimePreflightRows = [
  {
    rowId: 'tracking-full-product-ui-runtime-preflight-retention-production-write',
    area: 'retention-settings-production-write-result',
    sourceMissingArtifactRef:
      'output/tracking-plan-proof/product-parent-child-ui-runtime/04-retention-settings-production-write-result.png',
    acceptanceCriteria: [
      'Parent retention settings write is executed through the production runtime path.',
      'The parent product UI renders the completed write result with source evidence and service status.',
      'The artifact includes the command result, persisted setting revision, screenshot, and no-product-ready claim boundary.',
    ],
    manualValidationCommands: [
      'cmd /c npm run dev:agent',
      'cmd /c npm run dev:portal',
      'manual: capture production retention settings product UI write result artifact under product-parent-child-ui-runtime',
    ],
    artifactAcceptanceNotes: [
      'Hosted local write-result screenshot is not enough for production retention UI runtime.',
      'Artifact must prove product runtime write/result rendering, not only local derivation.',
    ],
    auditRefs: ['tracking-full-product-ui-runtime-preflight-retention-write-audit'],
  },
  {
    rowId: 'tracking-full-product-ui-runtime-preflight-child-check-in',
    area: 'rendered-child-device-check-in',
    sourceMissingArtifactRef:
      'output/tracking-plan-proof/product-parent-child-ui-runtime/05-child-device-rendered-check-in-runtime.png',
    acceptanceCriteria: [
      'Child runtime receives and renders the check-in request on the actual child surface.',
      'The child check-in response is captured with parent receipt and runtime observation refs.',
      'The artifact includes child UI screenshot, delivery envelope, response payload, and device/runtime log reference.',
    ],
    manualValidationCommands: [
      'manual: run child-device check-in request against rendered child runtime UI',
      'manual: capture child UI screenshot and parent receipt under product-parent-child-ui-runtime',
    ],
    artifactAcceptanceNotes: [
      'Hosted child-check-in readiness card is not rendered child-device runtime proof.',
      'Artifact must prove delivery and execution on the child runtime surface.',
    ],
    auditRefs: ['tracking-full-product-ui-runtime-preflight-child-check-in-audit'],
  },
  {
    rowId: 'tracking-full-product-ui-runtime-preflight-child-consent',
    area: 'rendered-child-device-location-consent',
    sourceMissingArtifactRef:
      'output/tracking-plan-proof/product-parent-child-ui-runtime/06-child-device-rendered-location-consent-runtime.png',
    acceptanceCriteria: [
      'Child runtime renders the location consent state through the actual child surface.',
      'Consent decision evidence is tied to parent-visible tracking status and source refs.',
      'The artifact includes child UI screenshot, consent payload, parent receipt, and runtime log reference.',
    ],
    manualValidationCommands: [
      'manual: run child location consent flow on rendered child runtime UI',
      'manual: capture consent screenshot, payload, parent receipt, and logs under product-parent-child-ui-runtime',
    ],
    artifactAcceptanceNotes: [
      'Hosted disclosure/readiness cards do not satisfy rendered child consent runtime proof.',
      'Artifact must prove the child runtime consent flow and parent receipt together.',
    ],
    auditRefs: ['tracking-full-product-ui-runtime-preflight-child-consent-audit'],
  },
  {
    rowId: 'tracking-full-product-ui-runtime-preflight-safe-help',
    area: 'child-device-safe-help-response',
    sourceMissingArtifactRef:
      'output/tracking-plan-proof/product-parent-child-ui-runtime/07-child-device-safe-help-response-runtime.png',
    acceptanceCriteria: [
      'Child runtime renders safe/help response UI and sends a response through the real runtime path.',
      'Parent product UI receives and displays the safe/help response with source evidence refs.',
      'The artifact includes child UI screenshot, response payload, parent receipt, and runtime log reference.',
    ],
    manualValidationCommands: [
      'manual: run child safe/help response flow on rendered child runtime UI',
      'manual: capture child response screenshot, parent receipt, and logs under product-parent-child-ui-runtime',
    ],
    artifactAcceptanceNotes: [
      'Hosted parent route proof cannot satisfy rendered child safe/help runtime evidence.',
      'Artifact must prove runtime child response and parent receipt, not only copy coverage.',
    ],
    auditRefs: ['tracking-full-product-ui-runtime-preflight-safe-help-audit'],
  },
] as const;

export const TrackingFullProductUiRuntimePreflightRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingFullProductUiRuntimePreflightRowIdSchema,
    generatedAt: ParentTimestampSchema,
    area: TrackingFullProductUiRuntimePreflightAreaSchema,
    requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingFullProductUiRuntimePreflightStatusSchema,
    sourceRuntimeArtifactGateProofRef: TrackingFullProductUiRuntimePreflightPathSchema,
    sourceMissingArtifactRef: TrackingFullProductUiRuntimePreflightPathSchema,
    acceptanceCriteria: Schema.Array(TrackingFullProductUiRuntimePreflightCriterionSchema),
    manualValidationCommands: Schema.Array(TrackingFullProductUiRuntimePreflightCommandSchema),
    requiredArtifacts: Schema.Array(TrackingFullProductUiRuntimePreflightPathSchema),
    presentArtifacts: Schema.Array(TrackingFullProductUiRuntimePreflightPathSchema),
    missingArtifacts: Schema.Array(TrackingFullProductUiRuntimePreflightPathSchema),
    artifactAcceptanceNotes: Schema.Array(TrackingFullProductUiRuntimePreflightCriterionSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    fullProductUiRuntimeClaimed: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    retentionProductionWriteClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    productionProductUiClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.acceptanceCriteria.length >= 3 || 'Product UI preflight rows need criteria'))
    .pipe(Schema.filter((row) => row.manualValidationCommands.length >= 2 || 'Product UI preflight rows need commands'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Product UI preflight rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.missingArtifacts.length > 0 || 'Product UI preflight rows stay manual-required until artifacts exist'
      )
    )
);

export const TrackingFullProductUiRuntimePreflightProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-full-product-ui-runtime-preflight-proof'),
    generatedAt: ParentTimestampSchema,
    sourceRuntimeArtifactGateProofRef: TrackingFullProductUiRuntimePreflightPathSchema,
    rows: Schema.Array(TrackingFullProductUiRuntimePreflightRowSchema),
    summary: Schema.Struct({
      rowCount: Schema.Number,
      manualRequiredRowCount: Schema.Number,
      requiredArtifactCount: Schema.Number,
      presentArtifactCount: Schema.Literal(0),
      missingArtifactCount: Schema.Number,
      productReadyRowCount: Schema.Literal(0),
    }),
    proofClaims: Schema.Struct({
      fullProductUiRuntimePreflightGenerated: Schema.Literal(true),
      runtimeArtifactGateObserved: Schema.Literal(true),
      hardRuntimeUiArtifactsStillMissing: Schema.Literal(true),
      noFullProductUiRuntimeClaim: Schema.Literal(true),
      noChildDeviceRuntimeClaim: Schema.Literal(true),
      noRetentionProductionWriteClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryRuntimeClaim: Schema.Literal(true),
      noProductionProductUiClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      fullProductUiRuntimeClaimed: Schema.Literal(false),
      childDeviceRuntimeClaimed: Schema.Literal(false),
      retentionProductionWriteClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryRuntimeClaimed: Schema.Literal(false),
      productionProductUiClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          proof.rows.length === RequiredTrackingFullProductUiRuntimePreflightRows.length ||
          'Product UI runtime preflight must cover every hard UI row'
      )
    )
    .pipe(Schema.filter((proof) => proof.summary.rowCount === proof.rows.length || 'Product UI preflight row mismatch'))
);

export type TrackingFullProductUiRuntimePreflightProof = Infer<typeof TrackingFullProductUiRuntimePreflightProofSchema>;
export type TrackingFullProductUiRuntimePreflightRow = Infer<typeof TrackingFullProductUiRuntimePreflightRowSchema>;

export const RequiredTrackingFullProductUiRuntimePreflightPlan = {
  sourceRuntimeArtifactGateProofRef: 'test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json',
  rows: RequiredTrackingFullProductUiRuntimePreflightRows,
} as const;

export function buildTrackingFullProductUiRuntimePreflightProof(
  generatedAt: string,
  runtimeArtifactGateProof: TrackingFullProductUiRuntimeArtifactGateProof
): TrackingFullProductUiRuntimePreflightProof {
  assertRuntimeGateStillRequiresHardUiArtifacts(runtimeArtifactGateProof);
  const rows = RequiredTrackingFullProductUiRuntimePreflightRows.map((row) => preflightRow(generatedAt, row));

  return TrackingFullProductUiRuntimePreflightProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-full-product-ui-runtime-preflight-proof',
    generatedAt,
    sourceRuntimeArtifactGateProofRef:
      RequiredTrackingFullProductUiRuntimePreflightPlan.sourceRuntimeArtifactGateProofRef,
    rows,
    summary: {
      rowCount: rows.length,
      manualRequiredRowCount: rows.filter((row) => row.status === 'manual-required').length,
      requiredArtifactCount: rows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
      presentArtifactCount: 0,
      missingArtifactCount: rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      productReadyRowCount: 0,
    },
    proofClaims: {
      fullProductUiRuntimePreflightGenerated: true,
      runtimeArtifactGateObserved: true,
      hardRuntimeUiArtifactsStillMissing: true,
      noFullProductUiRuntimeClaim: true,
      noChildDeviceRuntimeClaim: true,
      noRetentionProductionWriteClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryRuntimeClaim: true,
      noProductionProductUiClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      fullProductUiRuntimeClaimed: false,
      childDeviceRuntimeClaimed: false,
      retentionProductionWriteClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    },
  });
}

function assertRuntimeGateStillRequiresHardUiArtifacts(
  runtimeArtifactGateProof: TrackingFullProductUiRuntimeArtifactGateProof
): void {
  const [runtimeGateRow] = runtimeArtifactGateProof.rows;
  if (!runtimeGateRow) throw new Error('Full product UI runtime artifact gate proof is missing its row.');
  for (const requiredRow of RequiredTrackingFullProductUiRuntimePreflightRows) {
    if (!runtimeGateRow.missingArtifacts.some((artifact) => artifact === requiredRow.sourceMissingArtifactRef)) {
      throw new Error(
        `Full product UI runtime preflight requires missing artifact ${requiredRow.sourceMissingArtifactRef}.`
      );
    }
  }
  if (runtimeArtifactGateProof.productClaims.productClaimReady) {
    throw new Error('Full product UI runtime preflight cannot run against product-ready UI claims.');
  }
}

function preflightRow(
  generatedAt: string,
  row: (typeof RequiredTrackingFullProductUiRuntimePreflightRows)[number]
): TrackingFullProductUiRuntimePreflightRow {
  return TrackingFullProductUiRuntimePreflightRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: row.rowId,
    generatedAt,
    area: row.area,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'manual-required',
    sourceRuntimeArtifactGateProofRef:
      RequiredTrackingFullProductUiRuntimePreflightPlan.sourceRuntimeArtifactGateProofRef,
    sourceMissingArtifactRef: row.sourceMissingArtifactRef,
    acceptanceCriteria: [...row.acceptanceCriteria],
    manualValidationCommands: [...row.manualValidationCommands],
    requiredArtifacts: [row.sourceMissingArtifactRef],
    presentArtifacts: [],
    missingArtifacts: [row.sourceMissingArtifactRef],
    artifactAcceptanceNotes: [...row.artifactAcceptanceNotes],
    auditRefs: [...row.auditRefs],
    fullProductUiRuntimeClaimed: false,
    childDeviceRuntimeClaimed: false,
    retentionProductionWriteClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    productionProductUiClaimed: false,
    productClaimReady: false,
  });
}
