import type { ParsedBrowserUrl } from './browser_url_intelligence';
import {
  firstSuffixIndex,
  normalizedAuthority,
  pathFromSuffix,
  queryFromSuffix,
} from './browser_url_intelligence_url_text';

export function parseUrl(value: string | null): ParsedBrowserUrl | null {
  if (value === null) {
    return null;
  }
  const separatorIndex = value.indexOf('://');
  if (separatorIndex <= 0) {
    return null;
  }
  const scheme = value.slice(0, separatorIndex).toLowerCase();
  if (scheme !== 'http' && scheme !== 'https') {
    return null;
  }
  const remainder = value.slice(separatorIndex + 3);
  const authorityEnd = firstSuffixIndex(remainder);
  const authority = authorityEnd === null ? remainder : remainder.slice(0, authorityEnd);
  if (authority.length === 0 || authority.includes('@')) {
    return null;
  }
  const normalized = normalizedAuthority(authority);
  if (normalized === null) {
    return null;
  }
  const suffix = authorityEnd === null ? '' : remainder.slice(authorityEnd);
  return {
    normalizedUrl: `${scheme}://${normalized.authority}${suffix}`,
    domain: normalized.domain,
    path: pathFromSuffix(suffix),
    query: queryFromSuffix(suffix),
  };
}
