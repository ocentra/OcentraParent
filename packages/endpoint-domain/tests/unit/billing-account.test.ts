import { describe, expect, it } from 'vitest';
import {
  AccountDistributionApiPath,
  AccountDistributionContractVersion,
  AccountDistributionEndpointId,
  AccountDistributionEndpointSupport,
  AccountDistributionHeader,
  AccountDistributionQueryParam,
  BillingAccountApiPath,
  BillingAccountBoundaryState,
  BillingAccountContractVersion,
  BillingAccountEndpointId,
  BillingAccountEndpointSupport,
  BillingAccountHeader,
  BillingAccountQueryParam,
} from '@ocentra-parent/schema-domain/endpoint-billing-account';

const expectedBillingEndpointIds = {
  AccountStatus: 'billing-account.account-status',
  PlanEntitlementSnapshot: 'billing-account.plan-entitlement-snapshot',
  SubscriptionStatus: 'billing-account.subscription-status',
  DeviceLimitDecision: 'billing-account.device-limit-decision',
} as const;

const expectedBillingApiPaths = {
  AccountStatus: '/api/v1/billing-account/account-status',
  PlanEntitlementSnapshot: '/api/v1/billing-account/plan-entitlement-snapshot',
  SubscriptionStatus: '/api/v1/billing-account/subscription-status',
  DeviceLimitDecision: '/api/v1/billing-account/device-limit-decision',
} as const;

const expectedBillingSupport = {
  AccountStatus: 'contract-only',
  PlanEntitlementSnapshot: 'contract-only',
  SubscriptionStatus: 'contract-only',
  DeviceLimitDecision: 'contract-only',
} as const;

const expectedBillingHeaders = {
  ContractVersion: 'X-Ocentra-Billing-Account-Version',
  ParentIntentId: 'X-Ocentra-Parent-Intent-Id',
  ParentAccountId: 'X-Ocentra-Parent-Account-Id',
  FamilyId: 'X-Ocentra-Family-Id',
  EntitlementSnapshotId: 'X-Ocentra-Entitlement-Snapshot-Id',
  DeviceRegistrationId: 'X-Ocentra-Device-Registration-Id',
} as const;

const expectedBillingQueryParams = {
  ParentAccountId: 'parentAccountId',
  FamilyId: 'familyId',
  PlanId: 'planId',
  EntitlementScope: 'entitlementScope',
  SubscriptionStatus: 'subscriptionStatus',
  DeviceId: 'deviceId',
  RequestId: 'requestId',
} as const;

const expectedBillingVersions = {
  AccountStatus: 'billing-account.account-status.v1',
  PlanEntitlementSnapshot: 'billing-account.plan-entitlement-snapshot.v1',
  SubscriptionStatus: 'billing-account.subscription-status.v1',
  DeviceLimitDecision: 'billing-account.device-limit-decision.v1',
} as const;

const expectedDistributionEndpointIds = {
  DownloadSurface: 'billing-account.download-surface',
  UpdateStatus: 'billing-account.update-status',
  ReleaseStatus: 'billing-account.release-status',
} as const;

const expectedDistributionApiPaths = {
  DownloadSurface: '/api/v1/account-distribution/download-surface',
  UpdateStatus: '/api/v1/account-distribution/update-status',
  ReleaseStatus: '/api/v1/account-distribution/release-status',
} as const;

const expectedDistributionSupport = {
  DownloadSurface: 'contract-only',
  UpdateStatus: 'contract-only',
  ReleaseStatus: 'contract-only',
} as const;

const expectedDistributionHeaders = {
  ContractVersion: 'X-Ocentra-Account-Distribution-Version',
  ParentIntentId: 'X-Ocentra-Parent-Intent-Id',
  ParentAccountId: 'X-Ocentra-Parent-Account-Id',
  ReleaseChannel: 'X-Ocentra-Release-Channel',
  DownloadSurface: 'X-Ocentra-Download-Surface',
} as const;

const expectedDistributionQueryParams = {
  ParentAccountId: 'parentAccountId',
  Platform: 'platform',
  ReleaseChannel: 'releaseChannel',
  ArtifactKind: 'artifactKind',
  Version: 'version',
  RequestId: 'requestId',
} as const;

const expectedDistributionVersions = {
  DownloadSurface: 'billing-account.download-surface.v1',
  UpdateStatus: 'billing-account.update-status.v1',
  ReleaseStatus: 'billing-account.release-status.v1',
} as const;

const expectedBoundaryState = {
  RouteContract: 'defined',
  StripeSdk: 'not-included',
  BillingProviderBackend: 'not-implemented',
  AccountBackend: 'not-implemented',
  ChildActivityCustody: 'not-supported',
  PortalUi: 'not-implemented',
} as const;

describe('billing account endpoint constants', () => {
  it('BillingAccountApiPath: declares account and entitlement route contracts', () => {
    expect(BillingAccountEndpointId).toEqual(expectedBillingEndpointIds);
    expect(BillingAccountApiPath).toEqual(expectedBillingApiPaths);
    expect(BillingAccountEndpointSupport).toEqual(expectedBillingSupport);
  });

  it('BillingAccountHeader: scopes account requests without provider secrets or child activity custody', () => {
    expect(BillingAccountHeader).toEqual(expectedBillingHeaders);
    expect(BillingAccountQueryParam).toEqual(expectedBillingQueryParams);
  });

  it('AccountDistributionApiPath: separates download and update status surface contracts', () => {
    expect(AccountDistributionEndpointId).toEqual(expectedDistributionEndpointIds);
    expect(AccountDistributionApiPath).toEqual(expectedDistributionApiPaths);
    expect(AccountDistributionEndpointSupport).toEqual(expectedDistributionSupport);
  });

  it('AccountDistributionHeader: scopes distribution requests to account and release context', () => {
    expect(AccountDistributionHeader).toEqual(expectedDistributionHeaders);
    expect(AccountDistributionQueryParam).toEqual(expectedDistributionQueryParams);
  });

  it('BillingAccountContractVersion: records endpoint versions and explicit non-claims', () => {
    expect(BillingAccountContractVersion).toEqual(expectedBillingVersions);
    expect(AccountDistributionContractVersion).toEqual(expectedDistributionVersions);
    expect(BillingAccountBoundaryState).toEqual(expectedBoundaryState);
  });
});
