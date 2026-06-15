import type { StackTrace } from './stackTrace';

export interface StackFrame {
  readonly functionName: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
}

function normalizePath(value: string): string {
  return value.replace(/\\/g, '/');
}

function decodeFilePath(value: string): string {
  if (value.startsWith('file://')) {
    const url = new URL(value);
    return normalizePath(
      decodeURIComponent(url.pathname).replace(/^\/([A-Za-z]:)/, '$1')
    );
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
  const match = /^(.*):(\d+):(\d+)$/.exec(trimmed);
  if (match == null) {
    const filePath = decodeFilePath(trimmed);
    return {
      file: fileNameFromPath(filePath),
      filePath,
      line: null,
      column: null,
    };
  }

  const rawFilePath = match[1];
  const rawLine = match[2];
  const rawColumn = match[3];
  if (rawFilePath == null || rawLine == null || rawColumn == null) {
    const filePath = decodeFilePath(trimmed);
    return {
      file: fileNameFromPath(filePath),
      filePath,
      line: null,
      column: null,
    };
  }
  const filePath = decodeFilePath(rawFilePath);
  return {
    file: fileNameFromPath(filePath),
    filePath,
    line: Number(rawLine),
    column: Number(rawColumn),
  };
}

function parseFrameLine(line: string): StackFrame | null {
  const trimmed = line.trim();
  if (!trimmed.startsWith('at ')) {
    return null;
  }

  const body = trimmed.slice(3);
  const withFunctionMatch = /^(.*?) \((.*)\)$/.exec(body);
  if (withFunctionMatch != null) {
    const functionName = withFunctionMatch[1];
    const location = withFunctionMatch[2];
    const normalizedFunctionName =
      functionName != null && functionName.trim().length > 0 ? functionName.trim() : null;
    return {
      functionName: normalizedFunctionName,
      ...parseLocation(location ?? body),
    };
  }

  return {
    functionName: null,
    ...parseLocation(body),
  };
}

export function parseStackTrace(stackTrace: StackTrace): StackFrame[] {
  return String(stackTrace)
    .split(/\r?\n/)
    .map(parseFrameLine)
    .filter((frame): frame is StackFrame => frame != null);
}
