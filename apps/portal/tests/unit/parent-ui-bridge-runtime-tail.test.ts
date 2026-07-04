import { readdirSync, readFileSync, statSync } from 'node:fs';
import { dirname, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';

const BridgeContractFile = 'generated/parent-ui-bridge.ts';
const ProductSnapshotOnlyRouteFiles = [
  'src/ParentPortalRoute.tsx',
  'src/AiRuntimeRoutePanel.tsx',
  'src/PolicyPreviewRoutePanel.tsx',
  'src/SocialAuditExplanationRoutePanel.tsx',
  'src/SocialDashboardRoutePanel.tsx',
  'src/SocialAlertReportRoutePanel.tsx',
  'src/TrackingStatusRoutePanel.tsx',
  'src/AppGamePolicyReadinessRoutePanel.tsx',
  'src/AppGamePlatformProofStatusRoutePanel.tsx',
  'src/AppGameChildRuntimeTransportReceiptRoutePanel.tsx',
  'src/AppGameAdapterDispatchRoutePanel.tsx',
  'src/AppGameTimerParentSurfaceRoutePanel.tsx',
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
    if ((entry.name.endsWith('.ts') || entry.name.endsWith('.tsx')) && statSync(entryPath).isFile()) {
      files.push(entryPath);
    }
  }

  return files;
}

it('product bridge guard: product route rendering keeps the route shell snapshot-only', () => {
  const parentPortalRouteSource = readFileSync(resolve(TestDirectory, '..', 'src/ParentPortalRoute.tsx'), 'utf8');
  const proofPanelsSocialSource = readFileSync(
    resolve(TestDirectory, '..', 'src/portal-proof-panels-social-renderers.tsx'),
    'utf8'
  );
  const proofPanelsAppGameSource = readFileSync(
    resolve(TestDirectory, '..', 'src/portal-proof-panels-app-game-renderers.tsx'),
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
  expect(proofPanelsSocialSource).toContain('BrowserParentExplanationRoutePanel');
  expect(proofPanelsSocialSource).toContain('SocialAuditExplanationRoutePanel');
  expect(proofPanelsSocialSource).toContain('SocialDashboardRoutePanel');
  expect(proofPanelsSocialSource).toContain('SocialAlertReportRoutePanel');
  expect(proofPanelsSocialSource).toContain('browserParentExplanationPanel');
  expect(proofPanelsSocialSource).toContain('socialAuditExplanationPanel');
  expect(proofPanelsSocialSource).toContain('socialDashboardPanel');
  expect(proofPanelsSocialSource).toContain('socialAlertReportPanel');
  expect(proofPanelsSocialSource).toContain('browserActionIntentStreamStatusPanel');
  expect(proofPanelsSocialSource).not.toContain('appGameNotificationParentSurfaceIntentReadModel');
  expect(proofPanelsSocialSource).not.toContain('appGamePolicyReadinessReadModel');
  expect(proofPanelsSocialSource).not.toContain('appGamePlatformProofStatusReadModel');
  expect(proofPanelsSocialSource).not.toContain('appGameChildRuntimeTransportReceiptReadModel');
  expect(proofPanelsSocialSource).not.toContain('appGameAdapterExecutionReadinessReadModel');
  expect(proofPanelsSocialSource).not.toContain('appGameAdapterDispatchPreflightReadModel');
  expect(proofPanelsSocialSource).not.toContain('appGameAdapterDispatchResultReadModel');
  expect(proofPanelsSocialSource).not.toContain('appGameAdapterDispatchExecutedResult');
  expect(proofPanelsSocialSource).not.toContain('appGameTimerParentSurfaceReadModel');
  expect(proofPanelsAppGameSource).toContain('AppGameNotificationParentSurfaceRoutePanel');
  expect(proofPanelsAppGameSource).toContain('AppGamePolicyReadinessRoutePanel');
  expect(proofPanelsAppGameSource).toContain('AppGamePlatformProofStatusRoutePanel');
  expect(proofPanelsAppGameSource).toContain('AppGameChildRuntimeTransportReceiptRoutePanel');
  expect(proofPanelsAppGameSource).toContain('AppGameAdapterDispatchRoutePanel');
  expect(proofPanelsAppGameSource).toContain('AppGameTimerParentSurfaceRoutePanel');
  expect(proofPanelsAppGameSource).toContain('appGameNotificationParentSurfacePanel');
  expect(proofPanelsAppGameSource).toContain('appGamePolicyReadinessPanel');
  expect(proofPanelsAppGameSource).toContain('appGamePlatformProofStatusPanel');
  expect(proofPanelsAppGameSource).toContain('appGameChildRuntimeTransportReceiptPanel');
  expect(proofPanelsAppGameSource).toContain('appGameAdapterDispatchPanel');
  expect(proofPanelsAppGameSource).toContain('appGameTimerParentSurfacePanel');
  expect(proofPanelsAppGameSource).not.toContain('createAppGameNotificationParentSurfacePanelIntent');
  expect(proofPanelsAppGameSource).not.toContain('createAppGamePolicyReadinessPanelIntent');
  expect(proofPanelsAppGameSource).not.toContain('createAppGamePlatformProofStatusPanelIntent');
  expect(proofPanelsAppGameSource).not.toContain('createAppGameChildRuntimeTransportReceiptPanelIntent');
  expect(proofPanelsAppGameSource).not.toContain('createAppGameAdapterDispatchPreflightPanelIntent');
  expect(proofPanelsAppGameSource).not.toContain('createAppGameAdapterDispatchResultPanelIntent');
  expect(proofPanelsAppGameSource).not.toContain('createAppGameTimerParentSurfacePanelIntent');
  expect(proofPanelsAppGameSource).not.toContain('createAppGameTimerParentPreferenceSetupRequestPayload');

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
  const portalSourceFiles = [
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
  for (const sourceFile of portalSourceFiles) {
    const source = readFileSync(resolve(TestDirectory, '..', sourceFile), 'utf8');

    if (sourceFile === 'src/ParentPortalRoute.tsx' || sourceFile === 'src/portal-app-behavior.ts') {
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

  const resolverCallers = ProductLiveActivityResolverCallers;
  for (const sourceFile of resolverCallers) {
    const source = readFileSync(resolve(TestDirectory, '..', sourceFile), 'utf8');

    if (
      sourceFile === 'src/diagnostics-export.ts' ||
      sourceFile === 'src/ParentPortalRoute.tsx' ||
      sourceFile === 'src/portal-app-behavior.ts' ||
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
