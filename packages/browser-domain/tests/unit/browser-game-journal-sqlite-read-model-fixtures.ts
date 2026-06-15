export function browserGameReadModelSnapshot() {
  return {
    schemaVersion: 'browser-game-journal-sqlite-read-model-contract',
    readModelId: 'browser-game-read-model-snapshot-main',
    familyId: 'family-main',
    childProfileId: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    generatedAt: '2026-06-03T12:55:00.000Z',
    sourceProofRefs: ['parent-evidence-browser-journal-replay-proof', 'parent-evidence-app-game-session-report-proof'],
    rows: [
      browserGameReadModelRow(),
      browserGameReadModelRow({
        rowId: 'browser-game-read-model-row-app-game-session',
        sourceKind: 'app-game-session-report',
        browserEvidenceReadModelRef: null,
        appGameSessionReportRef: 'parent-evidence-app-game-session-report',
        reasonCodes: ['app-game-session-read-model-present'],
      }),
      browserGameReadModelRow({
        rowId: 'browser-game-read-model-row-adapter-audit',
        sourceKind: 'adapter-plan-audit',
        browserEvidenceReadModelRef: null,
        adapterPlanAuditRef: 'parent-evidence-managed-browser-game-adapter-audit',
        reasonCodes: ['adapter-audit-ref-present'],
      }),
      manualRequiredRow(),
      unavailableRow(),
    ],
    claimBoundaries: claimBoundaries(),
  };
}

export function browserGameReadModelRow(overrides = {}) {
  return {
    rowId: 'browser-game-read-model-row-browser-evidence',
    sourceKind: 'managed-browser-evidence',
    rowState: 'partial-proof',
    journalState: 'journal-replayed',
    sqliteState: 'read-model-present',
    browserEvidenceReadModelRef: 'parent-evidence-browser-evidence-read-model',
    appGameSessionReportRef: null,
    adapterPlanAuditRef: null,
    policyCandidateRef: 'parent-evidence-browser-game-policy-candidate',
    journalEntryRefs: ['parent-evidence-browser-game-journal-entry'],
    sqliteRowRefs: ['parent-evidence-browser-game-sqlite-row'],
    proofRefs: ['parent-evidence-browser-journal-replay-proof'],
    eventCount: 1,
    rowCount: 1,
    reasonCodes: ['browser-journal-replay-proof-present', 'sqlite-read-model-proof-present'],
    rawUrlIncluded: false,
    rawPageBodyIncluded: false,
    rawGamePayloadIncluded: false,
    rawGameTitleIncluded: false,
    rawAccountOrPurchaseIncluded: false,
    childCookieSessionReused: false,
    cloudTitleCertaintyClaimed: false,
    browserMutationClaimed: false,
    renderedUiClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

export function manualRequiredRow() {
  return browserGameReadModelRow({
    rowId: 'browser-game-read-model-row-cloud-manual-required',
    sourceKind: 'manual-required',
    rowState: 'manual-required',
    journalState: 'manual-required',
    sqliteState: 'manual-required',
    browserEvidenceReadModelRef: null,
    policyCandidateRef: null,
    journalEntryRefs: [],
    sqliteRowRefs: [],
    proofRefs: ['parent-evidence-cloud-gaming-manual-gap'],
    eventCount: 0,
    rowCount: 0,
    reasonCodes: ['cloud-gaming-read-model-manual-required'],
  });
}

export function unavailableRow() {
  return browserGameReadModelRow({
    rowId: 'browser-game-read-model-row-native-unavailable',
    sourceKind: 'unavailable',
    rowState: 'unavailable',
    journalState: 'unavailable',
    sqliteState: 'unavailable',
    browserEvidenceReadModelRef: null,
    policyCandidateRef: null,
    journalEntryRefs: [],
    sqliteRowRefs: [],
    proofRefs: ['parent-evidence-native-game-unavailable'],
    eventCount: 0,
    rowCount: 0,
    reasonCodes: ['native-game-control-unavailable'],
  });
}

function claimBoundaries() {
  return {
    rawUrlStorage: 'not-claimed',
    rawPageBodyStorage: 'not-claimed',
    rawGamePayloadStorage: 'not-claimed',
    rawGameTitleStorage: 'not-claimed',
    rawAccountOrPurchaseStorage: 'not-claimed',
    childCookieSessionReuse: 'not-claimed',
    cloudTitleCertainty: 'not-claimed',
    browserMutation: 'not-claimed',
    renderedUi: 'not-claimed',
    finalPolicyDecision: 'not-claimed',
    enforcement: 'not-claimed',
  };
}
