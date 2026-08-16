/* generated from crates/network-core/src/network_control_catalog_metadata_text.ts.txt */

import { slugToken, splitOptionLabels, titleFromToken } from './catalog-metadata-text';

export function networkQuestionFromSourceText(sourceText: string, explicitQuestion: string | null): string {
  if (explicitQuestion !== null && explicitQuestion.length > 0) {
    return explicitQuestion;
  }
  const trimmed = sourceText.replace(/\.$/u, '');
  if (trimmed.endsWith('?')) {
    return trimmed;
  }
  if (/^Capability matrix row \|/u.test(trimmed)) {
    const capability = /Capability=([^|]+)/u.exec(trimmed)?.[1]?.trim() ?? 'network capability';
    return `Represent ${capability} capability status.`;
  }
  const colonIndex = trimmed.indexOf(':');
  return colonIndex === -1
    ? `Represent ${trimmed.charAt(0).toLowerCase()}${trimmed.slice(1)}?`
    : `Configure ${trimmed.slice(0, colonIndex).toLowerCase()}.`;
}

export function networkExplicitOptionLabels(sourceText: string): readonly string[] {
  if (/^Capability matrix row \|/u.test(sourceText)) {
    return sourceText
      .split(' | ')
      .slice(1)
      .map((part) => {
        const separatorIndex = part.indexOf('=');
        const heading = separatorIndex === -1 ? 'Cell' : part.slice(0, separatorIndex);
        const value = separatorIndex === -1 ? part : part.slice(separatorIndex + 1);
        return `${heading}: ${value}`;
      });
  }
  const colonIndex = sourceText.indexOf(':');
  return colonIndex === -1
    ? []
    : splitOptionLabels(sourceText.slice(colonIndex + 1), /,|;|\bor\b/iu, 16).map(networkCleanOptionLabel);
}

function networkCleanOptionLabel(value: string): string {
  return titleFromToken(value.trim().replace(/\.$/u, '').replace(/\s+/gu, '-'));
}

export const networkSlugToken = slugToken;
export const networkTitleFromToken = titleFromToken;
