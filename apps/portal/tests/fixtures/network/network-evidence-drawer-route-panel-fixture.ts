import type { ParentActivityNetworkFlowReadModelSnapshot } from '../../../generated/parent-ui-bridge';
import { resolveSnapshotLiveActivityState } from '../../../src/route-live-activity-state';
import { networkFlowReadModelSnapshot } from '../../live-activity/live-activity-state-test-support';

const browserNetworkReadModel = networkFlowReadModelSnapshot('browser-network-metadata.example.test');
const degradedNetworkFlowReadModel: ParentActivityNetworkFlowReadModelSnapshot = {
  ...browserNetworkReadModel,
  capabilityStatus: 'degraded',
  rows: browserNetworkReadModel.rows.map((row) => ({
    ...row,
    capabilityStatus: 'degraded',
  })),
};

export const NetworkEvidenceDrawerRoutePanelFixture = {
  liveActivity: resolveSnapshotLiveActivityState({
    browserInterventionReadModel: null,
    browserManagedStatus: null,
    lanAddDeviceReadModel: null,
    networkFlowReadModel: degradedNetworkFlowReadModel,
  }),
  networkEvidenceSummary: {
    aiAuditRef: 'ai-audit-1',
    policyDecisionRef: 'policy-decision-1',
    networkEvidenceGrade: 'gold',
    interventionResultRef: 'intervention-result-1',
  },
} as const;
