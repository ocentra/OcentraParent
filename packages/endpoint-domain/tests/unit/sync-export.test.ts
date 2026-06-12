import { describe, expect, it } from 'vitest';
import {
  ParentOwnedSyncExportApiPath,
  ParentOwnedSyncExportBoundaryState,
  ParentOwnedSyncExportContractVersion,
  ParentOwnedSyncExportEndpointId,
  ParentOwnedSyncExportEndpointSupport,
  ParentOwnedSyncExportHeader,
  ParentOwnedSyncExportQueryParam,
  RemoteConnectorStatusApiPath,
  RemoteConnectorStatusContractVersion,
  RemoteConnectorStatusEndpointId,
  RemoteConnectorStatusEndpointSupport,
  RemoteConnectorStatusHeader,
  RemoteConnectorStatusQueryParam,
} from '../../src/constants/sync-export';

const expectedSyncExportEndpointIds = {
  ExportManifest: 'sync-export.parent-owned.export-manifest',
  ExportStatus: 'sync-export.parent-owned.export-status',
  SyncCursor: 'sync-export.parent-owned.sync-cursor',
  SyncBatchStatus: 'sync-export.parent-owned.sync-batch-status',
  ImportPreview: 'sync-export.parent-owned.import-preview',
  DeleteStatus: 'sync-export.parent-owned.delete-status',
} as const;

const expectedSyncExportApiPaths = {
  ExportManifest: '/api/v1/sync-export/export-manifest',
  ExportStatus: '/api/v1/sync-export/export-status',
  SyncCursor: '/api/v1/sync-export/sync-cursor',
  SyncBatchStatus: '/api/v1/sync-export/sync-batch-status',
  ImportPreview: '/api/v1/sync-export/import-preview',
  DeleteStatus: '/api/v1/sync-export/delete-status',
} as const;

const expectedSyncExportEndpointSupport = {
  ExportManifest: 'contract-only',
  ExportStatus: 'contract-only',
  SyncCursor: 'contract-only',
  SyncBatchStatus: 'contract-only',
  ImportPreview: 'contract-only',
  DeleteStatus: 'contract-only',
} as const;

const expectedSyncExportHeaders = {
  ContractVersion: 'X-Ocentra-Sync-Export-Version',
  ParentIntentId: 'X-Ocentra-Parent-Intent-Id',
  FamilyId: 'X-Ocentra-Family-Id',
  ChildDeviceId: 'X-Ocentra-Child-Device-Id',
  StorageConnectorId: 'X-Ocentra-Storage-Connector-Id',
  DataCustody: 'X-Ocentra-Data-Custody',
} as const;

const expectedSyncExportQueryParams = {
  FamilyId: 'familyId',
  ChildDeviceId: 'childDeviceId',
  DataClass: 'dataClass',
  ExportFormat: 'exportFormat',
  ManifestVersion: 'manifestVersion',
  SyncCursor: 'syncCursor',
  StorageConnectorId: 'storageConnectorId',
  RequestId: 'requestId',
} as const;

const expectedConnectorEndpointIds = {
  StatusSummary: 'remote-connector.parent-owned.status-summary',
  ProviderStatus: 'remote-connector.parent-owned.provider-status',
  RevocationStatus: 'remote-connector.parent-owned.revocation-status',
  ReportCompileStatus: 'remote-connector.parent-owned.report-compile-status',
} as const;

const expectedConnectorApiPaths = {
  StatusSummary: '/api/v1/remote-connectors/status',
  ProviderStatus: '/api/v1/remote-connectors/provider-status',
  RevocationStatus: '/api/v1/remote-connectors/revocation-status',
  ReportCompileStatus: '/api/v1/remote-connectors/report-compile-status',
} as const;

const expectedConnectorEndpointSupport = {
  StatusSummary: 'contract-only',
  ProviderStatus: 'contract-only',
  RevocationStatus: 'contract-only',
  ReportCompileStatus: 'contract-only',
} as const;

const expectedConnectorHeaders = {
  ContractVersion: 'X-Ocentra-Connector-Status-Version',
  ParentIntentId: 'X-Ocentra-Parent-Intent-Id',
  FamilyId: 'X-Ocentra-Family-Id',
  StorageConnectorId: 'X-Ocentra-Storage-Connector-Id',
} as const;

const expectedConnectorQueryParams = {
  FamilyId: 'familyId',
  StorageConnectorId: 'storageConnectorId',
  ProviderKey: 'providerKey',
  FolderRef: 'folderRef',
  IncludeCapabilities: 'includeCapabilities',
  IncludeRevoked: 'includeRevoked',
} as const;

const expectedSyncExportVersions = {
  ExportManifest: 'sync-export.export-manifest.v1',
  ExportStatus: 'sync-export.export-status.v1',
  SyncCursor: 'sync-export.sync-cursor.v1',
  SyncBatchStatus: 'sync-export.sync-batch-status.v1',
  ImportPreview: 'sync-export.import-preview.v1',
  DeleteStatus: 'sync-export.delete-status.v1',
} as const;

const expectedConnectorVersions = {
  StatusSummary: 'remote-connector.status-summary.v1',
  ProviderStatus: 'remote-connector.provider-status.v1',
  RevocationStatus: 'remote-connector.revocation-status.v1',
  ReportCompileStatus: 'remote-connector.report-compile-status.v1',
} as const;

const expectedBoundaryState = {
  RouteContract: 'defined',
  TransferRuntime: 'not-implemented',
  ConnectorOAuth: 'not-implemented',
  OcentraHostedFamilyDataCustody: 'not-supported',
  AccountSubscriptionBackend: 'not-implemented',
  PortalUi: 'not-implemented',
} as const;

describe('sync export endpoint constants', () => {
  it('ParentOwnedSyncExportApiPath: declares versioned sync export status routes as contract-only', () => {
    expect(ParentOwnedSyncExportEndpointId).toEqual(expectedSyncExportEndpointIds);
    expect(ParentOwnedSyncExportApiPath).toEqual(expectedSyncExportApiPaths);
    expect(ParentOwnedSyncExportEndpointSupport).toEqual(expectedSyncExportEndpointSupport);
  });

  it('ParentOwnedSyncExportHeader: exposes custody and request scoping headers', () => {
    expect(ParentOwnedSyncExportHeader).toEqual(expectedSyncExportHeaders);
    expect(ParentOwnedSyncExportQueryParam).toEqual(expectedSyncExportQueryParams);
  });

  it('RemoteConnectorStatusApiPath: separates remote connector status from transfer runtime', () => {
    expect(RemoteConnectorStatusEndpointId).toEqual(expectedConnectorEndpointIds);
    expect(RemoteConnectorStatusApiPath).toEqual(expectedConnectorApiPaths);
    expect(RemoteConnectorStatusEndpointSupport).toEqual(expectedConnectorEndpointSupport);
  });

  it('RemoteConnectorStatusHeader: exposes connector status query boundaries without OAuth claims', () => {
    expect(RemoteConnectorStatusHeader).toEqual(expectedConnectorHeaders);
    expect(RemoteConnectorStatusQueryParam).toEqual(expectedConnectorQueryParams);
  });

  it('ParentOwnedSyncExportContractVersion: records versions and explicit non-implementation boundaries', () => {
    expect(ParentOwnedSyncExportContractVersion).toEqual(expectedSyncExportVersions);
    expect(RemoteConnectorStatusContractVersion).toEqual(expectedConnectorVersions);
    expect(ParentOwnedSyncExportBoundaryState).toEqual(expectedBoundaryState);
  });
});
