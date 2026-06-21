import {
  decodeStackTrace,
  type StackTrace as SharedStackTrace,
} from '@ocentra-parent/schema-domain/logging-contracts';

export type StackTrace = SharedStackTrace;

export function getStackTrace(): StackTrace {
  return decodeStackTrace(new Error().stack ?? '');
}
