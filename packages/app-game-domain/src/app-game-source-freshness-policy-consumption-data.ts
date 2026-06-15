import {
  AppGameSourceFreshnessPolicyConsumptionMatrixSchema,
  evaluateAppGameSourceFreshnessPolicyReadiness,
} from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceFreshnessCapabilityStatus as CapabilityStatus,
  AppGameSourceFreshnessPolicyConsumptionMatrixId,
  AppGameSourceFreshnessPolicyTargetKind as TargetKind,
  AppGameSourceFreshnessReadModelState as ReadModelState,
  AppGameSourceFreshnessRequirementKind as RequirementKind,
  AppGameSourceFreshnessSourceKind as SourceKind,
} from './app-game-source-freshness-policy-consumption-values';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';

const GeneratedAt = '2026-06-04T12:55:00.000Z';
const FreshObservedAt = '2026-06-04T12:54:00.000Z';
const StaleObservedAt = '2026-06-04T09:00:00.000Z';

export const AppGameSourceFreshnessPolicyConsumptionRequests = [
  {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    policyRequestId: 'source-freshness-native-app-ready-request',
    target: {
      targetKind: TargetKind.NativeApp,
      targetRef: 'app-target-parental-controls-helper',
    },
    requiredSources: [RequirementKind.Inventory, RequirementKind.Runtime, RequirementKind.Foreground],
    maxSourceAgeMs: 600000,
    sourceRowsFromActivityReadModel: true,
    rawPrivateSourceRowsIncluded: false,
    requestedAt: GeneratedAt,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.OsInstalledRecord,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: FreshObservedAt,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-inventory-parental-controls-helper'],
      },
      {
        sourceKind: SourceKind.ProcessSnapshot,
        state: ReadModelState.Ready,
        rowCount: 2,
        lastObservedAt: FreshObservedAt,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-runtime-parental-controls-helper'],
      },
      {
        sourceKind: SourceKind.ForegroundWindow,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: FreshObservedAt,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-foreground-parental-controls-helper'],
      },
    ],
  },
  {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    policyRequestId: 'source-freshness-native-game-ready-request',
    target: {
      targetKind: TargetKind.NativeGame,
      targetRef: 'game-target-launcher-child-game',
    },
    requiredSources: [
      RequirementKind.Inventory,
      RequirementKind.Runtime,
      RequirementKind.Foreground,
      RequirementKind.Launcher,
    ],
    maxSourceAgeMs: 600000,
    sourceRowsFromActivityReadModel: true,
    rawPrivateSourceRowsIncluded: false,
    requestedAt: GeneratedAt,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.StorePackage,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: FreshObservedAt,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-store-package-child-game'],
      },
      {
        sourceKind: SourceKind.ProcessStart,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: FreshObservedAt,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-runtime-child-game'],
      },
      {
        sourceKind: SourceKind.ForegroundWindow,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: FreshObservedAt,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-foreground-child-game'],
      },
      {
        sourceKind: SourceKind.LauncherManifest,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: FreshObservedAt,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-launcher-child-game'],
      },
    ],
  },
  {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    policyRequestId: 'source-freshness-native-game-manual-request',
    target: {
      targetKind: TargetKind.NativeGame,
      targetRef: 'game-target-stale-or-missing-game',
    },
    requiredSources: [RequirementKind.Runtime, RequirementKind.Foreground, RequirementKind.Launcher],
    maxSourceAgeMs: 600000,
    sourceRowsFromActivityReadModel: true,
    rawPrivateSourceRowsIncluded: false,
    requestedAt: GeneratedAt,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.ProcessSnapshot,
        state: ReadModelState.Stale,
        rowCount: 1,
        lastObservedAt: StaleObservedAt,
        capabilityStatus: CapabilityStatus.Stale,
        evidence: ['evidence-game-runtime-stale'],
      },
      {
        sourceKind: SourceKind.LauncherManifest,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: FreshObservedAt,
        capabilityStatus: CapabilityStatus.NotClaimed,
        evidence: ['evidence-game-launcher-not-claimed'],
      },
    ],
  },
] as const;

export const AppGameSourceFreshnessPolicyConsumptionMatrix = AppGameSourceFreshnessPolicyConsumptionMatrixSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  matrixId: AppGameSourceFreshnessPolicyConsumptionMatrixId,
  generatedAt: GeneratedAt,
  readiness: AppGameSourceFreshnessPolicyConsumptionRequests.map((request, index) =>
    evaluateAppGameSourceFreshnessPolicyReadiness(
      request,
      `source-freshness-policy-readiness-${index + 1}`,
      GeneratedAt
    )
  ),
});
