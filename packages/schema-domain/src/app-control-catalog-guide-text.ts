import {
  guideCapabilityRequirementFor,
  guideProofRequirementFor,
} from './app-control-catalog-guide-requirements';

export function guideHelperTextFor(sectionTitle: string, groupTitle: string, sourceText: string): string {
  const proof = guideProofRequirementFor(sectionTitle, groupTitle, sourceText);
  return proof ?? guideCapabilityRequirementFor(sectionTitle, groupTitle, sourceText);
}

export function questionFromGuideText(sourceText: string): string {
  const trimmed = sourceText.replace(/\.$/u, '');
  if (trimmed.endsWith('?')) {
    return trimmed;
  }
  const colonIndex = trimmed.indexOf(':');
  if (colonIndex !== -1) {
    return `Configure ${trimmed.slice(0, colonIndex).toLowerCase()}.`;
  }
  return `Represent ${trimmed.charAt(0).toLowerCase()}${trimmed.slice(1)}?`;
}
