import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const generatedOrThinAdapterFiles = [
  'capabilities.ts',
  'capability-data.ts',
  'enforcement.ts',
  'eventing.ts',
  'evidence-kinds.ts',
  'family-reference-primitives.ts',
  'family-references.ts',
  'notification-local-outbox.ts',
  'notification-v3-provider-retry.ts',
  'social-alert-report-intent.ts',
  'social-alert-report-intent-values.ts',
  'social-alert-report-provider-preflight-proof.ts',
  'social-alert-report-provider-receipt-boundary-proof.ts',
  'social-alert-report-provider-status-handoff-proof.ts',
  'social-audit-explanation-read-model-values.ts',
  'social-dashboard-ux-values.ts',
  'social-policy-compiler-values.ts',
  'v0-8-notification-provider-status-boundary.ts',
] as const;

const genericHelperFiles = [
  'catalog-metadata-text.ts',
  'effect.ts',
  'event-primitives.ts',
  'evidence-primitives.ts',
  'literal-contracts.ts',
  'text-contracts.ts',
] as const;

function readFirstLine(relativePath: string): string {
  const source = readFileSync(new URL(`../../src/${relativePath}`, import.meta.url), 'utf8');
  return source.split(/\r?\n/u)[0] ?? '';
}

describe('schema-domain tail boundary', () => {
  it('marks the remaining tail as generated, thin adapter, or generic helper only', () => {
    for (const file of generatedOrThinAdapterFiles) {
      const firstLine = readFirstLine(file);
      expect(firstLine.startsWith('/* generated from ') || firstLine.startsWith('/* thin adapter over ')).toBe(true);
    }

    for (const file of genericHelperFiles) {
      const firstLine = readFirstLine(file);
      expect(firstLine.startsWith('/* generic helper for ')).toBe(true);
    }
  });
});
