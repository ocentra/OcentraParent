import { GeneratedStackTraceSchema, type GeneratedStackTrace as StackTrace } from '../generated-logging-contracts';

export function getStackTrace(): StackTrace {
  return GeneratedStackTraceSchema.parse(new Error().stack ?? '');
}
