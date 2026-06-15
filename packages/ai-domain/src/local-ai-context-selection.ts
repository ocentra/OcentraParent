import type {
  LocalAiContextReasonCode,
  LocalAiEvidenceContextKind,
  LocalAiEvidenceContextRefId,
  LocalAiEvidenceContextSourceRef,
  LocalAiEvidenceCustody,
  LocalAiParentRuleContextRef,
  LocalAiStoredEvidenceContextBuildInput,
} from './local-ai-context';
import type { LocalAiGraphReference, LocalAiMemoryReference } from './local-ai-references';
import type { LocalModelRuntimeStatus } from './local-ai-runtime';

const HostedNonActivityCustody = 'ocentra-hosted-non-activity';

export interface LocalAiEvidenceContextSelection {
  selectedEvidenceReferences: LocalAiEvidenceContextSourceRef[];
  forbiddenCustodyReferences: LocalAiEvidenceContextSourceRef[];
  unallowedCustodyReferences: LocalAiEvidenceContextSourceRef[];
  selectedRuntimeReferences: LocalModelRuntimeStatus[];
  selectedMemoryReferences: LocalAiMemoryReference[];
  selectedGraphReferences: LocalAiGraphReference[];
  selectedParentRuleContextReferences: LocalAiParentRuleContextRef[];
  ungroundedParentRuleContextReferences: LocalAiParentRuleContextRef[];
  missingEvidenceKinds: LocalAiEvidenceContextKind[];
  degradedSourceRefs: LocalAiEvidenceContextRefId[];
  additionalDegradedReasons: LocalAiContextReasonCode[];
}

export function refIdsForKind(
  evidenceReferences: readonly LocalAiEvidenceContextSourceRef[],
  evidenceKind: LocalAiEvidenceContextKind
): LocalAiEvidenceContextRefId[] {
  return evidenceReferences
    .filter((reference) => reference.evidenceKind === evidenceKind)
    .map((reference) => reference.evidenceRefId);
}

export function uniqueCustodyLabels(
  evidenceReferences: readonly LocalAiEvidenceContextSourceRef[]
): LocalAiEvidenceCustody[] {
  return [...new Set(evidenceReferences.map((reference) => reference.custody))];
}

export function uniqueReasonCodes(
  evidenceReferences: readonly LocalAiEvidenceContextSourceRef[],
  reasonKey: 'degradedReasons' | 'unknownReasons',
  additionalReasons: readonly LocalAiContextReasonCode[] = []
): LocalAiContextReasonCode[] {
  return [...new Set([...evidenceReferences.flatMap((reference) => reference[reasonKey]), ...additionalReasons])];
}

function selectedEvidenceReferenceIds(evidenceReferences: readonly LocalAiEvidenceContextSourceRef[]): Set<string> {
  const referenceIds = new Set<string>();
  for (const reference of evidenceReferences) {
    referenceIds.add(reference.evidenceRefId);
    referenceIds.add(reference.evidence.evidenceReferenceId);
    for (const sourceReference of reference.sourceEvidenceReferences) {
      referenceIds.add(sourceReference.evidenceReferenceId);
    }
  }
  return referenceIds;
}

function hasSelectedEvidenceGrounding(
  reference:
    | LocalAiMemoryReference['sourceEvidenceReferences'][number]
    | LocalAiGraphReference['sourceEvidenceReferences'][number],
  selectedReferenceIds: ReadonlySet<string>
): boolean {
  return selectedReferenceIds.has(reference.evidenceReferenceId);
}

function selectGroundedMemoryReferences(
  input: LocalAiStoredEvidenceContextBuildInput,
  selectedReferenceIds: ReadonlySet<string>
): LocalAiMemoryReference[] {
  return input.memoryReferences.filter((reference) =>
    reference.sourceEvidenceReferences.every((sourceReference) =>
      hasSelectedEvidenceGrounding(sourceReference, selectedReferenceIds)
    )
  );
}

function selectGroundedGraphReferences(
  input: LocalAiStoredEvidenceContextBuildInput,
  selectedReferenceIds: ReadonlySet<string>
): LocalAiGraphReference[] {
  return input.graphReferences.filter((reference) =>
    reference.sourceEvidenceReferences.every((sourceReference) =>
      hasSelectedEvidenceGrounding(sourceReference, selectedReferenceIds)
    )
  );
}

function parentRuleHasSelectedEvidenceGrounding(
  reference: LocalAiParentRuleContextRef,
  selectedReferenceIds: ReadonlySet<string>
): boolean {
  return reference.targetEvidenceRefs.every((targetEvidenceRef) => selectedReferenceIds.has(targetEvidenceRef));
}

function selectGroundedParentRuleContextReferences(
  input: LocalAiStoredEvidenceContextBuildInput,
  selectedReferenceIds: ReadonlySet<string>
): LocalAiParentRuleContextRef[] {
  return input.request.parentRuleContextReferences.filter((reference) =>
    parentRuleHasSelectedEvidenceGrounding(reference, selectedReferenceIds)
  );
}

function selectRuntimeReferences(input: LocalAiStoredEvidenceContextBuildInput): LocalModelRuntimeStatus[] {
  if (input.request.modelTaskRequirements.length === 0) {
    return [...input.runtimeReferences];
  }
  return input.runtimeReferences.filter(
    (runtimeReference) =>
      runtimeReference.loadState === 'loaded' &&
      input.request.modelTaskRequirements.every((capabilityFlag) =>
        runtimeReference.capabilityFlags.includes(capabilityFlag)
      )
  );
}

function pushReasonCode(reasonCodes: LocalAiContextReasonCode[], reasonCode: LocalAiContextReasonCode): void {
  if (!reasonCodes.includes(reasonCode)) {
    reasonCodes.push(reasonCode);
  }
}

export function selectLocalAiEvidenceContextInput(
  input: LocalAiStoredEvidenceContextBuildInput
): LocalAiEvidenceContextSelection {
  const allowedCustody = new Set(input.request.allowedCustody);
  const forbiddenCustodyReferences = input.evidenceReferences.filter(
    (reference) => reference.custody === HostedNonActivityCustody
  );
  const unallowedCustodyReferences = input.evidenceReferences.filter(
    (reference) => reference.custody !== HostedNonActivityCustody && !allowedCustody.has(reference.custody)
  );
  const selectedEvidenceReferences = input.evidenceReferences.filter(
    (reference) => reference.custody !== HostedNonActivityCustody && allowedCustody.has(reference.custody)
  );
  const selectedRuntimeReferences = selectRuntimeReferences(input);
  const selectedReferenceIds = selectedEvidenceReferenceIds(selectedEvidenceReferences);
  const selectedMemoryReferences = selectGroundedMemoryReferences(input, selectedReferenceIds);
  const selectedGraphReferences = selectGroundedGraphReferences(input, selectedReferenceIds);
  const selectedParentRuleContextReferences = selectGroundedParentRuleContextReferences(input, selectedReferenceIds);
  const ungroundedParentRuleContextReferences = input.request.parentRuleContextReferences.filter(
    (reference) => !parentRuleHasSelectedEvidenceGrounding(reference, selectedReferenceIds)
  );
  const additionalDegradedReasons: LocalAiContextReasonCode[] = [];
  if (unallowedCustodyReferences.length > 0) {
    pushReasonCode(additionalDegradedReasons, 'custody-unavailable');
  }
  if (input.request.modelTaskRequirements.length > 0 && selectedRuntimeReferences.length === 0) {
    pushReasonCode(additionalDegradedReasons, 'model-unavailable');
  }
  if (selectedMemoryReferences.length !== input.memoryReferences.length) {
    pushReasonCode(additionalDegradedReasons, 'memory-ungrounded');
  }
  if (selectedGraphReferences.length !== input.graphReferences.length) {
    pushReasonCode(additionalDegradedReasons, 'graph-ungrounded');
  }
  if (selectedParentRuleContextReferences.length === 0) {
    pushReasonCode(additionalDegradedReasons, 'parent-rule-missing');
  }
  return {
    selectedEvidenceReferences,
    forbiddenCustodyReferences,
    unallowedCustodyReferences,
    selectedRuntimeReferences,
    selectedMemoryReferences,
    selectedGraphReferences,
    selectedParentRuleContextReferences,
    ungroundedParentRuleContextReferences,
    missingEvidenceKinds: input.request.requiredEvidenceKinds.filter(
      (evidenceKind) => refIdsForKind(selectedEvidenceReferences, evidenceKind).length === 0
    ),
    degradedSourceRefs: unallowedCustodyReferences.map((reference) => reference.evidenceRefId),
    additionalDegradedReasons,
  };
}
