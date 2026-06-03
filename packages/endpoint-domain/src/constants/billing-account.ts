import { decodeApiPath, decodeEndpointId, decodeHeaderName, decodeQueryParam } from '../types/brands';
import { ApiVersion } from './versions';

const billingAccountPrefix = `/api/${ApiVersion.V1}/billing-account`;
const accountDistributionPrefix = `/api/${ApiVersion.V1}/account-distribution`;

export const BillingAccountEndpointId = {
  AccountStatus: decodeEndpointId('billing-account.account-status'),
  PlanEntitlementSnapshot: decodeEndpointId('billing-account.plan-entitlement-snapshot'),
  SubscriptionStatus: decodeEndpointId('billing-account.subscription-status'),
  DeviceLimitDecision: decodeEndpointId('billing-account.device-limit-decision'),
} as const;

export const BillingAccountApiPath = {
  AccountStatus: decodeApiPath(`${billingAccountPrefix}/account-status`),
  PlanEntitlementSnapshot: decodeApiPath(`${billingAccountPrefix}/plan-entitlement-snapshot`),
  SubscriptionStatus: decodeApiPath(`${billingAccountPrefix}/subscription-status`),
  DeviceLimitDecision: decodeApiPath(`${billingAccountPrefix}/device-limit-decision`),
} as const;

export const BillingAccountEndpointSupport = {
  AccountStatus: 'contract-only',
  PlanEntitlementSnapshot: 'contract-only',
  SubscriptionStatus: 'contract-only',
  DeviceLimitDecision: 'contract-only',
} as const;

export const BillingAccountHeader = {
  ContractVersion: decodeHeaderName('X-Ocentra-Billing-Account-Version'),
  ParentIntentId: decodeHeaderName('X-Ocentra-Parent-Intent-Id'),
  ParentAccountId: decodeHeaderName('X-Ocentra-Parent-Account-Id'),
  FamilyId: decodeHeaderName('X-Ocentra-Family-Id'),
  EntitlementSnapshotId: decodeHeaderName('X-Ocentra-Entitlement-Snapshot-Id'),
  DeviceRegistrationId: decodeHeaderName('X-Ocentra-Device-Registration-Id'),
} as const;

export const BillingAccountQueryParam = {
  ParentAccountId: decodeQueryParam('parentAccountId'),
  FamilyId: decodeQueryParam('familyId'),
  PlanId: decodeQueryParam('planId'),
  EntitlementScope: decodeQueryParam('entitlementScope'),
  SubscriptionStatus: decodeQueryParam('subscriptionStatus'),
  DeviceId: decodeQueryParam('deviceId'),
  RequestId: decodeQueryParam('requestId'),
} as const;

export const BillingAccountContractVersion = {
  AccountStatus: 'billing-account.account-status.v1',
  PlanEntitlementSnapshot: 'billing-account.plan-entitlement-snapshot.v1',
  SubscriptionStatus: 'billing-account.subscription-status.v1',
  DeviceLimitDecision: 'billing-account.device-limit-decision.v1',
} as const;

export const AccountDistributionEndpointId = {
  DownloadSurface: decodeEndpointId('billing-account.download-surface'),
  UpdateStatus: decodeEndpointId('billing-account.update-status'),
  ReleaseStatus: decodeEndpointId('billing-account.release-status'),
} as const;

export const AccountDistributionApiPath = {
  DownloadSurface: decodeApiPath(`${accountDistributionPrefix}/download-surface`),
  UpdateStatus: decodeApiPath(`${accountDistributionPrefix}/update-status`),
  ReleaseStatus: decodeApiPath(`${accountDistributionPrefix}/release-status`),
} as const;

export const AccountDistributionEndpointSupport = {
  DownloadSurface: 'contract-only',
  UpdateStatus: 'contract-only',
  ReleaseStatus: 'contract-only',
} as const;

export const AccountDistributionHeader = {
  ContractVersion: decodeHeaderName('X-Ocentra-Account-Distribution-Version'),
  ParentIntentId: decodeHeaderName('X-Ocentra-Parent-Intent-Id'),
  ParentAccountId: decodeHeaderName('X-Ocentra-Parent-Account-Id'),
  ReleaseChannel: decodeHeaderName('X-Ocentra-Release-Channel'),
  DownloadSurface: decodeHeaderName('X-Ocentra-Download-Surface'),
} as const;

export const AccountDistributionQueryParam = {
  ParentAccountId: decodeQueryParam('parentAccountId'),
  Platform: decodeQueryParam('platform'),
  ReleaseChannel: decodeQueryParam('releaseChannel'),
  ArtifactKind: decodeQueryParam('artifactKind'),
  Version: decodeQueryParam('version'),
  RequestId: decodeQueryParam('requestId'),
} as const;

export const AccountDistributionContractVersion = {
  DownloadSurface: 'billing-account.download-surface.v1',
  UpdateStatus: 'billing-account.update-status.v1',
  ReleaseStatus: 'billing-account.release-status.v1',
} as const;

export const BillingAccountBoundaryState = {
  RouteContract: 'defined',
  StripeSdk: 'not-included',
  BillingProviderBackend: 'not-implemented',
  AccountBackend: 'not-implemented',
  ChildActivityCustody: 'not-supported',
  PortalUi: 'not-implemented',
} as const;
