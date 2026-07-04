/* generated from crates/logging-core/src/stack_trace_runtime.rs */

import type { GeneratedStackFrame } from './stack-trace-runtime';

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
