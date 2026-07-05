import { readdirSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';

const ProductBridgeSourceFiles = [
  'src/main.ts',
  'src/host-bridge.ts',
  'src/portal-runtime-controller.ts',
  'src/portal-runtime-controller-actions.ts',
  'src/portal-runtime-controller-session.ts',
  'src/portal-runtime-controller-session-subscription.ts',
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
  'src/portal-app-behavior.ts',
  'src/route-live-activity-state.ts',
];
const TestDirectory = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const ProductSourceDirectory = resolve(TestDirectory, '..', 'src');
const RetiredSchemaDomainPortalContractsSpecifier =
  '@ocentra-parent/' + 'schema-domain/' + 'portal-contracts';
const RetiredSchemaDomainLoggingContractsSpecifier =
  '@ocentra-parent/' + 'schema-domain/' + 'logging-contracts';
const RetiredSchemaDomainGeneratedLoggingContractsSpecifier =
  '@ocentra-parent/' + 'schema-domain/generated/' + 'logging-contracts';

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
  const mainSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-runtime-controller.ts'), 'utf8');
  const sessionSource = readFileSync(
    resolve(TestDirectory, '..', 'src/portal-runtime-controller-session.ts'),
    'utf8'
  );
  const subscriptionSource = readFileSync(
    resolve(TestDirectory, '..', 'src/portal-runtime-controller-session-subscription.ts'),
    'utf8'
  );
  const hostBridgeSource = readFileSync(resolve(TestDirectory, '..', 'src/host-bridge.ts'), 'utf8');

  expect(mainSource).toContain('createPortalRuntimeSession');
  expect(mainSource).not.toContain('bridge.subscribe(');
  expect(sessionSource).toContain('createPortalRuntimeSubscriptionManager');
  expect(sessionSource).toContain('disposeRouteSubscription');
  expect(subscriptionSource).toContain('bridge.subscribe(');
  expect(hostBridgeSource).toContain('ParentBridgeCommand.Subscribe');
  expect(hostBridgeSource).toContain('ParentBridgeCommand.Unsubscribe');
  expect(hostBridgeSource).toContain('ParentHostBridgeRuntime.TauriEventModule');
  expect(hostBridgeSource).toContain('createUnavailableDevWebHostBridge');
  expect(hostBridgeSource).not.toContain('createPresentationOnlyDevWebHostBridge');
  expect(hostBridgeSource).not.toContain('createDevSnapshot(');
  expect(hostBridgeSource).not.toContain('createUnavailableDevBridgeSnapshot');
  expect(hostBridgeSource).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
  expect(hostBridgeSource).not.toContain('@ocentra-parent/portal-domain/contracts');
  expect(hostBridgeSource).not.toContain('@ocentra-parent/portal-domain/routes');
});

it('product bridge guard: portal app route content stays on generated bridge and package route helpers', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/PortalApp.tsx'), 'utf8');
  const routeContentSource = readFileSync(resolve(TestDirectory, '..', 'src/portal-route-content.tsx'), 'utf8');

  expect(source).toContain("from '@ocentra-parent/portal-domain/routes'");
  expect(source).toContain('PortalRouteDescriptors');
  expect(source).toContain('ParentRouteId');
  expect(routeContentSource).toContain('renderPortalRouteContent');
  expect(routeContentSource).toContain('renderCommands');
  expect(routeContentSource).toContain('renderEvents');
  expect(routeContentSource).toContain('renderDevLogPanel');
  expect(routeContentSource).toContain('ParentRoute.Commands');
  expect(routeContentSource).toContain('ParentRoute.Events');
  expect(routeContentSource).toContain('ParentRoute.Logs');
  expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
  expect(source).toContain('./portal-route-content');
  expect(source).not.toContain('./portal-route-descriptor');
  expect(source).not.toContain('PortalRouteValue');
});

it('product bridge guard: portal main route parsing uses Rust-generated route helpers', () => {
  const source = readFileSync(resolve(TestDirectory, '..', 'src/main.ts'), 'utf8');
  const bridgeContractSource = readFileSync(resolve(TestDirectory, '..', BridgeContractFile), 'utf8');

  expect(source).toContain("from '../generated/parent-ui-bridge'");
  expect(source).toContain('parentRouteFromHashPath');
  expect(source).toContain('parentRouteHashPath');
  expect(source).toContain('ParentRoute.Overview');
  expect(source).not.toContain('@ocentra-parent/portal-domain/routes');
  expect(source).not.toContain('PortalRoutes');
  expect(bridgeContractSource).toContain('ParentRoute.AppLayout');
  expect(bridgeContractSource).toContain('ParentRoute.FrameTuner');
  expect(bridgeContractSource).toContain("AppLayout: 'app-layout'");
  expect(bridgeContractSource).toContain("FrameTuner: 'frame-tuner'");
  expect(bridgeContractSource).not.toContain("FrameTuner: 'app-layout'");
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
  expect(refreshSource).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
  expect(refreshSource).not.toContain('PortalConnectionState');
  expect(networkDrawerSource).toContain('networkEvidenceSummary: networkEvidenceSummary ?? null');
  expect(networkDrawerSource).not.toContain('liveActivity.policyPreviewReadModel');
  expect(networkRefreshSource).toContain("from '../generated/parent-ui-bridge'");
  expect(networkRefreshSource).toContain('ParentBridgeConnectionState');
  expect(networkRefreshSource).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
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
    expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
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
    expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
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
  expect(parentPortalRouteSource).toContain("browserPanelSnapshot(state, 'browserParentExplanation')");
  expect(parentPortalRouteSource).toContain("browserPanelSnapshot(state, 'socialAlertReport')");
  expect(parentPortalRouteSource).toContain("browserPanelSnapshot(state, 'browserActionIntentStreamStatus')");
  expect(parentPortalRouteSource).toContain('state.routeSnapshot?.browserPanels');
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
    expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
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
  expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
  expect(source).not.toContain('PortalRoute.');
  expect(source).not.toContain('PortalConnectionState.');
  expect(source).not.toContain('PortalDom.HashPrefix');
});

it('product bridge guard: portal app shell uses Rust-generated route IDs and constants', () => {
  const generatedOnlyShellFiles = [
    'src/main.ts',
    'src/PortalApp.tsx',
    'src/PortalUnifiedChrome.tsx',
    'src/portal-route-content.tsx',
  ];

  for (const file of generatedOnlyShellFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', file), 'utf8');

    expect(source).toContain("from '../generated/parent-ui-bridge'");
    expect(source).toContain('ParentRoute');
    expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
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
  expect(detailListSource).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
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
    expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
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
    expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
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
    expect(source).not.toContain(RetiredSchemaDomainPortalContractsSpecifier);
    expect(source).not.toContain('type PortalDetailValue');
    expect(source).not.toContain('decodePortalDetailValue');
  }
});
