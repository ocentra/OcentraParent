/* generated from crates/logging-core/src/stack_trace_runtime.rs */

export interface GeneratedStackFrame {
  readonly functionName: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
}

export function normalizeGeneratedStackPath(value: string): string {
  return value.replaceAll('\\', '/');
}

function generatedPercentDecode(value: string): string {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
}

function generatedTrimWindowsFileUrlPrefix(pathname: string): string {
  if (pathname.length >= 3 && pathname[0] === '/' && /[A-Za-z]/.test(pathname[1] ?? '') && pathname[2] === ':') {
    return pathname.slice(1);
  }
  return pathname;
}

export function decodeGeneratedStackFilePath(value: string): string {
  if (value.startsWith('file://')) {
    return normalizeGeneratedStackPath(generatedTrimWindowsFileUrlPrefix(generatedPercentDecode(value.slice('file://'.length))));
  }
  return normalizeGeneratedStackPath(value);
}

export function fileNameFromGeneratedPath(filePath: string | null): string | null {
  if (filePath == null) {
    return null;
  }
  const normalized = normalizeGeneratedStackPath(filePath);
  const lastSlash = normalized.lastIndexOf('/');
  return lastSlash >= 0 ? normalized.slice(lastSlash + 1) : normalized;
}

export function moduleNameFromGeneratedPath(filePath: string): string {
  const fileName = (fileNameFromGeneratedPath(filePath) ?? '').replace(/\.[^.]+$/, '');
  return fileName
    .split(/[^a-zA-Z0-9]+/)
    .filter((segment) => segment.length > 0)
    .map((segment) => segment.slice(0, 1).toUpperCase() + segment.slice(1))
    .join('');
}

function parseGeneratedIntegerSegment(value: string, start: number, end: number): number | null {
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

function parseGeneratedLocationWithoutLineInfo(location: string): Omit<GeneratedStackFrame, 'functionName'> {
  const filePath = decodeGeneratedStackFilePath(location);
  return {
    file: fileNameFromGeneratedPath(filePath),
    filePath,
    line: null,
    column: null,
  };
}

function parseGeneratedLocation(location: string): Omit<GeneratedStackFrame, 'functionName'> {
  const trimmed = location.trim();
  const columnSeparator = trimmed.lastIndexOf(':');
  if (columnSeparator < 0) {
    return parseGeneratedLocationWithoutLineInfo(trimmed);
  }

  const lineSeparator = trimmed.lastIndexOf(':', columnSeparator - 1);
  if (lineSeparator < 0) {
    return parseGeneratedLocationWithoutLineInfo(trimmed);
  }

  const line = parseGeneratedIntegerSegment(trimmed, lineSeparator + 1, columnSeparator);
  const column = parseGeneratedIntegerSegment(trimmed, columnSeparator + 1, trimmed.length);
  if (line == null || column == null) {
    return parseGeneratedLocationWithoutLineInfo(trimmed);
  }

  const filePath = decodeGeneratedStackFilePath(trimmed.slice(0, lineSeparator));
  return {
    file: fileNameFromGeneratedPath(filePath),
    filePath,
    line,
    column,
  };
}

function parseGeneratedFrameLine(line: string): GeneratedStackFrame | null {
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
      ...parseGeneratedLocation(location),
    };
  }

  return {
    functionName: null,
    ...parseGeneratedLocation(body),
  };
}

export function parseGeneratedStackTrace(stackTrace: string): GeneratedStackFrame[] {
  return String(stackTrace)
    .split('\n')
    .map((line) => parseGeneratedFrameLine(line.endsWith('\r') ? line.slice(0, -1) : line))
    .filter((frame): frame is GeneratedStackFrame => frame != null);
}

export function resolveGeneratedLoggerContext(
  moduleName: string,
  frame: GeneratedStackFrame | null,
  moduleContextSuffix: string
): string {
  if (frame?.functionName != null && frame.functionName.trim().length > 0) {
    return frame.functionName.includes('.') ? frame.functionName : `${moduleName}.${frame.functionName}`;
  }
  return `${moduleName}.${moduleContextSuffix}`;
}

export function resolveGeneratedLoggerSource(moduleName: string, frame: GeneratedStackFrame | null): string {
  if (frame?.functionName != null && frame.functionName.includes('.')) {
    return frame.functionName.split('.')[0] ?? moduleName;
  }
  return moduleName;
}
