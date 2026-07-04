import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserInterventionDeliveryStateSchema } from './browser-intervention-schemas';
import { BrowserPolicyAdapterProofRefSchema } from './browser-ai-policy-evaluator-values';
import { BrowserAiPostAnalysisActionPlanSchema } from './browser-ai-post-analysis-action-schemas';
import { browserAiChildUxSnapshotIsConsistent } from './browser-ai-child-ux-rules';
import { nonEmptyArraySchema, optionalSchema } from './browser-ai-schema-shared';
import {
  BrowserAiChildUxSnapshotIdSchema,
  BrowserAiChildUxStateSchema,
  BrowserAiChildUxSurfaceSchema,
  BrowserAiChildUxTextToken,
  BrowserAiChildUxTextTokenSchema,
  BrowserAiChildUxToneSchema,
} from '@ocentra-parent/schema-domain/browser-ai-child-ux-values';

export {
  BrowserAiChildUxSnapshotIdSchema,
  BrowserAiChildUxStateSchema,
  BrowserAiChildUxSurfaceSchema,
  BrowserAiChildUxTextToken,
  BrowserAiChildUxTextTokenSchema,
  BrowserAiChildUxToneSchema,
};

const EvidenceIdsSchema = nonEmptyArraySchema(ActivityEvidenceIdSchema, 'Expected child UX evidence ids');
const OptionalAdapterProofRefSchema = optionalSchema(BrowserPolicyAdapterProofRefSchema);
const OptionalPostAnalysisActionPlanSchema = optionalSchema(BrowserAiPostAnalysisActionPlanSchema);
const OptionalTextTokenSchema = optionalSchema(BrowserAiChildUxTextTokenSchema);

export const BrowserAiChildUxSchemaVersion = 1;

const BrowserAiChildUxSnapshotBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiChildUxSchemaVersion),
  snapshotId: BrowserAiChildUxSnapshotIdSchema,
  createdAt: ActivityTimestampSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  state: BrowserAiChildUxStateSchema,
  tone: BrowserAiChildUxToneSchema,
  surface: BrowserAiChildUxSurfaceSchema,
  primaryTextToken: BrowserAiChildUxTextTokenSchema,
  secondaryTextToken: OptionalTextTokenSchema,
  deliveryState: BrowserInterventionDeliveryStateSchema,
  adapterProofRef: OptionalAdapterProofRefSchema,
  postAnalysisActionPlan: OptionalPostAnalysisActionPlanSchema,
  rawCopyClaimed: Schema.Boolean,
  visualRenderClaimed: Schema.Boolean,
  surveillanceCopyClaimed: Schema.Boolean,
  shamingCopyClaimed: Schema.Boolean,
});

export const BrowserAiChildUxSnapshotSchema = withParser(
  BrowserAiChildUxSnapshotBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiChildUxSnapshotIsConsistent(value) ||
        'Expected child UX snapshot to use calm tokenized copy and proof-backed delivery'
    )
  )
);

export const decodeBrowserAiChildUxSnapshot = Schema.decodeUnknownSync(BrowserAiChildUxSnapshotSchema);

export type BrowserAiChildUxSnapshot = Infer<typeof BrowserAiChildUxSnapshotSchema>;
