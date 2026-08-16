import type { ParsedBrowserUrl } from './browser_url_intelligence';

export function firstSuffixIndex(value: string): number | null {
  const indexes = ['/', '?', '#'].map((separator) => value.indexOf(separator)).filter((index) => index >= 0);
  return indexes.length === 0 ? null : Math.min(...indexes);
}

export function normalizedAuthority(value: string): { readonly authority: string; readonly domain: string } | null {
  const [host, port] = splitHostAndPort(value);
  const domain = normalizedHost(host);
  if (domain === null) {
    return null;
  }
  return {
    authority: port === null ? domain : `${domain}:${port}`,
    domain,
  };
}

function splitHostAndPort(value: string): readonly [string, string | null] {
  if (value.split(':').length - 1 === 1) {
    const separatorIndex = value.lastIndexOf(':');
    const host = value.slice(0, separatorIndex);
    const port = value.slice(separatorIndex + 1);
    if (host.length > 0 && /^[0-9]+$/.test(port)) {
      return [host, port];
    }
  }
  return [value, null];
}

function normalizedHost(value: string): string | null {
  const normalized = value.replace(/\.+$/, '').toLowerCase();
  if (normalized.length === 0 || normalized.includes('/')) {
    return null;
  }
  return normalized;
}

export function pathFromSuffix(value: string): string {
  if (!value.startsWith('/')) {
    return '/';
  }
  const queryIndex = value.indexOf('?');
  const hashIndex = value.indexOf('#');
  const endIndexes = [queryIndex, hashIndex].filter((index) => index >= 0);
  const endIndex = endIndexes.length === 0 ? value.length : Math.min(...endIndexes);
  return value.slice(0, endIndex);
}

export function queryFromSuffix(value: string): string | null {
  const queryStart = value.indexOf('?');
  if (queryStart < 0) {
    return null;
  }
  const hashIndex = value.indexOf('#', queryStart);
  return value.slice(queryStart + 1, hashIndex < 0 ? value.length : hashIndex);
}

export function pathSegments(parsed: ParsedBrowserUrl) {
  return parsed.path
    .split('/')
    .map((segment) => segment.trim())
    .filter(Boolean);
}

export function firstPathSegment(parsed: ParsedBrowserUrl) {
  return pathSegments(parsed)[0] ?? null;
}

export function domainMatchesAny(domain: string, bases: readonly string[]) {
  return bases.some((base) => domain === base || domain.endsWith(`.${base}`));
}

export function hasText(value: string | null): value is string {
  return value !== null && value.length > 0;
}
