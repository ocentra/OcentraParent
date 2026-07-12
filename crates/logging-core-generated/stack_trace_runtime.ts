/* generated from crates/logging-core/src/stack_trace_runtime.rs */

import {
  decodeGeneratedStackFilePath as decodeGeneratedStackFilePathImpl,
  fileNameFromGeneratedPath as fileNameFromGeneratedPathImpl,
  moduleNameFromGeneratedPath as moduleNameFromGeneratedPathImpl,
  normalizeGeneratedStackPath as normalizeGeneratedStackPathImpl,
} from './stack-trace-runtime-path';
import { parseGeneratedStackTrace as parseGeneratedStackTraceImpl } from './stack-trace-runtime-parse';
import {
  resolveGeneratedLoggerContext as resolveGeneratedLoggerContextImpl,
  resolveGeneratedLoggerSource as resolveGeneratedLoggerSourceImpl,
} from './stack-trace-runtime-source';

export interface GeneratedStackFrame {
  readonly functionName: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
}

export const normalizeGeneratedStackPath = normalizeGeneratedStackPathImpl;
export const decodeGeneratedStackFilePath = decodeGeneratedStackFilePathImpl;
export const fileNameFromGeneratedPath = fileNameFromGeneratedPathImpl;
export const moduleNameFromGeneratedPath = moduleNameFromGeneratedPathImpl;
export const parseGeneratedStackTrace = parseGeneratedStackTraceImpl;
export const resolveGeneratedLoggerContext = resolveGeneratedLoggerContextImpl;
export const resolveGeneratedLoggerSource = resolveGeneratedLoggerSourceImpl;
