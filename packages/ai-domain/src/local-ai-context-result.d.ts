import { type LocalAiContextBuildState, type LocalAiContextReasonCode, type LocalAiEvidenceContext, type LocalAiEvidenceContextBuildResult, type LocalAiEvidenceContextKind, type LocalAiEvidenceContextRefId, type LocalAiEvidenceContextSummary, type LocalAiRejectedField, type LocalAiStoredEvidenceContextBuildInput } from '@ocentra-parent/ai-domain/local-ai-context';
export interface LocalAiEvidenceContextBoundaryCounts {
    forbiddenCustodyReferenceCount: number;
    unallowedCustodyReferenceCount: number;
    ungroundedParentRuleReferenceCount: number;
}
export declare function contextForInput(input: LocalAiStoredEvidenceContextBuildInput, boundaryCounts: LocalAiEvidenceContextBoundaryCounts, additionalDegradedReasons: readonly LocalAiContextReasonCode[]): LocalAiEvidenceContext;
export declare function resultFor(input: LocalAiStoredEvidenceContextBuildInput, state: LocalAiContextBuildState, context: LocalAiEvidenceContext | null, rejectedFields: LocalAiRejectedField[], missingEvidenceKinds: LocalAiEvidenceContextKind[], degradedSourceRefs: LocalAiEvidenceContextRefId[], custodyBoundarySummary: LocalAiEvidenceContextSummary, validationGateSummary: LocalAiEvidenceContextSummary): LocalAiEvidenceContextBuildResult;
//# sourceMappingURL=local-ai-context-result.d.ts.map