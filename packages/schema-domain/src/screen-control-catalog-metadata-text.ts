/* thin adapter helpers for Rust-seeded screen control catalog metadata */

import { lowerFirst, titleizeWords, uniqueStrings } from './catalog-metadata-text';
import type {
  ScreenControlCatalogControlKind,
  ScreenControlCatalogSourceKind,
} from './screen-control-catalog-schema';

const SourceQuestionPrefixes = {
  'update-command': 'Support',
  'capability-state-meaning': 'Show',
} as const satisfies Partial<Record<ScreenControlCatalogSourceKind, string>>;

export function screenQuestionFromSeed(
  sourceKind: ScreenControlCatalogSourceKind,
  sourceText: string,
  controlKind: ScreenControlCatalogControlKind
): string {
  if (sourceKind === 'authoring-field') {
    return sourceText.endsWith('?') ? sourceText : `${sourceText}?`;
  }
  const fixedPrefix = SourceQuestionPrefixes[sourceKind];
  if (fixedPrefix !== undefined) {
    return `${fixedPrefix} ${sourceText.split(':')[0]}${fixedPrefix === 'Support' ? '?' : ' capability state?'}`;
  }
  if (controlKind === 'read-only-status') {
    return `Represent ${lowerFirst(sourceText.replace(/\.$/u, ''))}.`;
  }
  const colonIndex = sourceText.indexOf(':');
  return colonIndex === -1
    ? `Use ${lowerFirst(sourceText.replace(/\.$/u, ''))}?`
    : `Choose ${lowerFirst(sourceText.slice(0, colonIndex))}.`;
}

export function screenExplicitOptionLabels(sourceText: string): string[] {
  const colonIndex = sourceText.indexOf(':');
  if (colonIndex === -1) {
    return [];
  }
  return uniqueStrings(
    sourceText
      .slice(colonIndex + 1)
      .replace(/\.$/u, '')
      .split(/,|;|\bor\b/u)
      .map((part) => titleizeWords(part.trim()))
      .filter((part) => part.length > 0 && part.length < 80)
  );
}
