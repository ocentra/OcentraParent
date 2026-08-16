import { PortalDevTextToken, resolvePortalDevText } from './display-text';

export function formatSummaryText(values: readonly (string | null | undefined)[]): string {
  return optionalSummaryText(values) ?? notReportedText();
}

export function uniqueStrings(values: readonly string[]): readonly string[] {
  return [...new Set(values)];
}

export function compareIsoDesc(left: string, right: string): number {
  return right.localeCompare(left);
}

function optionalSummaryText(values: readonly (string | null | undefined)[]): string | null {
  const normalized = uniqueStrings(
    values.filter((value): value is string => value !== null && value !== undefined && value.length > 0)
  );
  return normalized.length === 0 ? null : normalized.join(' | ');
}

function notReportedText(): string {
  return resolvePortalDevText(PortalDevTextToken.NotReported);
}
