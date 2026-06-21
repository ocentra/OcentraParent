import type { Infer } from './effect';
import type {
  TrackingAcknowledgementSchema,
  TrackingAiProviderRouteSchema,
  TrackingAlertIntentSchema,
  TrackingChildCheckInRequestSchema,
  TrackingChildCheckInResponseSchema,
  TrackingEscalationChainSchema,
  TrackingEvidenceTraceSchema,
  TrackingLocationAiAnalysisInputSchema,
  TrackingLocationAiAnalysisResultSchema,
  TrackingLocationPolicyReadModelSchema,
  TrackingMissingDeviceCaseSchema,
  TrackingPolicyDecisionSchema,
  TrackingPolicyRuleSchema,
  TrackingTemporaryLiveTrackingGrantSchema,
} from './tracking-location-policy';
import type { TrackingPlatformProofRouteSchema } from './tracking-location-policy-platform-proof';

export type TrackingEvidenceTrace = Infer<typeof TrackingEvidenceTraceSchema>;
export type TrackingPolicyRule = Infer<typeof TrackingPolicyRuleSchema>;
export type TrackingPolicyDecision = Infer<typeof TrackingPolicyDecisionSchema>;
export type TrackingAcknowledgement = Infer<typeof TrackingAcknowledgementSchema>;
export type TrackingChildCheckInRequest = Infer<typeof TrackingChildCheckInRequestSchema>;
export type TrackingChildCheckInResponse = Infer<typeof TrackingChildCheckInResponseSchema>;
export type TrackingLocationAiAnalysisInput = Infer<typeof TrackingLocationAiAnalysisInputSchema>;
export type TrackingLocationAiAnalysisResult = Infer<typeof TrackingLocationAiAnalysisResultSchema>;
export type TrackingAiProviderRoute = Infer<typeof TrackingAiProviderRouteSchema>;
export type TrackingAlertIntent = Infer<typeof TrackingAlertIntentSchema>;
export type TrackingEscalationChain = Infer<typeof TrackingEscalationChainSchema>;
export type TrackingTemporaryLiveTrackingGrant = Infer<typeof TrackingTemporaryLiveTrackingGrantSchema>;
export type TrackingMissingDeviceCase = Infer<typeof TrackingMissingDeviceCaseSchema>;
export type TrackingPlatformProofRoute = Infer<typeof TrackingPlatformProofRouteSchema>;
export type TrackingLocationPolicyReadModel = Infer<typeof TrackingLocationPolicyReadModelSchema>;
