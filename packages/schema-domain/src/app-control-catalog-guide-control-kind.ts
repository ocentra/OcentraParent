import type { AppControlCatalogOptionSeed } from './app-control-catalog-data';
import type { AppControlKind } from './app-control-catalog-schema';
import { cleanOptionLabel } from './app-control-catalog-string';
import { matrixOptionSeedsFromSourceText } from './app-control-catalog-guide-policy';

export function guideControlKindFor(sectionTitle: string, groupTitle: string, sourceText: string): AppControlKind {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`.toLowerCase();
  if (/^Capability matrix row \|/u.test(sourceText)) {
    return 'read-only-status';
  }
  if (/minutes|seconds|hours|days|duration|budget|timer/u.test(searchable)) {
    return 'number';
  }
  if (/audit|retention|custody|delete|redact/u.test(searchable)) {
    return 'retention';
  }
  if (/actions|terminate|block|shield|suspend|hide|install|uninstall|launch/u.test(searchable)) {
    return 'action-list';
  }
  if (/targets|identity|category|package|bundle|process|window|unknown app/u.test(searchable)) {
    return 'multi-choice';
  }
  return guideOptionsFor(sourceText).length > 0 ? 'single-choice' : 'toggle';
}

export function guideOptionsFor(sourceText: string): readonly AppControlCatalogOptionSeed[] {
  const matrixOptions = matrixOptionSeedsFromSourceText(sourceText);
  if (matrixOptions.length > 0) {
    return matrixOptions;
  }
  const colonIndex = sourceText.indexOf(':');
  if (colonIndex === -1) {
    return [];
  }
  return sourceText
    .slice(colonIndex + 1)
    .replace(/\.$/u, '')
    .split(/,|;|\bor\b/iu)
    .map((part) => cleanOptionLabel(part))
    .filter((part) => part.length > 0);
}
