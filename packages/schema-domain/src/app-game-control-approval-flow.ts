import {
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { ParentEvidenceReferenceSchema } from './family-references';
import { ParentTimestampSchema } from './family-reference-primitives';

export const AppGameControlApprovalCandidateKindSchema = withParser(
  Schema.Literal(
    'new-inventory-app',
    'unknown-runtime-process',
    'portable-executable',
    'installer-or-updater',
    'launcher-game-candidate',
    'unknown-game-like-executable'
  )
);

export const AppGameControlApprovalCandidateSourceSchema = withParser(
  Schema.Literal('inventory', 'runtime', 'foreground', 'launcher', 'installer', 'portable-executable')
);

export const AppGameControlChildReasonStateSchema = withParser(
  Schema.Literal('not-requested', 'reason-ref-backed', 'unavailable', 'manual-required')
);

export const AppGameControlParentResponseScopeSchema = withParser(
  Schema.Literal(
    'allow-once',
    'allow-this-app-game',
    'allow-category',
    'ask-child-why',
    'deny',
    'report-only',
    'block-if-supported'
  )
);

export const AppGameControlApprovalPersistenceStateSchema = withParser(
  Schema.Literal('not-persisted', 'replayable', 'replayed', 'storage-unavailable')
);

export const AppGameControlUnansweredFallbackSchema = withParser(
  Schema.Literal('deny', 'expire', 'observe-only', 'manual-required')
);

export const AppGameControlApprovalFlowReferenceSchema = brandedNonEmptyStringSchema('AppGameControlApprovalFlowReference');

export const AppGameControlSettingReferenceSchema = withParser(
  Schema.Struct({
    settingId: brandedNonEmptyStringSchema('AppGameControlSettingId'),
    writesTo: brandedNonEmptyStringSchema('AppGameControlWritePath'),
  })
);

export const AppGameControlApprovalCandidateSchema = withParser(
  Schema.Struct({
    candidateId: brandedNonEmptyStringSchema('AppGameControlApprovalCandidateId'),
    candidateKind: AppGameControlApprovalCandidateKindSchema,
    candidateSource: AppGameControlApprovalCandidateSourceSchema,
    detectedAt: ParentTimestampSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  }).pipe(
    Schema.filter(
      (candidate) => candidate.evidenceReferences.length > 0 || 'Expected app/game approval candidates to cite evidence'
    )
  )
);

