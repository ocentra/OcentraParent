/* generated from crates/tracking-core/src/tracking_control_catalog_metadata.ts.txt */

import {
  trackingCapabilityRequirementFor,
  trackingCapabilityStateFor,
  trackingCapabilityStateFromSourceState,
  trackingControlKindFor,
  trackingEffectStatusFor,
  trackingFallbackFor,
  trackingPolicyLaneFor,
  trackingProofRequirementFor,
  trackingRuntimeOwnerFor,
} from './tracking-control-catalog-metadata-classifiers';
import {
  trackingCardKindFor,
  trackingExplicitOptionLabels,
  trackingHelperTextFor,
  trackingLayoutHintsFor,
  trackingProposalKinds,
  trackingQuestionFromSourceText,
  trackingSelectionModeFor,
  trackingSlugToken,
  trackingTitleFromToken,
} from './tracking-control-catalog-metadata-layout';

export const policyLaneFor = trackingPolicyLaneFor;
export const controlKindFor = (sourceText: string, explicitControlKind: string | null) =>
  trackingControlKindFor(sourceText, explicitControlKind, trackingProposalKinds());
export const selectionModeFor = trackingSelectionModeFor;
export const cardKindFor = trackingCardKindFor;
export const layoutHintsFor = trackingLayoutHintsFor;
export const effectStatusFor = trackingEffectStatusFor;
export const runtimeOwnerFor = trackingRuntimeOwnerFor;
export const capabilityStateFor = trackingCapabilityStateFor;
export const capabilityStateFromSourceState = trackingCapabilityStateFromSourceState;
export const capabilityRequirementFor = trackingCapabilityRequirementFor;
export const proofRequirementFor = trackingProofRequirementFor;
export const fallbackFor = trackingFallbackFor;
export const helperTextFor = trackingHelperTextFor;
export const questionFromSourceText = trackingQuestionFromSourceText;
export const explicitOptionLabels = trackingExplicitOptionLabels;
export const slugToken = trackingSlugToken;
export const titleFromToken = trackingTitleFromToken;
