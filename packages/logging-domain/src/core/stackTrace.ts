import type { GeneratedStackTrace as StackTrace } from '@ocentra-parent/schema-domain/generated/logging-contracts';

export function getStackTrace(): StackTrace {
  return new Error().stack ?? '';
}
