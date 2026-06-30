import { readdirSync, readFileSync } from 'node:fs';
import { dirname, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';

const ProductBridgeSourceFiles = [
  'src/main.ts',
  'src/host-bridge.ts',
  'src/portal-state.ts',
  'src/PortalApp.tsx',
  'src/ParentPortalRoute.tsx',
];
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
const ProductSnapshotOnlyRouteFiles = [
  'src/ParentPortalRoute.tsx',
  ...ProductSnapshotRefreshFiles.map(({ file }) => file),
];
const ProductLiveActivityResolverCallers = [
  'src/diagnostics-export.ts',
  'src/ParentPortalRoute.tsx',
  'src/PortalApp.tsx',
  'src/route-live-activity-state.ts',
];
const TestDirectory = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const ProductSourceDirectory = resolve(TestDirectory, '..', 'src');

function listSourceFiles(directory: string): string[] {
  const entries = readdirSync(directory, { withFileTypes: true });
  const files: string[] = [];

  for (const entry of entries) {
    const entryPath = resolve(directory, entry.name);
    if (entry.isDirectory()) {
      files.push(...listSourceFiles(entryPath));
      continue;
    }
    if (entry.name.endsWith('.ts') || entry.name.endsWith('.tsx')) {
      files.push(entryPath);
    }
  }

  return files;
}

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
  expect(hostBridgeSource).toContain('ParentBridgeCommand.Subscribe');
  expect(hostBridgeSource).toContain('ParentBridgeCommand.Unsubscribe');
  expect(hostBridgeSource).toContain('ParentHostBridgeRuntime.TauriEventModule');
  expect(hostBridgeSource).toContain('createUnavailableDevWebHostBridge');
  expect(hostBridgeSource).not.toContain('createPresentationOnlyDevWebHostBridge');
  expect(hostBridgeSource).not.toContain('createDevSnapshot(');
  expect(hostBridgeSource).not.toContain('createUnavailableDevBridgeSnapshot');
  expect(hostBridgeSource).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(hostBridgeSource).not.toContain('@ocentra-parent/portal-domain/contracts');
  expect(hostBridgeSource).not.toContain('@ocentra-parent/portal-domain/routes');
});

it('product bridge guard: developer route panel uses Rust-generated route constants instead of the TS route catalog', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/PortalDeveloperRoutePanel.tsx'), 'utf8');
  const contentSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-route-content.ts'), 'utf8');

  expect(source).toContain("from '../generated/parent-ui-bridge'");
  expect(source).toContain('ParentRoute.Commands');
  expect(source).toContain('ParentRoute.Events');
  expect(source).toContain('ParentRoute.Logs');
  expect(source).toContain('ParentRouteId');
  expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(source).not.toContain('@ocentra-parent/portal-domain/routes');
  expect(source).not.toContain('PortalRouteValue');
  expect(source).not.toContain('isPortalDeveloperRoute');
  expect(source).not.toContain('isPortalDeveloperCommandRoute');
  expect(source).not.toContain('isPortalDeveloperEventRoute');
  expect(source).not.toContain('isPortalDeveloperLogRoute');
  expect(contentSource).toContain("from '../generated/parent-ui-bridge'");
  expect(contentSource).toContain('ParentRoute.Commands');
  expect(contentSource).toContain('ParentRoute.Events');
  expect(contentSource).toContain('ParentRoute.Logs');
  expect(contentSource).toContain('ParentRouteId');
  expect(contentSource).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(contentSource).not.toContain('PortalRoute.');
  expect(contentSource).not.toContain('PortalRouteValue');
});

it('product bridge guard: portal main route parsing uses Rust-generated route helpers', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/main.ts'), 'utf8');
  const portalContractsSource = readFileSync(
    resolve(TestDirectory, '..', '..', '..', 'packages/schema-domain/src/portal-contracts.ts'),
    'utf8'
  );
  const generatedPortalContractsSource = readFileSync(
    resolve(TestDirectory, '..', '..', '..', 'packages/schema-domain/src/generated/portal-contracts.ts'),
    'utf8'
  );

  expect(source).toContain("from '../generated/parent-ui-bridge'");
  expect(source).toContain('parentRouteFromHashPath');
  expect(source).toContain('parentRouteHashPath');
  expect(source).toContain('ParentRoute.Overview');
  expect(source).not.toContain('@ocentra-parent/portal-domain/routes');
  expect(source).not.toContain('PortalRoutes');
  expect(portalContractsSource).toContain('GeneratedPortalRoute.AppLayout');
  expect(portalContractsSource).toContain('GeneratedPortalRoute.FrameTuner');
  expect(generatedPortalContractsSource).toContain("AppLayout: 'app-layout'");
  expect(generatedPortalContractsSource).toContain("FrameTuner: 'frame-tuner'");
  expect(generatedPortalContractsSource).not.toContain("FrameTuner: 'app-layout'");
});

it('product bridge guard: portal route predicates use Rust-generated route constants', () => {
  const predicateFiles = [
    'src/NetworkEvidenceDrawerRoutePanel.tsx',
    'src/SetupFirstRunRoutePanel.tsx',
    'src/portal-route-refresh.ts',
    'src/use-portal-network-activity-refresh.ts',
  ];

  for (const file of predicateFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).not.toContain('@ocentra-parent/portal-domain/routes');
    expect(source).not.toContain('isPortalNetworkEvidenceDrawerRoute');
    expect(source).not.toContain('isPortalInlineNetworkEvidenceDrawerRoute');
    expect(source).not.toContain('isPortalSetupFirstRunRoute');
  }

  const setupSource = readFileSync(resolve(TestDirectory, '..', 'src/SetupFirstRunRoutePanel.tsx'), 'utf8');
  const parentPortalRouteSource = readFileSync(resolve(TestDirectory, '..', 'src/ParentPortalRoute.tsx'), 'utf8');
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');
  const refreshSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-route-refresh.ts'), 'utf8');
  const networkDrawerSource = readFileSync(
    resolve(TestDirectory, '..', 'src/NetworkEvidenceDrawerRoutePanel.tsx'),
    'utf8'
  );
  const networkRefreshSource = readFileSync(
    resolve(TestDirectory, '..', 'src/use-portal-network-activity-refresh.ts'),
    'utf8'
  );

  expect(bridgeContractSource).toContain('ParentSetupFirstRunPanelSnapshot');
  expect(bridgeContractSource).toContain('setupFirstRunPanel?: ParentSetupFirstRunPanelSnapshot | null');
  expect(setupSource).toContain('isParentSetupFirstRunRoute');
  expect(setupSource).toContain('ParentSetupFirstRunPanelSnapshot');
  expect(setupSource).not.toContain('@ocentra-parent/portal-domain/setup-first-run-panel');
  expect(parentPortalRouteSource).toContain('setupFirstRunPanel');
  expect(refreshSource).toContain('ParentRoute.Activity');
  expect(refreshSource).toContain('ParentRoute.NetworkActivity');
  expect(refreshSource).toContain('ParentBridgeConnectionState.Connected');
  expect(refreshSource).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(refreshSource).not.toContain('PortalConnectionState');
  expect(networkDrawerSource).toContain('networkEvidenceSummary: networkEvidenceSummary ?? null');
  expect(networkDrawerSource).not.toContain('liveActivity.policyPreviewReadModel');
  expect(networkRefreshSource).toContain("from '../generated/parent-ui-bridge'");
  expect(networkRefreshSource).toContain('ParentBridgeConnectionState');
  expect(networkRefreshSource).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(networkRefreshSource).not.toContain('PortalConnectionState');
});

it('product bridge guard: app game route panels use Rust-generated route-family predicates', () => {
  const appGameRouteFiles = [
    'src/AppGameAdapterDispatchRoutePanel.tsx',
    'src/AppGameTimerParentSurfaceRoutePanel.tsx',
    'src/AppGamePolicyReadinessRoutePanel.tsx',
    'src/AppGamePlatformProofStatusRoutePanel.tsx',
    'src/AppGameChildRuntimeTransportReceiptRoutePanel.tsx',
    'src/AppGameNotificationParentSurfaceRoutePanel.tsx',
  ];
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(bridgeContractSource).toContain('ParentAppGameParentSurfaceRoutes');
  expect(bridgeContractSource).toContain('ParentRoute.AppGameSessions');
  expect(bridgeContractSource).toContain('isParentAppGameParentSurfaceRoute');

  for (const file of appGameRouteFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).toContain('isParentAppGameParentSurfaceRoute');
    expect(source).toContain('ParentRouteId');
    expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
    expect(source).not.toContain('@ocentra-parent/portal-domain/routes');
    expect(source).not.toContain('PortalRouteValue');
    expect(source).not.toContain('isPortalAppGameParentSurfaceRoute');
  }
});

it('product bridge guard: browser route panels use Rust-generated route-family predicates', () => {
  const browserRouteFiles = [
    'src/BrowserParentExplanationRoutePanel.tsx',
    'src/SocialAuditExplanationRoutePanel.tsx',
    'src/SocialDashboardRoutePanel.tsx',
    'src/SocialAlertReportRoutePanel.tsx',
  ];
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(bridgeContractSource).toContain('ParentBrowserParentSurfaceRoutes');
  expect(bridgeContractSource).toContain('ParentRoute.ProofPanels');
  expect(bridgeContractSource).toContain('isParentBrowserParentSurfaceRoute');
  expect(bridgeContractSource).toContain('ParentBrowserPanelSnapshot');
  expect(bridgeContractSource).toContain('browserParentExplanation?: ParentBrowserPanelSnapshot | null');
  expect(bridgeContractSource).toContain('socialAlertReport?: ParentBrowserPanelSnapshot | null');
  expect(bridgeContractSource).toContain('browserActionIntentStreamStatus?: ParentBrowserPanelSnapshot | null');

  for (const file of browserRouteFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).toContain('isParentBrowserParentSurfaceRoute');
    expect(source).toContain('ParentRouteId');
    expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
    expect(source).not.toContain('@ocentra-parent/portal-domain/routes');
    expect(source).not.toContain('PortalRouteValue');
    expect(source).not.toContain('isPortalBrowserParentSurfaceRoute');
  }

  const browserParentExplanationSource = readFileSync(
    resolve(TestDirectory, '..', 'src/BrowserParentExplanationRoutePanel.tsx'),
    'utf8'
  );
  const socialAuditSource = readFileSync(resolve(TestDirectory, '..', 'src/SocialAuditExplanationRoutePanel.tsx'), 'utf8');
  const socialDashboardSource = readFileSync(resolve(TestDirectory, '..', 'src/SocialDashboardRoutePanel.tsx'), 'utf8');
  const socialAlertReportSource = readFileSync(resolve(TestDirectory, '..', 'src/SocialAlertReportRoutePanel.tsx'), 'utf8');
  const parentPortalRouteSource = readFileSync(resolve(TestDirectory, '..', 'src/ParentPortalRoute.tsx'), 'utf8');

  expect(browserParentExplanationSource).not.toContain('@ocentra-parent/portal-domain/browser-parent-explanation-panel');
  expect(socialAuditSource).not.toContain('@ocentra-parent/portal-domain/social-audit-explanation-panel');
  expect(socialDashboardSource).not.toContain('@ocentra-parent/portal-domain/social-dashboard-panel');
  expect(socialAlertReportSource).not.toContain('@ocentra-parent/portal-domain/social-alert-report-panel');
  expect(socialAlertReportSource).not.toContain('@ocentra-parent/portal-domain/social-alert-report-parent-surface-panel');
  expect(socialAlertReportSource).not.toContain('@ocentra-parent/portal-domain/social-parent-notification-delivery-panel');
  expect(socialAlertReportSource).not.toContain('@ocentra-parent/portal-domain/browser-action-intent-stream-status');
  expect(socialAlertReportSource).not.toContain('@ocentra-parent/portal-domain/browser-social-provider-receipt-stream-status');
  expect(socialAlertReportSource).not.toContain(
    '@ocentra-parent/portal-domain/browser-social-provider-receipt-ingestion-readiness-status'
  );
  expect(socialAlertReportSource).not.toContain('./live-activity-state');
  expect(parentPortalRouteSource).toContain('browserPanels?.browserParentExplanation');
  expect(parentPortalRouteSource).toContain('browserPanels?.socialAlertReport');
  expect(parentPortalRouteSource).toContain('browserPanels?.browserActionIntentStreamStatus');
});

it('product bridge guard: ai policy and screen route panels use Rust-generated route-family predicates', () => {
  const routeFamilyPanels = [
    {
      file: 'src/AiRuntimeRoutePanel.tsx',
      generatedRoutes: 'ParentAiRuntimeRoutes',
      generatedRoute: 'ParentRoute.AiRuntime',
      generatedPredicate: 'isParentAiRuntimeRoute',
      legacyPredicate: 'isPortalAiRuntimeRoute',
    },
    {
      file: 'src/PolicyPreviewRoutePanel.tsx',
      generatedRoutes: 'ParentPolicyPreviewRoutes',
      generatedRoute: 'ParentRoute.RuleManagement',
      generatedPredicate: 'isParentPolicyPreviewRoute',
      legacyPredicate: 'isPortalPolicyPreviewRoute',
    },
    {
      file: 'src/ScreenSettingsRoutePanel.tsx',
      generatedRoutes: 'ParentScreenSettingsRoutes',
      generatedRoute: 'ParentRoute.SettingsRules',
      generatedPredicate: 'isParentScreenSettingsRoute',
      legacyPredicate: 'isPortalScreenSettingsRoute',
    },
    {
      file: 'src/ScreenSummaryRoutePanel.tsx',
      generatedRoutes: 'ParentScreenSummaryRoutes',
      generatedRoute: 'ParentRoute.ScreenAnalysis',
      generatedPredicate: 'isParentScreenSummaryRoute',
      legacyPredicate: 'isPortalScreenSummaryRoute',
    },
  ];
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  for (const panel of routeFamilyPanels) {
    const source = readFileSync(resolve(TestDirectory, '..', panel.file), 'utf8');

    expect(bridgeContractSource).toContain(panel.generatedRoutes);
    expect(bridgeContractSource).toContain(panel.generatedRoute);
    expect(bridgeContractSource).toContain(panel.generatedPredicate);
    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).toContain(panel.generatedPredicate);
    expect(source).toContain('ParentRouteId');
    expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
    expect(source).not.toContain('@ocentra-parent/portal-domain/routes');
    expect(source).not.toContain('PortalRouteValue');
    expect(source).not.toContain(panel.legacyPredicate);
  }
});

it('product bridge guard: parent portal route shell uses Rust-generated route helpers', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/ParentPortalRoute.tsx'), 'utf8');

  expect(source).toContain("from '../generated/parent-ui-bridge'");
  expect(source).toContain('ParentRoute.Assistant');
  expect(source).toContain('ParentRoute.Diagnostics');
  expect(source).toContain('ParentRoute.ProofPanels');
  expect(source).toContain('ParentBridgeConnectionState.Connected');
  expect(source).toContain('ParentHostBridgeRuntime.RouteHashPrefix');
  expect(source).toContain('parentRouteHashPath(ParentRoute.FrameTuner)');
  expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(source).not.toContain('PortalRoute.');
  expect(source).not.toContain('PortalConnectionState.');
  expect(source).not.toContain('PortalDom.HashPrefix');
});

it('product bridge guard: portal app shell uses Rust-generated route IDs and constants', () => {
  const generatedOnlyShellFiles = [
    'src/main.ts',
    'src/PortalApp.tsx',
    'src/PortalUnifiedChrome.tsx',
    'src/PortalProofPanelsRoutePanel.tsx',
  ];

  for (const file of generatedOnlyShellFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).toContain('ParentRoute');
    expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
    expect(source).not.toContain('PortalRouteValue');
    expect(source).not.toContain('PortalRoute.');
    expect(source).not.toContain('PortalDom.HashPrefix');
  }
});

it('product bridge guard: portal detail values use Rust-generated bridge edge types', () => {
  const detailListSource = readFileSync(resolve(TestDirectory, '..', 'src/detail-list.ts'), 'utf8');
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(detailListSource).toContain("from '../generated/parent-ui-bridge'");
  expect(detailListSource).toContain('ParentPortalDetailValue');
  expect(detailListSource).toContain('decodeParentPortalDetailValue');
  expect(detailListSource).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(detailListSource).not.toContain('type PortalDetailValue');
  expect(detailListSource).not.toContain('decodePortalDetailValue');

  expect(bridgeContractSource).toContain('export type ParentPortalDetailValue = string;');
  expect(bridgeContractSource).toContain('export type ParentPortalClipboardText = string;');
  expect(bridgeContractSource).toContain('export type ParentTrackingStatusProofArtifact = string;');
  expect(bridgeContractSource).toContain('decodeParentPortalDetailValue');
  expect(bridgeContractSource).toContain('decodeParentPortalClipboardText');
  expect(bridgeContractSource).toContain('decodeParentTrackingStatusProofArtifact');
});

it('product bridge guard: portal clipboard edges use Rust-generated bridge edge types', () => {
  const clipboardSource = readFileSync(resolve(TestDirectory, '..', 'src/clipboard.ts'), 'utf8');
  const diagnosticsExportSource = readFileSync(resolve(TestDirectory, '..', 'src/diagnostics-export.ts'), 'utf8');
  const commandResultSource = readFileSync(resolve(TestDirectory, '..', 'src/command-result-panel.ts'), 'utf8');

  for (const source of [clipboardSource, diagnosticsExportSource, commandResultSource]) {
    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
    expect(source).not.toContain('type PortalClipboardText');
    expect(source).not.toContain('decodePortalClipboardText');
  }

  expect(clipboardSource).toContain('ParentPortalClipboardText');
  expect(diagnosticsExportSource).toContain('ParentPortalClipboardText');
  expect(diagnosticsExportSource).toContain('decodeParentPortalClipboardText');
  expect(commandResultSource).toContain('decodeParentPortalClipboardText');
  expect(commandResultSource).not.toContain('@ocentra-parent/portal-domain/app-game-timer-parent-surface-panel');
  expect(commandResultSource).not.toContain('AppGameTimerParentSurfacePanelDetail');
});

it('product bridge guard: simple portal detail edges use Rust-generated bridge detail values', () => {
  const detailEdgeSources = [
    readFileSync(resolve(TestDirectory, '..', 'src/diagnostics-panel.ts'), 'utf8'),
    readFileSync(resolve(TestDirectory, '..', 'src/dev-log-panel.ts'), 'utf8'),
    readFileSync(resolve(TestDirectory, '..', 'src/portal-pending-surfaces.ts'), 'utf8'),
  ];

  for (const source of detailEdgeSources) {
    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).toContain('decodeParentPortalDetailValue');
    expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
    expect(source).not.toContain('decodePortalDetailValue');
  }
});

it('product bridge guard: browser detail helpers use Rust-generated bridge detail values', () => {
  const detailHelperSources = [
    readFileSync(resolve(TestDirectory, '..', 'src/browser-status-panel.ts'), 'utf8'),
    readFileSync(resolve(TestDirectory, '..', 'src/browser-intervention-panel.ts'), 'utf8'),
  ];

  for (const source of detailHelperSources) {
    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).toContain('ParentPortalDetailValue');
    expect(source).toContain('decodeParentPortalDetailValue');
    expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
    expect(source).not.toContain('type PortalDetailValue');
    expect(source).not.toContain('decodePortalDetailValue');
  }
});

it('product bridge guard: tracking status surfaces use Rust-generated panel snapshots', () => {
  const trackingMetricSources = [
    readFileSync(resolve(TestDirectory, '..', 'src/portal-product-metric.ts'), 'utf8'),
    readFileSync(resolve(TestDirectory, '..', 'src/TrackingStatusRoutePanel.tsx'), 'utf8'),
  ];

  for (const source of trackingMetricSources) {
    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
    expect(source).not.toContain('@ocentra-parent/portal-domain/tracking-retention-settings-hosted-ui-proof');
    expect(source).not.toContain('@ocentra-parent/portal-domain/tracking-evidence-drawer-hosted-ui-proof');
  }

  expect(trackingMetricSources[1]).toContain('ParentTrackingStatusPanelSnapshot');
  expect(trackingMetricSources[1]).toContain('activityTrackingPanel');
  expect(trackingMetricSources[1]).not.toContain('EMPTY_TRACKING_STATUS_PANEL');
});

it('product bridge guard: portal dev logging uses Rust-generated logging DTO types at the TS edge', () => {
  const loggingEdgeSources = [
    readFileSync(resolve(TestDirectory, '..', 'src/dev-logger.ts'), 'utf8'),
    readFileSync(resolve(TestDirectory, '..', '..', '..', 'packages/portal-domain/src/dev-logger.ts'), 'utf8'),
    readFileSync(resolve(TestDirectory, '..', '..', '..', 'packages/logging-domain/src/core/stackTrace.ts'), 'utf8'),
  ];

  for (const source of loggingEdgeSources) {
    expect(source).toContain('@ocentra-parent/schema-domain/generated/logging-contracts');
    expect(source).not.toContain('@ocentra-parent/schema-domain/logging-contracts');
    expect(source).not.toContain('decodeStackTrace');
    expect(source).not.toContain('decodeLogEntryId');
    expect(source).not.toContain('decodeLogTimestamp');
  }
});

it('product bridge guard: portal route descriptors and sidebar use Rust-generated route metadata', () => {
  const descriptorSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-route-descriptor.ts'), 'utf8');
  const sidebarSource = readFileSync(resolve(TestDirectory, '..', 'src/PortalSidebar.tsx'), 'utf8');
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');
  const portalStateSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-state.ts'), 'utf8');

  expect(descriptorSource).toContain("from '../generated/parent-ui-bridge'");
  expect(descriptorSource).toContain('ParentRouteMetadata');
  expect(descriptorSource).toContain('ParentSidebarRoutes');
  expect(sidebarSource).toContain("from '../generated/parent-ui-bridge'");
  expect(sidebarSource).toContain('ParentSidebarRouteGroups');
  expect(sidebarSource).toContain('ParentBridgeConnectionState.Connected');
  expect(sidebarSource).toContain('parentRouteHashPath');
  expect(descriptorSource).not.toContain('@ocentra-parent/portal-domain/routes');
  expect(sidebarSource).not.toContain('@ocentra-parent/portal-domain/routes');
  expect(sidebarSource).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(sidebarSource).not.toContain('PortalConnectionState');
  expect(portalStateSource).toContain('ParentBridgeConnectionState.Disconnected');
  expect(portalStateSource).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(portalStateSource).not.toContain('PortalConnectionState');
  expect(bridgeContractSource).toContain('export const ParentRouteMetadata');
  expect(bridgeContractSource).toContain('export const ParentSidebarRoutes');
  expect(bridgeContractSource).toContain('export const ParentSidebarRouteGroups');
});

it('product bridge guard: portal dev tool window uses Rust-generated route helpers', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/portal-dev-tool-window.ts'), 'utf8');

  expect(source).toContain("from '../generated/parent-ui-bridge'");
  expect(source).toContain('ParentRoute.FrameTuner');
  expect(source).toContain('parentRouteHashPath');
  expect(source).toContain('ParentHostBridgeRuntime.TauriInternalWindowKey');
  expect(source).not.toContain('@ocentra-parent/portal-domain/routes');
  expect(source).not.toContain('@ocentra-parent/schema-domain/portal-contracts');
  expect(source).not.toContain('PortalDevToolWindow');
  expect(source).not.toContain('portalDevToolUrl');
});

it('product bridge guard: portal shell uses explicit Rust-owned action kinds for auto route refresh flows', () => {
  const mainSource = readFileSync(resolve(TestDirectory, '..', 'src/main.ts'), 'utf8');
  const portalAppSource = readFileSync(resolve(TestDirectory, '..', 'src/PortalApp.tsx'), 'utf8');
  const networkRefreshSource = readFileSync(
    resolve(TestDirectory, '..', 'src/use-portal-network-activity-refresh.ts'),
    'utf8'
  );
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(mainSource).toContain('ParentUiActionKind.LanPairingBrowserDiscoveryScanRequested');
  expect(mainSource).toContain('ParentUiActionKind.NetworkFlowReadModelRefreshRequested');
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

  expect(mainSource).toContain('ParentUiActionKind.RefreshRoute');
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

it('product bridge guard: product route rendering keeps the route shell snapshot-only', () => {
  const parentPortalRouteSource = readFileSync(resolve(TestDirectory, '..', 'src/ParentPortalRoute.tsx'), 'utf8');
  const proofPanelsRouteSource = readFileSync(
    resolve(TestDirectory, '..', 'src/PortalProofPanelsRoutePanel.tsx'),
    'utf8'
  );

  expect(parentPortalRouteSource).toContain('const routeLiveActivity = state.routeSnapshot?.liveActivity ?? null;');
  expect(parentPortalRouteSource).toContain('resolveSnapshotLiveActivityState(routeLiveActivity)');
  expect(parentPortalRouteSource).not.toContain('resolveLiveActivityState(');
  expect(parentPortalRouteSource).not.toContain('state.events');
  expect(parentPortalRouteSource).toContain('activityState.networkEvidenceSummary ?? null');
  expect(parentPortalRouteSource).toContain('activityState.policyPreviewPanel ?? null');
  expect(parentPortalRouteSource).toContain('activityState.appGameNotificationParentSurfacePanel ?? null');
  expect(parentPortalRouteSource).toContain('activityState.appGamePolicyReadinessPanel ?? null');
  expect(parentPortalRouteSource).toContain('activityState.appGamePlatformProofStatusPanel ?? null');
  expect(parentPortalRouteSource).toContain('activityState.appGameChildRuntimeTransportReceiptPanel ?? null');
  expect(parentPortalRouteSource).toContain('activityState.appGameAdapterDispatchPanel ?? null');
  expect(parentPortalRouteSource).toContain('activityState.appGameTimerParentSurfacePanel ?? null');
  expect(proofPanelsRouteSource).toContain('AppGameNotificationParentSurfaceRoutePanel');
  expect(proofPanelsRouteSource).toContain('AppGamePolicyReadinessRoutePanel');
  expect(proofPanelsRouteSource).toContain('AppGamePlatformProofStatusRoutePanel');
  expect(proofPanelsRouteSource).toContain('AppGameChildRuntimeTransportReceiptRoutePanel');
  expect(proofPanelsRouteSource).toContain('AppGameAdapterDispatchRoutePanel');
  expect(proofPanelsRouteSource).toContain('AppGameTimerParentSurfaceRoutePanel');
  expect(proofPanelsRouteSource).toContain('BrowserParentExplanationRoutePanel');
  expect(proofPanelsRouteSource).toContain('SocialAuditExplanationRoutePanel');
  expect(proofPanelsRouteSource).toContain('SocialDashboardRoutePanel');
  expect(proofPanelsRouteSource).toContain('SocialAlertReportRoutePanel');
  expect(proofPanelsRouteSource).toContain('appGameNotificationParentSurfacePanel');
  expect(proofPanelsRouteSource).toContain('appGamePolicyReadinessPanel');
  expect(proofPanelsRouteSource).toContain('appGamePlatformProofStatusPanel');
  expect(proofPanelsRouteSource).toContain('appGameChildRuntimeTransportReceiptPanel');
  expect(proofPanelsRouteSource).toContain('appGameAdapterDispatchPanel');
  expect(proofPanelsRouteSource).toContain('appGameTimerParentSurfacePanel');
  expect(proofPanelsRouteSource).toContain('browserParentExplanationPanel');
  expect(proofPanelsRouteSource).toContain('socialAuditExplanationPanel');
  expect(proofPanelsRouteSource).toContain('socialDashboardPanel');
  expect(proofPanelsRouteSource).toContain('socialAlertReportPanel');
  expect(proofPanelsRouteSource).toContain('browserActionIntentStreamStatusPanel');
  expect(proofPanelsRouteSource).not.toContain('appGameNotificationParentSurfaceIntentReadModel');
  expect(proofPanelsRouteSource).not.toContain('appGamePolicyReadinessReadModel');
  expect(proofPanelsRouteSource).not.toContain('appGamePlatformProofStatusReadModel');
  expect(proofPanelsRouteSource).not.toContain('appGameChildRuntimeTransportReceiptReadModel');
  expect(proofPanelsRouteSource).not.toContain('appGameAdapterExecutionReadinessReadModel');
  expect(proofPanelsRouteSource).not.toContain('appGameAdapterDispatchPreflightReadModel');
  expect(proofPanelsRouteSource).not.toContain('appGameAdapterDispatchResultReadModel');
  expect(proofPanelsRouteSource).not.toContain('appGameAdapterDispatchExecutedResult');
  expect(proofPanelsRouteSource).not.toContain('appGameTimerParentSurfaceReadModel');
  expect(proofPanelsRouteSource).not.toContain('createAppGameNotificationParentSurfacePanelIntent');
  expect(proofPanelsRouteSource).not.toContain('createAppGamePolicyReadinessPanelIntent');
  expect(proofPanelsRouteSource).not.toContain('createAppGamePlatformProofStatusPanelIntent');
  expect(proofPanelsRouteSource).not.toContain('createAppGameChildRuntimeTransportReceiptPanelIntent');
  expect(proofPanelsRouteSource).not.toContain('createAppGameAdapterDispatchPreflightPanelIntent');
  expect(proofPanelsRouteSource).not.toContain('createAppGameAdapterDispatchResultPanelIntent');
  expect(proofPanelsRouteSource).not.toContain('createAppGameTimerParentSurfacePanelIntent');
  expect(proofPanelsRouteSource).not.toContain('createAppGameTimerParentPreferenceSetupRequestPayload');

  for (const file of ProductSnapshotOnlyRouteFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    expect(source).not.toContain('@ocentra-parent/agent-protocol-domain');
    expect(source).not.toContain('parentRouteEventSnapshotToAgentEventEnvelope');
  }
});

it('product bridge guard: product route rendering keeps live-activity wiring snapshot-only', () => {
  const diagnosticsExportSource = readFileSync(resolve(TestDirectory, '..', 'src/diagnostics-export.ts'), 'utf8');
  const routeLiveActivityStateSource = readFileSync(
    resolve(TestDirectory, '..', 'src/route-live-activity-state.ts'),
    'utf8'
  );
  const policyPreviewRouteSource = readFileSync(
    resolve(TestDirectory, '..', 'src/PolicyPreviewRoutePanel.tsx'),
    'utf8'
  );

  expect(routeLiveActivityStateSource).not.toContain('snapshot.appGameAdapterDispatchPreflightReadModel');
  expect(routeLiveActivityStateSource).not.toContain('snapshot.appGameAdapterDispatchResultReadModel');
  expect(routeLiveActivityStateSource).not.toContain('snapshot.appGameAdapterDispatchExecutedResult');
  expect(routeLiveActivityStateSource).not.toContain('snapshot.appGameTimerParentSurfaceReadModel');
  expect(routeLiveActivityStateSource).not.toContain('appGameAdapterExecutionReadinessEvent');
  expect(routeLiveActivityStateSource).not.toContain('appGameAdapterExecutionReadinessReadModel');
  expect(routeLiveActivityStateSource).not.toContain('appGameAdapterDispatchPreflightEvent');
  expect(routeLiveActivityStateSource).not.toContain('appGameAdapterDispatchPreflightReadModel');
  expect(routeLiveActivityStateSource).not.toContain('appGameAdapterDispatchResultEvent');
  expect(routeLiveActivityStateSource).not.toContain('appGameAdapterDispatchResultReadModel');
  expect(routeLiveActivityStateSource).not.toContain('appGameAdapterDispatchExecutedEvent');
  expect(routeLiveActivityStateSource).not.toContain('appGameAdapterDispatchExecutedResult');
  expect(routeLiveActivityStateSource).not.toContain('appGameTimerParentSurfaceEvent');
  expect(routeLiveActivityStateSource).not.toContain('appGameTimerParentSurfaceReadModel');
  expect(routeLiveActivityStateSource).not.toContain('appGameNotificationReadinessEvent');
  expect(routeLiveActivityStateSource).not.toContain('appGamePlatformProofStatusEvent');
  expect(routeLiveActivityStateSource).not.toContain('appGameChildRuntimeTransportReceiptEvent');
  expect(routeLiveActivityStateSource).not.toContain('appGamePolicyReadinessEvent');
  expect(diagnosticsExportSource).toContain(
    'resolveSnapshotLiveActivityState(state.routeSnapshot?.liveActivity ?? null)'
  );
  expect(policyPreviewRouteSource).not.toContain('liveActivity.policyPreviewEvent');
  expect(policyPreviewRouteSource).not.toContain('liveActivity.policyPreviewReadModel');
  expect(policyPreviewRouteSource).not.toContain('createPolicyPreviewPanelIntent');
  expect(policyPreviewRouteSource).toContain('type ParentPolicyPreviewPanelSnapshot');
  expect(routeLiveActivityStateSource).not.toContain('snapshot.policyPreviewEvent');
  expect(routeLiveActivityStateSource).not.toContain('snapshot.policyPreviewReadModel');
});

it('product bridge guard: product route rendering keeps the bridge contract thin', () => {
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(bridgeContractSource).toContain('ParentAppGameActionRowSnapshot');
  expect(bridgeContractSource).toContain('ParentAppGameAdapterDispatchPanelSnapshot');
  expect(bridgeContractSource).toContain('ParentAppGameTimerParentSurfacePanelSnapshot');
  expect(bridgeContractSource).toContain('appGameAdapterDispatchPanel?: ParentAppGameAdapterDispatchPanelSnapshot | null;');
  expect(bridgeContractSource).toContain('appGameTimerParentSurfacePanel?: ParentAppGameTimerParentSurfacePanelSnapshot | null;');
  expect(bridgeContractSource).not.toContain('policyPreviewEvent?: ParentRouteEventSnapshot | null;');
  expect(bridgeContractSource).not.toContain(
    'policyPreviewReadModel?: ParentPolicyPreviewReadModelSnapshot | null;'
  );
  expect(bridgeContractSource).not.toContain('appGameAdapterDispatchPreflightReadModel?: ParentUnknownRecord | null;');
  expect(bridgeContractSource).not.toContain('appGameAdapterDispatchResultReadModel?: ParentUnknownRecord | null;');
  expect(bridgeContractSource).not.toContain('appGameAdapterDispatchExecutedResult?: ParentUnknownRecord | null;');
  expect(bridgeContractSource).not.toContain('appGameTimerParentSurfaceReadModel?: ParentUnknownRecord | null;');
});

it('product bridge guard: raw-event live-activity reduction is no longer used by the app runtime', () => {
  for (const sourceFile of ProductBridgeSourceFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', sourceFile), 'utf8');

    if (sourceFile === 'src/ParentPortalRoute.tsx') {
      expect(source).toContain('resolveSnapshotLiveActivityState(');
      expect(source).not.toContain('resolveLiveActivityState(');
      continue;
    }

      expect(source).not.toContain('resolveLiveActivityState(');
    }

    const portalDomainLiveActivitySource = readFileSync(
      resolve(TestDirectory, '..', '..', '..', 'packages', 'portal-domain', 'src', 'live-activity-state.ts'),
      'utf8'
    );

    expect(portalDomainLiveActivitySource).not.toContain('export function resolveLiveActivityState(');
    expect(portalDomainLiveActivitySource).not.toContain('parseBrowserManagedStatus(');
    expect(portalDomainLiveActivitySource).not.toContain('parseNetworkFlowReadModel(');
    expect(portalDomainLiveActivitySource).not.toContain('latestEvent(');

    const liveActivityTypeModuleSource = readFileSync(resolve(TestDirectory, '..', 'src/live-activity-state.ts'), 'utf8');
    expect(liveActivityTypeModuleSource).toContain('export type PortalLiveActivityState =');
    expect(liveActivityTypeModuleSource).not.toContain('resolveSnapshotLiveActivityState(');
    expect(liveActivityTypeModuleSource).not.toContain('resolveLiveActivityState(');

  for (const sourceFile of ProductLiveActivityResolverCallers) {
    const source = readFileSync(resolve(TestDirectory, '..', sourceFile), 'utf8');

    if (
      sourceFile === 'src/diagnostics-export.ts' ||
      sourceFile === 'src/ParentPortalRoute.tsx' ||
      sourceFile === 'src/PortalApp.tsx' ||
      sourceFile === 'src/route-live-activity-state.ts'
    ) {
      expect(source).toContain('resolveSnapshotLiveActivityState(');
      expect(source).not.toContain('resolveLiveActivityState(');
      continue;
    }
  }
});

it('product bridge guard: product source stays decoupled from the TS agent protocol domain', () => {
  for (const sourceFile of listSourceFiles(ProductSourceDirectory)) {
    const source = readFileSync(sourceFile, 'utf8');
    const relativePath = relative(resolve(TestDirectory, '..'), sourceFile);
    if (source.includes('@ocentra-parent/agent-protocol-domain')) {
      throw new Error(`unexpected TS protocol-domain import in ${relativePath}`);
    }
  }
});
