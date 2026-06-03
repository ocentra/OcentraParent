import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  BrowserGameHoldBlockActionSchema,
  BrowserGameHoldBlockAdapterPlanIdSchema,
  BrowserGameHoldBlockAdapterSchemaVersionSchema,
  BrowserGameHoldBlockAdapterStateSchema,
  BrowserGameHoldBlockDeliveryModeSchema,
  BrowserGameHoldBlockFallbackActionSchema,
  BrowserGameHoldBlockReasonSchema,
  BrowserGameHoldBlockTargetKindSchema,
} from './browser-game-hold-block-adapter-values';
import {
  browserGameHoldBlockAdapterPlanIsHonest,
  browserGameHoldBlockAdapterSnapshotIsComplete,
} from './browser-game-hold-block-adapter-guards';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const BrowserGameHoldBlockEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game hold/block evidence refs')
);
const BrowserGameHoldBlockReasonCodesSchema = Schema.Array(BrowserGameHoldBlockReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game hold/block reason codes')
);

const BrowserGameHoldBlockAdapterPlanBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameHoldBlockAdapterSchemaVersionSchema,
  planId: BrowserGameHoldBlockAdapterPlanIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  createdAt: ParentTimestampSchema,
  targetKind: BrowserGameHoldBlockTargetKindSchema,
  requestedAction: BrowserGameHoldBlockActionSchema,
  adapterState: BrowserGameHoldBlockAdapterStateSchema,
  deliveryMode: BrowserGameHoldBlockDeliveryModeSchema,
  fallbackAction: BrowserGameHoldBlockFallbackActionSchema,
  sourceEvidenceRefs: BrowserGameHoldBlockEvidenceRefsSchema,
  policyCandidateRef: OptionalParentEvidenceRefSchema,
  childUxSurfaceRef: OptionalParentEvidenceRefSchema,
  managedInterventionAdapterProofRef: OptionalParentEvidenceRefSchema,
  adapterAuditRef: OptionalParentEvidenceRefSchema,
  reasonCodes: BrowserGameHoldBlockReasonCodesSchema,
  rawUrlIncluded: Schema.Boolean,
  rawPageBodyIncluded: Schema.Boolean,
  rawGamePayloadIncluded: Schema.Boolean,
  childCookieSessionReused: Schema.Boolean,
  unmanagedBrowserExactUrlClaimed: Schema.Boolean,
  browserMutationExecutedClaimed: Schema.Boolean,
  renderedChildPageClaimed: Schema.Boolean,
  notificationDeliveredClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  timeLimitAppliedClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

export const BrowserGameHoldBlockAdapterPlanSchema = withParser(
  BrowserGameHoldBlockAdapterPlanBaseSchema.pipe(
    Schema.filter(
      (plan) =>
        browserGameHoldBlockAdapterPlanIsHonest(plan) ||
        'Expected browser-game hold/block adapter plan to stay proof-linked and non-executing'
    )
  )
);

export const BrowserGameHoldBlockAdapterClaimBoundariesSchema = withParser(
  Schema.Struct({
    rawUrlStorage: Schema.Literal('not-claimed'),
    rawPageBodyStorage: Schema.Literal('not-claimed'),
    rawGamePayloadStorage: Schema.Literal('not-claimed'),
    childCookieSessionReuse: Schema.Literal('not-claimed'),
    unmanagedExactUrl: Schema.Literal('not-claimed'),
    browserMutationExecution: Schema.Literal('not-claimed'),
    renderedChildPage: Schema.Literal('not-claimed'),
    notificationDelivery: Schema.Literal('not-claimed'),
    finalPolicyDecision: Schema.Literal('not-claimed'),
    timeLimitApplication: Schema.Literal('not-claimed'),
    cloudFrameAnalysis: Schema.Literal('not-claimed'),
    nativeGameControl: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
  })
);

const BrowserGameHoldBlockAdapterSnapshotBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameHoldBlockAdapterSchemaVersionSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  generatedAt: ParentTimestampSchema,
  plans: Schema.Array(BrowserGameHoldBlockAdapterPlanSchema),
  claimBoundaries: BrowserGameHoldBlockAdapterClaimBoundariesSchema,
});

export const BrowserGameHoldBlockAdapterSnapshotSchema = withParser(
  BrowserGameHoldBlockAdapterSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        browserGameHoldBlockAdapterSnapshotIsComplete(snapshot) ||
        'Expected browser-game hold/block adapter snapshot to cover managed hold, block, warn, manual, and unavailable paths'
    )
  )
);

export const decodeBrowserGameHoldBlockAdapterPlan = Schema.decodeUnknownSync(BrowserGameHoldBlockAdapterPlanSchema);
export const decodeBrowserGameHoldBlockAdapterSnapshot = Schema.decodeUnknownSync(
  BrowserGameHoldBlockAdapterSnapshotSchema
);

export type BrowserGameHoldBlockAdapterPlan = Infer<typeof BrowserGameHoldBlockAdapterPlanSchema>;
export type BrowserGameHoldBlockAdapterSnapshot = Infer<typeof BrowserGameHoldBlockAdapterSnapshotSchema>;
