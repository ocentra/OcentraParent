import type { GeneratedStackTrace as StackTrace } from '../generated-logging-contracts';
import { parseGeneratedStackTrace, type GeneratedStackFrame } from '../stack-trace-runtime';

export type StackFrame = GeneratedStackFrame;

export function parseStackTrace(stackTrace: StackTrace): StackFrame[] {
  return parseGeneratedStackTrace(String(stackTrace));
}
