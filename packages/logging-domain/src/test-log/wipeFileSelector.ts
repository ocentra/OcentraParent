import path from 'node:path';
import { utf8Bytes } from '../core/logTextCustody';

const MaximumWipeFileSelectorBytes = 4_096;
const InvalidWindowsPathCharacterPattern = /[<>:"|?*]/u;
const WindowsReservedPathSegmentPattern = /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/iu;

function invalidSelector(): never {
  throw new Error('wipe file selector must be an exact normalized relative path');
}

export function normalizeWipeFileSelector(value: string): string {
  if (value.length > MaximumWipeFileSelectorBytes) {
    return invalidSelector();
  }
  const trimmed = value.trim();
  const slashed = trimmed.replace(/\\/gu, '/');
  const normalized = path.posix.normalize(slashed);
  const segments = normalized.split('/');
  if (invalidSelectorShape(value, trimmed, slashed, normalized) || segments.some(invalidSelectorSegment)) {
    invalidSelector();
  }
  return process.platform === 'win32' ? normalized.toLowerCase() : normalized;
}

function invalidSelectorShape(value: string, trimmed: string, slashed: string, normalized: string): boolean {
  return (
    trimmed.length === 0 ||
    trimmed !== value ||
    utf8Bytes(trimmed) > MaximumWipeFileSelectorBytes ||
    normalized !== slashed ||
    path.posix.isAbsolute(normalized) ||
    /^[A-Za-z]:/u.test(normalized) ||
    normalized === '.' ||
    normalized === '..' ||
    normalized.startsWith('../')
  );
}

function invalidSelectorSegment(segment: string): boolean {
  return (
    segment.length === 0 ||
    segment === '.' ||
    segment === '..' ||
    segment.endsWith('.') ||
    segment.endsWith(' ') ||
    InvalidWindowsPathCharacterPattern.test(segment) ||
    WindowsReservedPathSegmentPattern.test(segment) ||
    [...segment].some((character) => character.charCodeAt(0) <= 0x1f)
  );
}
