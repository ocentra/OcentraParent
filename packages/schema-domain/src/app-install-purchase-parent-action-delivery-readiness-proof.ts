import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel } from './app-install-purchase-child-device-delivery-runtime-writer-proof';
import { AppInstallPurchaseParentActionRuntimeHandoffProofReadModel } from './app-install-purchase-parent-action-runtime-handoff-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseParentActionDeliveryReadinessRowGenerated,
  parentActionDeliveryReadinessProofIsHonestGenerated,
  parentActionDeliveryReadinessRowIsHonestGenerated,
  summarizeAppInstallPurchaseParentActionDeliveryReadinessProofGenerated,
} from './generated/app-install-purchase-delivery-runtime-helpers';
const ParentActionDeliveryReadinessProofVersion = 'app-install-purchase-parent-action-delivery-readiness-proof';
const SourceParentActionRuntimeHandoffProofVersion = 'app-install-purchase-parent-action-runtime-handoff-proof';
const SourceChildDeviceDeliveryRuntimeWriterProofVersion =
  'app-install-purchase-child-device-delivery-runtime-writer-proof';
const ParentActionDeliveryReadinessTimestamp = '2026-06-05T16:35:00.000Z';
const ParentActionDeliveryReadinessClaimBoundary =
  'parent action delivery readiness proof only; no parent action runtime delivery no runtime writer execution no runtime writer delivery no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredDecisionActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RequiredRuntimeHandoffStatuses = ['queued-for-runtime-writer', 'manual-review-required'] as const;
const RequiredChildDeliveryEnvelopeStates = ['child-delivery-envelope-ready', 'manual-review-required'] as const;
const ParentActionDeliveryReadinessStates = ['parent-action-delivery-ready', 'manual-review-required'] as const;
const ParentActionDeliveryReadinessNonClaims = [
  'no-parent-action-runtime-delivery',
  'no-runtime-writer-execution',
  'no-runtime-writer-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ParentActionDeliveryReadinessBoundaryFragments = [
  'no parent action runtime delivery',
  'no runtime writer execution',
  'no runtime writer delivery',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no child activity data',
  'no app blocking',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseParentActionDeliveryReadinessProofSchemaVersionSchema = withParser(
  Schema.Literal(ParentActionDeliveryReadinessProofVersion)
);
const AppInstallPurchaseParentActionDeliveryReadinessActionSchema = withParser(
  Schema.Literal(...RequiredDecisionActions)
);
const AppInstallPurchaseParentActionDeliveryReadinessRuntimeStatusSchema = withParser(
  Schema.Literal(...RequiredRuntimeHandoffStatuses)
);
const AppInstallPurchaseParentActionDeliveryReadinessChildEnvelopeSchema = withParser(
  Schema.Literal(...RequiredChildDeliveryEnvelopeStates)
);
const AppInstallPurchaseParentActionDeliveryReadinessStateSchema = withParser(
  Schema.Literal(...ParentActionDeliveryReadinessStates)
);
const AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseParentActionDeliveryReadinessExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentActionDeliveryReadinessAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseParentActionDeliveryReadinessCustodyClaimSchema = withParser(
  Schema.Literal('no-child-activity-data')
);
const AppInstallPurchaseParentActionDeliveryReadinessNonClaimSchema = withParser(
  Schema.Literal(...ParentActionDeliveryReadinessNonClaims)
);

const ParentActionDeliveryReadinessRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseParentActionDeliveryReadinessRowId'
);
const ParentActionDeliveryReadinessRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseParentActionDeliveryReadinessRef'
);
const ParentActionDeliveryReadinessAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseParentActionDeliveryReadinessAuditRef'
);
const ParentActionDeliveryReadinessClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseParentActionDeliveryReadinessClaimBoundary'
);

const ParentActionDeliveryReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseParentActionDeliveryReadinessProofSchemaVersionSchema,
  parentActionDeliveryReadinessRowId: ParentActionDeliveryReadinessRowIdSchema,
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  sourceParentActionRuntimeHandoffRowId: ParentActionDeliveryReadinessRefSchema,
  sourceDecisionAction: AppInstallPurchaseParentActionDeliveryReadinessActionSchema,
  sourceRuntimeHandoffStatus: AppInstallPurchaseParentActionDeliveryReadinessRuntimeStatusSchema,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: Schema.Literal(
    SourceChildDeviceDeliveryRuntimeWriterProofVersion
  ),
  sourceChildDeviceDeliveryRuntimeWriterRowId: ParentActionDeliveryReadinessRefSchema,
  sourceChildDeliveryEnvelopeState: AppInstallPurchaseParentActionDeliveryReadinessChildEnvelopeSchema,
  parentActionDeliveryReadinessState: AppInstallPurchaseParentActionDeliveryReadinessStateSchema,
  parentActionAuditEventRefs: Schema.Array(ParentActionDeliveryReadinessAuditRefSchema),
  childDeliveryTargetRefs: Schema.Array(ParentActionDeliveryReadinessRefSchema),
  reportRuntimeRefs: Schema.Array(ParentActionDeliveryReadinessRefSchema),
  parentActionRuntimeDeliveryClaim: AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema,
  runtimeWriterExecutionClaim: AppInstallPurchaseParentActionDeliveryReadinessExecutionClaimSchema,
  runtimeWriterDeliveryClaim: AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseParentActionDeliveryReadinessExecutionClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseParentActionDeliveryReadinessAdapterClaimSchema,
  childDeviceDeliveryClaim: AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema,
  appBlockingClaim: AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema,
  childDataCustody: AppInstallPurchaseParentActionDeliveryReadinessCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema,
  claimBoundary: ParentActionDeliveryReadinessClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ParentActionDeliveryReadinessRowCandidate = Infer<typeof ParentActionDeliveryReadinessRowBaseSchema>;

export const AppInstallPurchaseParentActionDeliveryReadinessRowSchema = withParser(
  ParentActionDeliveryReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        parentActionDeliveryReadinessRowIsHonest(row) ||
        'Expected parent action delivery readiness rows to link parent handoff and child envelope refs without delivery, writer, provider, adapter, custody, interception, or blocking claims'
    )
  )
);

const ParentActionDeliveryReadinessProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseParentActionDeliveryReadinessProofSchemaVersionSchema,
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: Schema.Literal(
    SourceChildDeviceDeliveryRuntimeWriterProofVersion
  ),
  parentActionDeliveryReadinessRows: Schema.Array(AppInstallPurchaseParentActionDeliveryReadinessRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseParentActionDeliveryReadinessNonClaimSchema),
  knownGaps: Schema.Array(ParentActionDeliveryReadinessRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseParentActionDeliveryReadinessProof = Infer<
  typeof ParentActionDeliveryReadinessProofBaseSchema
>;

export const AppInstallPurchaseParentActionDeliveryReadinessProofSchema = withParser(
  ParentActionDeliveryReadinessProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        parentActionDeliveryReadinessProofIsHonest(proof) ||
        'Expected app install/purchase parent action delivery readiness proof to cover review actions and preserve delivery non-claims'
    )
  )
);

export const AppInstallPurchaseParentActionDeliveryReadinessKnownGaps = [
  'Parent action delivery readiness rows are contract/proof rows only; no parent action runtime delivery worker is implemented.',
  'Runtime writer execution/delivery, provider/store execution, platform adapters, child-device delivery, runtime report delivery, interception, app blocking, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI and a real parent action delivery runtime path exist.',
] as const;

export const AppInstallPurchaseParentActionDeliveryReadinessProofReadModel =
  AppInstallPurchaseParentActionDeliveryReadinessProofSchema.parse({
    schemaVersion: ParentActionDeliveryReadinessProofVersion,
    sourceParentActionRuntimeHandoffProofVersion: SourceParentActionRuntimeHandoffProofVersion,
    sourceChildDeviceDeliveryRuntimeWriterProofVersion: SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    parentActionDeliveryReadinessRows:
      AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows.map(
        parentActionDeliveryReadinessRow
      ),
    nonClaims: ParentActionDeliveryReadinessNonClaims,
    knownGaps: AppInstallPurchaseParentActionDeliveryReadinessKnownGaps,
    updatedAt: ParentActionDeliveryReadinessTimestamp,
  });

export function summarizeAppInstallPurchaseParentActionDeliveryReadinessProof(
  proof: AppInstallPurchaseParentActionDeliveryReadinessProof
) {
  return summarizeAppInstallPurchaseParentActionDeliveryReadinessProofGenerated(proof);
}

function parentActionDeliveryReadinessRow(
  row: (typeof AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows)[number]
) {
  return buildAppInstallPurchaseParentActionDeliveryReadinessRowGenerated(
    row,
    childDeliveryRowForAction(row.sourceDecisionAction),
    SourceParentActionRuntimeHandoffProofVersion,
    SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    ParentActionDeliveryReadinessClaimBoundary,
    ParentActionDeliveryReadinessTimestamp
  );
}

function childDeliveryRowForAction(action: (typeof RequiredDecisionActions)[number]) {
  return AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel.childDeviceDeliveryRuntimeWriterRows.find(
    (row) => row.sourceDecisionAction === action
  )!;
}

function parentActionDeliveryReadinessRowIsHonest(row: ParentActionDeliveryReadinessRowCandidate): boolean {
  return parentActionDeliveryReadinessRowIsHonestGenerated(
    row,
    SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    ParentActionDeliveryReadinessBoundaryFragments
  );
}

function parentActionDeliveryReadinessProofIsHonest(
  proof: AppInstallPurchaseParentActionDeliveryReadinessProof
): boolean {
  return (
    parentActionDeliveryReadinessProofIsHonestGenerated(
      proof,
      SourceParentActionRuntimeHandoffProofVersion,
      SourceChildDeviceDeliveryRuntimeWriterProofVersion,
      RequiredDecisionActions,
      RequiredRuntimeHandoffStatuses,
      ParentActionDeliveryReadinessStates,
      RequiredChildDeliveryEnvelopeStates,
      ParentActionDeliveryReadinessNonClaims
    ) &&
    proof.parentActionDeliveryReadinessRows.every((row) => parentActionDeliveryReadinessRowIsHonest(row)) &&
    proof.knownGaps.length > 0
  );
}
