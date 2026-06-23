import { decodeStackTrace, type StackTrace } from '@ocentra-parent/schema-domain/logging-contracts';

export function getStackTrace(): StackTrace {
  return decodeStackTrace(new Error().stack ?? '');
}
