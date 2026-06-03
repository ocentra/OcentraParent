import type {
  BrowserActiveProofSource as BrowserActiveProofSourceType,
  BrowserActiveTabState as BrowserActiveTabStateType,
  BrowserBridgeKind as BrowserBridgeKindType,
  BrowserCapabilityStatus as BrowserCapabilityStatusType,
  BrowserChannel as BrowserChannelType,
  BrowserCustodyLabel as BrowserCustodyLabelType,
  BrowserEvidenceReadModel,
  BrowserEvidenceRecentSummary,
  BrowserFamily as BrowserFamilyType,
  BrowserManagedSessionStatus,
  BrowserManagedState as BrowserManagedStateType,
  BrowserQueryVisibilityLabel as BrowserQueryVisibilityLabelType,
  BrowserTabEvidence,
  BrowserUnmanagedProcessEvidence,
  BrowserUnmanagedProcessReadModel,
} from './browser-schemas';
import type {
  BrowserUnmanagedDetectionConfidence as BrowserUnmanagedDetectionConfidenceType,
  BrowserUnmanagedDetectionReason as BrowserUnmanagedDetectionReasonType,
  BrowserUnmanagedProcessKind as BrowserUnmanagedProcessKindType,
} from './browser-unmanaged-process-schemas';
import type {
  BrowserActiveTabCapability as BrowserActiveTabCapabilityType,
  BrowserExactUrlCapability as BrowserExactUrlCapabilityType,
  BrowserInventoryInstallState as BrowserInventoryInstallStateType,
  BrowserInventoryReadModel,
  BrowserInventoryRow,
  BrowserInventoryRunningState as BrowserInventoryRunningStateType,
  BrowserManagedProfileState as BrowserManagedProfileStateType,
  BrowserManagementTier as BrowserManagementTierType,
  BrowserSupportTier as BrowserSupportTierType,
  BrowserUnmanagedFallbackCapability as BrowserUnmanagedFallbackCapabilityType,
} from './browser-inventory-schemas';
import type {
  BrowserExtensionHeartbeatState as BrowserExtensionHeartbeatStateType,
  BrowserExtensionInstallState as BrowserExtensionInstallStateType,
  BrowserExtensionManagedProfileBinding as BrowserExtensionManagedProfileBindingType,
  BrowserExtensionMinimumPermissionState as BrowserExtensionMinimumPermissionStateType,
  BrowserExtensionNativeHostBoundary,
  BrowserExtensionRuntimeSignal as BrowserExtensionRuntimeSignalType,
  BrowserNativeHostMessageState as BrowserNativeHostMessageStateType,
} from './browser-extension-native-host-schemas';
import type {
  BrowserInventoryPlatform as BrowserInventoryPlatformType,
  BrowserInventoryPlatformMatrix as BrowserInventoryPlatformMatrixType,
  BrowserInventoryPlatformMatrixEntry as BrowserInventoryPlatformMatrixEntryType,
  BrowserInventoryPlatformProofRequirement as BrowserInventoryPlatformProofRequirementType,
  BrowserInventoryPlatformProofState as BrowserInventoryPlatformProofStateType,
} from './browser-platform-inventory-matrix';

export * from './browser-values';
export { BrowserActiveProofSource, BrowserCustodyLabel, BrowserQueryVisibilityLabel } from './browser-values';

export {
  BrowserActiveProofSourceSchema,
  BrowserActiveTabStateSchema,
  BrowserAdapterIdSchema,
  BrowserBridgeEndpointRefSchema,
  BrowserBridgeKindSchema,
  BrowserCapabilityStatusSchema,
  BrowserChannelSchema,
  BrowserCustodyLabelSchema,
  BrowserDegradedReasonSchema,
  BrowserDomainSchema,
  BrowserEvidenceReadModelSchema,
  BrowserEvidenceRecentSummarySchema,
  BrowserEvidenceSchemaVersion,
  BrowserFamilySchema,
  BrowserManagedSessionStatusSchema,
  BrowserManagedSessionIdSchema,
  BrowserManagedStateSchema,
  BrowserOriginSchema,
  BrowserPageTitleSchema,
  BrowserProfileIdSchema,
  BrowserProfilePathRefSchema,
  BrowserQueryVisibilityLabelSchema,
  BrowserTabEvidenceSchema,
  BrowserTabIdSchema,
  BrowserUnmanagedProcessEvidenceSchema,
  BrowserUnmanagedProcessReadModelSchema,
  BrowserUrlSchema,
  BrowserVersionSchema,
  BrowserWindowIdSchema,
  decodeBrowserUrl,
} from './browser-schemas';
export {
  BrowserUnmanagedDetectionConfidenceSchema,
  BrowserUnmanagedDetectionReasonSchema,
  BrowserUnmanagedExecutablePathRefSchema,
  BrowserUnmanagedProcessHashRefSchema,
  BrowserUnmanagedProcessKindSchema,
  BrowserUnmanagedProcessNameSchema,
  BrowserUnmanagedSignatureRefSchema,
} from './browser-unmanaged-process-schemas';
export { BrowserTargetIdSchema } from './browser-target-schemas';
export {
  BrowserExtensionHeartbeatStateSchema,
  BrowserExtensionIdSchema,
  BrowserExtensionInstallStateSchema,
  BrowserExtensionManagedProfileBindingSchema,
  BrowserExtensionMinimumPermissionStateSchema,
  BrowserExtensionNativeHostBoundarySchema,
  BrowserExtensionNativeHostSchemaVersion,
  BrowserExtensionRuntimeSignalSchema,
  BrowserNativeHostIdSchema,
  BrowserNativeHostMaxMessageLengthBytes,
  BrowserNativeHostMessageStateSchema,
} from './browser-extension-native-host-schemas';
export {
  BrowserPerformanceBudgetIdSchema,
  BrowserPerformanceBudgetProofSourceSchema,
  BrowserPerformanceBudgetStateSchema,
  BrowserPerformanceHealthReadModelSchema,
  BrowserPerformanceHealthRowSchema,
  BrowserPerformanceHealthSchemaVersion,
  BrowserPerformanceHealthStateSchema,
} from './browser-performance-health-schemas';
export * from './browser-url-metadata-schemas';
export {
  BrowserUrlShapeClassificationIdSchema,
  BrowserUrlShapeClassificationResultSchema,
  BrowserUrlShapeConfidenceSchema,
  BrowserUrlIntelligenceMemoryHitIdSchema,
  BrowserUrlIntelligenceMemorySchemaVersion,
  BrowserUrlIntelligenceMemoryDecisionKindSchema,
  BrowserUrlIntelligenceMemoryHitSchema,
  BrowserUrlIntelligenceMemoryHitStateSchema,
  BrowserUrlIntelligenceMemoryKeyKindSchema,
  BrowserUrlIntelligenceMemoryKeySchema,
  BrowserUrlIntelligenceMemoryStaleReasonSchema,
  BrowserUrlShapePlatformIdsSchema,
  BrowserUrlShapePlatformSchema,
  BrowserUrlShapeReasonCodeSchema,
  BrowserUrlShapeSchemaVersion,
  BrowserUrlShapeSourceKindSchema,
  BrowserUrlShapeTargetKindSchema,
  decodeBrowserUrlIntelligenceMemoryHit,
  decodeBrowserUrlShapeClassificationResult,
} from './browser-url-intelligence-schemas';
export { parseBrowserUrlShape } from './browser-url-intelligence';
export {
  BrowserActiveTabCapabilitySchema,
  BrowserExactUrlCapabilitySchema,
  BrowserExecutablePathRefSchema,
  BrowserInventoryInstallStateSchema,
  BrowserInventoryReadModelSchema,
  BrowserInventoryReasonCodeSchema,
  BrowserInventoryRowIdSchema,
  BrowserInventoryRowSchema,
  BrowserInventoryRunningStateSchema,
  BrowserManagedProfileStateSchema,
  BrowserManagementTierSchema,
  BrowserProductNameSchema,
  BrowserSupportTierSchema,
  BrowserUnmanagedFallbackCapabilitySchema,
} from './browser-inventory-schemas';
export {
  BrowserInventoryPlatformMatrixEntrySchema,
  BrowserInventoryPlatformMatrixSchema,
  BrowserInventoryPlatformProofRequirementSchema,
  BrowserInventoryPlatformProofStateSchema,
  BrowserInventoryPlatformSchema,
  decodeBrowserInventoryPlatformMatrix,
  decodeBrowserInventoryPlatformMatrixEntry,
} from './browser-platform-inventory-matrix';
export {
  BrowserInterventionAction,
  BrowserInterventionActionIdSchema,
  BrowserInterventionActionSchema,
  BrowserInterventionAuditIdSchema,
  BrowserBoundaryState,
  BrowserBoundaryStateSchema,
  BrowserExactUrlClaimState,
  BrowserExactUrlClaimStateSchema,
  BrowserInterventionCapabilityState,
  BrowserInterventionCapabilityStateSchema,
  BrowserInterventionDecisionSource,
  BrowserInterventionDecisionSourceSchema,
  BrowserInterventionDeliveryState,
  BrowserInterventionDeliveryStateSchema,
  BrowserInterventionIdSchema,
  BrowserInterventionMechanism,
  BrowserInterventionMechanismSchema,
  BrowserInterventionOutcome,
  BrowserInterventionOutcomeSchema,
  BrowserInterventionReadModelSchema,
  BrowserInterventionRowSchema,
  BrowserInterventionSchemaVersion,
  BrowserInterventionTargetType,
  BrowserInterventionTargetTypeSchema,
  BrowserPolicyDecisionIdSchema,
  BrowserTargetValueSchema,
  BrowserUnmanagedDetectionState,
  BrowserUnmanagedDetectionStateSchema,
  BrowserUnmanagedEnforcementState,
  BrowserUnmanagedEnforcementStateSchema,
  BrowserUnmanagedFallbackActionState,
  BrowserUnmanagedFallbackActionStateSchema,
  type BrowserBoundaryState as BrowserBoundaryStateType,
  type BrowserExactUrlClaimState as BrowserExactUrlClaimStateType,
  type BrowserInterventionDeliveryState as BrowserInterventionDeliveryStateType,
  type BrowserInterventionReadModel,
  type BrowserInterventionRow,
  type BrowserUnmanagedDetectionState as BrowserUnmanagedDetectionStateType,
  type BrowserUnmanagedFallbackActionState as BrowserUnmanagedFallbackActionStateType,
} from './browser-intervention';

export type BrowserActiveTabState = BrowserActiveTabStateType;
export type BrowserActiveProofSource = BrowserActiveProofSourceType;
export type BrowserActiveTabCapability = BrowserActiveTabCapabilityType;
export type BrowserBridgeKind = BrowserBridgeKindType;
export type BrowserCapabilityStatus = BrowserCapabilityStatusType;
export type BrowserChannel = BrowserChannelType;
export type BrowserCustodyLabel = BrowserCustodyLabelType;
export type BrowserExtensionHeartbeatState = BrowserExtensionHeartbeatStateType;
export type BrowserExtensionInstallState = BrowserExtensionInstallStateType;
export type BrowserExtensionManagedProfileBinding = BrowserExtensionManagedProfileBindingType;
export type BrowserExtensionMinimumPermissionState = BrowserExtensionMinimumPermissionStateType;
export type BrowserExtensionRuntimeSignal = BrowserExtensionRuntimeSignalType;
export type BrowserNativeHostMessageState = BrowserNativeHostMessageStateType;
export type BrowserExactUrlCapability = BrowserExactUrlCapabilityType;
export type BrowserInventoryInstallState = BrowserInventoryInstallStateType;
export type BrowserInventoryRunningState = BrowserInventoryRunningStateType;
export type BrowserManagedProfileState = BrowserManagedProfileStateType;
export type BrowserManagementTier = BrowserManagementTierType;
export type BrowserSupportTier = BrowserSupportTierType;
export type BrowserUnmanagedFallbackCapability = BrowserUnmanagedFallbackCapabilityType;
export type BrowserInventoryPlatform = BrowserInventoryPlatformType;
export type BrowserInventoryPlatformMatrix = BrowserInventoryPlatformMatrixType;
export type BrowserInventoryPlatformMatrixEntry = BrowserInventoryPlatformMatrixEntryType;
export type BrowserInventoryPlatformProofRequirement = BrowserInventoryPlatformProofRequirementType;
export type BrowserInventoryPlatformProofState = BrowserInventoryPlatformProofStateType;
export type BrowserUnmanagedDetectionConfidence = BrowserUnmanagedDetectionConfidenceType;
export type BrowserUnmanagedDetectionReason = BrowserUnmanagedDetectionReasonType;
export type BrowserUnmanagedProcessKind = BrowserUnmanagedProcessKindType;
export type {
  BrowserEvidenceReadModel,
  BrowserEvidenceRecentSummary,
  BrowserExtensionNativeHostBoundary,
  BrowserInventoryReadModel,
  BrowserInventoryRow,
  BrowserManagedSessionStatus,
  BrowserTabEvidence,
  BrowserUnmanagedProcessEvidence,
  BrowserUnmanagedProcessReadModel,
};
export type BrowserFamily = BrowserFamilyType;
export type BrowserManagedState = BrowserManagedStateType;
export type BrowserQueryVisibilityLabel = BrowserQueryVisibilityLabelType;
