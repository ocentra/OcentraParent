type AppGameInstallStoreHandoffSignalKindRuleInput =
  | 'new-inventory-detected'
  | 'installer-updater-process'
  | 'store-package-install'
  | 'game-purchase-signal'
  | 'uninstall-detected'
  | 'tamper-uninstall-candidate';

type AppGameInstallStoreHandoffDecisionAuthorityRuleInput =
  | 'evidence-context-only'
  | 'approval-feature-handoff'
  | 'tamper-uninstall-feature-handoff'
  | 'manual-review-required';

type AppGameInstallStoreHandoffCapabilityStateRuleInput =
  | 'supported'
  | 'unavailable'
  | 'degraded'
  | 'dry-run'
  | 'observe-only'
  | 'manual-required';

type AppGameInstallStoreHandoffApprovalSupportStateRuleInput = 'supported' | 'manual-required' | 'unavailable';

interface AppGameInstallStoreHandoffRuleInput {
  readonly signalKind: AppGameInstallStoreHandoffSignalKindRuleInput;
  readonly decisionAuthority: AppGameInstallStoreHandoffDecisionAuthorityRuleInput;
  readonly capabilityState: AppGameInstallStoreHandoffCapabilityStateRuleInput;
  readonly approvalSupportState: AppGameInstallStoreHandoffApprovalSupportStateRuleInput;
  readonly storeSignalUse: 'not-store-signal' | 'context-only-not-decision';
  readonly approvalRequestRef: unknown | null;
  readonly manualRequirement: unknown | null;
  readonly parentVisibleManualState: unknown | null;
  readonly evidenceReferences: readonly unknown[];
  readonly destinationFeatureDocs: readonly string[];
  readonly expectationDocRefs: readonly string[];
  readonly noClaimBoundaries: readonly string[];
  readonly adapterExecutionClaim: 'not-claimed';
  readonly policyDecisionClaim: 'not-claimed';
}

const storeOrPurchaseSignals: readonly AppGameInstallStoreHandoffSignalKindRuleInput[] = [
  'store-package-install',
  'game-purchase-signal',
];

const installApprovalSignals: readonly AppGameInstallStoreHandoffSignalKindRuleInput[] = [
  'new-inventory-detected',
  'installer-updater-process',
  'store-package-install',
  'game-purchase-signal',
];

const uninstallSignals: readonly AppGameInstallStoreHandoffSignalKindRuleInput[] = [
  'uninstall-detected',
  'tamper-uninstall-candidate',
];

const appInstallFeatureDoc = 'docs/features/app-install-purchase-approval.md';
const tamperFeatureDoc = 'docs/features/enforcement-integrity-tamper.md';
const tamperExpectationDoc = 'docs/expectations/tamper-uninstall-protection.md';

export function appGameInstallStoreHandoffRowIsHonest(row: AppGameInstallStoreHandoffRuleInput): boolean {
  return (
    row.evidenceReferences.length > 0 &&
    storeAndPurchaseSignalsStayContextOnly(row) &&
    installApprovalHandoffsCarryEvidence(row) &&
    uninstallAndTamperSignalsRouteToTamper(row) &&
    manualRequiredRowsAreParentVisible(row) &&
    row.noClaimBoundaries.includes('no-platform-adapter-execution') &&
    row.adapterExecutionClaim === 'not-claimed' &&
    row.policyDecisionClaim === 'not-claimed'
  );
}

function storeAndPurchaseSignalsStayContextOnly(row: AppGameInstallStoreHandoffRuleInput): boolean {
  if (!storeOrPurchaseSignals.includes(row.signalKind)) {
    return true;
  }

  return (
    row.storeSignalUse === 'context-only-not-decision' &&
    row.decisionAuthority !== 'evidence-context-only' &&
    row.noClaimBoundaries.includes('store-signal-not-safety-decision') &&
    row.capabilityState !== 'supported'
  );
}

function installApprovalHandoffsCarryEvidence(row: AppGameInstallStoreHandoffRuleInput): boolean {
  if (!installApprovalSignals.includes(row.signalKind)) {
    return true;
  }

  return (
    row.destinationFeatureDocs.includes(appInstallFeatureDoc) &&
    row.approvalRequestRef !== null &&
    row.evidenceReferences.length > 0 &&
    row.noClaimBoundaries.includes('not-generic-app-blocking')
  );
}

function uninstallAndTamperSignalsRouteToTamper(row: AppGameInstallStoreHandoffRuleInput): boolean {
  if (!uninstallSignals.includes(row.signalKind)) {
    return true;
  }

  return (
    row.destinationFeatureDocs.includes(tamperFeatureDoc) &&
    row.expectationDocRefs.includes(tamperExpectationDoc) &&
    row.approvalRequestRef === null &&
    row.decisionAuthority === 'tamper-uninstall-feature-handoff'
  );
}

function manualRequiredRowsAreParentVisible(row: AppGameInstallStoreHandoffRuleInput): boolean {
  if (row.capabilityState !== 'manual-required' && row.approvalSupportState !== 'manual-required') {
    return true;
  }

  return row.manualRequirement !== null && row.parentVisibleManualState !== null;
}
