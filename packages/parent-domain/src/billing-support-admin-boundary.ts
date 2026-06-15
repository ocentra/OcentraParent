import * as Domain from '@ocentra-parent/billing-domain/billing-support-admin-boundary';
import * as DomainValues from '@ocentra-parent/billing-domain/billing-support-admin-boundary-values';

export const BillingSupportAdminSchemaVersionSchema =
  DomainValues.BillingSupportAdminSchemaVersionSchema;
export const BillingSupportAdminActionSchema =
  DomainValues.BillingSupportAdminActionSchema;
export const BillingSupportAdminRuntimeStateSchema =
  DomainValues.BillingSupportAdminRuntimeStateSchema;
export const BillingSupportAdminDataClassSchema =
  DomainValues.BillingSupportAdminDataClassSchema;
export const BillingSupportAdminNonClaimSchema =
  DomainValues.BillingSupportAdminNonClaimSchema;
export const BillingSupportAdminProviderSecretCustodySchema =
  DomainValues.BillingSupportAdminProviderSecretCustodySchema;
export const BillingSupportAdminPortalUiClaimSchema =
  DomainValues.BillingSupportAdminPortalUiClaimSchema;
export const BillingSupportAdminProviderContactClaimSchema =
  DomainValues.BillingSupportAdminProviderContactClaimSchema;
export const BillingSupportAdminBackendUploadClaimSchema =
  DomainValues.BillingSupportAdminBackendUploadClaimSchema;
export const BillingSupportAdminChildActivityCustodyClaimSchema =
  DomainValues.BillingSupportAdminChildActivityCustodyClaimSchema;
export const BillingSupportAdminBoundaryIdSchema =
  DomainValues.BillingSupportAdminBoundaryIdSchema;
export const BillingSupportAdminCaseReferenceSchema =
  DomainValues.BillingSupportAdminCaseReferenceSchema;
export const BillingSupportAdminAuditReferenceSchema =
  DomainValues.BillingSupportAdminAuditReferenceSchema;
export const BillingSupportAdminFailureStateSchema =
  Domain.BillingSupportAdminFailureStateSchema;
export const BillingSupportAdminBoundaryRowSchema =
  Domain.BillingSupportAdminBoundaryRowSchema;
export const BillingSupportAdminBoundaryProofSchema =
  Domain.BillingSupportAdminBoundaryProofSchema;

export type BillingSupportAdminAction = DomainValues.BillingSupportAdminAction;
export type BillingSupportAdminRuntimeState =
  DomainValues.BillingSupportAdminRuntimeState;
export type BillingSupportAdminDataClass = DomainValues.BillingSupportAdminDataClass;
export type BillingSupportAdminNonClaim = DomainValues.BillingSupportAdminNonClaim;
export type BillingSupportAdminFailureState =
  Domain.BillingSupportAdminFailureState;
export type BillingSupportAdminBoundaryRow =
  Domain.BillingSupportAdminBoundaryRow;
export type BillingSupportAdminBoundaryProof =
  Domain.BillingSupportAdminBoundaryProof;

export const decodeBillingSupportAdminBoundaryProof =
  Domain.decodeBillingSupportAdminBoundaryProof;
export const summarizeBillingSupportAdminActions =
  DomainValues.summarizeBillingSupportAdminActions;
export const summarizeBillingSupportAdminRuntimeStates =
  DomainValues.summarizeBillingSupportAdminRuntimeStates;
