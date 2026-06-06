import type { ReactElement } from 'react';
import {
  PortalDetails,
  PortalDom,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import {
  emptyNetworkProductReadinessStatusSummary,
  type NetworkProductReadinessStatusSummary,
} from './network-product-readiness-status';

export function NetworkRiskPerformanceReadinessCard({
  status,
}: {
  readonly status: NetworkProductReadinessStatusSummary | null;
}): ReactElement {
  const summary = status ?? emptyNetworkProductReadinessStatusSummary();
  return (
    <article className={networkRiskPerformanceCardClassName()}>
      <h2>{PortalDetails.RiskBudgetDetails}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <NetworkRiskReadinessDetails summary={summary} />
        <NetworkPerformanceReadinessDetails summary={summary} />
      </dl>
    </article>
  );
}

function NetworkRiskReadinessDetails({
  summary,
}: {
  readonly summary: NetworkProductReadinessStatusSummary;
}): ReactElement {
  return (
    <>
      <NetworkRiskPerformanceDetail label={PortalDetails.RiskEvaluation} value={summary.riskEvaluationRef} />
      <NetworkRiskPerformanceDetail label={PortalDetails.Profile} value={summary.riskChildProfileRef} />
      <NetworkRiskPerformanceDetail label={PortalDetails.PolicyPreview} value={summary.riskHouseholdPolicyRef} />
      <NetworkRiskPerformanceDetail label={PortalDetails.RuntimeReference} value={summary.riskCascadeRef} />
      <NetworkRiskPerformanceDetail label={PortalDetails.Level} value={summary.riskAgeBand} />
      <NetworkRiskPerformanceDetail label={PortalDetails.PolicyPreview} value={summary.riskBudgetState} />
      <NetworkRiskPerformanceDetail label={PortalDetails.ManualReview} value={summary.riskInterventionState} />
      <NetworkRiskPerformanceDetail label={PortalDetails.RowCount} value={summary.riskTotalPoints} />
      <NetworkRiskPerformanceDetail label={PortalDetails.RiskPointBreakdown} value={summary.riskPointBreakdown} />
      <NetworkRiskPerformanceDetail label={PortalDetails.RiskSignalReferences} value={summary.riskCitedSignalRefs} />
      <NetworkRiskPerformanceDetail label={PortalDetails.RiskAuditReferences} value={summary.riskCitedAuditRefs} />
      <NetworkRiskPerformanceDetail label={PortalDetails.EvidenceReferences} value={summary.riskCitedEvidenceRefs} />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.ParentRuleContextReferences}
        value={summary.riskCitedParentRuleRefs}
      />
      <NetworkRiskPerformanceDetail label={PortalDetails.EventHistory} value={summary.riskCitedPriorEventRefs} />
      <NetworkRiskPerformanceDetail label={PortalDetails.RiskAdapterProofState} value={summary.riskAdapterProofState} />
      <NetworkRiskPerformanceDetail label={PortalDetails.DecisionSource} value={summary.riskBudgetAdvisoryOnly} />
    </>
  );
}

function NetworkPerformanceReadinessDetails({
  summary,
}: {
  readonly summary: NetworkProductReadinessStatusSummary;
}): ReactElement {
  return (
    <>
      <NetworkRiskPerformanceDetail
        label={PortalDetails.PerformanceDetails}
        value={summary.performanceBenchmarkRunRef}
      />
      <NetworkRiskPerformanceDetail label={PortalDetails.EvidenceReferences} value={summary.performanceFixtureSetRef} />
      <NetworkRiskPerformanceDetail label={PortalDetails.EventHistory} value={summary.performanceEventHistoryRef} />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.ResourceClass}
        value={summary.performanceResourceSnapshotRef}
      />
      <NetworkRiskPerformanceDetail label={PortalDetails.ExecutionState} value={summary.performanceState} />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.PerformanceRegressionCodes}
        value={summary.performanceRegressionCodes}
      />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.PerformanceScenarioCounts}
        value={summary.performanceScenarioCounts}
      />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.PerformanceLatencyMetrics}
        value={summary.performanceLatencyMetrics}
      />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.PerformanceThroughputMetrics}
        value={summary.performanceThroughputMetrics}
      />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.PerformanceResourceMetrics}
        value={summary.performanceResourceMetrics}
      />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.PerformanceQualityMetrics}
        value={summary.performanceQualityMetrics}
      />
      <NetworkRiskPerformanceDetail label={PortalDetails.DryRun} value={summary.performancePathStates} />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.ProductionSloClaim}
        value={summary.performanceProductionSloClaimed}
      />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.AdapterDispatch}
        value={summary.performanceAdapterExecutionClaimed}
      />
      <NetworkRiskPerformanceDetail
        label={PortalDetails.HostFiltering}
        value={summary.performanceHostFilteringClaimed}
      />
    </>
  );
}

function NetworkRiskPerformanceDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: PortalDetailValue;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function networkRiskPerformanceCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
