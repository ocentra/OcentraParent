import type { LocalAiContextReasonCode, LocalAiEvidenceContextKind, LocalAiEvidenceContextRefId, LocalAiEvidenceContextSourceRef, LocalAiEvidenceCustody, LocalAiParentRuleContextRef, LocalAiStoredEvidenceContextBuildInput } from '@ocentra-parent/ai-domain/local-ai-context';
import type { LocalAiGraphReference, LocalAiMemoryReference } from './local-ai-references';
import type { LocalModelRuntimeStatus } from '@ocentra-parent/ai-domain/local-ai-runtime';
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
export declare function refIdsForKind(evidenceReferences: readonly LocalAiEvidenceContextSourceRef[], evidenceKind: LocalAiEvidenceContextKind): LocalAiEvidenceContextRefId[];
export declare function uniqueCustodyLabels(evidenceReferences: readonly LocalAiEvidenceContextSourceRef[]): LocalAiEvidenceCustody[];
export declare function uniqueReasonCodes(evidenceReferences: readonly LocalAiEvidenceContextSourceRef[], reasonKey: 'degradedReasons' | 'unknownReasons', additionalReasons?: readonly LocalAiContextReasonCode[]): LocalAiContextReasonCode[];
export declare function selectLocalAiEvidenceContextInput(input: LocalAiStoredEvidenceContextBuildInput): LocalAiEvidenceContextSelection;
//# sourceMappingURL=local-ai-context-selection.d.ts.map