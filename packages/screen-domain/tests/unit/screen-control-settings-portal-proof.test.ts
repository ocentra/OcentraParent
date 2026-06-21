import { describe, expect, it } from 'vitest';

import {
  ScreenControlSettingsPortalProofSchema,
  screenControlSettingsPortalProof,
} from '@ocentra-parent/schema-domain/screen-control-settings-portal-proof';

describe('screen control settings portal proof', () => {
  it('summarizes the real Screen control catalog for parent settings rendering', () => {
    const proof = screenControlSettingsPortalProof();

    expect(ScreenControlSettingsPortalProofSchema.safeParse(proof).success).toBe(true);
    expect(proof.title).toBe('Screen settings and capability proof');
    expect(proof.metrics.map((metric) => [metric.label, metric.value])).toEqual([
      ['Catalog settings', '474'],
      ['Catalog tabs', '11'],
      ['Proof-required controls', '68'],
      ['Unavailable sensitive modes', '9'],
    ]);
    expect(proof.gates.map((gate) => [gate.status, gate.capabilityState, gate.runtimeOwner])).toEqual([
      ['unavailable', 'unavailable', 'parent-owned-storage'],
      ['unavailable', 'unavailable', 'portal-only'],
      ['unavailable', 'unavailable', 'parent-owned-storage'],
      ['proof-required', 'available', 'os-adapter'],
      ['needs-effect-wiring', 'available', 'local-ai-runtime'],
    ]);
    expect(proof.gates.map((gate) => gate.statusText)).toEqual([
      'unavailable / unavailable',
      'unavailable / unavailable',
      'unavailable / unavailable',
      'proof-required / available',
      'needs-effect-wiring / available',
    ]);
    expect(proof.gates.map((gate) => gate.sourceDocument)).toEqual([
      'docs/screen-evidence-analysis-schema-proposal.md',
      'docs/screen-evidence-analysis-schema-proposal.md',
      'docs/screen-evidence-analysis-schema-proposal.md',
      'docs/screen-evidence-analysis-schema-proposal.md',
      'docs/screen-evidence-analysis-capability-guide.md',
    ]);
  });
});
