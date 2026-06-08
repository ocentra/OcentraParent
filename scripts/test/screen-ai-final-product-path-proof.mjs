import {
  BrowserScenarioIds,
  LiveScenarioIds,
  ProofPath,
  SourcePaths,
  existsPath,
  readJson,
  repoPath,
  writeProofOutputs,
} from './screen-ai-final-product-path-proof-values.mjs';

const failures = [];
const liveOperator = load(SourcePaths.liveOperator);
const liveOperatorAi = load(SourcePaths.liveOperatorAi);
const actionDispatch = load(SourcePaths.actionDispatch);
const aiPlanClosure = load(SourcePaths.aiPlanClosure);
const blockActionDispatch = load(SourcePaths.blockActionDispatch);
const deletionRetentionCustody = load(SourcePaths.deletionRetentionCustody);
const finalAdapterAudit = load(SourcePaths.finalAdapterAudit);
const householdMeshScreenAi = load(SourcePaths.householdMeshScreenAi);
const householdProviderResultValidation = load(SourcePaths.householdProviderResultValidation);
const portalChain = load(SourcePaths.portalChain);
const protectedSurface = load(SourcePaths.protectedSurface);
const readModel = load(SourcePaths.readModel);
const noRawScreenTransferMesh = load(SourcePaths.noRawScreenTransferMesh);
const retentionSweeper = load(SourcePaths.retentionSweeper);
const screenPlanClosure = load(SourcePaths.screenPlanClosure);
const serviceReadModel = load(SourcePaths.serviceReadModel);
const serviceAnalysisRowReady = load(SourcePaths.serviceAnalysisRowReady);
const serviceCaptureEventProducer = load(SourcePaths.serviceCaptureEventProducer);
const serviceDeletionEventProducer = load(SourcePaths.serviceDeletionEventProducer);
const serviceEventBridge = load(SourcePaths.serviceEventBridge);
const serviceEventSubscription = load(SourcePaths.serviceEventSubscription);
const servicePolicyRefProducer = load(SourcePaths.servicePolicyRefProducer);

const liveRows = validateLiveOperator();
const closure = {
  realTriggerRows: liveRows.length,
  browserLiveRows: liveRows.filter((row) => BrowserScenarioIds.has(row.scenarioId)).length,
  localAiRows: liveRows.filter((row) => row.localAiAnalyzed).length,
  policyDryRunRows: liveRows.filter((row) => row.policyDryRun).length,
  parentExplanationSnapshots: liveRows.filter((row) => row.parentExplanationSnapshotExists).length,
  rawDeletionRows: liveRows.filter((row) => row.rawImageDeleted).length,
  actionDispatchProven: validateActionDispatch(),
  portalReadModelProven: validatePortalChain(),
  readModelRows: validateReadModel(),
  serviceBackedReadModelProven: validateServiceReadModel(),
  serviceEventChainProven: validateServiceEventChain(),
  retentionCustodyProven: validateDeletionCustody(),
  protectedSurfaceSkipProven: validateProtectedSurface(),
  finalAdapterAuditProven: validateFinalAdapterAudit(),
  householdMeshBoundaryProven: validateHouseholdMeshBoundary(),
  screenPlanClosureAudited: validateScreenPlanClosure(),
  aiPlanClosureAudited: validateAiPlanClosure(),
};

assert(
  closure.realTriggerRows === LiveScenarioIds.length,
  `expected ${LiveScenarioIds.length} live/operator trigger rows, got ${closure.realTriggerRows}`
);
assert(closure.browserLiveRows === BrowserScenarioIds.size, 'expected all required live browser URL rows');
assert(closure.localAiRows === 8, `expected 8 local AI analyzed rows, got ${closure.localAiRows}`);
assert(closure.policyDryRunRows === 8, `expected 8 policy dry-run rows, got ${closure.policyDryRunRows}`);
assert(
  closure.parentExplanationSnapshots === 8,
  `expected 8 parent explanation snapshots, got ${closure.parentExplanationSnapshots}`
);
assert(closure.rawDeletionRows === 8, `expected 8 raw deletion rows, got ${closure.rawDeletionRows}`);

if (failures.length > 0) {
  throw new Error(
    `Screen AI final product path proof failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}`
  );
}

const proof = {
  status: 'ok',
  proofKind: 'screen-ai-final-product-path-proof',
  generatedAt: new Date().toISOString(),
  sourceArtifacts: SourcePaths,
  closure: {
    ...closure,
    finalPathEvidenceComplete: true,
    screenAndAiPrerequisitesStacked: closure.screenPlanClosureAudited && closure.aiPlanClosureAudited,
    broadBrowserNetworkMobileProductComplete: false,
    adapterProductCompleteBlockedByAudit: true,
    finalPipelineProductComplete: false,
    finalPipelineProductCompleteBlockedByAdapterGate: true,
    custodyArtifactRows: finalAdapterAudit.closure?.custodyArtifactRows,
    householdMeshConsumesRedactedRefsOnly: closure.householdMeshBoundaryProven,
    serviceEventProducersAndSubscriberCovered: closure.serviceEventChainProven,
    singleRuntimeSessionRerun: false,
    retainedRealRunArtifactsVerified: true,
    rawScreenshotsRetainedByDefault: false,
    remoteAiUsedForChildSafety: false,
  },
  liveRows,
  nonClaims: [
    'This verifier validates retained real-run artifacts and does not rerun the live operator capture or model inference session.',
    'Managed-browser trigger producer ownership, authenticated-account social proof, and broad browser/network/mobile/Linux adapters remain separate unless their own execution artifacts are cited.',
    'The custody-aware final adapter audit is required by this proof and keeps broad/browser/network/mobile/Linux product-complete adapter execution blocked.',
    'The screen-plan and AI-plan closure audits are required by this proof; they stack prerequisites without overriding remaining external adapter and platform gates.',
    'Household mesh provider routing artifacts are required by this proof; provider work may carry redacted/custody refs only and child-agent validation remains local before policy.',
    'Service event producer/subscriber artifacts are required by this proof; the retained final path still does not rerun one single live service session.',
    'The proof closes the stacked real trigger-to-analysis-to-policy-to-action/read-model-to-deletion evidence path from current artifacts; it does not make raw screenshot retention or live view product claims.',
  ],
};

writeProofOutputs(proof);
console.log(`screen-ai-final-product-path-proof-ok:${ProofPath}`);

function validateLiveOperator() {
  assert(liveOperator.proof === 'screen-ai-live-operator-proof', 'live operator summary proof id mismatch');
  assert(liveOperator.fullRequiredMatrixComplete === true, 'live operator matrix is not complete');
  assert(liveOperator.liveExternalUrlProof === true, 'live operator missing live external URL proof');
  assert(liveOperator.localVlmAnalysisProof === true, 'live operator missing local VLM proof');
  assert(liveOperator.policyDryRunProof === true, 'live operator missing policy dry-run proof');
  assert(liveOperator.rawImagesDeletedAfterAnalysis === true, 'live operator missing raw deletion proof');
  assert(liveOperator.controlledFixtureProof === false, 'live operator still marks controlled fixture proof');
  assert(liveOperatorAi.proof === 'screen-ai-live-operator-proof', 'AI live operator summary proof id mismatch');

  return LiveScenarioIds.map((scenarioId) => validateLiveScenario(scenarioId));
}

function validateLiveScenario(scenarioId) {
  const summaryRow = liveOperator.scenarios.find((row) => row.scenarioId === scenarioId);
  assert(Boolean(summaryRow), `missing live operator scenario ${scenarioId}`);

  if (scenarioId === 'protected-unsupported-state') {
    return validateProtectedScenario(summaryRow);
  }

  const scenarioRoot = `output/ai-plan-proof/live-operator/${scenarioId}`;
  const source = load(`${scenarioRoot}/01-redacted-source-evidence.json`);
  const capture = load(`${scenarioRoot}/02-capture-proof-ref.json`);
  const ai = load(`${scenarioRoot}/06-ai-result.json`);
  const policy = load(`${scenarioRoot}/07-policy-decision.json`);
  const deletion = load(`${scenarioRoot}/08-deletion-after-analysis.json`);
  const explanationPath = `${scenarioRoot}/10-parent-explanation.png`;

  if (BrowserScenarioIds.has(scenarioId)) {
    assert(source.liveExternalUrl === true, `${scenarioId} is not marked as live external URL`);
    assert(source.protocol === 'https', `${scenarioId} source protocol is not HTTPS`);
    assert(source.pageReadiness?.loaded === true, `${scenarioId} page readiness is not loaded`);
    assert(source.pageReadiness?.visibleTextLength > 0, `${scenarioId} has no visible text readiness evidence`);
    assert(
      Object.values(source.pageReadiness?.readinessAssertions ?? {}).every(Boolean),
      `${scenarioId} readiness failed`
    );
  }

  assert(capture.captureMetadata?.captured === true, `${scenarioId} capture metadata is not captured`);
  assert(capture.captureMetadata?.status === 'available', `${scenarioId} capture status is not available`);
  assert(capture.captureMetadata?.rawImagePersistedInProof === false, `${scenarioId} persisted raw image in proof`);
  assert(capture.rawImagePathNotRetained === true, `${scenarioId} raw image path retained`);
  assert(ai.screenResult?.providerKind === 'localVision', `${scenarioId} did not use localVision`);
  assert(ai.screenResult?.imageDeletionState === 'deleted', `${scenarioId} screen result image not deleted`);
  assert(ai.screenResult?.rawImageRetained === false, `${scenarioId} screen result retained raw image`);
  assert(ai.localAiSafetyResult?.modelRuntime?.privacyMode === 'local-only', `${scenarioId} AI runtime not local-only`);
  assert(policy.policyDecision?.dryRun === true, `${scenarioId} policy decision is not dry-run`);
  assert(
    policy.policyDecision?.localAiResultId === ai.localAiSafetyResult?.resultId,
    `${scenarioId} policy lost AI result ref`
  );
  assert(deletion.rawImageDeletedAfterAnalysis === true, `${scenarioId} raw image not deleted`);
  assert(deletion.existsAfterDelete === false, `${scenarioId} raw temp still exists after delete`);
  assert(existsPath(repoPath(explanationPath)), `${scenarioId} parent explanation screenshot missing`);

  return {
    scenarioId,
    realTrigger: true,
    category: ai.screenResult.primaryCategory,
    policyAction: policy.policyDecision.action,
    localAiAnalyzed: true,
    policyDryRun: true,
    rawImageDeleted: true,
    parentExplanationSnapshotExists: true,
  };
}

function validateProtectedScenario(summaryRow) {
  const source = load(
    'output/ai-plan-proof/live-operator/protected-unsupported-state/01-redacted-source-evidence.json'
  );
  assert(summaryRow.status === 'passed', 'protected scenario did not pass');
  assert(summaryRow.policyDecisionValidated === false, 'protected scenario claimed policy validation');
  assert(
    source.protectedOrUnsupportedState === 'protectedSurface',
    'protected scenario did not preserve protected state'
  );
  assert(source.liveExternalUrl === false, 'protected scenario claimed live external URL');
  return {
    scenarioId: 'protected-unsupported-state',
    realTrigger: true,
    category: null,
    policyAction: null,
    localAiAnalyzed: false,
    policyDryRun: false,
    rawImageDeleted: false,
    parentExplanationSnapshotExists: false,
  };
}

function validateActionDispatch() {
  assert(actionDispatch.policyDecisionLinkedToAdapter === true, 'time-limit action did not link policy to adapter');
  assert(actionDispatch.realWindowsAdapterProof === true, 'time-limit adapter proof is not real Windows proof');
  assert(actionDispatch.rawImageDeletedBeforeDispatch === true, 'time-limit dispatch occurred before raw deletion');
  assert(actionDispatch.adapterResultCode === 'process-terminated', 'time-limit adapter did not terminate process');
  assert(blockActionDispatch.policyDecisionLinkedToAdapter === true, 'block action did not link policy to adapter');
  assert(blockActionDispatch.realWindowsBlockAdapterProof === true, 'block adapter proof is not real Windows proof');
  assert(blockActionDispatch.adapterStatus === 'actually-enforced', 'block adapter was not actually enforced');
  assert(blockActionDispatch.rawImageDeletedBeforeDispatch === true, 'block dispatch occurred before raw deletion');
  return true;
}

function validatePortalChain() {
  assert(portalChain.status === 'ok', 'portal chain status is not ok');
  const rendered = new Set(portalChain.renderedAssertions ?? []);
  for (const expected of ['AI provider localVision', 'Policy eligible Yes', 'Raw image deleted']) {
    assert(rendered.has(expected), `portal chain missing rendered assertion ${expected}`);
  }
  assert(existsPath(portalChain.artifact?.screenshot), 'portal chain screenshot missing');
  return true;
}

function validateReadModel() {
  assert(readModel.status === 'ok', 'parent explanation read-model proof status is not ok');
  assert(readModel.summary?.rowCount > 0, 'parent explanation read-model has no rows');
  assert(readModel.summary?.rawImageShown === false, 'read-model shows raw image');
  assert(readModel.summary?.rawImageRetained === false, 'read-model retains raw image');
  assert(readModel.summary?.remoteAiUsed === false, 'read-model used remote AI');
  assert(readModel.summary?.portalRuntimeClaimed === false, 'read-model claims portal runtime');
  return readModel.summary.rowCount;
}

function validateServiceReadModel() {
  assert(serviceReadModel.status === 'ok', 'service read-model proof status is not ok');
  assert(
    serviceReadModel.proofKind === 'screen-summary-parent-explanation-service-read-model',
    'service read-model proof kind mismatch'
  );
  assert(
    serviceReadModel.closure?.serviceBackedWebSocketReadModel === true,
    'service read-model is not service-backed WebSocket proof'
  );
  assert(
    serviceReadModel.closure?.queryStoreIngestPreservedExplanationRefs === true,
    'service read-model lost query-store explanation refs'
  );
  assert(
    serviceReadModel.closure?.rawScreenshotsRetainedByDefault === false,
    'service read-model retains raw screenshots'
  );
  assert(serviceReadModel.closure?.remoteAiUsedForChildSafety === false, 'service read-model used remote AI');
  assert(serviceReadModel.closure?.portalUiRenderingClaimed === false, 'service read-model claims portal UI');
  assert(serviceReadModel.serviceEvent?.activityReadModelKind === 'screen', 'service event is not screen read model');
  assert(serviceReadModel.serviceEvent?.activitySurfaceState === 'ready', 'service event is not ready');
  assert(serviceReadModel.row?.imageDeletionState === 'deleted', 'service row image is not deleted');
  assert(
    serviceReadModel.row?.custodyState === 'child-device-journal',
    'service row custody is not child-device journal'
  );
  assert((serviceReadModel.row?.parentExplanationRefs ?? []).length > 0, 'service row has no parent explanation refs');
  assert(
    (serviceReadModel.row?.deletionReasons ?? []).includes('screen-image-deleted'),
    'service row lacks deleted-image reason'
  );
  return true;
}

function validateServiceEventChain() {
  assert(
    serviceEventSubscription.proofMode === 'screen-service-event-subscription',
    'service event subscription proof mode mismatch'
  );
  assert(
    serviceEventSubscription.claimsProved?.some((claim) =>
      claim.includes('retains the screen event subscription runtime')
    ),
    'service startup subscription retention claim missing'
  );
  assert(
    serviceEventSubscription.claimsProved?.some((claim) =>
      claim.includes('row-ready subscriber and dispatches through the real event bus')
    ),
    'service row-ready real-event-bus subscription claim missing'
  );
  assert(
    serviceCaptureEventProducer.proofMode === 'screen-service-capture-event-producer',
    'service capture event producer proof mode mismatch'
  );
  assert(
    serviceCaptureEventProducer.claimsProved?.some((claim) =>
      claim.includes('service cadence runtime publishes capture/queue events')
    ),
    'service cadence capture event producer claim missing'
  );
  assert(
    serviceCaptureEventProducer.claimsProved?.some((claim) =>
      claim.includes('service foreground runtime publishes capture/queue events')
    ),
    'service foreground capture event producer claim missing'
  );
  assert(
    serviceAnalysisRowReady.proofMode === 'screen-service-analysis-row-ready',
    'service analysis row-ready proof mode mismatch'
  );
  assert(
    serviceAnalysisRowReady.claimsProved?.some((claim) => claim.includes('publishes screen.service.row.ready')),
    'service analysis row-ready publication claim missing'
  );
  assert(
    servicePolicyRefProducer.proofMode === 'screen-service-policy-ref-producer',
    'service policy-ref producer proof mode mismatch'
  );
  assert(
    servicePolicyRefProducer.claimsProved?.some((claim) =>
      claim.includes('carry policy decision, action, reason, parent rule, explanation, and deletion proof refs')
    ),
    'service policy-ref producer refs claim missing'
  );
  assert(
    serviceDeletionEventProducer.proofMode === 'screen-service-deletion-event-producer',
    'service deletion event producer proof mode mismatch'
  );
  assert(
    serviceDeletionEventProducer.claimsProved?.some((claim) =>
      claim.includes('retention sweeper runtime publishes deletion events')
    ),
    'service deletion event producer claim missing'
  );
  assert(serviceEventBridge.proofMode === 'screen-service-event-bridge', 'service event bridge proof mode mismatch');
  assert(
    serviceEventBridge.claimsProved?.some((claim) =>
      claim.includes('rows publish the ordered typed screen event chain')
    ),
    'service event bridge ordered chain claim missing'
  );
  assert(
    serviceEventBridge.claimsProved?.some((claim) =>
      claim.includes(
        'degraded AI rows publish capture, queue, AI, deletion, and portal events without policy or action refs'
      )
    ),
    'service event bridge degraded chain claim missing'
  );
  for (const expectedEvent of [
    'screen.capture.observed',
    'screen.queue.encrypted',
    'screen.ai.analysis.requested',
    'screen.ai.analysis.completed',
    'screen.summary.committed',
    'screen.policy.decision.completed',
    'screen.action.dry-run.recorded',
    'screen.deletion.committed',
    'screen.portal-read-model.updated',
  ]) {
    assert(
      serviceEventSubscription.eventChain?.includes(expectedEvent),
      `service subscription missing ${expectedEvent}`
    );
    assert(serviceEventBridge.eventChain?.includes(expectedEvent), `service bridge missing ${expectedEvent}`);
  }
  return true;
}

function validateDeletionCustody() {
  assert(
    Object.values(deletionRetentionCustody.assertions ?? {}).every(Boolean),
    'deletion-retention-custody assertion failed'
  );
  assert(
    retentionSweeper.assertions?.retentionSweeperRemovedExpiredQueueRecord === true,
    'retention sweeper did not remove queue'
  );
  assert(
    retentionSweeper.assertions?.expiredDeletionSurfacedInActivityReadModel === true,
    'expired deletion not in read model'
  );
  assert(retentionSweeper.ephemeralPathsDeletedAfterProof === true, 'retention sweeper left ephemeral paths');
  return true;
}

function validateProtectedSurface() {
  assert(protectedSurface.status === 'ok' || protectedSurface.proof, 'protected surface proof missing');
  return true;
}

function validateFinalAdapterAudit() {
  assert(
    finalAdapterAudit.status === 'blocked-by-upstream-adapter-artifacts',
    'final adapter audit status is not blocked'
  );
  assert(
    finalAdapterAudit.closure?.windowsOwnedProcessAdaptersProved === true,
    'final adapter audit lost Windows owned-process proof'
  );
  assert(
    finalAdapterAudit.closure?.broadBrowserNetworkMobileProductComplete === false,
    'final adapter audit unexpectedly claims product-complete adapters'
  );
  assert(
    finalAdapterAudit.closure?.openChecklistRowRetained === true,
    'final adapter audit did not retain open checklist row'
  );
  assert(
    finalAdapterAudit.closure?.custodyArtifactRows === 3,
    'final adapter audit did not consume three custody artifacts'
  );
  assert(finalAdapterAudit.closure?.claimUpgradeRows === 0, 'final adapter audit contains claim upgrades');
  assert((finalAdapterAudit.blockedRows ?? []).length === 6, 'final adapter audit blocked row count changed');
  assert((finalAdapterAudit.custodyRows ?? []).length === 3, 'final adapter audit custody row count changed');
  assert(
    finalAdapterAudit.custodyRows?.every((row) => row.finalAdapterCompletionClaimed === false) === true,
    'final adapter audit custody row claims final completion'
  );
  assert(
    finalAdapterAudit.custodyRows?.every((row) => row.productCompleteAdapterRowStillOpen === true) === true,
    'final adapter audit custody row closes product-complete row'
  );
  return true;
}

function validateHouseholdMeshBoundary() {
  assert(
    householdMeshScreenAi.proofMode === 'household-mesh-screen-ai',
    'household mesh screen AI proof mode mismatch'
  );
  for (const expectedEvent of [
    'screen.mesh.work.queued',
    'screen.mesh.claim.granted',
    'screen.mesh.lease.created',
    'screen.mesh.provider-result.returned',
    'screen.mesh.child-result.accepted',
    'screen.mesh.policy.requested',
  ]) {
    assert(householdMeshScreenAi.eventChain?.includes(expectedEvent), `household mesh missing ${expectedEvent}`);
  }
  assert(
    (householdMeshScreenAi.claimsProved ?? []).some((claim) => claim.includes('provider claim and lease phases')),
    'household mesh proof does not claim provider claim/lease phases'
  );
  assert(
    (householdMeshScreenAi.claimsProved ?? []).some((claim) =>
      claim.includes('validates provider result before policy')
    ),
    'household mesh proof lacks child-agent validation before policy'
  );
  assert(
    (householdMeshScreenAi.claimsProved ?? []).some((claim) =>
      claim.includes('cannot publish policy or enforcement events')
    ),
    'household mesh proof allows provider-authored policy/enforcement'
  );
  assert(
    (householdMeshScreenAi.claimsProved ?? []).some((claim) => claim.includes('not raw screenshot transfer')),
    'household mesh proof lacks no-raw-screenshot-transfer claim'
  );
  assert(
    noRawScreenTransferMesh.proofMode === 'no-raw-screen-transfer-mesh',
    'no-raw screen transfer proof mode mismatch'
  );
  assert(
    (noRawScreenTransferMesh.claimsProved ?? []).some((claim) => claim.includes('not raw screenshot transfer')),
    'no-raw screen transfer proof lacks no-raw-transfer claim'
  );
  assert(
    noRawScreenTransferMesh.claimsNotProved?.includes(
      'production mesh bridge transport over authenticated LAN messages'
    ),
    'no-raw screen transfer proof lost production transport non-claim'
  );
  assert(
    householdProviderResultValidation.proofMode === 'household-ai-provider-result-validation',
    'provider result validation proof mode mismatch'
  );
  for (const rejectionCase of [
    'duplicate-result',
    'expired-lease',
    'wrong-provider',
    'wrong-claim',
    'evidence-mismatch',
    'custody-mismatch',
    'raw-image-transfer',
    'provider-authority-violation',
  ]) {
    assert(
      householdProviderResultValidation.rejectionCases?.includes(rejectionCase),
      `provider result validation missing ${rejectionCase}`
    );
  }
  assert(
    (householdProviderResultValidation.claimsProved ?? []).some((claim) =>
      claim.includes('validates provider result before policy')
    ),
    'provider result validation proof lacks child-agent validation claim'
  );
  return true;
}

function validateScreenPlanClosure() {
  assert(screenPlanClosure.proof === 'screen-plan-closure-audit', 'screen-plan closure proof id mismatch');
  assert(screenPlanClosure.checklist?.openCount === 0, 'screen-plan closure still has open table rows');
  assert(
    (screenPlanClosure.checklist?.partialCount ?? 0) > 0,
    'screen-plan closure lost external partial-gate tracking'
  );
  assert(
    screenPlanClosure.assertions?.readinessProofsPresent === true,
    'screen-plan closure readiness proofs are not present'
  );
  assert(
    screenPlanClosure.assertions?.adapterAuditKeepsProductCompletionBlocked === true,
    'screen-plan closure no longer keeps adapter completion blocked'
  );
  assert(
    screenPlanClosure.assertions?.custodyArtifactsDoNotUpgradeClaims === true,
    'screen-plan closure custody artifacts upgrade claims'
  );
  assert(screenPlanClosure.assertions?.noProductCompleteClaim === true, 'screen-plan closure claims product complete');
  assert(
    (screenPlanClosure.remainingProductGates ?? []).length > 0,
    'screen-plan closure lost remaining product gates'
  );
  return true;
}

function validateAiPlanClosure() {
  assert(aiPlanClosure.proof === 'local-ai-plan-closure-audit-proof', 'AI-plan closure proof id mismatch');
  assert(aiPlanClosure.checklist?.openCount === 0, 'AI-plan closure still has open table rows');
  assert(
    aiPlanClosure.closure?.controlledCapturedScreensAnalyzed === true,
    'AI-plan closure lost controlled captured-screen analysis'
  );
  assert(
    aiPlanClosure.closure?.liveOperatorArtifactsAnalyzed === true,
    'AI-plan closure lost live operator analysis coverage'
  );
  assert(
    aiPlanClosure.closure?.serviceOcrAnalyzedCapturedPixels === true,
    'AI-plan closure lost service OCR captured-pixel proof'
  );
  assert(
    aiPlanClosure.closure?.storedEvidenceCanReachLocalAiInput === true,
    'AI-plan closure lost stored-evidence local AI input proof'
  );
  assert(
    aiPlanClosure.closure?.providerRuntimeAndSchedulerCovered === true,
    'AI-plan closure lost provider runtime or scheduler coverage'
  );
  assert(
    aiPlanClosure.closure?.policyOnlyConsumptionCovered === true,
    'AI-plan closure lost policy-only consumption coverage'
  );
  assert(aiPlanClosure.closure?.remoteApiAiClaimed === false, 'AI-plan closure claims remote/API AI');
  assert(aiPlanClosure.closure?.rawPromptRetained === false, 'AI-plan closure retains raw prompt');
  assert(aiPlanClosure.closure?.rawImageRetainedByDefault === false, 'AI-plan closure retains raw image by default');
  assert(aiPlanClosure.closure?.modelQualityClaimed === false, 'AI-plan closure claims model quality');
  assert(aiPlanClosure.closure?.enforcementClaimedByAiPlan === false, 'AI-plan closure claims enforcement');
  assert(
    aiPlanClosure.closure?.finalProductCompleteDeferredToPipeline === true,
    'AI-plan closure no longer defers final product-complete to pipeline'
  );
  return true;
}

function load(path) {
  return readJson(path, assert);
}

function assert(condition, message) {
  if (!condition) {
    failures.push(message);
  }
}
