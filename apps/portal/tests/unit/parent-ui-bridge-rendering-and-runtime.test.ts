import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';

const BridgeContractFile = 'generated/parent-ui-bridge.ts';
const ProductSnapshotRefreshFiles = [
  {
    file: 'src/AiRuntimeRoutePanel.tsx',
    forbidden: ['AgentCommand.LocalAiRuntimeStatusGet', 'AgentEvent.LocalAiRuntimeStatusReported'],
  },
  {
    file: 'src/PolicyPreviewRoutePanel.tsx',
    forbidden: ['AgentCommand.PolicyPreviewReadModelGet', 'AgentEvent.PolicyPreviewReadModelReported'],
  },
  {
    file: 'src/SocialAuditExplanationRoutePanel.tsx',
    forbidden: [
      'AgentCommand.BrowserSocialAuditExplanationReadModelGet',
      'AgentEvent.BrowserSocialAuditExplanationReadModelReported',
    ],
  },
  {
    file: 'src/SocialDashboardRoutePanel.tsx',
    forbidden: [
      'AgentCommand.BrowserSocialDashboardReadModelGet',
      'AgentEvent.BrowserSocialDashboardReadModelReported',
    ],
  },
  {
    file: 'src/SocialAlertReportRoutePanel.tsx',
    forbidden: [
      'AgentCommand.BrowserSocialAlertReportReadModelGet',
      'AgentEvent.BrowserSocialAlertReportReadModelReported',
      'AgentCommand.BrowserSocialParentNotificationDeliveryReadModelGet',
      'AgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported',
      'AgentCommand.BrowserSocialAlertReportParentSurfaceReadModelGet',
      'AgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported',
    ],
  },
  {
    file: 'src/TrackingStatusRoutePanel.tsx',
    forbidden: [
      'AgentCommand.ActivityTrackingReadModelGet',
      'AgentEvent.ActivityTrackingReadModelReported',
      'AgentCommand.ActivityTrackingRetentionSettingsWrite',
      'AgentEvent.ActivityTrackingRetentionSettingsWriteRequested',
    ],
  },
  {
    file: 'src/AppGamePolicyReadinessRoutePanel.tsx',
    forbidden: [
      'AgentCommand.ActivityAppGamePolicyReadinessReadModelGet',
      'AgentEvent.ActivityAppGamePolicyReadinessReadModelReported',
    ],
  },
  {
    file: 'src/AppGamePlatformProofStatusRoutePanel.tsx',
    forbidden: [
      'AgentCommand.ActivityAppGamePlatformProofStatusReadModelGet',
      'AgentEvent.ActivityAppGamePlatformProofStatusReadModelReported',
    ],
  },
  {
    file: 'src/AppGameChildRuntimeTransportReceiptRoutePanel.tsx',
    forbidden: [
      'AgentCommand.ActivityAppGameChildRuntimeTransportReceiptReadModelGet',
      'AgentEvent.ActivityAppGameChildRuntimeTransportReceiptReadModelReported',
    ],
  },
  {
    file: 'src/AppGameAdapterDispatchRoutePanel.tsx',
    forbidden: [
      'AgentCommand.ActivityAppGameAdapterDispatchPreflightReadModelGet',
      'AgentEvent.ActivityAppGameAdapterDispatchPreflightReadModelReported',
      'AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet',
      'AgentEvent.ActivityAppGameAdapterDispatchResultReadModelReported',
      'AgentCommand.ActivityAppGameAdapterDispatchExecute',
      'AgentEvent.ActivityAppGameAdapterDispatchExecuted',
    ],
  },
  {
    file: 'src/AppGameTimerParentSurfaceRoutePanel.tsx',
    forbidden: [
      'AgentCommand.ActivityAppGameTimerParentSurfaceReadModelGet',
      'AgentEvent.ActivityAppGameTimerParentSurfaceReadModelReported',
      'AgentCommand.ActivityAppGameTimerParentPreferenceSetupRequest',
      'AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested',
    ],
  },
];
const ProductCommandBridgeFiles = [
  {
    file: 'src/TrackingStatusRoutePanel.tsx',
    requiredMainActions: ['ParentUiActionKind.TrackingRetentionSettingsWriteRequested'],
    requiredBridgeActions: ["'tracking-retention-settings-write-requested'"],
  },
  {
    file: 'src/ScreenSettingsWritableControls.tsx',
    requiredMainActions: [
      'ParentUiActionKind.ScreenSettingsGetRequested',
      'ParentUiActionKind.ScreenSettingsReplaceRequested',
    ],
    requiredBridgeActions: ["'screen-settings-get-requested'", "'screen-settings-replace-requested'"],
    forbidden: ['AgentCommand.ActivityScreenSettingsGet', 'AgentCommand.ActivityScreenSettingsReplace'],
  },
  {
    file: 'src/AppGameAdapterDispatchRoutePanel.tsx',
    requiredMainActions: ['ParentUiActionKind.AppGameAdapterDispatchExecuteRequested'],
    requiredBridgeActions: ["'app-game-adapter-dispatch-execute-requested'"],
  },
  {
    file: 'src/AppGameTimerParentSurfaceRoutePanel.tsx',
    requiredMainActions: ['ParentUiActionKind.AppGameTimerParentPreferenceSetupRequested'],
    requiredBridgeActions: ["'app-game-timer-parent-preference-setup-requested'"],
  },
];
const ProductOverlayPanelsRemovedFromRouteShell = [
  'AiRuntimeRoutePanel',
  'AppGameAdapterDispatchRoutePanel',
  'AppGameChildRuntimeTransportReceiptRoutePanel',
  'AppGameNotificationParentSurfaceRoutePanel',
  'AppGamePlatformProofStatusRoutePanel',
  'AppGamePolicyReadinessRoutePanel',
  'AppGameTimerParentSurfaceRoutePanel',
  'BrowserParentExplanationRoutePanel',
  'ScreenSettingsRoutePanel',
  'ScreenSummaryRoutePanel',
  'SocialAlertReportRoutePanel',
  'SocialAuditExplanationRoutePanel',
  'SocialDashboardRoutePanel',
];
const TestDirectory = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const RetiredSchemaDomainPortalContractsSpecifier = '@ocentra-parent/' + 'schema-domain/' + 'portal-contracts';
const RetiredSchemaDomainLoggingContractsSpecifier = '@ocentra-parent/' + 'schema-domain/' + 'logging-contracts';
const RetiredSchemaDomainGeneratedLoggingContractsSpecifier =
  '@ocentra-parent/' + 'schema-domain/generated/' + 'logging-contracts';

it('product bridge guard: tracking status surfaces use Rust-generated panel snapshots', () => {
  const trackingMetricSources = [
    readFileSync(resolve(TestDirectory, '..', 'src/portal-product-metric.ts'), 'utf8'),
    readFileSync(resolve(TestDirectory, '..', 'src/TrackingStatusRoutePanel.tsx'), 'utf8'),
  ];

  for (const source of trackingMetricSources) {
    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
    expect(source).not.toContain('@ocentra-parent/portal-domain/tracking-retention-settings-hosted-ui-proof');
    expect(source).not.toContain('@ocentra-parent/portal-domain/tracking-evidence-drawer-hosted-ui-proof');
  }

  expect(trackingMetricSources[1]).toContain('ParentTrackingStatusPanelSnapshot');
  expect(trackingMetricSources[1]).toContain('activityTrackingPanel');
  expect(trackingMetricSources[1]).not.toContain('EMPTY_TRACKING_STATUS_PANEL');
});

it('product bridge guard: portal dev logging uses Rust-generated logging DTO types at the TS edge', () => {
  const appLoggingEdgeSource = readFileSync(resolve(TestDirectory, '..', 'src/dev-logger.ts'), 'utf8');
  const portalDomainLoggingEdgeSource = readFileSync(
    resolve(TestDirectory, '..', '..', '..', 'packages/portal-domain/src/dev-logger.ts'),
    'utf8'
  );
  const retiredSchemaLoggingContractsSpecifier = RetiredSchemaDomainGeneratedLoggingContractsSpecifier;
  const localGeneratedLoggingEdgeSource = readFileSync(
    resolve(TestDirectory, '..', '..', '..', 'packages/logging-domain/src/core/stackTrace.ts'),
    'utf8'
  );

  for (const source of [appLoggingEdgeSource, portalDomainLoggingEdgeSource]) {
    expect(source).not.toContain(retiredSchemaLoggingContractsSpecifier);
    expect(source).not.toContain(RetiredSchemaDomainLoggingContractsSpecifier);
    expect(source).not.toContain('decodeStackTrace');
    expect(source).not.toContain('decodeLogEntryId');
    expect(source).not.toContain('decodeLogTimestamp');
  }

  expect(appLoggingEdgeSource).toContain('@ocentra-parent/logging-domain/generated/logging-contracts');
  expect(portalDomainLoggingEdgeSource).toContain('@ocentra-parent/logging-domain/generated/logging-contracts');
  expect(localGeneratedLoggingEdgeSource).toContain('../generated-logging-contracts');
  expect(localGeneratedLoggingEdgeSource).not.toContain(retiredSchemaLoggingContractsSpecifier);
  expect(localGeneratedLoggingEdgeSource).not.toContain(RetiredSchemaDomainLoggingContractsSpecifier);
  expect(localGeneratedLoggingEdgeSource).not.toContain('decodeStackTrace');
  expect(localGeneratedLoggingEdgeSource).not.toContain('decodeLogEntryId');
  expect(localGeneratedLoggingEdgeSource).not.toContain('decodeLogTimestamp');
});

it('product bridge guard: portal route descriptors and sidebar use Rust-generated route metadata', () => {
  const portalAppSource = readFileSync(resolve(TestDirectory, '..', 'src/PortalApp.tsx'), 'utf8');
  const sidebarSource = readFileSync(resolve(TestDirectory, '..', 'src/PortalSidebar.tsx'), 'utf8');
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');
  const portalStateSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-state.ts'), 'utf8');
  const portalAppBehaviorSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-app-behavior.ts'), 'utf8');

  expect(portalAppSource).toContain("from '@ocentra-parent/portal-domain/routes'");
  expect(portalAppSource).toContain('PortalRouteDescriptors');
  expect(portalAppSource).not.toContain('./portal-route-descriptor');
  expect(portalAppBehaviorSource).toContain('resolveSnapshotLiveActivityState(');
  expect(portalAppBehaviorSource).not.toContain('resolveLiveActivityState(');
  expect(sidebarSource).toContain("from '@ocentra-parent/portal-domain/routes'");
  expect(sidebarSource).toContain('PortalRouteDescriptors');
  expect(sidebarSource).toContain('parentRouteGroupLabel');
  expect(sidebarSource).toContain('ParentRouteMetadata');
  expect(sidebarSource).not.toContain('./portal-route-descriptor');
  expect(sidebarSource).toContain("from '../generated/parent-ui-bridge'");
  expect(sidebarSource).toContain('ParentSidebarRouteGroups');
  expect(sidebarSource).toContain('ParentBridgeConnectionState.Connected');
  expect(sidebarSource).toContain('parentRouteHashPath');
  expect(sidebarSource).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
  expect(sidebarSource).not.toContain('PortalConnectionState');
  expect(portalStateSource).toContain('ParentBridgeConnectionState.Disconnected');
  expect(portalStateSource).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
  expect(portalStateSource).not.toContain('PortalConnectionState');
  expect(bridgeContractSource).toContain('export const ParentRouteMetadata');
  expect(bridgeContractSource).toContain('export const ParentSidebarRoutes');
  expect(bridgeContractSource).toContain('export const ParentSidebarRouteGroups');
});

it('product bridge guard: portal dev tool window uses Rust-generated route helpers', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/portal-dev-tool-window.ts'), 'utf8');

  expect(source).toContain("from '../generated/parent-ui-bridge'");
  expect(source).toContain('ParentRoute.FrameTuner');
  expect(source).toContain('PortalDevToolWindow.FrameTunerHash');
  expect(source).toContain('ParentHostBridgeRuntime.TauriInternalWindowKey');
  expect(source).toContain('@ocentra-parent/portal-domain/routes');
  expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
  expect(source).toContain('PortalDevToolWindow');
  expect(source).toContain('portalDevToolUrl');
});

it('product bridge guard: portal shell uses explicit Rust-owned action kinds for auto route refresh flows', () => {
  const mainSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-runtime-controller.ts'), 'utf8');
  const actionsSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-runtime-controller-actions.ts'), 'utf8');
  const portalAppSource = readFileSync(resolve(TestDirectory, '..', 'src/PortalApp.tsx'), 'utf8');
  const networkRefreshSource = readFileSync(
    resolve(TestDirectory, '..', 'src/use-portal-network-activity-refresh.ts'),
    'utf8'
  );
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(mainSource).toContain('createPortalRuntimeActions');
  expect(actionsSource).toContain('ParentUiActionKind.LanPairingBrowserDiscoveryScanRequested');
  expect(actionsSource).toContain('ParentUiActionKind.NetworkFlowReadModelRefreshRequested');
  expect(portalAppSource).not.toContain('AgentCommand.LanPairingBrowserDiscoveryScan');
  expect(networkRefreshSource).not.toContain('AgentCommand.NetworkFlowReadModelGet');
  expect(bridgeContractSource).toContain("'lan-pairing-browser-discovery-scan-requested'");
  expect(bridgeContractSource).toContain("'network-flow-read-model-refresh-requested'");
});

it('product bridge guard: generated bridge contract carries Rust-returned action events and embedded snapshots', () => {
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(bridgeContractSource).toContain('readonly events: readonly ParentRouteEventSnapshot[];');
  expect(bridgeContractSource).toContain('readonly snapshot?: ParentUnknownRecord | null;');
  expect(bridgeContractSource).toContain(
    'readonly lanAddDeviceReadModel?: ParentLanAddDeviceReadModelSnapshot | null;'
  );
});

it('product bridge guard: product command flows use explicit Rust-owned action kinds', () => {
  const actionsSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-runtime-controller-actions.ts'), 'utf8');
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  for (const { file, requiredMainActions, requiredBridgeActions, forbidden = [] } of ProductCommandBridgeFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    for (const action of requiredMainActions) {
      expect(actionsSource).toContain(action);
    }
    for (const action of requiredBridgeActions) {
      expect(bridgeContractSource).toContain(action);
    }
    for (const value of forbidden) {
      expect(source).not.toContain(value);
    }
  }
});

it('product bridge guard: product snapshot route panels refresh through the Rust route snapshot action', () => {
  const actionsSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-runtime-controller-actions.ts'), 'utf8');

  expect(actionsSource).toContain('ParentUiActionKind.RefreshRoute');
  for (const { file, forbidden } of ProductSnapshotRefreshFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    expect(source).toContain('refreshRouteSnapshot');
    for (const value of forbidden) {
      expect(source).not.toContain(value);
    }
  }
});

it('product bridge guard: the product route shell does not mount parallel diagnostic overlays on top of the SVG surface', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/ParentPortalRoute.tsx'), 'utf8');

  for (const panel of ProductOverlayPanelsRemovedFromRouteShell) {
    expect(source).not.toContain(panel);
  }
});

it('product bridge guard: LAN target selection is persisted before route actions use the selected child device', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/ParentPortalRoute.tsx'), 'utf8');

  expect(source).toContain("from '@ocentra-parent/portal-domain/manage-target-selection'");
  expect(source).toContain('onTargetChange={writeStoredManageTargetSelection}');
});
