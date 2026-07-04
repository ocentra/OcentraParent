/* generated from crates/network-core/src/network_control_catalog_metadata.ts.txt */

import {
  networkCapabilityRequirementFor,
  networkCapabilityStateFor,
  networkCapabilityStateFromSourceState,
  networkEffectStatusFor,
  networkFallbackFor,
  networkPolicyLaneFor,
  networkProofRequirementFor,
  networkRuntimeOwnerFor,
} from './network-control-catalog-metadata-classifiers';
import {
  networkCardKindFor,
  networkControlKindFor,
  networkHelperTextFor,
  networkLayoutHintsFor,
  networkSelectionModeFor,
} from './network-control-catalog-metadata-layout';
import {
  networkExplicitOptionLabels,
  networkQuestionFromSourceText,
  networkSlugToken,
  networkTitleFromToken,
} from './network-control-catalog-metadata-text';

export const policyLaneFor = networkPolicyLaneFor;
export const controlKindFor = (sourceText: string, explicitKind: string | null) =>
  networkControlKindFor(sourceText, explicitKind, networkExplicitOptionLabels);
export const selectionModeFor = networkSelectionModeFor;
export const cardKindFor = networkCardKindFor;
export const layoutHintsFor = networkLayoutHintsFor;
export const effectStatusFor = networkEffectStatusFor;
export const runtimeOwnerFor = networkRuntimeOwnerFor;
export const capabilityStateFor = networkCapabilityStateFor;
export const capabilityStateFromSourceState = networkCapabilityStateFromSourceState;
export const capabilityRequirementFor = networkCapabilityRequirementFor;
export const proofRequirementFor = networkProofRequirementFor;
export const fallbackFor = networkFallbackFor;
export const helperTextFor = (sectionTitle: string, groupTitle: string, sourceText: string) =>
  networkHelperTextFor(
    sectionTitle,
    groupTitle,
    sourceText,
    networkProofRequirementFor,
    networkCapabilityRequirementFor
  );
export const questionFromSourceText = networkQuestionFromSourceText;
export const explicitOptionLabels = networkExplicitOptionLabels;
export const slugToken = networkSlugToken;
export const titleFromToken = networkTitleFromToken;
