import type { ParsedBrowserUrl } from './browser_url_intelligence';

export function queryParam(parsed: ParsedBrowserUrl, key: string): string | null {
  if (parsed.query === null) {
    return null;
  }
  for (const part of parsed.query.split('&')) {
    const separatorIndex = part.indexOf('=');
    const rawKey = separatorIndex < 0 ? part : part.slice(0, separatorIndex);
    if (rawKey === key) {
      const value = separatorIndex < 0 ? '' : part.slice(separatorIndex + 1);
      return value.length === 0 ? null : value.replaceAll('+', ' ');
    }
  }
  return null;
}
