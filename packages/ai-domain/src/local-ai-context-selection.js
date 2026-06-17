const HostedNonActivityCustody = 'ocentra-hosted-non-activity';
export function refIdsForKind(evidenceReferences, evidenceKind) {
    return evidenceReferences
        .filter((reference) => reference.evidenceKind === evidenceKind)
        .map((reference) => reference.evidenceRefId);
}
export function uniqueCustodyLabels(evidenceReferences) {
    return [...new Set(evidenceReferences.map((reference) => reference.custody))];
}
export function uniqueReasonCodes(evidenceReferences, reasonKey, additionalReasons = []) {
    return [...new Set([...evidenceReferences.flatMap((reference) => reference[reasonKey]), ...additionalReasons])];
}
function selectedEvidenceReferenceIds(evidenceReferences) {
    const referenceIds = new Set();
    for (const reference of evidenceReferences) {
        referenceIds.add(reference.evidenceRefId);
        referenceIds.add(reference.evidence.evidenceReferenceId);
        for (const sourceReference of reference.sourceEvidenceReferences) {
            referenceIds.add(sourceReference.evidenceReferenceId);
        }
    }
    return referenceIds;
}
function hasSelectedEvidenceGrounding(reference, selectedReferenceIds) {
    return selectedReferenceIds.has(reference.evidenceReferenceId);
}
function selectGroundedMemoryReferences(input, selectedReferenceIds) {
    return input.memoryReferences.filter((reference) => reference.sourceEvidenceReferences.every((sourceReference) => hasSelectedEvidenceGrounding(sourceReference, selectedReferenceIds)));
}
function selectGroundedGraphReferences(input, selectedReferenceIds) {
    return input.graphReferences.filter((reference) => reference.sourceEvidenceReferences.every((sourceReference) => hasSelectedEvidenceGrounding(sourceReference, selectedReferenceIds)));
}
function parentRuleHasSelectedEvidenceGrounding(reference, selectedReferenceIds) {
    return reference.targetEvidenceRefs.every((targetEvidenceRef) => selectedReferenceIds.has(targetEvidenceRef));
}
function selectGroundedParentRuleContextReferences(input, selectedReferenceIds) {
    return input.request.parentRuleContextReferences.filter((reference) => parentRuleHasSelectedEvidenceGrounding(reference, selectedReferenceIds));
}
function selectRuntimeReferences(input) {
    if (input.request.modelTaskRequirements.length === 0) {
        return [...input.runtimeReferences];
    }
    return input.runtimeReferences.filter((runtimeReference) => runtimeReference.loadState === 'loaded' &&
        input.request.modelTaskRequirements.every((capabilityFlag) => runtimeReference.capabilityFlags.includes(capabilityFlag)));
}
function pushReasonCode(reasonCodes, reasonCode) {
    if (!reasonCodes.includes(reasonCode)) {
        reasonCodes.push(reasonCode);
    }
}
export function selectLocalAiEvidenceContextInput(input) {
    const allowedCustody = new Set(input.request.allowedCustody);
    const forbiddenCustodyReferences = input.evidenceReferences.filter((reference) => reference.custody === HostedNonActivityCustody);
    const unallowedCustodyReferences = input.evidenceReferences.filter((reference) => reference.custody !== HostedNonActivityCustody && !allowedCustody.has(reference.custody));
    const selectedEvidenceReferences = input.evidenceReferences.filter((reference) => reference.custody !== HostedNonActivityCustody && allowedCustody.has(reference.custody));
    const selectedRuntimeReferences = selectRuntimeReferences(input);
    const selectedReferenceIds = selectedEvidenceReferenceIds(selectedEvidenceReferences);
    const selectedMemoryReferences = selectGroundedMemoryReferences(input, selectedReferenceIds);
    const selectedGraphReferences = selectGroundedGraphReferences(input, selectedReferenceIds);
    const selectedParentRuleContextReferences = selectGroundedParentRuleContextReferences(input, selectedReferenceIds);
    const ungroundedParentRuleContextReferences = input.request.parentRuleContextReferences.filter((reference) => !parentRuleHasSelectedEvidenceGrounding(reference, selectedReferenceIds));
    const additionalDegradedReasons = [];
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
        missingEvidenceKinds: input.request.requiredEvidenceKinds.filter((evidenceKind) => refIdsForKind(selectedEvidenceReferences, evidenceKind).length === 0),
        degradedSourceRefs: unallowedCustodyReferences.map((reference) => reference.evidenceRefId),
        additionalDegradedReasons,
    };
}
//# sourceMappingURL=local-ai-context-selection.js.map