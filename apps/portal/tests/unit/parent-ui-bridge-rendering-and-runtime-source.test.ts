import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';

const BridgeContractFile = 'generated/parent-ui-bridge.ts';
const TestDirectory = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const SnapshotOnlyPanelBindings = [
  'activityState.networkEvidenceSummary ?? null',
  'activityState.policyPreviewPanel ?? null',
  'activityState.appGameNotificationParentSurfacePanel ?? null',
  'activityState.appGamePolicyReadinessPanel ?? null',
  'activityState.appGamePlatformProofStatusPanel ?? null',
  'activityState.appGameChildRuntimeTransportReceiptPanel ?? null',
  'activityState.appGameAdapterDispatchPanel ?? null',
  'activityState.appGameTimerParentSurfacePanel ?? null',
];
const SocialProofPanelBindings = [
  'BrowserParentExplanationRoutePanel',
  'SocialAuditExplanationRoutePanel',
  'SocialDashboardRoutePanel',
  'SocialAlertReportRoutePanel',
  'browserParentExplanationPanel',
  'socialAuditExplanationPanel',
  'socialDashboardPanel',
  'socialAlertReportPanel',
  'browserActionIntentStreamStatusPanel',
];
const ForbiddenSocialReadModels = [
  'appGameNotificationParentSurfaceIntentReadModel',
  'appGamePolicyReadinessReadModel',
  'appGamePlatformProofStatusReadModel',
  'appGameChildRuntimeTransportReceiptReadModel',
  'appGameAdapterExecutionReadinessReadModel',
  'appGameAdapterDispatchPreflightReadModel',
  'appGameAdapterDispatchResultReadModel',
  'appGameAdapterDispatchExecutedResult',
  'appGameTimerParentSurfaceReadModel',
];
const AppGamePanelBindings = [
  'AppGameNotificationParentSurfaceRoutePanel',
  'AppGamePolicyReadinessRoutePanel',
  'AppGamePlatformProofStatusRoutePanel',
  'AppGameChildRuntimeTransportReceiptRoutePanel',
  'AppGameAdapterDispatchRoutePanel',
  'AppGameTimerParentSurfaceRoutePanel',
  'appGameNotificationParentSurfacePanel',
  'appGamePolicyReadinessPanel',
  'appGamePlatformProofStatusPanel',
  'appGameChildRuntimeTransportReceiptPanel',
  'appGameAdapterDispatchPanel',
  'appGameTimerParentSurfacePanel',
];
const ForbiddenAppGameIntentBuilders = [
  'createAppGameNotificationParentSurfacePanelIntent',
  'createAppGamePolicyReadinessPanelIntent',
  'createAppGamePlatformProofStatusPanelIntent',
  'createAppGameChildRuntimeTransportReceiptPanelIntent',
  'createAppGameAdapterDispatchPreflightPanelIntent',
  'createAppGameAdapterDispatchResultPanelIntent',
  'createAppGameTimerParentSurfacePanelIntent',
  'createAppGameTimerParentPreferenceSetupRequestPayload',
];

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
  expectContainsAll(parentPortalRouteSource, SnapshotOnlyPanelBindings);
  expectContainsAll(proofPanelsSocialSource, SocialProofPanelBindings);
  expectNotContainsAll(proofPanelsSocialSource, ForbiddenSocialReadModels);
  expectContainsAll(proofPanelsAppGameSource, AppGamePanelBindings);
  expectNotContainsAll(proofPanelsAppGameSource, ForbiddenAppGameIntentBuilders);
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
  expect(bridgeContractSource).toContain(
    'appGameAdapterDispatchPanel?: ParentAppGameAdapterDispatchPanelSnapshot | null;'
  );
  expect(bridgeContractSource).toContain(
    'appGameTimerParentSurfacePanel?: ParentAppGameTimerParentSurfacePanelSnapshot | null;'
  );
  expect(bridgeContractSource).not.toContain('policyPreviewEvent?: ParentRouteEventSnapshot | null;');
  expect(bridgeContractSource).not.toContain('policyPreviewReadModel?: ParentPolicyPreviewReadModelSnapshot | null;');
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

  const resolverCallers = [
    'src/diagnostics-export.ts',
    'src/ParentPortalRoute.tsx',
    'src/portal-app-behavior.ts',
    'src/route-live-activity-state.ts',
  ];
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

function expectContainsAll(source: string, snippets: readonly string[]): void {
  for (const snippet of snippets) {
    expect(source).toContain(snippet);
  }
}

function expectNotContainsAll(source: string, snippets: readonly string[]): void {
  for (const snippet of snippets) {
    expect(source).not.toContain(snippet);
  }
}
