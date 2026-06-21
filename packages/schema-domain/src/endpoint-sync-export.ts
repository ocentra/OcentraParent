import { decodeApiPath, decodeEndpointId, decodeHeaderName, decodeQueryParam } from './endpoint-brands';
import { ApiVersion } from './endpoint-constants';

const syncExportPrefix = `/api/${ApiVersion.V1}/sync-export`;
const remoteConnectorPrefix = `/api/${ApiVersion.V1}/remote-connectors`;

export const ParentOwnedSyncExportEndpointId = {
  ExportManifest: decodeEndpointId('sync-export.parent-owned.export-manifest'),
  ExportStatus: decodeEndpointId('sync-export.parent-owned.export-status'),
  SyncCursor: decodeEndpointId('sync-export.parent-owned.sync-cursor'),
  SyncBatchStatus: decodeEndpointId('sync-export.parent-owned.sync-batch-status'),
  ImportPreview: decodeEndpointId('sync-export.parent-owned.import-preview'),
  DeleteStatus: decodeEndpointId('sync-export.parent-owned.delete-status'),
} as const;

export const ParentOwnedSyncExportApiPath = {
  ExportManifest: decodeApiPath(`${syncExportPrefix}/export-manifest`),
  ExportStatus: decodeApiPath(`${syncExportPrefix}/export-status`),
  SyncCursor: decodeApiPath(`${syncExportPrefix}/sync-cursor`),
  SyncBatchStatus: decodeApiPath(`${syncExportPrefix}/sync-batch-status`),
  ImportPreview: decodeApiPath(`${syncExportPrefix}/import-preview`),
  DeleteStatus: decodeApiPath(`${syncExportPrefix}/delete-status`),
} as const;

export const ParentOwnedSyncExportEndpointSupport = {
  ExportManifest: 'contract-only',
  ExportStatus: 'contract-only',
  SyncCursor: 'contract-only',
  SyncBatchStatus: 'contract-only',
  ImportPreview: 'contract-only',
  DeleteStatus: 'contract-only',
} as const;

export const ParentOwnedSyncExportHeader = {
  ContractVersion: decodeHeaderName('X-Ocentra-Sync-Export-Version'),
  ParentIntentId: decodeHeaderName('X-Ocentra-Parent-Intent-Id'),
  FamilyId: decodeHeaderName('X-Ocentra-Family-Id'),
  ChildDeviceId: decodeHeaderName('X-Ocentra-Child-Device-Id'),
  StorageConnectorId: decodeHeaderName('X-Ocentra-Storage-Connector-Id'),
  DataCustody: decodeHeaderName('X-Ocentra-Data-Custody'),
} as const;

export const ParentOwnedSyncExportQueryParam = {
  FamilyId: decodeQueryParam('familyId'),
  ChildDeviceId: decodeQueryParam('childDeviceId'),
  DataClass: decodeQueryParam('dataClass'),
  ExportFormat: decodeQueryParam('exportFormat'),
  ManifestVersion: decodeQueryParam('manifestVersion'),
  SyncCursor: decodeQueryParam('syncCursor'),
  StorageConnectorId: decodeQueryParam('storageConnectorId'),
  RequestId: decodeQueryParam('requestId'),
} as const;

export const ParentOwnedSyncExportContractVersion = {
  ExportManifest: 'sync-export.export-manifest.v1',
  ExportStatus: 'sync-export.export-status.v1',
  SyncCursor: 'sync-export.sync-cursor.v1',
  SyncBatchStatus: 'sync-export.sync-batch-status.v1',
  ImportPreview: 'sync-export.import-preview.v1',
  DeleteStatus: 'sync-export.delete-status.v1',
} as const;

export const RemoteConnectorStatusEndpointId = {
  StatusSummary: decodeEndpointId('remote-connector.parent-owned.status-summary'),
  ProviderStatus: decodeEndpointId('remote-connector.parent-owned.provider-status'),
  RevocationStatus: decodeEndpointId('remote-connector.parent-owned.revocation-status'),
  ReportCompileStatus: decodeEndpointId('remote-connector.parent-owned.report-compile-status'),
} as const;

export const RemoteConnectorStatusApiPath = {
  StatusSummary: decodeApiPath(`${remoteConnectorPrefix}/status`),
  ProviderStatus: decodeApiPath(`${remoteConnectorPrefix}/provider-status`),
  RevocationStatus: decodeApiPath(`${remoteConnectorPrefix}/revocation-status`),
  ReportCompileStatus: decodeApiPath(`${remoteConnectorPrefix}/report-compile-status`),
} as const;

export const RemoteConnectorStatusEndpointSupport = {
  StatusSummary: 'contract-only',
  ProviderStatus: 'contract-only',
  RevocationStatus: 'contract-only',
  ReportCompileStatus: 'contract-only',
} as const;

export const RemoteConnectorStatusHeader = {
  ContractVersion: decodeHeaderName('X-Ocentra-Connector-Status-Version'),
  ParentIntentId: decodeHeaderName('X-Ocentra-Parent-Intent-Id'),
  FamilyId: decodeHeaderName('X-Ocentra-Family-Id'),
  StorageConnectorId: decodeHeaderName('X-Ocentra-Storage-Connector-Id'),
} as const;

export const RemoteConnectorStatusQueryParam = {
  FamilyId: decodeQueryParam('familyId'),
  StorageConnectorId: decodeQueryParam('storageConnectorId'),
  ProviderKey: decodeQueryParam('providerKey'),
  FolderRef: decodeQueryParam('folderRef'),
  IncludeCapabilities: decodeQueryParam('includeCapabilities'),
  IncludeRevoked: decodeQueryParam('includeRevoked'),
} as const;

export const RemoteConnectorStatusContractVersion = {
  StatusSummary: 'remote-connector.status-summary.v1',
  ProviderStatus: 'remote-connector.provider-status.v1',
  RevocationStatus: 'remote-connector.revocation-status.v1',
  ReportCompileStatus: 'remote-connector.report-compile-status.v1',
} as const;

export const ParentOwnedSyncExportBoundaryState = {
  RouteContract: 'defined',
  TransferRuntime: 'not-implemented',
  ConnectorOAuth: 'not-implemented',
  OcentraHostedFamilyDataCustody: 'not-supported',
  AccountSubscriptionBackend: 'not-implemented',
  PortalUi: 'not-implemented',
} as const;
