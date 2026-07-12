/* generic helper for catalog metadata text matching */

export type PatternValue<T> = readonly [pattern: RegExp, value: T];

export function matchPatternValue<T>(searchable: string, patterns: readonly PatternValue<T>[], fallback: T): T {
  for (const [pattern, value] of patterns) {
    if (pattern.test(searchable)) {
      return value;
    }
  }
  return fallback;
}

export function matchOptionalPatternValue<T>(searchable: string, patterns: readonly PatternValue<T>[]): T | null {
  return matchPatternValue(searchable, patterns, null);
}

export function splitOptionLabels(candidate: string, delimiters: RegExp, maxItems: number): string[] {
  return candidate
    .replace(/\.$/u, '')
    .split(delimiters)
    .map((part) => part.trim())
    .filter((part) => part.length > 1 && part.length < 80)
    .slice(0, maxItems);
}

export function slugToken(input: string, fallback = 'item'): string {
  let slug = '';
  let previousDash = false;
  for (const character of input.toLowerCase()) {
    const isAsciiLowercaseAlphaNumeric =
      (character >= 'a' && character <= 'z') || (character >= '0' && character <= '9');
    if (isAsciiLowercaseAlphaNumeric) {
      slug += character;
      previousDash = false;
      continue;
    }
    if (slug === '' || previousDash) {
      continue;
    }
    slug += '-';
    previousDash = true;
  }
  if (previousDash) {
    slug = slug.slice(0, -1);
  }
  return slug || fallback;
}

export function titleFromToken(input: string): string {
  return input
    .split('-')
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

export function titleizeWords(value: string): string {
  return value
    .split(/[\s-]+/u)
    .filter((part) => part.length > 0)
    .map((part) => `${part.charAt(0).toUpperCase()}${part.slice(1)}`)
    .join(' ');
}

export function lowerFirst(value: string): string {
  return `${value.charAt(0).toLowerCase()}${value.slice(1)}`;
}

export function uniqueStrings(values: readonly string[]): string[] {
  return [...new Set(values)];
}
