import { describe, expect, it } from 'vitest';
import { TrackingStatusProofArtifacts } from '../../src/tracking-status-proof-artifacts';
import {
  trackingFamilyDashboardHostedRollupProof,
  trackingStatusLiveSummary,
  trackingStatusProofRows,
  trackingStatusServiceDataCoverage,
  trackingUnsupportedManualPlatformProof,
} from '../../src/tracking-status-panel';
import {
  ExpectedFamilyDashboardHostedRollupRows,
  ExpectedLegacyTrackingServiceDataCoverage,
  ExpectedMissingDeviceRows,
  ExpectedNotificationParentSurfaceRows,
  ExpectedParentActionReadinessRows,
  ExpectedReportExportHostedUiProof,
  ExpectedReportPolicyConsumerHostedUiProof,
  ExpectedRetentionDeletedRow,
  ExpectedTrackingLiveSummary,
  ExpectedTrackingProofArtifacts,
  ExpectedTrackingServiceDataCoverage,
  ExpectedTrackingStateTitles,
  ExpectedUnsupportedManualRows,
  TrackingReadModel,
  TrackingRetentionSettingsWriteDefaults,
  legacyTrackingReadModel,
  parsedTrackingRetentionSettingsWriteResult,
  trackingProjectionInput,
} from './tracking-status-panel.fixtures';
import { trackingEvidenceDrawerHostedUiProof } from '../../src/tracking-evidence-drawer-hosted-ui-proof';
import { trackingRetentionSettingsHostedUiProof } from '../../src/tracking-retention-settings-hosted-ui-proof';
import { trackingChildCheckInProof, trackingChildRuntimeUiProof } from '../../src/tracking-child-check-in-proof';
import { trackingMissingDeviceHostedUiProof } from '../../src/tracking-missing-device-hosted-ui-proof';
import { trackingNotificationParentSurfaceHostedUiProof } from '../../src/tracking-notification-parent-surface-hosted-ui-proof';
import { trackingParentActionReadinessHostedUiProof } from '../../src/tracking-parent-action-readiness-hosted-ui-proof';
import { trackingReportExportHostedUiProof } from '../../src/tracking-report-export-hosted-ui-proof';
import { trackingReportPolicyConsumerHostedUiProof } from '../../src/tracking-report-policy-consumer-hosted-ui-proof';

describe('tracking status proof surface', () => {
  it('lists the first-target tracking states as fixture proof without product claims', () => {
    const rows = trackingStatusProofRows();

    expect(rows.map((row) => row.title)).toEqual(ExpectedTrackingStateTitles);
    expect(rows.every((row) => row.proofTier === 'P1 fixture proof')).toBe(true);
    expect(rows.every((row) => row.productClaim === 'No product claim')).toBe(true);
    expect(rows.every((row) => row.proofArtifact.startsWith('output/tracking-plan-proof/'))).toBe(true);
    expect(rows.map((row) => row.proofArtifact)).toEqual(ExpectedTrackingProofArtifacts);
    expect(rows.filter((row) => row.missingProof === 'Physical device proof required').map((row) => row.title)).toEqual(
      ['Permission required', 'Temporary live', 'Missing device']
    );
  });

  it('marks deleted location history as hidden without rendering deleted evidence ids', () => {
    const retentionRow = trackingStatusProofRows().find((row) => row.title === 'Retention deleted');

    expect(retentionRow).toEqual(ExpectedRetentionDeletedRow);
    expect(JSON.stringify(retentionRow)).not.toContain('location-evidence-1');
  });

  it('summarizes the live service-backed tracking read model without product completion claims', () => {
    const liveActivity = trackingProjectionInput(TrackingReadModel);

    expect(trackingStatusLiveSummary(liveActivity)).toEqual(ExpectedTrackingLiveSummary);
  });

  it('renders service-data coverage from the live read model without device or provider claims', () => {
    const liveActivity = trackingProjectionInput(TrackingReadModel);

    expect(trackingStatusServiceDataCoverage(liveActivity)).toEqual(ExpectedTrackingServiceDataCoverage);
  });

  it('falls back to legacy summary fields when active summary/count fields are absent', () => {
    const liveActivity = trackingProjectionInput(legacyTrackingReadModel());

    expect(trackingStatusLiveSummary(liveActivity)).toEqual(ExpectedTrackingLiveSummary);
    expect(trackingStatusServiceDataCoverage(liveActivity)).toEqual(ExpectedLegacyTrackingServiceDataCoverage);
  });

  it('renders evidence drawer proof from the selected citation without evaluator or dispatch claims', () => {
    const liveActivity = trackingProjectionInput(TrackingReadModel);
    const liveSummary = trackingStatusLiveSummary(liveActivity);

    expect(trackingEvidenceDrawerHostedUiProof(liveSummary.citations[0] ?? null)).toEqual({
      title: 'Evidence drawer proof',
      body: 'Hosted route renders a read-only evidence drawer from the selected service-backed citation without evaluating policy or dispatching actions.',
      proofTier: 'P2 service proof',
      drawerMode: 'read-only evidence drawer',
      sourceEventId: 'tracking-event-1',
      evidenceReferences: 'tracking-evidence-1',
      deletedEvidence: 'Not reported',
      proofArtifact: TrackingStatusProofArtifacts.HostedEvidenceDrawer,
      boundary:
        'Display-only evidence drill-in; policy evaluation, action dispatch, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      policyEvaluatorClaimedRows: '0',
      actionDispatchClaimedRows: '0',
      childDeviceDeliveryClaimedRows: '0',
      providerDeliveryClaimedRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
    });
  });
});

describe('tracking retention settings hosted proof surface', () => {
  it('renders retention local service write result without product-ready mutation claims', () => {
    expect(trackingRetentionSettingsHostedUiProof(parsedTrackingRetentionSettingsWriteResult())).toMatchObject({
      title: 'Retention settings read-model UI',
      proofArtifact: TrackingStatusProofArtifacts.RetentionSettingsReadModel,
      writeCommandProofArtifact: TrackingStatusProofArtifacts.RetentionSettingsWriteCommand,
      localStateProofArtifact: TrackingStatusProofArtifacts.RetentionLocalServiceState,
      writePreflight: {
        title: 'Retention local service write result',
        commandId: TrackingRetentionSettingsWriteDefaults.CommandId,
        settingsKind: TrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
        writeState: TrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
        acceptedAt: '2026-06-06T19:40:00.000Z',
        sourceMutationProofRefs: TrackingRetentionSettingsWriteDefaults.MutationProofRef,
        sourceWriterIntentRefs: TrackingRetentionSettingsWriteDefaults.WriterIntentRef,
        sourceReadModelProofRefs: TrackingRetentionSettingsWriteDefaults.ReadModelProofRefs[0],
        appliedRetentionWindowHours: '168',
        appliedDeleteAfterAlertResolved: '1',
        parentExportPrepared: '0',
        remoteSyncEnabled: '0',
        remoteAiEnabled: '0',
        localServiceStateRevision: '1',
        localServiceStateSnapshotRef: TrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef,
        durableSettingsPersistedRows: '1',
        commandTransportClaimedRows: '1',
        serviceWritePreflightClaimedRows: '1',
        serviceMutationExecutedRows: '1',
        platformRuntimeClaimedRows: '0',
        childDeviceDeliveryClaimedRows: '0',
        providerDeliveryClaimedRows: '0',
        notificationReceiptClaimedRows: '0',
        physicalDeviceClaimedRows: '0',
        authorityClaimedRows: '0',
        productClaimReadyRows: '0',
        parserReason: 'Not reported',
        boundary:
          'Portal command/result rendering proves local service mutation execution, local durable settings persistence, and local state revision only; product-ready writable settings, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
      },
    });
  });
});

describe('tracking dashboard and platform proof surface', () => {
  it('renders family dashboard rollup rows without provider, device, authority, or product-ready claims', () => {
    const proof = trackingFamilyDashboardHostedRollupProof();

    expect(proof).toEqual({
      title: 'Family dashboard tracking rollup',
      body: 'Hosted route renders family active, child attention, and retention audit rollups from existing tracking proof refs without claiming device delivery.',
      proofTier: 'P2 service proof',
      rowsReturned: '3',
      proofArtifact: TrackingStatusProofArtifacts.FamilyDashboardRollup,
      boundary:
        'Hosted dashboard rollup rendering only; child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      childDeviceDeliveryClaimedRows: '0',
      providerDeliveryClaimedRows: '0',
      notificationReceiptClaimedRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
      productClaimReadyRows: '0',
      rows: ExpectedFamilyDashboardHostedRollupRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:product ready|physical device proved|provider delivered)/iu);
  });

  it('renders report export read-model packets without raw export, mutation, runtime, or product claims', () => {
    const proof = trackingReportExportHostedUiProof();

    expect(proof).toEqual(ExpectedReportExportHostedUiProof);
    expect(JSON.stringify(proof)).not.toMatch(
      /(?:raw location payload exported|service mutation executed|product-ready export delivered)/iu
    );
  });

  it('renders report policy consumer rows without AI, mutation, device, or product claims', () => {
    const proof = trackingReportPolicyConsumerHostedUiProof();

    expect(proof).toEqual(ExpectedReportPolicyConsumerHostedUiProof);
    expect(JSON.stringify(proof)).not.toMatch(
      /(?:AI execution claimed|policy mutation executed|physical device proved|product ready)/iu
    );
  });

  it('renders unsupported/manual platform rows without invented capability or product claims', () => {
    const proof = trackingUnsupportedManualPlatformProof();

    expect(proof).toEqual({
      title: 'Unsupported/manual tracking platform proof',
      body: 'Unsupported platform and manual-required adapter rows render as degraded states without invented capability.',
      proofTier: 'P1 fixture proof',
      rowsReturned: '7',
      manualRequiredRows: '5',
      unavailableRows: '1',
      authorityRequiredRows: '1',
      fakeCapabilityRows: '0',
      productClaimReadyRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
      evidence: 'UI fixture proof',
      proofArtifact: TrackingStatusProofArtifacts.UnsupportedManualPlatform,
      missingProof: 'Manual proof required',
      boundary:
        'Hosted render-state proof only; physical-device, authority, provider delivery, and product readiness remain unclaimed.',
      productClaim: 'No product claim',
      rows: ExpectedUnsupportedManualRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:product ready|physical device proved|authority proved)/iu);
  });
});

describe('tracking notification parent-surface hosted proof surface', () => {
  it('renders notification history rows without delivery, receipt, or product claims', () => {
    const proof = trackingNotificationParentSurfaceHostedUiProof();

    expect(proof).toEqual({
      title: 'Notification history intent UI',
      body: 'Hosted route renders parent notification history, manual action, and provider unavailable rows from existing tracking notification proof refs without claiming provider delivery or receipt runtime.',
      proofTier: 'P2 service proof',
      rowsReturned: '3',
      proofArtifact: TrackingStatusProofArtifacts.NotificationParentSurfaceHistory,
      boundary:
        'Hosted notification history rendering only; preference mutation, quiet-hours runtime, provider delivery, receipt ingestion, child-device delivery, physical-device proof, authority, production storage, adapter dispatch, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      renderedParentNotificationUiRows: '3',
      parentPreferenceMutationRows: '0',
      providerDeliveryClaimedRows: '0',
      receiptIngestionClaimedRows: '0',
      childDeviceDeliveryClaimedRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
      productionStorageClaimedRows: '0',
      adapterDispatchClaimedRows: '0',
      productClaimReadyRows: '0',
      rows: ExpectedNotificationParentSurfaceRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:provider delivered|receipt ingested|product ready)/iu);
  });
});

describe('tracking parent action readiness hosted proof surface', () => {
  it('renders expected-place and acknowledgement action rows without runtime, delivery, or product claims', () => {
    const proof = trackingParentActionReadinessHostedUiProof();

    expect(proof).toEqual({
      title: 'Parent action readiness UI',
      body: 'Hosted route renders expected-place alert policy and parent acknowledgement action readiness rows from existing tracking proof refs without claiming live mutation or delivery runtime.',
      proofTier: 'P2 service proof',
      expectedPlaceProofArtifact: TrackingStatusProofArtifacts.ExpectedPlaceAlertPolicy,
      acknowledgementProofArtifact: TrackingStatusProofArtifacts.ParentAcknowledgementActionReadiness,
      boundary:
        'Hosted parent action readiness rendering only; live service mutation, alert delivery, provider delivery, receipt ingestion, child-device runtime, physical-device proof, authority, production workers, adapter dispatch, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      expectedPlaceRows: '4',
      acknowledgementActionRows: '5',
      renderedParentActionRows: '9',
      liveServiceMutationRows: '0',
      providerDeliveryClaimedRows: '0',
      notificationReceiptClaimedRows: '0',
      childDeviceRuntimeClaimedRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
      productionWorkerClaimedRows: '0',
      adapterDispatchClaimedRows: '0',
      productClaimReadyRows: '0',
      rows: ExpectedParentActionReadinessRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(
      /(?:service mutation executed|provider delivered|receipt ingested|product ready)/iu
    );
  });
});

describe('tracking missing-device hosted proof surface', () => {
  it('renders missing-device state rows without current-location, physical-device, or product claims', () => {
    const proof = trackingMissingDeviceHostedUiProof();

    expect(proof).toEqual({
      title: 'Missing-device state UI',
      body: 'Hosted route renders last-known, offline, contact-requested, and manual-required missing-device rows from existing WP29 proof without claiming current location or OS lost-mode runtime.',
      proofTier: 'P2 service proof',
      sourceProofArtifact: TrackingStatusProofArtifacts.MissingDeviceMode,
      boundary:
        'Hosted missing-device rendering only; current location runtime, powered-off tracking, remote sync, provider delivery, physical-device proof, OS lost-mode APIs, authority, production workers, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      renderedMissingDeviceRows: '4',
      lastKnownOnlyRows: '1',
      offlineRows: '1',
      contactRequestedRows: '1',
      manualRequiredRows: '1',
      currentLocationRuntimeClaimedRows: '0',
      poweredOffTrackingClaimedRows: '0',
      remoteSyncRuntimeClaimedRows: '0',
      providerDeliveryClaimedRows: '0',
      physicalDeviceProofClaimedRows: '0',
      osLostModeApiClaimedRows: '0',
      productClaimReadyRows: '0',
      rows: ExpectedMissingDeviceRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:current location proved|lost mode executed|product ready)/iu);
  });
});

describe('tracking child check-in proof surface', () => {
  it('renders child check-in copy as calm fixture proof without delivery claims', () => {
    const proof = trackingChildCheckInProof();

    expect(proof).toEqual({
      title: 'Child check-in request',
      body: 'Your parent is asking you to check in. Are you safe?',
      proofTier: 'P1 fixture proof',
      evidence: 'UI fixture proof',
      proofArtifact: TrackingStatusProofArtifacts.ChildCheckIn,
      copyBoundary: 'Calm copy, no accusation',
      safeAction: "I'm safe",
      helpAction: 'Need help',
      shareLocationAction: 'Share current location',
      callParentAction: 'Call parent',
      deliveryBoundary: 'Child-device delivery not proved',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:trouble|lying|bad place|delivered|product ready)/iu);
  });

  it('renders child runtime UI copy without delivery or product claims', () => {
    const proof = trackingChildRuntimeUiProof();

    expect(proof).toEqual({
      title: 'Child runtime UI proof',
      body: 'Child sees a clear tracking request, safe response, help response, and location-share consent copy.',
      proofTier: 'P2 service proof',
      evidence: 'UI fixture proof',
      proofArtifact: TrackingStatusProofArtifacts.ChildRuntimeUi,
      disclosure: 'Tracking request disclosed',
      safeResponse: 'Safe response visible',
      helpResponse: 'Help response visible',
      locationShareConsent: 'Location share asks consent',
      runtimeBoundary: 'Hosted proof only, not child-agent delivery',
      deliveryBoundary: 'Child-device delivery not proved',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:delivered|physical device proved|product ready)/iu);
  });
});
