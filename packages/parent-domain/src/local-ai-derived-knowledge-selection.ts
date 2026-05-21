import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  type ParentActionReference,
  ParentActionReferenceSchema,
  type ParentEvidenceReference,
  ParentEvidenceReferenceSchema,
} from './references';
import { type ParentPolicyVersion, ParentPolicyVersionSchema } from './reference-primitives';
import { LocalAiTimestampSchema } from './local-ai-primitives';
import { type LocalAiDerivedKnowledgeEntry, LocalAiDerivedKnowledgeEntrySchema } from './local-ai-derived-knowledge';

export const LocalAiDerivedKnowledgeUsabilityInputSchema = withParser(
  Schema.Struct({
    entries: Schema.Array(LocalAiDerivedKnowledgeEntrySchema),
    selectedEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    selectedPolicyVersions: Schema.Array(ParentPolicyVersionSchema),
    selectedParentActionReferences: Schema.Array(ParentActionReferenceSchema),
    asOf: LocalAiTimestampSchema,
  })
);

export type LocalAiDerivedKnowledgeUsabilityInput = Infer<typeof LocalAiDerivedKnowledgeUsabilityInputSchema>;

interface LocalAiDerivedKnowledgeGroundingSets {
  readonly evidenceReferenceIds: ReadonlySet<string>;
  readonly policyVersions: ReadonlySet<string>;
  readonly parentActionReferenceIds: ReadonlySet<string>;
}

function evidenceReferenceIds(references: readonly ParentEvidenceReference[]): Set<string> {
  return new Set(references.map((reference) => reference.evidenceReferenceId));
}

function policyVersions(references: readonly ParentPolicyVersion[]): Set<string> {
  return new Set(references);
}

function parentActionReferenceIds(references: readonly ParentActionReference[]): Set<string> {
  return new Set(references.map((reference) => reference.actionReferenceId));
}

function groundingSets(input: LocalAiDerivedKnowledgeUsabilityInput): LocalAiDerivedKnowledgeGroundingSets {
  return {
    evidenceReferenceIds: evidenceReferenceIds(input.selectedEvidenceReferences),
    policyVersions: policyVersions(input.selectedPolicyVersions),
    parentActionReferenceIds: parentActionReferenceIds(input.selectedParentActionReferences),
  };
}

function hasSelectedEvidenceCitations(
  entry: LocalAiDerivedKnowledgeEntry,
  selectedEvidenceReferenceIds: ReadonlySet<string>
): boolean {
  return entry.citations.sourceEvidenceReferences.every((reference) =>
    selectedEvidenceReferenceIds.has(reference.evidenceReferenceId)
  );
}

function hasSelectedPolicyCitations(
  entry: LocalAiDerivedKnowledgeEntry,
  selectedPolicyVersions: ReadonlySet<string>
): boolean {
  return entry.citations.sourcePolicyVersions.every((policyVersion) => selectedPolicyVersions.has(policyVersion));
}

function hasSelectedParentActionCitations(
  entry: LocalAiDerivedKnowledgeEntry,
  selectedParentActionReferenceIds: ReadonlySet<string>
): boolean {
  return entry.citations.sourceParentActionReferences.every((reference) =>
    selectedParentActionReferenceIds.has(reference.actionReferenceId)
  );
}

function isFreshForContext(entry: LocalAiDerivedKnowledgeEntry, asOf: string): boolean {
  return entry.expiresAt === null || String(entry.expiresAt) > asOf;
}

function isUsableDerivedKnowledgeEntry(
  entry: LocalAiDerivedKnowledgeEntry,
  input: LocalAiDerivedKnowledgeUsabilityInput,
  sets: LocalAiDerivedKnowledgeGroundingSets
): boolean {
  return (
    entry.entryStatus === 'usable' &&
    isFreshForContext(entry, String(input.asOf)) &&
    hasSelectedEvidenceCitations(entry, sets.evidenceReferenceIds) &&
    hasSelectedPolicyCitations(entry, sets.policyVersions) &&
    hasSelectedParentActionCitations(entry, sets.parentActionReferenceIds)
  );
}

export function selectUsableDerivedKnowledgeEntries(input: unknown): LocalAiDerivedKnowledgeEntry[] {
  const parsed = LocalAiDerivedKnowledgeUsabilityInputSchema.parse(input);
  const sets = groundingSets(parsed);
  return parsed.entries.filter((entry) => isUsableDerivedKnowledgeEntry(entry, parsed, sets));
}
