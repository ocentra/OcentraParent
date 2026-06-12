import type { TrackingFixtureProofTier, TrackingFixtureState } from './tracking-fixture-coverage-proof';

export const TrackingFixtureCoverageRequiredStates = [
  'fresh',
  'stale',
  'offline',
  'permission-denied',
  'low-accuracy',
  'ambiguous-nearby-place',
  'exception-active',
  'parent-acknowledged',
  'child-check-in-requested',
  'temporary-live-expired',
  'missing-device',
  'retention-deleted',
  'remote-sync-disabled',
  'remote-ai-disabled',
] as const satisfies ReadonlyArray<TrackingFixtureState>;

export const TrackingFixtureCoverageStateExpectations = {
  fresh: {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json'],
    proofRequirement: 'Parent-visible current location fixture state exists with evidence refs.',
  },
  stale: {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json'],
    proofRequirement: 'Parent-visible stale location fixture state exists with stale reason refs.',
  },
  offline: {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json'],
    proofRequirement: 'Parent-visible offline device fixture state exists with device-status refs.',
  },
  'permission-denied': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json'],
    proofRequirement: 'Parent-visible permission-required fixture state exists without live location claims.',
  },
  'low-accuracy': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json'],
    proofRequirement: 'Parent-visible low-accuracy fixture state exists with accuracy warning refs.',
  },
  'ambiguous-nearby-place': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: [
      'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json',
      'output/tracking-plan-proof/20-google-places-and-poi-provider-adapter/proof.json',
    ],
    proofRequirement: 'Nearby-place ambiguity fixture and provider adapter mapping proof exist.',
  },
  'exception-active': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/'],
    proofRequirement: 'Parent exception fixture proof exists for alert modification/suppression boundaries.',
  },
  'parent-acknowledged': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/'],
    proofRequirement: 'Parent acknowledgement fixture proof exists with audit refs.',
  },
  'child-check-in-requested': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: [
      'output/tracking-plan-proof/18-child-check-in-and-safe-help-flow/',
      'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-check-in.png',
    ],
    proofRequirement: 'Child check-in fixture and hosted card proof exist without child-device delivery claims.',
  },
  'temporary-live-expired': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/28-temporary-live-tracking-mode/proof.json'],
    proofRequirement: 'Temporary live expired/auto-stop fixture proof exists without live runtime claims.',
  },
  'missing-device': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: ['output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json'],
    proofRequirement: 'Missing-device parent-visible fixture state exists without lost-mode runtime claims.',
  },
  'retention-deleted': {
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    artifactRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/14-retention-delete-proof.json',
    ],
    proofRequirement: 'Retention-deleted fixture/read-model proof exists with tombstone refs.',
  },
  'remote-sync-disabled': {
    currentProofTier: 'P0_CONTRACT',
    artifactRefs: ['output/tracking-plan-proof/07-retention-and-custody-model/'],
    proofRequirement: 'Remote sync disabled-by-default contract proof exists.',
  },
  'remote-ai-disabled': {
    currentProofTier: 'P0_CONTRACT',
    artifactRefs: ['output/tracking-plan-proof/24-ai-provider-routing/'],
    proofRequirement: 'Remote AI disabled-by-default provider routing proof exists.',
  },
} as const satisfies Record<
  TrackingFixtureState,
  {
    readonly currentProofTier: TrackingFixtureProofTier;
    readonly artifactRefs: readonly string[];
    readonly proofRequirement: string;
  }
>;
