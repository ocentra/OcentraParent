import type { GeneratedStackTrace as StackTrace } from '@ocentra-parent/schema-domain/generated/logging-contracts';
import {
  parseGeneratedStackTrace,
  type GeneratedStackFrame,
} from '../generated/stack-trace-runtime';

export type StackFrame = GeneratedStackFrame;

export function parseStackTrace(stackTrace: StackTrace): StackFrame[] {
  return parseGeneratedStackTrace(String(stackTrace));
}
