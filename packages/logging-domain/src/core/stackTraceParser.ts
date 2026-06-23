import type { StackTrace } from '@ocentra-parent/schema-domain/logging-contracts';

export interface StackFrame {
  readonly functionName: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
}

function normalizePath(value: string): string {
  return value.replaceAll('\\', '/');
}

function isAsciiLetter(value: string): boolean {
  if (value.length === 0) {
    return false;
  }
  const code = value.charCodeAt(0);
  return (code >= 65 && code <= 90) || (code >= 97 && code <= 122);
}

function trimWindowsFileUrlPrefix(pathname: string): string {
  if (pathname.length >= 3 && pathname[0] === '/' && isAsciiLetter(pathname[1] ?? '') && pathname[2] === ':') {
    return pathname.slice(1);
  }
  return pathname;
}

function parseLocationWithoutLineInfo(location: string): Omit<StackFrame, 'functionName'> {
  const filePath = decodeFilePath(location);
  return {
    file: fileNameFromPath(filePath),
    filePath,
    line: null,
    column: null,
  };
}

function parseIntegerSegment(value: string, start: number, end: number): number | null {
  if (start >= end) {
    return null;
  }

  let parsed = 0;
  for (let index = start; index < end; index += 1) {
    const digit = value.charCodeAt(index) - 48;
    if (digit < 0 || digit > 9) {
      return null;
    }
    parsed = parsed * 10 + digit;
  }

  return parsed;
}

function decodeFilePath(value: string): string {
  if (value.startsWith('file://')) {
    const url = new URL(value);
    return normalizePath(trimWindowsFileUrlPrefix(decodeURIComponent(url.pathname)));
  }
  return normalizePath(value);
}

function fileNameFromPath(filePath: string | null): string | null {
  if (filePath == null) {
    return null;
  }
  const normalized = normalizePath(filePath);
  const lastSlash = normalized.lastIndexOf('/');
  return lastSlash >= 0 ? normalized.slice(lastSlash + 1) : normalized;
}

function parseLocation(location: string): Omit<StackFrame, 'functionName'> {
  const trimmed = location.trim();
  const columnSeparator = trimmed.lastIndexOf(':');
  if (columnSeparator < 0) {
    return parseLocationWithoutLineInfo(trimmed);
  }

  const lineSeparator = trimmed.lastIndexOf(':', columnSeparator - 1);
  if (lineSeparator < 0) {
    return parseLocationWithoutLineInfo(trimmed);
  }

  const line = parseIntegerSegment(trimmed, lineSeparator + 1, columnSeparator);
  const column = parseIntegerSegment(trimmed, columnSeparator + 1, trimmed.length);
  if (line == null || column == null) {
    return parseLocationWithoutLineInfo(trimmed);
  }

  const filePath = decodeFilePath(trimmed.slice(0, lineSeparator));
  return {
    file: fileNameFromPath(filePath),
    filePath,
    line,
    column,
  };
}

function parseFrameLine(line: string): StackFrame | null {
  const trimmed = line.trim();
  if (!trimmed.startsWith('at ')) {
    return null;
  }

  const body = trimmed.slice(3);
  const locationOpen = body.indexOf(' (');
  if (locationOpen >= 0 && body.endsWith(')')) {
    const functionName = body.slice(0, locationOpen).trim();
    const location = body.slice(locationOpen + 2, body.length - 1);
    return {
      functionName: functionName.length > 0 ? functionName : null,
      ...parseLocation(location),
    };
  }

  return {
    functionName: null,
    ...parseLocation(body),
  };
}

export function parseStackTrace(stackTrace: StackTrace): StackFrame[] {
  return String(stackTrace)
    .split('\n')
    .map((line) => parseFrameLine(line.endsWith('\r') ? line.slice(0, -1) : line))
    .filter((frame): frame is StackFrame => frame != null);
}
