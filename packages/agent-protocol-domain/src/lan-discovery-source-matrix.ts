import {
  LanDiscoverySourceAuthoritySchema,
  LanDiscoverySourceKindSchema,
  LanDiscoverySourceMatrixSchema,
  LanDiscoverySourceRowSchema,
  LanDiscoverySourceRuntimePathSchema,
  LanDiscoverySourceStatusSchema,
  LanDiscoverySourceUiSurfaceSchema,
  LanPlanWorkpackIdSchema,
  LanPlanWorkpackStatusRowSchema,
  type LanDiscoverySourceAuthority,
  type LanDiscoverySourceKind,
  type LanDiscoverySourceMatrix,
  type LanDiscoverySourceRow,
  type LanDiscoverySourceRuntimePath,
  type LanDiscoverySourceStatus,
  type LanDiscoverySourceUiSurface,
  type LanPlanWorkpackId,
  type LanPlanWorkpackStatusRow,
} from '@ocentra-parent/lan-domain/lan-discovery-source-matrix';
import {
  LanHouseholdProductProofStateSchema,
  type LanHouseholdProductProofState,
} from '@ocentra-parent/lan-domain/lan-pairing-product-proof';
import {
  LanProductionHouseholdProofRuntimeOwnerSchema,
  type LanProductionHouseholdProofRuntimeOwner,
} from '@ocentra-parent/lan-domain/lan-production-household-proof';

export const AgentLanPlanWorkpackIdSchema = LanPlanWorkpackIdSchema;
export const AgentLanDiscoverySourceKindSchema = LanDiscoverySourceKindSchema;
export const AgentLanDiscoverySourceStatusSchema = LanDiscoverySourceStatusSchema;
export const AgentLanDiscoverySourceAuthoritySchema = LanDiscoverySourceAuthoritySchema;
export const AgentLanDiscoverySourceRuntimePathSchema = LanDiscoverySourceRuntimePathSchema;
export const AgentLanDiscoverySourceUiSurfaceSchema = LanDiscoverySourceUiSurfaceSchema;
export const AgentLanDiscoverySourceProofStateSchema = LanHouseholdProductProofStateSchema;
export const AgentLanDiscoverySourceRuntimeOwnerSchema = LanProductionHouseholdProofRuntimeOwnerSchema;
export const AgentLanPlanWorkpackStatusRowSchema = LanPlanWorkpackStatusRowSchema;
export const AgentLanDiscoverySourceRowSchema = LanDiscoverySourceRowSchema;
export const AgentLanDiscoverySourceMatrixSchema = LanDiscoverySourceMatrixSchema;

export type AgentLanPlanWorkpackId = LanPlanWorkpackId;
export type AgentLanDiscoverySourceKind = LanDiscoverySourceKind;
export type AgentLanDiscoverySourceStatus = LanDiscoverySourceStatus;
export type AgentLanDiscoverySourceAuthority = LanDiscoverySourceAuthority;
export type AgentLanDiscoverySourceRuntimePath = LanDiscoverySourceRuntimePath;
export type AgentLanDiscoverySourceUiSurface = LanDiscoverySourceUiSurface;
export type AgentLanDiscoverySourceProofState = LanHouseholdProductProofState;
export type AgentLanDiscoverySourceRuntimeOwner = LanProductionHouseholdProofRuntimeOwner;
export type AgentLanPlanWorkpackStatusRow = LanPlanWorkpackStatusRow;
export type AgentLanDiscoverySourceRow = LanDiscoverySourceRow;
export type AgentLanDiscoverySourceMatrix = LanDiscoverySourceMatrix;
