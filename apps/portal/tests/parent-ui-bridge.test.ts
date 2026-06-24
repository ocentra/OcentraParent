import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';

const ProductBridgeSourceFiles = [
  'src/main.ts',
  'src/host-bridge.ts',
  'src/portal-state.ts',
  'src/PortalApp.tsx',
  'src/ParentPortalRoute.tsx',
];
const BridgeContractFile = 'src/generated/parent-ui-bridge.ts';
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
    forbidden: ['AgentCommand.BrowserSocialDashboardReadModelGet', 'AgentEvent.BrowserSocialDashboardReadModelReported'],
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
    requiredMainActions: ["action: 'tracking-retention-settings-write-requested'"],
    requiredBridgeActions: ["'tracking-retention-settings-write-requested'"],
  },
  {
    file: 'src/ScreenSettingsWritableControls.tsx',
    requiredMainActions: ["action: 'screen-settings-get-requested'", "action: 'screen-settings-replace-requested'"],
    requiredBridgeActions: ["'screen-settings-get-requested'", "'screen-settings-replace-requested'"],
    forbidden: ['AgentCommand.ActivityScreenSettingsGet', 'AgentCommand.ActivityScreenSettingsReplace'],
  },
  {
    file: 'src/AppGameAdapterDispatchRoutePanel.tsx',
    requiredMainActions: ["action: 'app-game-adapter-dispatch-execute-requested'"],
    requiredBridgeActions: ["'app-game-adapter-dispatch-execute-requested'"],
  },
  {
    file: 'src/AppGameTimerParentSurfaceRoutePanel.tsx',
    requiredMainActions: ["action: 'app-game-timer-parent-preference-setup-requested'"],
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
const TestDirectory = dirname(fileURLToPath(import.meta.url));

it('product bridge guard: portal source does not create browser WebSockets', () => {
  for (const sourceFile of ProductBridgeSourceFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', sourceFile), 'utf8');

    expect(source).not.toContain('new WebSocket');
    expect(source).not.toContain('WebSocket.OPEN');
    expect(source).not.toContain('connectWebSocket');
    expect(source).not.toContain('state.socket');
  }
});

it('product bridge guard: portal shell uses the host bridge subscription path instead of direct agent event listeners', () => {
  const mainSource = readFileSync(resolve(TestDirectory, '..', 'src/main.ts'), 'utf8');
  const hostBridgeSource = readFileSync(resolve(TestDirectory, '..', 'src/host-bridge.ts'), 'utf8');

  expect(mainSource).toContain('bridge.subscribe(');
  expect(mainSource).toContain('disposeRouteSubscription');
  expect(hostBridgeSource).toContain("'parent_subscribe_route'");
  expect(hostBridgeSource).toContain("'parent_unsubscribe_route'");
  expect(hostBridgeSource).toContain("@tauri-apps/api/event");
});

it('product bridge guard: portal shell uses explicit Rust-owned action kinds for auto route refresh flows', () => {
  const mainSource = readFileSync(resolve(TestDirectory, '..', 'src/main.ts'), 'utf8');
  const portalAppSource = readFileSync(resolve(TestDirectory, '..', 'src/PortalApp.tsx'), 'utf8');
  const networkRefreshSource = readFileSync(
    resolve(TestDirectory, '..', 'src/use-portal-network-activity-refresh.ts'),
    'utf8'
  );
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(mainSource).toContain("action: 'lan-pairing-browser-discovery-scan-requested'");
  expect(mainSource).toContain("action: 'network-flow-read-model-refresh-requested'");
  expect(portalAppSource).not.toContain('AgentCommand.LanPairingBrowserDiscoveryScan');
  expect(networkRefreshSource).not.toContain('AgentCommand.NetworkFlowReadModelGet');
  expect(bridgeContractSource).toContain("'lan-pairing-browser-discovery-scan-requested'");
  expect(bridgeContractSource).toContain("'network-flow-read-model-refresh-requested'");
});

it('product bridge guard: product command flows use explicit Rust-owned action kinds', () => {
  const mainSource = readFileSync(resolve(TestDirectory, '..', 'src/main.ts'), 'utf8');
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  for (const { file, requiredMainActions, requiredBridgeActions, forbidden = [] } of ProductCommandBridgeFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    for (const action of requiredMainActions) {
      expect(mainSource).toContain(action);
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
  const mainSource = readFileSync(resolve(TestDirectory, '..', 'src/main.ts'), 'utf8');

  expect(mainSource).toContain("action: 'refresh-route'");
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
