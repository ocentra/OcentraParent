import { describe, expect, it } from 'vitest';
import {
  AppGameNotificationParentSurfaceIntentReadModelSchema,
  RequiredAppGameNotificationParentSurfaceIntentNonClaims,
} from '@ocentra-parent/parent-domain/app-game-notification-parent-surface-intent';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import {
  createAppGameNotificationParentSurfacePanelIntent,
  type AppGameNotificationParentSurfacePanelIntent,
} from '../src/app-game-notification-parent-surface-panel';
import { shouldRenderAppGameNotificationParentSurfaceRoute } from '../src/AppGameNotificationParentSurfaceRoutePanel';

describe('app/game notification parent surface panel', () => {
  it('renders schema-backed parent-surface intent rows without runtime claims', () => {
    const readModel = AppGameNotificationParentSurfaceIntentReadModelSchema.parse(parentSurfaceReadModel());
    const intent = createAppGameNotificationParentSurfacePanelIntent(readModel);

    expect(metricPairs(intent)).toContainEqual(['Rows returned', '2']);
    expect(metricPairs(intent)).toContainEqual(['Status', '1 manual action']);
    expect(metricPairs(intent)).toContainEqual(['History visibility', '2 history rows']);
    expect(intent.productClaim).toContain(
      'provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed'
    );
    expect(intent.rows.map((row) => row.title)).toEqual([
      'app-game-notification-parent-surface-time-limit',
      'app-game-notification-parent-surface-unavailable',
    ]);
    expect(rowPairs(panelRow(intent, 0))).toContainEqual([
      'Evidence references',
      'provider-status-ref-time-limit, preference-result-time-limit',
    ]);
    expect(rowPairs(panelRow(intent, 0))).toContainEqual([
      'Runtime reference',
      'scheduler-entry-app-game-time-limit, outbox-record-app-game-time-limit',
    ]);
    expect(rowPairs(panelRow(intent, 1))).toContainEqual(['Status', 'unavailable-visible']);
  });

  it('keeps absent or invalid service input explicit instead of inventing rows', () => {
    const intent = createAppGameNotificationParentSurfacePanelIntent({ rows: [] });

    expect(intent.state).toBe('unavailable');
    expect(intent.rows).toEqual([]);
    expect(intent.emptyMessage).toBe('No app/game notification parent-surface intent has been reported yet.');
    expect(metricPairs(intent)).toContainEqual(['Rows returned', '0']);
    expect(metricPairs(intent)).toContainEqual(['Runtime reference', 'service event not reported']);
  });

  it('mounts only on the App/Game Sessions route', () => {
    expect(shouldRenderAppGameNotificationParentSurfaceRoute(PortalRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameNotificationParentSurfaceRoute(PortalRoute.Notifications)).toBe(false);
  });
});

function metricPairs(intent: AppGameNotificationParentSurfacePanelIntent) {
  return intent.metrics.map((metric) => [metric.label, metric.value]);
}

function rowPairs(row: AppGameNotificationParentSurfacePanelIntent['rows'][number]) {
  return row.details.map((detail) => [detail.label, detail.value]);
}

function panelRow(intent: AppGameNotificationParentSurfacePanelIntent, index: number) {
  const row = intent.rows[index];
  expect(row).toBeDefined();
  return row as AppGameNotificationParentSurfacePanelIntent['rows'][number];
}

function parentSurfaceReadModel() {
  return {
    schemaVersion: 'v0.6',
    intentId: 'app-game-notification-parent-surface-intent-proof',
    generatedAt: '2026-06-05T09:12:00Z',
    family: { familyId: 'family-app-game-parent-surface' },
    sourceProviderStatusHandoffId: 'app-game-provider-status-handoff-parent-surface',
    sourcePreferenceStatusHandoffId: 'app-game-preference-status-handoff-parent-surface',
    sourceContractRefs: [
      'app-game-notification-provider-status-handoff',
      'app-game-notification-preference-status-handoff',
    ],
    rows: [
      parentSurfaceRow('time-limit', {
        providerStatus: 'manual-required',
        deliveryResultState: 'manual-required',
        parentPreferenceState: 'manual-setup-required',
        quietHoursDecision: 'manual-required',
        parentSurfaceStatus: 'manual-action-required',
        historyVisibility: 'manual-review-only',
        preferenceVisibility: 'preference-setup-required',
        sourceRefsEnabled: true,
      }),
      parentSurfaceRow('unavailable', {
        providerStatus: 'unavailable',
        deliveryResultState: 'not-sent',
        parentPreferenceState: 'channel-disabled',
        quietHoursDecision: 'allow',
        parentSurfaceStatus: 'unavailable-visible',
        historyVisibility: 'unavailable-row-visible',
        preferenceVisibility: 'preference-disabled-visible',
        sourceRefsEnabled: false,
      }),
    ],
    manualActionRequiredCount: 1,
    unavailableVisibleCount: 1,
    historyVisibleCount: 2,
    preferenceSetupRequiredCount: 1,
    parentSurfaceNonClaims: RequiredAppGameNotificationParentSurfaceIntentNonClaims,
    parentNotificationUiRendered: false,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    productionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  } as const;
}

function parentSurfaceRow(
  label: string,
  input: {
    readonly providerStatus: string;
    readonly deliveryResultState: string;
    readonly parentPreferenceState: string;
    readonly quietHoursDecision: string;
    readonly parentSurfaceStatus: string;
    readonly historyVisibility: string;
    readonly preferenceVisibility: string;
    readonly sourceRefsEnabled: boolean;
  }
) {
  return {
    surfaceRowId: `app-game-notification-parent-surface-${label}`,
    sourceProviderHandoffRowId: `provider-status-handoff-${label}`,
    sourcePreferenceHandoffRowId: `preference-status-handoff-${label}`,
    sourceSchedulerEntryRef: input.sourceRefsEnabled ? `scheduler-entry-app-game-${label}` : null,
    sourceOutboxRecordRef: input.sourceRefsEnabled ? `outbox-record-app-game-${label}` : null,
    providerStatus: input.providerStatus,
    deliveryResultState: input.deliveryResultState,
    parentPreferenceState: input.parentPreferenceState,
    quietHoursDecision: input.quietHoursDecision,
    providerChannel: 'in-app',
    parentSurfaceStatus: input.parentSurfaceStatus,
    historyVisibility: input.historyVisibility,
    preferenceVisibility: input.preferenceVisibility,
    drillInRefs: [`provider-status-ref-${label}`, `preference-result-${label}`],
    auditRefs: [`provider-status-audit-${label}`, `preference-status-audit-${label}`],
    manualProofRequirements: [`manual-proof-provider-${label}`, `manual-proof-preference-${label}`],
    minimalSurfacePayloadBoundary:
      'Parent surface intent contains status refs and setup requirements only; sensitive app/game evidence stays behind authenticated drill-in.',
    sensitiveDetailIncluded: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
  } as const;
}
