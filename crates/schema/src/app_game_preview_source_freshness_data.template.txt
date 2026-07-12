/* generated from crates/schema/src/app_game_preview_source_freshness.rs */

import {
  AppGameSourceFreshnessCapabilityStatusGenerated as CapabilityStatus,
  AppGameSourceFreshnessPolicyTargetKindGenerated as TargetKind,
  AppGameSourceFreshnessReadModelStateGenerated as ReadModelState,
  AppGameSourceFreshnessRequirementKindGenerated as RequirementKind,
  AppGameSourceFreshnessSourceKindGenerated as SourceKind,
} from './generated-app-game-preview-source-freshness-values';

export const AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated = '2026-06-04T12:55:00.000Z' as const;
export const AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated = '2026-06-04T12:54:00.000Z' as const;
export const AppGameSourceFreshnessPolicyConsumptionStaleObservedAtGenerated = '2026-06-04T09:00:00.000Z' as const;

export const AppGameSourceFreshnessPolicyConsumptionRequestsGenerated = [
  {
    schemaVersion: 'v0.6',
    policyRequestId: 'source-freshness-native-app-ready-request',
    target: {
      targetKind: TargetKind.NativeApp,
      targetRef: 'app-target-parental-controls-helper',
    },
    requiredSources: [RequirementKind.Inventory, RequirementKind.Runtime, RequirementKind.Foreground],
    maxSourceAgeMs: 600000,
    sourceRowsFromActivityReadModel: true,
    rawPrivateSourceRowsIncluded: false,
    requestedAt: AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.OsInstalledRecord,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-inventory-parental-controls-helper'],
      },
      {
        sourceKind: SourceKind.ProcessSnapshot,
        state: ReadModelState.Ready,
        rowCount: 2,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-runtime-parental-controls-helper'],
      },
      {
        sourceKind: SourceKind.ForegroundWindow,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-app-foreground-parental-controls-helper'],
      },
    ],
  },
  {
    schemaVersion: 'v0.6',
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
    requestedAt: AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.StorePackage,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-store-package-child-game'],
      },
      {
        sourceKind: SourceKind.ProcessStart,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-runtime-child-game'],
      },
      {
        sourceKind: SourceKind.ForegroundWindow,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-foreground-child-game'],
      },
      {
        sourceKind: SourceKind.LauncherManifest,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Available,
        evidence: ['evidence-game-launcher-child-game'],
      },
    ],
  },
  {
    schemaVersion: 'v0.6',
    policyRequestId: 'source-freshness-native-game-manual-request',
    target: {
      targetKind: TargetKind.NativeGame,
      targetRef: 'game-target-stale-or-missing-game',
    },
    requiredSources: [RequirementKind.Runtime, RequirementKind.Foreground, RequirementKind.Launcher],
    maxSourceAgeMs: 600000,
    sourceRowsFromActivityReadModel: true,
    rawPrivateSourceRowsIncluded: false,
    requestedAt: AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated,
    sourceStatusRows: [
      {
        sourceKind: SourceKind.ProcessSnapshot,
        state: ReadModelState.Stale,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionStaleObservedAtGenerated,
        capabilityStatus: CapabilityStatus.Stale,
        evidence: ['evidence-game-runtime-stale'],
      },
      {
        sourceKind: SourceKind.LauncherManifest,
        state: ReadModelState.Ready,
        rowCount: 1,
        lastObservedAt: AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated,
        capabilityStatus: CapabilityStatus.NotClaimed,
        evidence: ['evidence-game-launcher-not-claimed'],
      },
    ],
  },
] as const;
