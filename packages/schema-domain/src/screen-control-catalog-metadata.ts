import {
  screenCapabilityRequirementFor,
  screenCapabilityStateFor,
  screenEffectStatusFor,
  screenFallbackFor,
  screenProofRequirementFor,
  screenRuntimeOwnerFor,
  screenUiTabFor,
} from './screen-control-catalog-metadata-classifiers';
import {
  screenCardKindFor,
  screenLayoutHintsFor,
  screenOptionsForSetting,
  screenSelectionModeFor,
} from './screen-control-catalog-metadata-layout';
import {
  screenEnabledConditionsFor,
  screenSourceEffectStatus,
  screenValidationRulesFor,
  screenVisibilityConditionsFor,
} from './screen-control-catalog-metadata-rules';
import {
  screenExplicitOptionLabels,
  screenQuestionFromSeed,
} from './screen-control-catalog-metadata-text';
import type {
  ScreenControlCatalogSourceKind,
} from './screen-control-catalog-schema';

export const ScreenControlCatalogSourceDocuments = [
  'docs/screen-evidence-analysis-capability-guide.md',
  'docs/screen-evidence-analysis-schema-proposal.md',
] as const;
export const ScreenControlCatalogSidePanelCategory = 'screen';

export const ScreenControlCatalogTargetScopeOptions = options(['Family', 'Per Child', 'Per Device', 'Per Platform']);

export const ScreenControlCatalogEffectModeOptions = options([
  'Off',
  'Observe',
  'Dry Run',
  'Notify',
  'Ask',
  'Warn',
  'Limit',
  'Block',
  'Enforce',
  'Audit Only',
]);
export const optionsForSetting = screenOptionsForSetting;
export const selectionModeFor = screenSelectionModeFor;
export const cardKindFor = screenCardKindFor;
export const layoutHintsFor = screenLayoutHintsFor;
export const questionFromSeed = screenQuestionFromSeed;
export const uiTabFor = screenUiTabFor;
export const effectStatusFor = screenEffectStatusFor;
export const capabilityStateFor = (
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
) => screenCapabilityStateFor(screenEffectStatusFor(sourceKind, sectionTitle, sourceText), sourceText);
export const runtimeOwnerFor = screenRuntimeOwnerFor;
export const capabilityRequirementFor = screenCapabilityRequirementFor;
export const proofRequirementFor = screenProofRequirementFor;
export const fallbackFor = (
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
) => {
  const status = screenSourceEffectStatus(sourceKind, sectionTitle, sourceText, screenEffectStatusFor);
  return status === 'permission-required' || status === 'permission-limited'
    ? 'Skip capture and audit permission-required or permission-limited state before any queue job is created.'
    : screenFallbackFor(status);
};
export const visibilityConditionsFor = screenVisibilityConditionsFor;
export const enabledConditionsFor = (
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
) => screenEnabledConditionsFor(screenSourceEffectStatus(sourceKind, sectionTitle, sourceText, screenEffectStatusFor));
export const validationRulesFor = (
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
) =>
  screenValidationRulesFor(
    screenSourceEffectStatus(sourceKind, sectionTitle, sourceText, screenEffectStatusFor),
    screenProofRequirementFor(sectionTitle, sourceText)
  );

function options(labels: readonly string[]) {
  return screenOptionsForSetting('', 'single-choice', labels);
}
