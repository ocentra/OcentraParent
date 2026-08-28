import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentSetupFirstRunPanelSnapshot } from '../../generated/parent-ui-bridge';
import { SetupFirstRunRoutePanel, shouldRenderSetupFirstRunRoute } from '../../src/SetupFirstRunRoutePanel';

const sampleSetupFirstRunPanelValue: ParentSetupFirstRunPanelSnapshot = {
  eyebrow: 'Setup route',
  title: 'Setup-first-run boundary status',
  body: 'The Start route exists, but live setup-first-run runtime state is not yet wired into the Rust parent snapshot. This panel reports that gap honestly instead of inventing onboarding progress.',
  summaryCardTitle: 'Current boundary status',
  summary:
    'Portal rendering and the Rust-owned route snapshot exist, but live setup/account/trust/custody state is unavailable here today.',
  summaryDetails: [
    { label: 'Route', value: 'start' },
    {
      label: 'Runtime state',
      value: 'unavailable',
    },
    {
      label: 'Snapshot owner',
      value: 'Rust parent runtime host bridge',
    },
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
        {
          label: 'Route shell',
          value: 'Start route is visible in the portal shell',
        },
        {
          label: 'Snapshot transport',
          value: 'Host bridge snapshot reaches TS presentation',
        },
        {
          label: 'Evidence boundary',
          value: 'Route-contract projection only',
        },
      ],
    },
    {
      title: 'What is missing',
      summary:
        'No live setup-first-run read model is wired here yet, so the panel must stay explicit about the missing runtime state.',
      details: [
        {
          label: 'Account/provider state',
          value: 'not wired',
        },
        {
          label: 'Pairing/trust state',
          value: 'not wired',
        },
        {
          label: 'Data-custody/readiness state',
          value: 'not wired',
        },
        {
          label: 'Completion claim',
          value: 'withheld until a live Rust snapshot exists',
        },
      ],
    },
    {
      title: 'Where it belongs',
      summary: 'When first-run becomes live, Rust must own the setup state and TS must remain pure rendering.',
      details: [
        {
          label: 'Rust owner',
          value: 'parent runtime + setup read model',
        },
        {
          label: 'TS role',
          value: 'presentation only',
        },
        {
          label: 'Proof rule',
          value: 'claim only what the live Rust snapshot can prove',
        },
      ],
    },
  ],
  productClaim:
    'This panel reports only whether the Start route has a live Rust-owned setup-first-run snapshot. It does not claim live account readiness, signed installer readiness, pairing trust, data-custody execution, or onboarding completion.',
};

function sampleSetupFirstRunPanel(): ParentSetupFirstRunPanelSnapshot {
  return sampleSetupFirstRunPanelValue;
}

describe('setup first-run portal route panel', () => {
  it('attaches only to the start route', () => {
    expect(shouldRenderSetupFirstRunRoute(ParentRoute.Start)).toBe(true);
    expect(shouldRenderSetupFirstRunRoute(ParentRoute.Overview)).toBe(false);
    expect(shouldRenderSetupFirstRunRoute(ParentRoute.Devices)).toBe(false);
  });

  it('renders an honest boundary-status panel instead of an invented setup state machine', () => {
    const markup = renderToStaticMarkup(createElement(SetupFirstRunRoutePanel, { panel: sampleSetupFirstRunPanel() }));

    expect(markup).toContain('Setup-first-run boundary status');
    expect(markup).toContain('data-ocentra-setup-proof="first-run-route"');
    expect(markup).toContain('Current boundary status');
    expect(markup).toContain('What is real now');
    expect(markup).toContain('What is missing');
    expect(markup).toContain('Where it belongs');
    expect(markup).toContain('unavailable');
    expect(markup).toContain('Start route is visible in the portal shell');
    expect(markup).toContain('Host bridge snapshot reaches TS presentation');
    expect(markup).toContain('Route-contract projection only');
    expect(markup).toContain('Account/provider state');
    expect(markup).toContain('Pairing/trust state');
    expect(markup).toContain('Data-custody/readiness state');
    expect(markup).toContain('withheld until a live Rust snapshot exists');
    expect(markup).toContain('parent runtime + setup read model');
    expect(markup).toContain('presentation only');
    expect(markup).toContain('claim only what the live Rust snapshot can prove');
    expect(markup).not.toContain('welcome-screen');
    expect(markup).not.toContain('setup-complete-screen');
    expect(markup).not.toContain('parent-desktop-runtime-package-plan');
  });

  it('renders manual-required authority and source boundaries supplied by the Rust snapshot', () => {
    const panel: ParentSetupFirstRunPanelSnapshot = {
      ...sampleSetupFirstRunPanelValue,
      summaryDetails: [
        ...sampleSetupFirstRunPanelValue.summaryDetails,
        { label: 'Setup state', value: 'manual-required' },
      ],
      cards: [
        {
          ...sampleSetupFirstRunPanelValue.cards[0]!,
          details: [
            ...sampleSetupFirstRunPanelValue.cards[0]!.details,
            { label: 'LAN authority', value: 'observation only; ownership and trust remain unavailable' },
          ],
        },
        {
          ...sampleSetupFirstRunPanelValue.cards[1]!,
          details: [
            ...sampleSetupFirstRunPanelValue.cards[1]!.details,
            { label: 'Account identity', value: 'manual-required' },
          ],
        },
        {
          ...sampleSetupFirstRunPanelValue.cards[2]!,
          details: [
            ...sampleSetupFirstRunPanelValue.cards[2]!.details,
            { label: 'Degraded/manual state', value: 'manual-required' },
          ],
        },
      ],
    };
    const markup = renderToStaticMarkup(createElement(SetupFirstRunRoutePanel, { panel }));

    expect(markup).toContain('Setup state');
    expect(markup).toContain('manual-required');
    expect(markup).toContain('LAN authority');
    expect(markup).toContain('observation only; ownership and trust remain unavailable');
    expect(markup).toContain('Account identity');
    expect(markup).toContain('Degraded/manual state');
    expect(markup).not.toContain('onboarding complete');
  });

  it('renders an unavailable panel when the Rust snapshot is missing', () => {
    const markup = renderToStaticMarkup(createElement(SetupFirstRunRoutePanel, { panel: null }));

    expect(markup).toContain('Start route unavailable');
    expect(markup).toContain('Parent Rust snapshot unavailable for the setup-first-run route.');
    expect(markup).not.toContain('Setup-first-run boundary status');
    expect(markup).not.toContain('Current boundary status');
  });
});
