/* generated from crates/logging-core/src/stack_trace_runtime.rs */

import { decodeGeneratedStackFilePath, fileNameFromGeneratedPath } from './stack-trace-runtime-path';
import type { GeneratedStackFrame } from './stack-trace-runtime';

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
