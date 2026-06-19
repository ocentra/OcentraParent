import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialParentPolicyDecisionCandidateSchema,
  type SocialParentPolicyDecisionCandidate,
} from './social-policy-compiler';
import { SocialPolicyEvidenceRefsSchema } from './social-policy-compiler-values';

export const SocialManagedBrowserPolicyExecutionState = {
  ManagedInterventionExecuted: 'managed-browser-intervention-executed',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialManagedBrowserPolicyExecutionStateSchema = withParser(
  Schema.Literal(...Object.values(SocialManagedBrowserPolicyExecutionState))
);

const SocialManagedBrowserPolicyExecutionBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  executionId: ParentEvidenceReferenceIdSchema,
  sourceDecisionCandidate: SocialParentPolicyDecisionCandidateSchema,
  executionEvidenceRefs: SocialPolicyEvidenceRefsSchema,
  managedBrowserInterventionEvidenceRef: Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null),
  childInterventionEndpointRef: Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null),
  targetUrlEvidenceRef: Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null),
  screenshotEvidenceRefs: Schema.Array(ParentEvidenceReferenceIdSchema),
  executionState: SocialManagedBrowserPolicyExecutionStateSchema,
  managedSessionObserved: Schema.Boolean,
  exactManagedUrlObserved: Schema.Boolean,
  liveSurfaceCapturedBeforeMutation: Schema.Boolean,
  browserMutationObserved: Schema.Boolean,
  childInterventionExecuted: Schema.Boolean,
  managedInterventionEnforced: Schema.Boolean,
  finalPolicyExecutionClaimed: Schema.Boolean,
  unmanagedBrowserClaimed: Schema.Literal(false),
  broadOsEnforcementClaimed: Schema.Literal(false),
  providerDeliveryAttempted: Schema.Literal(false),
  nativeAppControlClaimed: Schema.Literal(false),
  applePlatformClaimed: Schema.Literal(false),
  rawUrlPersisted: Schema.Literal(false),
  rawPageContentPersisted: Schema.Literal(false),
  createdAt: ParentTimestampSchema,
});

export const SocialManagedBrowserPolicyExecutionSchema = withParser(
  SocialManagedBrowserPolicyExecutionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialManagedBrowserPolicyExecutionIsCoherent(value) ||
        'Expected social managed browser execution to be evidence-backed and scoped to managed sessions'
    )
  )
);

export type SocialManagedBrowserPolicyExecution = Infer<typeof SocialManagedBrowserPolicyExecutionSchema>;

export type BuildSocialManagedBrowserPolicyExecutionInput = {
  readonly executionId: string;
  readonly sourceDecisionCandidate: SocialParentPolicyDecisionCandidate;
  readonly executionEvidenceRefs: ReadonlyArray<string>;
  readonly managedBrowserInterventionEvidenceRef: string;
  readonly childInterventionEndpointRef: string;
  readonly targetUrlEvidenceRef: string;
  readonly screenshotEvidenceRefs: ReadonlyArray<string>;
  readonly createdAt: string;
};

export function buildSocialManagedBrowserPolicyExecution(
  input: BuildSocialManagedBrowserPolicyExecutionInput
): SocialManagedBrowserPolicyExecution {
  return SocialManagedBrowserPolicyExecutionSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    executionId: input.executionId,
    sourceDecisionCandidate: input.sourceDecisionCandidate,
    executionEvidenceRefs: input.executionEvidenceRefs,
    managedBrowserInterventionEvidenceRef: input.managedBrowserInterventionEvidenceRef,
    childInterventionEndpointRef: input.childInterventionEndpointRef,
    targetUrlEvidenceRef: input.targetUrlEvidenceRef,
    screenshotEvidenceRefs: input.screenshotEvidenceRefs,
    executionState: SocialManagedBrowserPolicyExecutionState.ManagedInterventionExecuted,
    managedSessionObserved: true,
    exactManagedUrlObserved: true,
    liveSurfaceCapturedBeforeMutation: true,
    browserMutationObserved: true,
    childInterventionExecuted: true,
    managedInterventionEnforced: true,
    finalPolicyExecutionClaimed: true,
    unmanagedBrowserClaimed: false,
    broadOsEnforcementClaimed: false,
    providerDeliveryAttempted: false,
    nativeAppControlClaimed: false,
    applePlatformClaimed: false,
    rawUrlPersisted: false,
    rawPageContentPersisted: false,
    createdAt: input.createdAt,
  });
}

export function summarizeSocialManagedBrowserPolicyExecution(execution: SocialManagedBrowserPolicyExecution) {
  return {
    executionState: execution.executionState,
    actionCandidate: execution.sourceDecisionCandidate.actionCandidate,
    finalPolicyExecutionClaimed: execution.finalPolicyExecutionClaimed,
    browserMutationObserved: execution.browserMutationObserved,
    childInterventionExecuted: execution.childInterventionExecuted,
    managedInterventionEnforced: execution.managedInterventionEnforced,
    broadOsEnforcementClaimed: execution.broadOsEnforcementClaimed,
    unmanagedBrowserClaimed: execution.unmanagedBrowserClaimed,
    providerDeliveryAttempted: execution.providerDeliveryAttempted,
    nativeAppControlClaimed: execution.nativeAppControlClaimed,
    applePlatformClaimed: execution.applePlatformClaimed,
  };
}

function socialManagedBrowserPolicyExecutionIsCoherent(
  value: Infer<typeof SocialManagedBrowserPolicyExecutionBaseSchema>
): boolean {
  if (value.executionState === SocialManagedBrowserPolicyExecutionState.ManagedInterventionExecuted) {
    return managedExecutionIsCoherent(value);
  }

  return manualOrUnavailableExecutionIsCoherent(value);
}

type SocialManagedBrowserPolicyExecutionInput = Infer<typeof SocialManagedBrowserPolicyExecutionBaseSchema>;

function managedExecutionIsCoherent(value: SocialManagedBrowserPolicyExecutionInput): boolean {
  return (
    socialPolicyActionCanExecuteManagedIntervention(value.sourceDecisionCandidate.actionCandidate) &&
    value.sourceDecisionCandidate.compilerMode === 'contract-only' &&
    value.sourceDecisionCandidate.finalPolicyDecisionClaimed === false &&
    value.sourceDecisionCandidate.runtimeGateExecutedClaimed === false &&
    managedExecutionRefsArePresent(value) &&
    managedExecutionObservedRuntimePath(value)
  );
}

function managedExecutionRefsArePresent(value: SocialManagedBrowserPolicyExecutionInput): boolean {
  return (
    value.managedBrowserInterventionEvidenceRef !== null &&
    value.childInterventionEndpointRef !== null &&
    value.targetUrlEvidenceRef !== null &&
    value.screenshotEvidenceRefs.length > 0
  );
}

function managedExecutionObservedRuntimePath(value: SocialManagedBrowserPolicyExecutionInput): boolean {
  return (
    value.managedSessionObserved &&
    value.exactManagedUrlObserved &&
    value.liveSurfaceCapturedBeforeMutation &&
    value.browserMutationObserved &&
    value.childInterventionExecuted &&
    value.managedInterventionEnforced &&
    value.finalPolicyExecutionClaimed
  );
}

function manualOrUnavailableExecutionIsCoherent(value: SocialManagedBrowserPolicyExecutionInput): boolean {
  return (
    value.managedBrowserInterventionEvidenceRef === null &&
    value.childInterventionEndpointRef === null &&
    value.targetUrlEvidenceRef === null &&
    value.screenshotEvidenceRefs.length === 0 &&
    value.managedSessionObserved === false &&
    value.exactManagedUrlObserved === false &&
    value.liveSurfaceCapturedBeforeMutation === false &&
    value.browserMutationObserved === false &&
    value.childInterventionExecuted === false &&
    value.managedInterventionEnforced === false &&
    value.finalPolicyExecutionClaimed === false
  );
}

function socialPolicyActionCanExecuteManagedIntervention(
  actionCandidate: SocialParentPolicyDecisionCandidate['actionCandidate']
): boolean {
  return (
    actionCandidate === 'block-candidate' ||
    actionCandidate === 'warn-candidate' ||
    actionCandidate === 'parent-review-candidate'
  );
}
