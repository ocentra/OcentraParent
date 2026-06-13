import {
  LocalAiEvidenceContextBuildResultSchema,
  type LocalAiContextBuildState,
  type LocalAiContextReasonCode,
  type LocalAiEvidenceContext,
  type LocalAiEvidenceContextBuildResult,
  type LocalAiEvidenceContextKind,
  type LocalAiEvidenceContextRefId,
  type LocalAiEvidenceContextSummary,
  type LocalAiEvidenceContextValidationSummary,
  type LocalAiRejectedField,
  type LocalAiStoredEvidenceContextBuildInput,
} from '@ocentra-parent/ai-domain/local-ai-context';
import { refIdsForKind, uniqueCustodyLabels, uniqueReasonCodes } from './local-ai-context-selection';

export interface LocalAiEvidenceContextBoundaryCounts {
  forbiddenCustodyReferenceCount: number;
  unallowedCustodyReferenceCount: number;
  ungroundedParentRuleReferenceCount: number;
}

function validationSummary(
  input: LocalAiStoredEvidenceContextBuildInput,
  boundaryCounts: LocalAiEvidenceContextBoundaryCounts
): LocalAiEvidenceContextValidationSummary {
  const sourceEvidenceReferenceCount = input.evidenceReferences.reduce(
    (count, reference) => count + reference.sourceEvidenceReferences.length,
    0
  );
  return {
    evidenceReferenceCount: input.evidenceReferences.length,
    sourceEvidenceReferenceCount,
    runtimeReferenceCount: input.runtimeReferences.length,
    memoryReferenceCount: input.memoryReferences.length,
    graphReferenceCount: input.graphReferences.length,
    parentRuleReferenceCount: input.request.parentRuleContextReferences.length,
    ungroundedParentRuleReferenceCount: boundaryCounts.ungroundedParentRuleReferenceCount,
    forbiddenCustodyReferenceCount: boundaryCounts.forbiddenCustodyReferenceCount,
    unallowedCustodyReferenceCount: boundaryCounts.unallowedCustodyReferenceCount,
  };
}

export function contextForInput(
  input: LocalAiStoredEvidenceContextBuildInput,
  boundaryCounts: LocalAiEvidenceContextBoundaryCounts,
  additionalDegradedReasons: readonly LocalAiContextReasonCode[]
): LocalAiEvidenceContext {
  return {
    schemaVersion: input.request.schemaVersion,
    contextId: input.contextId,
    requestId: input.request.requestId,
    childProfile: input.request.childProfile,
    device: input.request.device,
    evidenceReferences: input.evidenceReferences,
    browserEvidenceRefs: refIdsForKind(input.evidenceReferences, 'browser'),
    appGameEvidenceRefs: refIdsForKind(input.evidenceReferences, 'app-game'),
    networkFlowEvidenceRefs: refIdsForKind(input.evidenceReferences, 'network-flow'),
    screenSummaryRefs: refIdsForKind(input.evidenceReferences, 'screen-summary'),
    parentRuleReferences: input.request.parentRuleContextReferences.map((reference) => reference.rule.ruleId),
    parentRuleContextReferences: input.request.parentRuleContextReferences,
    recentActivitySummaryRefs: refIdsForKind(input.evidenceReferences, 'recent-activity'),
    memoryReferences: input.memoryReferences,
    graphReferences: input.graphReferences,
    localModelRuntimeRefs: input.runtimeReferences.map((reference) => reference.runtimeReferenceId),
    promptVersion: input.request.promptVersion,
    custodyLabels: uniqueCustodyLabels(input.evidenceReferences),
    degradedReasons: uniqueReasonCodes(input.evidenceReferences, 'degradedReasons', additionalDegradedReasons),
    unknownReasons: uniqueReasonCodes(input.evidenceReferences, 'unknownReasons'),
    validationSummary: validationSummary(input, boundaryCounts),
  };
}

export function resultFor(
  input: LocalAiStoredEvidenceContextBuildInput,
  state: LocalAiContextBuildState,
  context: LocalAiEvidenceContext | null,
  rejectedFields: LocalAiRejectedField[],
  missingEvidenceKinds: LocalAiEvidenceContextKind[],
  degradedSourceRefs: LocalAiEvidenceContextRefId[],
  custodyBoundarySummary: LocalAiEvidenceContextSummary,
  validationGateSummary: LocalAiEvidenceContextSummary
): LocalAiEvidenceContextBuildResult {
  return LocalAiEvidenceContextBuildResultSchema.parse({
    schemaVersion: input.request.schemaVersion,
    requestId: input.request.requestId,
    state,
    context,
    rejectedFields,
    missingEvidenceKinds,
    degradedSourceRefs,
    custodyBoundarySummary,
    validationGateSummary,
    auditEvidenceReferences: input.evidenceReferences.map((reference) => reference.evidence),
  });
}
