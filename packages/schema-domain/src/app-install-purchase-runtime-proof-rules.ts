/* compatibility shim over Rust-generated app-install purchase runtime proof rules */

import type {
  GeneratedAppInstallPurchaseRuntimeChildDeliveryRow,
  GeneratedAppInstallPurchaseRuntimePlatformArtifactRow,
  GeneratedAppInstallPurchaseRuntimeProof,
  GeneratedAppInstallPurchaseRuntimeReportIntegrationRow,
  GeneratedAppInstallPurchaseRuntimeStatusReadinessRow,
} from './generated/app-install-purchase-runtime-proof-rules';
import {
  appInstallPurchaseRuntimeChildDeliveryRowIsHonestGenerated,
  appInstallPurchaseRuntimePlatformArtifactRowIsHonestGenerated,
  appInstallPurchaseRuntimeProofIsHonestGenerated,
  appInstallPurchaseRuntimeReportIntegrationRowIsHonestGenerated,
  appInstallPurchaseRuntimeStatusReadinessRowIsHonestGenerated,
} from './generated/app-install-purchase-runtime-proof-rules';

export function appInstallPurchaseRuntimeProofIsHonest(proof: GeneratedAppInstallPurchaseRuntimeProof): boolean {
  return appInstallPurchaseRuntimeProofIsHonestGenerated(proof);
}

export function appInstallPurchaseRuntimePlatformArtifactRowIsHonest(
  row: GeneratedAppInstallPurchaseRuntimePlatformArtifactRow
): boolean {
  return appInstallPurchaseRuntimePlatformArtifactRowIsHonestGenerated(row);
}

export function appInstallPurchaseRuntimeChildDeliveryRowIsHonest(
  row: GeneratedAppInstallPurchaseRuntimeChildDeliveryRow
): boolean {
  return appInstallPurchaseRuntimeChildDeliveryRowIsHonestGenerated(row);
}

export function appInstallPurchaseRuntimeReportIntegrationRowIsHonest(
  row: GeneratedAppInstallPurchaseRuntimeReportIntegrationRow
): boolean {
  return appInstallPurchaseRuntimeReportIntegrationRowIsHonestGenerated(row);
}

export function appInstallPurchaseRuntimeStatusReadinessRowIsHonest(
  row: GeneratedAppInstallPurchaseRuntimeStatusReadinessRow
): boolean {
  return appInstallPurchaseRuntimeStatusReadinessRowIsHonestGenerated(row);
}
