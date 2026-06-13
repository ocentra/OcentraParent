import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentActionReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  AppGameChildUxCapabilityState,
  AppGameChildUxClaimState,
  AppGameChildUxCopyToken,
  AppGameChildUxExplanationSource,
  AppGameChildUxPrimaryAction,
  AppGameChildUxSurfaceState,
  AppGameChildUxTargetKind,
  appGameChildUxClaimMatchesSurface,
  appGameChildUxCopyTokensMatchSurface,
  appGameChildUxRequestRefsAreAuditable,
  appGameChildUxStateIsHonest,
} from './app-game-child-facing-ux-rules';

export const AppGameChildUxStateIdSchema = brandedNonEmptyStringSchema('AppGameChildUxStateId');
export const AppGameChildUxTargetRefSchema = brandedNonEmptyStringSchema('AppGameChildUxTargetRef');
export const AppGameChildUxCopyTokenSchema = withParser(Schema.Literal(...Object.values(AppGameChildUxCopyToken)));
export const AppGameChildUxReasonRefSchema = brandedNonEmptyStringSchema('AppGameChildUxReasonRef');
export const AppGameChildUxStatusRefSchema = brandedNonEmptyStringSchema('AppGameChildUxStatusRef');
export const AppGameChildUxDiagnosticRefSchema = brandedNonEmptyStringSchema('AppGameChildUxDiagnosticRef');

export const AppGameChildUxSurfaceStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxSurfaceState))
);
export const AppGameChildUxTargetKindSchema = withParser(Schema.Literal(...Object.values(AppGameChildUxTargetKind)));
export const AppGameChildUxCapabilityStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxCapabilityState))
);
export const AppGameChildUxClaimStateSchema = withParser(Schema.Literal(...Object.values(AppGameChildUxClaimState)));
export const AppGameChildUxPrimaryActionSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxPrimaryAction))
);
export const AppGameChildUxExplanationSourceSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxExplanationSource))
);

export const AppGameChildUxTargetSchema = withParser(
  Schema.Struct({
    targetKind: AppGameChildUxTargetKindSchema,
    targetRef: AppGameChildUxTargetRefSchema,
    childSafeDisplayLabelToken: AppGameChildUxCopyTokenSchema,
  })
);

export const AppGameChildUxCardSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    childUxStateId: AppGameChildUxStateIdSchema,
    device: ParentDeviceReferenceSchema,
    target: AppGameChildUxTargetSchema,
    surfaceState: AppGameChildUxSurfaceStateSchema,
    capabilityState: AppGameChildUxCapabilityStateSchema,
    claimState: AppGameChildUxClaimStateSchema,
    explanationSource: AppGameChildUxExplanationSourceSchema,
    titleToken: AppGameChildUxCopyTokenSchema,
    bodyToken: AppGameChildUxCopyTokenSchema,
    primaryAction: AppGameChildUxPrimaryActionSchema,
    primaryActionToken: AppGameChildUxCopyTokenSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    childReasonReferences: Schema.Array(AppGameChildUxReasonRefSchema),
    childStatusReferences: Schema.Array(AppGameChildUxStatusRefSchema),
    approvalRequestRef: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    privateDiagnosticReferences: Schema.Array(AppGameChildUxDiagnosticRefSchema),
    adapterActionRef: Schema.Union(ParentActionReferenceSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (childUx) =>
        appGameChildUxCopyTokensMatchSurface(childUx) || 'Expected child UX copy tokens to match the visible state'
    ),
    Schema.filter(
      (childUx) =>
        appGameChildUxClaimMatchesSurface(childUx) || 'Expected child UX claim state to match the visible state'
    ),
    Schema.filter(
      (childUx) =>
        appGameChildUxRequestRefsAreAuditable(childUx) ||
        'Expected child ask-parent actions to cite approval, evidence, child reason, and child status refs'
    ),
    Schema.filter(
      (childUx) =>
        appGameChildUxStateIsHonest(childUx) ||
        'Expected child UX states to hide diagnostics and keep manual/unavailable states honest'
    )
  )
);

export type AppGameChildUxCard = Infer<typeof AppGameChildUxCardSchema>;

