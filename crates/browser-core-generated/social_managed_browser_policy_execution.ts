/* generated from crates/browser-core/src/social_managed_browser_policy_execution.rs */

import type { SocialParentPolicyDecisionCandidate } from './social_policy_compiler_contract';

export const SocialManagedBrowserPolicyExecutionTemplate = {
  executionState: 'managed-browser-intervention-executed',
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
} as const;

export type SocialManagedBrowserPolicyExecutionTemplate = typeof SocialManagedBrowserPolicyExecutionTemplate;

type SocialManagedBrowserPolicyExecutionState =
  | 'managed-browser-intervention-executed'
  | 'manual-required'
  | 'unavailable';

type SocialManagedBrowserPolicyExecutionInput = {
  readonly executionId: string;
  readonly sourceDecisionCandidate: SocialParentPolicyDecisionCandidate;
  readonly executionEvidenceRefs: readonly string[];
  readonly managedBrowserInterventionEvidenceRef: string;
  readonly childInterventionEndpointRef: string;
  readonly targetUrlEvidenceRef: string;
  readonly screenshotEvidenceRefs: readonly string[];
  readonly createdAt: string;
};

type SocialManagedBrowserPolicyExecutionLike = {
  readonly executionState: SocialManagedBrowserPolicyExecutionState;
  readonly sourceDecisionCandidate: Pick<
    SocialParentPolicyDecisionCandidate,
    'actionCandidate' | 'compilerMode' | 'finalPolicyDecisionClaimed' | 'runtimeGateExecutedClaimed'
  >;
  readonly managedBrowserInterventionEvidenceRef: string | null;
  readonly childInterventionEndpointRef: string | null;
  readonly targetUrlEvidenceRef: string | null;
  readonly screenshotEvidenceRefs: readonly string[];
  readonly managedSessionObserved: boolean;
  readonly exactManagedUrlObserved: boolean;
  readonly liveSurfaceCapturedBeforeMutation: boolean;
  readonly browserMutationObserved: boolean;
  readonly childInterventionExecuted: boolean;
  readonly managedInterventionEnforced: boolean;
  readonly finalPolicyExecutionClaimed: boolean;
  readonly unmanagedBrowserClaimed: boolean;
  readonly broadOsEnforcementClaimed: boolean;
  readonly providerDeliveryAttempted: boolean;
  readonly nativeAppControlClaimed: boolean;
  readonly applePlatformClaimed: boolean;
  readonly rawUrlPersisted: boolean;
  readonly rawPageContentPersisted: boolean;
};

type SocialManagedBrowserPolicyExecutionSummary = {
  readonly executionState: SocialManagedBrowserPolicyExecutionState;
  readonly actionCandidate: SocialParentPolicyDecisionCandidate['actionCandidate'];
  readonly finalPolicyExecutionClaimed: boolean;
  readonly browserMutationObserved: boolean;
  readonly childInterventionExecuted: boolean;
  readonly managedInterventionEnforced: boolean;
  readonly broadOsEnforcementClaimed: boolean;
  readonly unmanagedBrowserClaimed: boolean;
  readonly providerDeliveryAttempted: boolean;
  readonly nativeAppControlClaimed: boolean;
  readonly applePlatformClaimed: boolean;
};

export function socialManagedBrowserPolicyExecutionTemplate(): SocialManagedBrowserPolicyExecutionTemplate {
  return SocialManagedBrowserPolicyExecutionTemplate;
}

export function buildGeneratedSocialManagedBrowserPolicyExecution(input: SocialManagedBrowserPolicyExecutionInput) {
  return {
    schemaVersion: 'v0.6',
    executionId: input.executionId,
    sourceDecisionCandidate: input.sourceDecisionCandidate,
    executionEvidenceRefs: [...input.executionEvidenceRefs],
    managedBrowserInterventionEvidenceRef: input.managedBrowserInterventionEvidenceRef,
    childInterventionEndpointRef: input.childInterventionEndpointRef,
    targetUrlEvidenceRef: input.targetUrlEvidenceRef,
    screenshotEvidenceRefs: [...input.screenshotEvidenceRefs],
    ...SocialManagedBrowserPolicyExecutionTemplate,
    createdAt: input.createdAt,
  };
}

export function summarizeGeneratedSocialManagedBrowserPolicyExecution(
  execution: SocialManagedBrowserPolicyExecutionLike
): SocialManagedBrowserPolicyExecutionSummary {
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

export function generatedSocialManagedBrowserPolicyExecutionIsCoherent(
  value: SocialManagedBrowserPolicyExecutionLike
): boolean {
  if (value.executionState === 'managed-browser-intervention-executed') {
    return (
      generatedSocialPolicyActionCanExecuteManagedIntervention(value.sourceDecisionCandidate.actionCandidate) &&
      value.sourceDecisionCandidate.compilerMode === 'contract-only' &&
      value.sourceDecisionCandidate.finalPolicyDecisionClaimed === false &&
      value.sourceDecisionCandidate.runtimeGateExecutedClaimed === false &&
      value.managedBrowserInterventionEvidenceRef !== null &&
      value.childInterventionEndpointRef !== null &&
      value.targetUrlEvidenceRef !== null &&
      value.screenshotEvidenceRefs.length > 0 &&
      value.managedSessionObserved &&
      value.exactManagedUrlObserved &&
      value.liveSurfaceCapturedBeforeMutation &&
      value.browserMutationObserved &&
      value.childInterventionExecuted &&
      value.managedInterventionEnforced &&
      value.finalPolicyExecutionClaimed
    );
  }

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

function generatedSocialPolicyActionCanExecuteManagedIntervention(
  actionCandidate: SocialParentPolicyDecisionCandidate['actionCandidate']
): boolean {
  return (
    actionCandidate === 'block-candidate' ||
    actionCandidate === 'warn-candidate' ||
    actionCandidate === 'parent-review-candidate'
  );
}
