/* thin adapter over Rust-generated browser control catalog metadata helpers */

import { BrowserControlFieldIdSchema, BrowserControlOptionIdSchema } from './browser-control-identifiers';
import type {
  BrowserControlFullCatalogCapabilityState,
  BrowserControlFullCatalogCardKind,
  BrowserControlFullCatalogControlKind,
  BrowserControlFullCatalogEffectStatus,
  BrowserControlFullCatalogLayoutHints,
  BrowserControlFullCatalogOption,
  BrowserControlFullCatalogRule,
  BrowserControlFullCatalogRuntimeOwner,
  BrowserControlFullCatalogSelectionMode,
} from './browser-control-full-catalog-schema';
import {
  capabilityRequirementForGenerated,
  capabilityStateForSectionGenerated,
  cardKindForGenerated,
  controlKindForGenerated,
  effectStatusForSectionGenerated,
  enabledConditionsForGenerated,
  fallbackForGenerated,
  helperTextForGenerated,
  layoutHintsForGenerated,
  optionsFromSourceTextGenerated,
  proofRequirementForGenerated,
  questionFromSourceTextGenerated,
  runtimeOwnerForSectionGenerated,
  selectionModeForGenerated,
  validationRulesForGenerated,
  visibilityConditionsForGenerated,
} from './generated-browser-policy-control-catalog-helpers';

export function optionsFromSourceText(sourceText: string): BrowserControlFullCatalogOption[] {
  return optionsFromSourceTextGenerated(sourceText).map(parseCatalogOption);
}

export function selectionModeFor(
  sourceText: string,
  settingOptions: readonly BrowserControlFullCatalogOption[]
): BrowserControlFullCatalogSelectionMode {
  return selectionModeForGenerated(sourceText, settingOptions) as BrowserControlFullCatalogSelectionMode;
}

export function cardKindFor(
  selectionMode: BrowserControlFullCatalogSelectionMode,
  settingOptions: readonly BrowserControlFullCatalogOption[]
): BrowserControlFullCatalogCardKind {
  return cardKindForGenerated(selectionMode, settingOptions) as BrowserControlFullCatalogCardKind;
}

export function controlKindFor(
  sourceText: string,
  selectionMode: BrowserControlFullCatalogSelectionMode,
  settingOptions: readonly BrowserControlFullCatalogOption[]
): BrowserControlFullCatalogControlKind {
  return controlKindForGenerated(sourceText, selectionMode, settingOptions) as BrowserControlFullCatalogControlKind;
}

export function layoutHintsFor(
  selectionMode: BrowserControlFullCatalogSelectionMode,
  settingOptions: readonly BrowserControlFullCatalogOption[]
): BrowserControlFullCatalogLayoutHints {
  return layoutHintsForGenerated(selectionMode, settingOptions) as BrowserControlFullCatalogLayoutHints;
}

export function questionFromSourceText(sourceText: string): string {
  return questionFromSourceTextGenerated(sourceText);
}

export function helperTextFor(sectionTitle: string, sourceText: string): string {
  return helperTextForGenerated(sectionTitle, sourceText);
}

export function effectStatusForSection(
  sectionTitle: string,
  sourceText: string
): BrowserControlFullCatalogEffectStatus {
  return effectStatusForSectionGenerated(sectionTitle, sourceText) as BrowserControlFullCatalogEffectStatus;
}

export function runtimeOwnerForSection(
  sectionTitle: string,
  sourceText: string
): BrowserControlFullCatalogRuntimeOwner {
  return runtimeOwnerForSectionGenerated(sectionTitle, sourceText) as BrowserControlFullCatalogRuntimeOwner;
}

export function capabilityStateForSection(
  sectionTitle: string,
  sourceText: string
): BrowserControlFullCatalogCapabilityState {
  return capabilityStateForSectionGenerated(sectionTitle, sourceText) as BrowserControlFullCatalogCapabilityState;
}

export function capabilityRequirementFor(sectionTitle: string, sourceText: string): string {
  return capabilityRequirementForGenerated(sectionTitle, sourceText);
}

export function proofRequirementFor(sectionTitle: string, sourceText: string): string | null {
  return proofRequirementForGenerated(sectionTitle, sourceText);
}

export function fallbackFor(sectionTitle: string, sourceText: string): string {
  return fallbackForGenerated(sectionTitle, sourceText);
}

export function visibilityConditionsFor(): BrowserControlFullCatalogRule[] {
  return visibilityConditionsForGenerated().map(parseCatalogRule);
}

export function enabledConditionsFor(sectionTitle: string, sourceText: string): BrowserControlFullCatalogRule[] {
  return enabledConditionsForGenerated(sectionTitle, sourceText).map(parseCatalogRule);
}

export function validationRulesFor(sectionTitle: string, sourceText: string): BrowserControlFullCatalogRule[] {
  return validationRulesForGenerated(sectionTitle, sourceText).map(parseCatalogRule);
}

function parseCatalogOption(option: {
  readonly optionId: string;
  readonly label: string;
  readonly value: string;
  readonly originalSourceText: string;
  readonly meaning: string | null;
  readonly defaultSelected: boolean;
}): BrowserControlFullCatalogOption {
  return {
    ...option,
    optionId: BrowserControlOptionIdSchema.parse(option.optionId),
  };
}

function parseCatalogRule(rule: { readonly ruleId: string; readonly description: string }): BrowserControlFullCatalogRule {
  return {
    ...rule,
    ruleId: BrowserControlFieldIdSchema.parse(rule.ruleId),
  };
}
