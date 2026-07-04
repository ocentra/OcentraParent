import type { AppControlCatalogOptionSeed } from './app-control-catalog-data';
import { slugToken } from './app-control-catalog-string';

export function guidePolicyLaneFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/^Capability matrix row \|/u.test(sourceText)) {
    return 'evidence';
  }
  if (/time|duration|budget|schedule|timer|foreground today|grace/iu.test(searchable)) {
    return 'schedule';
  }
  if (/approval|ask parent|approve|deny|extend|unanswered/iu.test(searchable)) {
    return 'approvals';
  }
  if (/audit|retention|custody|report|redact|storage|journal|visible|summary/iu.test(searchable)) {
    return /report|summary|visible|parent sees/iu.test(searchable) ? 'reports' : 'audit';
  }
  if (
    /install|uninstall|setup|mdm|managed-device|device owner|entitlement|supervised|custody model/iu.test(searchable)
  ) {
    return 'setup';
  }
  if (/enforce|block|shield|suspend|hide|terminate|strict action|adapter result|rollback|launch/iu.test(searchable)) {
    return 'enforcement';
  }
  if (
    /evidence|inventory|process|window|foreground|package|identity|category|session|running|unknown|proof/iu.test(
      searchable
    )
  ) {
    return 'evidence';
  }
  return 'rules';
}

export function matrixOptionSeedsFromSourceText(sourceText: string): readonly AppControlCatalogOptionSeed[] {
  if (!/^Capability matrix row \|/u.test(sourceText)) {
    return [];
  }
  return sourceText
    .split(' | ')
    .slice(1)
    .map((part) => {
      const separatorIndex = part.indexOf('=');
      const heading = separatorIndex === -1 ? 'Cell' : part.slice(0, separatorIndex);
      const value = separatorIndex === -1 ? part : part.slice(separatorIndex + 1);
      return {
        value: `matrix-${slugToken(heading)}`,
        label: `${heading}: ${value}`,
        meaning: `Capability matrix answer for ${heading}.`,
      };
    });
}
