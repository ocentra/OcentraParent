import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';

import { BaselineScreenControlCatalog, screenControlCatalogSettings } from './screen-control-catalog';
import {
  type ScreenControlCatalog,
  type ScreenControlCatalogCapabilityState,
  type ScreenControlCatalogEffectStatus,
  type ScreenControlCatalogSetting,
} from './screen-control-catalog-schema';

export const ScreenControlSettingsPortalMetricSchema = withParser(
  Schema.Struct({
    label: NonEmptyStringSchema,
    value: NonEmptyStringSchema,
    detail: NonEmptyStringSchema,
  })
);

export const ScreenControlSettingsPortalGateSchema = withParser(
  Schema.Struct({
    label: NonEmptyStringSchema,
    status: NonEmptyStringSchema,
    statusText: NonEmptyStringSchema,
    capabilityState: NonEmptyStringSchema,
    runtimeOwner: NonEmptyStringSchema,
    detail: NonEmptyStringSchema,
    sourceDocument: NonEmptyStringSchema,
  })
);

export const ScreenControlSettingsPortalProofSchema = withParser(
  Schema.Struct({
    title: NonEmptyStringSchema,
    note: NonEmptyStringSchema,
    metrics: Schema.Array(ScreenControlSettingsPortalMetricSchema),
    gates: Schema.Array(ScreenControlSettingsPortalGateSchema),
  })
);

export type ScreenControlSettingsPortalMetric = Infer<typeof ScreenControlSettingsPortalMetricSchema>;
export type ScreenControlSettingsPortalGate = Infer<typeof ScreenControlSettingsPortalGateSchema>;
export type ScreenControlSettingsPortalProof = Infer<typeof ScreenControlSettingsPortalProofSchema>;

const PortalProofText = {
  title: 'Screen settings and capability proof',
  note: 'Read-only Settings proof from the Screen control catalog; child runtime owns capture, queue, local analysis, policy handoff, and audit.',
  totalSettingsLabel: 'Catalog settings',
  totalSettingsDetail: 'Screen settings parsed from the current capability guide and schema proposal.',
  tabCountLabel: 'Catalog tabs',
  tabCountDetail: 'Parent-facing Screen categories available for read-only rendering.',
  proofRequiredLabel: 'Proof-required controls',
  proofRequiredDetail:
    'Strict behavior requires platform capture, local analysis, deletion, or policy proof before use.',
  unavailableLabel: 'Unavailable sensitive modes',
  unavailableDetail:
    'Raw retention, hosted processing, hidden capture, continuous recording, and unsupported sensitive states fail closed.',
} as const;

const ImportantGateSourceTexts = [
  'Allow Ocentra-hosted processing of child screen images?',
  'Show raw screenshots in parent reports by default?',
  'Retain raw screenshots or recordings?',
  'Allow screen summaries to be used by policy?',
  'local OCR/vision returns schema-valid output;',
] as const;

export function screenControlSettingsPortalProof(
  catalog: ScreenControlCatalog = BaselineScreenControlCatalog
): ScreenControlSettingsPortalProof {
  const settings = screenControlCatalogSettings(catalog);
  return ScreenControlSettingsPortalProofSchema.parse({
    title: PortalProofText.title,
    note: PortalProofText.note,
    metrics: [
      metric(PortalProofText.totalSettingsLabel, settings.length, PortalProofText.totalSettingsDetail),
      metric(PortalProofText.tabCountLabel, catalog.tabs.length, PortalProofText.tabCountDetail),
      metric(
        PortalProofText.proofRequiredLabel,
        countEffectStatus(settings, 'proof-required'),
        PortalProofText.proofRequiredDetail
      ),
      metric(
        PortalProofText.unavailableLabel,
        countEffectStatus(settings, 'unavailable'),
        PortalProofText.unavailableDetail
      ),
    ],
    gates: ImportantGateSourceTexts.map((sourceText) => gateFromSetting(settingBySourceText(settings, sourceText))),
  });
}

function metric(label: string, value: number, detail: string): ScreenControlSettingsPortalMetric {
  return {
    label,
    value: String(value),
    detail,
  };
}

function gateFromSetting(setting: ScreenControlCatalogSetting): ScreenControlSettingsPortalGate {
  return {
    label: setting.uiQuestionText,
    status: setting.effectStatus,
    statusText: statusText(setting.effectStatus, setting.capabilityState),
    capabilityState: setting.capabilityState,
    runtimeOwner: setting.runtimeOwner,
    detail: gateDetail(setting),
    sourceDocument: setting.sourceDocument,
  };
}

function gateDetail(setting: ScreenControlCatalogSetting): string {
  return (
    setting.unsafeOrUnsupportedFallback ??
    setting.proofRequirement ??
    setting.capabilityRequirement ??
    setting.helperText ??
    setting.sourceText
  );
}

function statusText(
  effectStatus: ScreenControlCatalogEffectStatus,
  capabilityState: ScreenControlCatalogCapabilityState
): string {
  return `${effectStatus} / ${capabilityState}`;
}

function countEffectStatus(
  settings: readonly ScreenControlCatalogSetting[],
  effectStatus: ScreenControlCatalogEffectStatus
): number {
  return settings.filter((setting) => setting.effectStatus === effectStatus).length;
}

function settingBySourceText(settings: readonly ScreenControlCatalogSetting[], sourceText: string) {
  const setting = settings.find((candidate) => candidate.sourceText === sourceText);
  if (setting === undefined) {
    throw new Error(`Missing Screen control catalog setting: ${sourceText}`);
  }
  return setting;
}
