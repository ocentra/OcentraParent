import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiEvidenceContextBuildResultSchema,
  LocalAiEvidenceContextSummarySchema,
  LocalAiRejectedFieldSchema,
  LocalAiStoredEvidenceContextBuildInputSchema,
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
} from './local-ai-context';
import {
  refIdsForKind,
  selectLocalAiEvidenceContextInput,
  uniqueCustodyLabels,
  uniqueReasonCodes,
} from './local-ai-context-selection';

const decodeRejectedField = Schema.decodeUnknownSync(LocalAiRejectedFieldSchema);
const decodeContextSummary = Schema.decodeUnknownSync(LocalAiEvidenceContextSummarySchema);

const EvidenceReferencesField = decodeRejectedField('evidenceReferences');
const HostedCustodySummary = decodeContextSummary(
  'ocentra-hosted non-activity metadata cannot source child-activity evidence'
);
const HostedCustodyGate = decodeContextSummary('rejected forbidden custody before local model input');
const UnallowedCustodySummary = decodeContextSummary('evidence custody was not allowed by context request');
const UnallowedCustodyGate = decodeContextSummary('rejected unallowed custody before local model input');
const NoEvidenceSummary = decodeContextSummary('no child-activity evidence selected for context');
const InsufficientEvidenceGate = decodeContextSummary('insufficient stored evidence for local model input');
const StoredEvidenceSummary = decodeContextSummary('context uses only child-device or parent-owned stored evidence');
const PartialContextGate = decodeContextSummary('partial context built with explicit missing evidence kinds');
const ReadyContextGate = decodeContextSummary('ready stored-evidence context for local-only model input');

function validationSummary(
  input: LocalAiStoredEvidenceContextBuildInput,
  forbiddenCustodyReferenceCount: number,
  unallowedCustodyReferenceCount: number
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
    forbiddenCustodyReferenceCount,
    unallowedCustodyReferenceCount,
  };
}

function contextForInput(
  input: LocalAiStoredEvidenceContextBuildInput,
  forbiddenCustodyReferenceCount: number,
  unallowedCustodyReferenceCount: number,
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
    parentRuleReferences: input.request.parentRuleReferences,
    recentActivitySummaryRefs: refIdsForKind(input.evidenceReferences, 'recent-activity'),
    memoryReferences: input.memoryReferences,
    graphReferences: input.graphReferences,
    localModelRuntimeRefs: input.runtimeReferences.map((reference) => reference.runtimeReferenceId),
    promptVersion: input.request.promptVersion,
    custodyLabels: uniqueCustodyLabels(input.evidenceReferences),
    degradedReasons: uniqueReasonCodes(input.evidenceReferences, 'degradedReasons', additionalDegradedReasons),
    unknownReasons: uniqueReasonCodes(input.evidenceReferences, 'unknownReasons'),
    validationSummary: validationSummary(input, forbiddenCustodyReferenceCount, unallowedCustodyReferenceCount),
  };
}

function resultFor(
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

export function buildLocalAiEvidenceContext(input: unknown): LocalAiEvidenceContextBuildResult {
  const parsed = LocalAiStoredEvidenceContextBuildInputSchema.parse(input);
  const selection = selectLocalAiEvidenceContextInput(parsed);
  if (selection.forbiddenCustodyReferences.length > 0) {
    return resultFor(
      parsed,
      'rejected',
      null,
      [EvidenceReferencesField],
      [],
      selection.forbiddenCustodyReferences.map((reference) => reference.evidenceRefId),
      HostedCustodySummary,
      HostedCustodyGate
    );
  }
  const selectedInput: LocalAiStoredEvidenceContextBuildInput = {
    ...parsed,
    evidenceReferences: selection.selectedEvidenceReferences,
    runtimeReferences: selection.selectedRuntimeReferences,
    memoryReferences: selection.selectedMemoryReferences,
    graphReferences: selection.selectedGraphReferences,
  };
  if (selectedInput.evidenceReferences.length === 0 && selection.unallowedCustodyReferences.length > 0) {
    return resultFor(
      parsed,
      'rejected',
      null,
      [EvidenceReferencesField],
      selection.missingEvidenceKinds,
      selection.degradedSourceRefs,
      UnallowedCustodySummary,
      UnallowedCustodyGate
    );
  }

  const context =
    selectedInput.evidenceReferences.length > 0
      ? contextForInput(
          selectedInput,
          selection.forbiddenCustodyReferences.length,
          selection.unallowedCustodyReferences.length,
          selection.additionalDegradedReasons
        )
      : null;
  if (selectedInput.evidenceReferences.length === 0) {
    return resultFor(
      parsed,
      'insufficient',
      null,
      [],
      selection.missingEvidenceKinds,
      [],
      NoEvidenceSummary,
      InsufficientEvidenceGate
    );
  }
  if (selection.missingEvidenceKinds.length > 0 || selection.additionalDegradedReasons.length > 0) {
    return resultFor(
      parsed,
      'partial',
      context,
      selection.unallowedCustodyReferences.length > 0 ? [EvidenceReferencesField] : [],
      selection.missingEvidenceKinds,
      selection.degradedSourceRefs,
      selection.unallowedCustodyReferences.length > 0 ? UnallowedCustodySummary : StoredEvidenceSummary,
      PartialContextGate
    );
  }
  return resultFor(parsed, 'ready', context, [], [], [], StoredEvidenceSummary, ReadyContextGate);
}
