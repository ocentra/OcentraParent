import { describe, expect, it } from 'vitest';
import { createSetupFirstRunPanelIntent, readableSetupValue } from '../../src/setup-first-run-panel';

type ParentSetupFirstRunPanelSnapshot = Parameters<typeof createSetupFirstRunPanelIntent>[0];

describe('setup first-run panel intent', () => {
  it('projects the Rust-owned setup snapshot without inventing local state', () => {
    const intent = createSetupFirstRunPanelIntent(sampleSetupFirstRunPanel());

    expect(intent).toEqual(
      expect.objectContaining({
        eyebrow: 'Setup route',
        title: 'Setup-first-run boundary status',
        body: 'The Start route exists, but live setup-first-run runtime state is not yet wired into the Rust parent snapshot. This panel reports that gap honestly instead of inventing onboarding progress.',
        summaryCardTitle: 'Current boundary status',
        summary:
          'Portal rendering and the Rust-owned route snapshot exist, but live setup/account/trust/custody state is unavailable here today.',
        productClaim:
          'This panel reports only whether the Start route has a live Rust-owned setup-first-run snapshot. It does not claim live account readiness, signed installer readiness, pairing trust, data-custody execution, or onboarding completion.',
      })
    );

    expect(intent.summaryDetails).toEqual([
      { label: 'Route', value: 'start' },
      { label: 'Runtime state', value: 'unavailable' },
      { label: 'Snapshot owner', value: 'Rust parent runtime host bridge' },
      {
        label: 'Product claim',
        value:
          'This panel reports only whether the Start route has a live Rust-owned setup-first-run snapshot. It does not claim live account readiness, signed installer readiness, pairing trust, data-custody execution, or onboarding completion.',
      },
    ]);

    expect(intent.cards).toEqual([
      {
        title: 'What is real now',
        summary: 'The Start route can render an honest Rust-owned boundary panel without inventing setup progress.',
        details: [
          { label: 'Route shell', value: 'Start route is visible in the portal shell' },
          { label: 'Snapshot transport', value: 'Host bridge snapshot reaches TS presentation' },
          { label: 'Evidence boundary', value: 'Route-contract projection only' },
        ],
      },
      {
        title: 'What is missing',
        summary:
          'No live setup-first-run read model is wired here yet, so the panel must stay explicit about the missing runtime state.',
        details: [
          { label: 'Account/provider state', value: 'not wired' },
          { label: 'Pairing/trust state', value: 'not wired' },
          { label: 'Data-custody/readiness state', value: 'not wired' },
          { label: 'Completion claim', value: 'withheld until a live Rust snapshot exists' },
        ],
      },
      {
        title: 'Where it belongs',
        summary: 'When first-run becomes live, Rust must own the setup state and TS must remain pure rendering.',
        details: [
          { label: 'Rust owner', value: 'parent runtime + setup read model' },
          { label: 'TS role', value: 'presentation only' },
          { label: 'Proof rule', value: 'claim only what the live Rust snapshot can prove' },
        ],
      },
    ]);
  });

  it('reuses shared readable portal labels where they already exist', () => {
    expect(readableSetupValue('manual-required')).toBe('Manual required');
    expect(readableSetupValue('live-local')).toBe('Live local');
    expect(readableSetupValue('productionReady')).toBe('ProductionReady');
  });
});

function sampleSetupFirstRunPanel(): ParentSetupFirstRunPanelSnapshot {
  return {
    eyebrow: 'Setup route',
    title: 'Setup-first-run boundary status',
    body: 'The Start route exists, but live setup-first-run runtime state is not yet wired into the Rust parent snapshot. This panel reports that gap honestly instead of inventing onboarding progress.',
    summaryCardTitle: 'Current boundary status',
    summary:
      'Portal rendering and the Rust-owned route snapshot exist, but live setup/account/trust/custody state is unavailable here today.',
    summaryDetails: [
      { label: 'Route', value: 'start' },
      { label: 'Runtime state', value: 'unavailable' },
      { label: 'Snapshot owner', value: 'Rust parent runtime host bridge' },
      {
        label: 'Product claim',
        value:
          'This panel reports only whether the Start route has a live Rust-owned setup-first-run snapshot. It does not claim live account readiness, signed installer readiness, pairing trust, data-custody execution, or onboarding completion.',
      },
    ],
    cards: [
      {
        title: 'What is real now',
        summary: 'The Start route can render an honest Rust-owned boundary panel without inventing setup progress.',
        details: [
          { label: 'Route shell', value: 'Start route is visible in the portal shell' },
          { label: 'Snapshot transport', value: 'Host bridge snapshot reaches TS presentation' },
          { label: 'Evidence boundary', value: 'Route-contract projection only' },
        ],
      },
      {
        title: 'What is missing',
        summary:
          'No live setup-first-run read model is wired here yet, so the panel must stay explicit about the missing runtime state.',
        details: [
          { label: 'Account/provider state', value: 'not wired' },
          { label: 'Pairing/trust state', value: 'not wired' },
          { label: 'Data-custody/readiness state', value: 'not wired' },
          { label: 'Completion claim', value: 'withheld until a live Rust snapshot exists' },
        ],
      },
      {
        title: 'Where it belongs',
        summary: 'When first-run becomes live, Rust must own the setup state and TS must remain pure rendering.',
        details: [
          { label: 'Rust owner', value: 'parent runtime + setup read model' },
          { label: 'TS role', value: 'presentation only' },
          { label: 'Proof rule', value: 'claim only what the live Rust snapshot can prove' },
        ],
      },
    ],
    productClaim:
      'This panel reports only whether the Start route has a live Rust-owned setup-first-run snapshot. It does not claim live account readiness, signed installer readiness, pairing trust, data-custody execution, or onboarding completion.',
  };
}
