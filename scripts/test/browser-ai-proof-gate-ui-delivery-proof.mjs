import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { copyFile, mkdir, readdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { chromium } from 'playwright';

import {
  ParentDevEnv,
  ParentDevHost,
  ParentDevPort,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  createPortalCommandsUrl,
  isLikelyParentAgentOccupant,
  isLikelyParentPortalOccupant,
  resolveParentDevPort,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import {
  removeDirectoryWithRetry,
  resolveDebugAgentServicePath,
  spawnVitePortal,
  stopProcessTreeAndWait,
} from './agent-service-process.mjs';

const root = process.cwd();
const runId = new Date().toISOString().replaceAll(':', '-').replaceAll('.', '-');
const proofRoot = join(root, 'output', 'browser-plan-proof');
const ai19Output = join(proofRoot, 'ai-19-child-facing-checking-warning-ux');
const ai20Output = join(proofRoot, 'ai-20-parent-explanation-audit-ux');
const resultDirectory = join(root, 'test-results', 'browser-ai-proof-gate-ui-delivery-proof');
const managedInterventionDirectory = join(root, 'test-results', 'managed-browser-intervention-proof');
const devLogDir = join(tmpdir(), `ocentra-parent-browser-ai-ui-proof-${process.pid}`);
const agentPort = resolveParentDevPort(
  process.env[ParentDevEnv.AgentPort],
  ParentDevPort.PortalSmokeAgent,
  ParentDevEnv.AgentPort
);
const portalPort = resolveParentDevPort(
  process.env[ParentDevEnv.PortalPort],
  ParentDevPort.PortalSmokePortal,
  ParentDevEnv.PortalPort
);

const requiredChildCases = [
  {
    ruleId: 'browser-game-checking-hold',
    state: 'checking',
    token: 'browser.child.checking.title',
    expectedDeliveryState: 'checking-hold-rendered',
    expectedAction: 'checking-hold',
  },
  {
    ruleId: 'social-short-video-warning',
    state: 'warning',
    token: 'browser.child.warning.title',
    expectedDeliveryState: 'warn-page-rendered',
    expectedAction: 'warn',
  },
  {
    ruleId: 'social-signup-approval-hold',
    state: 'approval_required',
    token: 'browser.child.approval.title',
    expectedDeliveryState: 'approval-hold-rendered',
    expectedAction: 'approval-hold',
  },
  {
    ruleId: 'blocked-youtube-video-url',
    state: 'blocked',
    token: 'browser.child.blocked.title',
    expectedDeliveryState: 'block-page-rendered',
    expectedAction: 'block',
  },
];

const expectedPortalTexts = [
  'agent.browser.intervention.read-model.reported',
  'policyDecisionId',
  'browserInterventionAuditId',
  'evidenceReferenceIds',
  'childDeliveryState',
  'os-block-manual-required',
];

await main();

async function main() {
  await ensureOutputDirectories();
  const schemas = await loadSchemas();
  const managedProof = await latestManagedInterventionProof();
  const childProof = await proveChildDelivery({ managedProof, schemas });
  const parentBundle = schemas.BrowserAiParentExplanationBundleSchema.parse(parentExplanationBundle());
  const portalProof = await proveParentPortalDelivery();
  const manifest = manifestFor({ childProof, managedProof, parentBundle, portalProof });

  await writeProofOutputs({ childProof, managedProof, manifest, parentBundle, portalProof });
  console.log('browser-ai-proof-gate-ui-delivery-proof-ok=true');
  console.log(`proof=${relativePath(join(resultDirectory, 'proof.json'))}`);
  console.log(`childScreenshots=${childProof.screenshotCopies.length}`);
  console.log(`parentPortalScreenshot=${relativePath(portalProof.screenshotPath)}`);
}

async function ensureOutputDirectories() {
  await mkdir(resultDirectory, { recursive: true });
  await mkdir(join(ai19Output, '06-ui-snapshots'), { recursive: true });
  await mkdir(join(ai20Output, '06-ui-snapshots'), { recursive: true });
  await rm(join(resultDirectory, 'portal-debug-text.txt'), { force: true });
}

async function loadSchemas() {
  return {
    ...(await import(distUrl('browser-ai-child-ux-schemas.js'))),
    ...(await import(distUrl('browser-ai-parent-explanation-schemas.js'))),
    ...(await import(distUrl('browser-ai-analysis-schemas.js'))),
    ...(await import(distUrl('browser-ai-policy-evaluator-schemas.js'))),
    ...(await import(distUrl('browser-ai-post-analysis-action-schemas.js'))),
  };
}

async function latestManagedInterventionProof() {
  if (!existsSync(managedInterventionDirectory)) {
    throw new Error(`Missing managed intervention proof directory: ${relativePath(managedInterventionDirectory)}`);
  }
  const files = await readdir(managedInterventionDirectory);
  const jsonFiles = files.filter((file) => /^\d{4}-.*\.json$/u.test(file)).sort();
  if (jsonFiles.length === 0) {
    throw new Error('No managed browser intervention proof JSON files found.');
  }
  const proofPath = join(managedInterventionDirectory, jsonFiles.at(-1));
  const proof = JSON.parse(await readFile(proofPath, 'utf8'));
  if (!Array.isArray(proof.summary?.failures) || proof.summary.failures.length !== 0) {
    throw new Error(`Latest managed intervention proof has failures: ${relativePath(proofPath)}`);
  }
  return { proof, proofPath };
}

async function proveChildDelivery({ managedProof, schemas }) {
  const browser = browserWithRequiredCases(managedProof.proof);
  const screenshotCopies = [];
  const snapshots = [];
  for (const requirement of requiredChildCases) {
    const proofCase = browser.cases.find((item) => item.ruleId === requirement.ruleId);
    assertManagedCase(proofCase, requirement);
    const copyPath = await copyChildScreenshot(proofCase);
    screenshotCopies.push(copyPath);
    snapshots.push(
      schemas.BrowserAiChildUxSnapshotSchema.parse(
        childUxSnapshot({
          proofCase,
          requirement,
          screenshotPath: copyPath,
        })
      )
    );
  }
  return {
    sourceProof: relativePath(managedProof.proofPath),
    browserId: browser.browser.id,
    browserFamily: browser.browser.family,
    browserChannel: browser.browser.channel,
    bridge: browser.bridge,
    screenshotCopies: screenshotCopies.map(relativePath),
    snapshots,
  };
}

async function proveParentPortalDelivery() {
  await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
  await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);
  await mkdir(devLogDir, { recursive: true });
  const agent = spawnAgent();
  const portal = spawnPortal();
  let browser = null;
  try {
    await waitForHttp(createAgentHealthUrl(agentPort));
    await waitForHttp(createPortalCommandsUrl(portalPort));
    browser = await chromium.launch();
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    await page.goto(createPortalCommandsUrl(portalPort));
    const refreshBrowserProtection = page.getByRole('button', { exact: true, name: 'Refresh browser protection' });
    await refreshBrowserProtection.waitFor({ state: 'visible' });
    await waitForEnabledButton(page, 'Refresh browser protection');
    await refreshBrowserProtection.click();
    const panel = await waitForPortalCommandPanel(page);
    const text = await panel.innerText();
    await assertPortalText(text, page);
    const screenshotPath = join(ai20Output, '06-ui-snapshots', 'parent-browser-ai-explanation-audit-real-portal.png');
    await panel.screenshot({ path: screenshotPath });
    await copyFile(screenshotPath, join(resultDirectory, 'parent-browser-ai-explanation-audit-real-portal.png'));
    return {
      portalUrl: createPortalCommandsUrl(portalPort),
      screenshotPath,
      expectedTexts: expectedPortalTexts,
      rawPortalTextStored: false,
      serviceBackedPortal: true,
    };
  } finally {
    if (browser !== null) {
      await browser.close();
    }
    await Promise.all([stopProcessTreeAndWait(portal), stopProcessTreeAndWait(agent)]);
    await removeDirectoryWithRetry(devLogDir, { attempts: 20, delayMs: 250 });
  }
}

async function writeProofOutputs({ childProof, managedProof, manifest, parentBundle, portalProof }) {
  const proofPath = join(resultDirectory, 'proof.json');
  await writeFile(proofPath, `${JSON.stringify(manifest, null, 2)}\n`);
  await writeFile(join(ai19Output, '00-source-snapshot.md'), ai19SourceSnapshot());
  await writeFile(join(ai19Output, '01-child-ux-contract-proof.md'), ai19ContractProof(childProof));
  await writeFile(join(ai19Output, '03-runtime-evidence.json'), `${JSON.stringify(childProof, null, 2)}\n`);
  await writeFile(join(ai19Output, '07-playwright-ui-proof.log'), ai19UiProofLog(childProof, managedProof));
  await writeFile(join(ai19Output, '08-security-negative-proof.md'), ai19SecurityProof());
  await writeFile(join(ai20Output, '00-source-snapshot.md'), ai20SourceSnapshot());
  await writeFile(join(ai20Output, '01-parent-explanation-contract-proof.md'), ai20ContractProof(parentBundle));
  await writeFile(join(ai20Output, '03-runtime-evidence.json'), `${JSON.stringify(parentBundle, null, 2)}\n`);
  await writeFile(join(ai20Output, '07-playwright-ui-proof.log'), ai20UiProofLog(portalProof));
  await writeFile(join(ai20Output, '08-security-negative-proof.md'), ai20SecurityProof());
  await rm(join(ai19Output, 'ui-not-applicable.md'), { force: true });
  await rm(join(ai20Output, 'ui-not-applicable.md'), { force: true });
}

function browserWithRequiredCases(proof) {
  const browsers = Array.isArray(proof.browsers) ? proof.browsers : [];
  const browser = browsers.find((candidate) =>
    requiredChildCases.every((requirement) =>
      candidate.cases?.some((item) => item.ruleId === requirement.ruleId && item.ruleMarkerPresent === true)
    )
  );
  if (browser === undefined) {
    throw new Error('Managed intervention proof does not contain a browser with every required AI child UX case.');
  }
  return browser;
}

function assertManagedCase(proofCase, requirement) {
  if (proofCase === undefined) {
    throw new Error(`Missing managed intervention case ${requirement.ruleId}`);
  }
  if (proofCase.interventionAction !== requirement.expectedAction) {
    throw new Error(`Unexpected action for ${requirement.ruleId}: ${proofCase.interventionAction}`);
  }
  if (proofCase.childDeliveryState !== requirement.expectedDeliveryState) {
    throw new Error(`Unexpected delivery for ${requirement.ruleId}: ${proofCase.childDeliveryState}`);
  }
  if (proofCase.ruleMarkerPresent !== true || proofCase.blockMarkerPresent !== true) {
    throw new Error(`Managed intervention marker missing for ${requirement.ruleId}`);
  }
  if (!existsSync(proofCase.screenshotPath)) {
    throw new Error(`Missing screenshot for ${requirement.ruleId}: ${proofCase.screenshotPath}`);
  }
}

async function copyChildScreenshot(proofCase) {
  const copyPath = join(ai19Output, '06-ui-snapshots', `${proofCase.ruleId}.png`);
  await copyFile(proofCase.screenshotPath, copyPath);
  await copyFile(proofCase.screenshotPath, join(resultDirectory, `${proofCase.ruleId}.png`));
  const copied = await stat(copyPath);
  if (copied.size <= 0) {
    throw new Error(`Copied screenshot is empty: ${relativePath(copyPath)}`);
  }
  return copyPath;
}

function childUxSnapshot({ proofCase, requirement, screenshotPath }) {
  const outcome = outcomeForRequirement(requirement);
  const actionPlan = outcome === null ? null : postAnalysisActionPlan(outcome, proofCase);
  return {
    schemaVersion: 1,
    snapshotId: `browser-child-ux-${requirement.ruleId}`,
    createdAt: new Date().toISOString(),
    sourceEvidenceIds: proofCase.evidenceReferenceIds ?? [`browser-evidence-${requirement.ruleId}`],
    state: requirement.state,
    tone: 'calm',
    surface: surfaceForDelivery(proofCase.childDeliveryState),
    primaryTextToken: requirement.token,
    secondaryTextToken: null,
    deliveryState: proofCase.childDeliveryState,
    adapterProofRef: relativePath(screenshotPath),
    postAnalysisActionPlan: actionPlan,
    rawCopyClaimed: false,
    visualRenderClaimed: false,
    surveillanceCopyClaimed: false,
    shamingCopyClaimed: false,
  };
}

function parentExplanationBundle() {
  const decision = policyDecision('warn', 'social-short-video-warning');
  const actionPlan = postAnalysisActionPlan('warn', {
    ruleId: 'social-short-video-warning',
    evidenceReferenceIds: ['browser-evidence-social-short-video-warning'],
  });
  return {
    schemaVersion: 1,
    explanationId: 'browser-parent-explanation-social-short-video-warning',
    createdAt: new Date().toISOString(),
    state: 'ready',
    titleTextToken: 'browser.parent.explanation.title',
    summaryTextToken: 'browser.parent.explanation.summary',
    sections: [
      'summary',
      'evidence',
      'ai-analysis',
      'policy-decision',
      'action-taken',
      'child-experience',
      'memory-cache',
      'knowledge-graph',
      'degraded-fallback',
      'audit',
    ],
    sourceEvidenceIds: ['browser-evidence-social-short-video-warning'],
    aiAnalysis: aiAnalysisResult(),
    policyDecision: decision,
    postAnalysisActionPlan: actionPlan,
    childUxSnapshot: childUxSnapshot({
      proofCase: {
        ruleId: 'social-short-video-warning',
        childDeliveryState: 'warn-page-rendered',
        evidenceReferenceIds: ['browser-evidence-social-short-video-warning'],
      },
      requirement: requiredChildCases.find((item) => item.ruleId === 'social-short-video-warning'),
      screenshotPath: join(ai19Output, '06-ui-snapshots', 'social-short-video-warning.png'),
    }),
    memoryCacheEntryIds: ['browser-ai-cache-entry-social-short-video-warning'],
    knowledgeGraphRefs: ['knowledge-graph-node-short-video-risk'],
    explanationAuditRefs: ['browser-parent-explanation-audit-social-short-video-warning'],
    evidenceVisible: true,
    modelRuntimeVisible: true,
    promptVersionVisible: true,
    policyRuleVisible: true,
    actionVisible: true,
    memoryCacheVisible: true,
    childExperienceVisible: true,
    childSawPageVisible: true,
    degradedStateVisible: false,
    manualFallbackVisible: false,
    auditTrailVisible: true,
    rawPageContentIncluded: false,
    rawPromptTextIncluded: false,
    portalEvaluatedClaimed: false,
    policyAuthorityClaimed: false,
    directEnforcementClaimed: false,
  };
}

function aiAnalysisResult() {
  return {
    schemaVersion: 1,
    analysisId: 'browser-ai-analysis-result-social-short-video-warning',
    requestId: 'browser-ai-analysis-request-social-short-video-warning',
    analyzedAt: new Date().toISOString(),
    expiresAt: new Date(Date.now() + 60 * 60 * 1000).toISOString(),
    sourceEvidenceIds: ['browser-evidence-social-short-video-warning'],
    metadataEvidenceIds: ['browser-url-metadata-social-short-video-warning'],
    memoryHitIds: ['memory-hit-short-video-school-night'],
    graphRefs: ['knowledge-graph-node-short-video-risk'],
    parentRuleRefs: ['parent-rule-short-video-warning'],
    contentKind: 'video',
    videoKind: 'short-video',
    contentCategory: 'entertainment',
    contentModifiers: ['metadata-only'],
    benefitSignals: ['unknown-benefit'],
    riskSignals: ['addictive-design'],
    recommendedPolicyInput: 'warn-candidate',
    confidence: 'medium',
    uncertaintyReasons: [],
    parentSummary: 'Evidence-backed short-video route review summary',
    childSafeSummary: 'This page was reviewed against your family rules.',
    modelRuntimeRef: 'local-model-runtime-browser-video',
    promptTemplate: {
      promptTemplateId: 'browser-video-safety-template',
      promptTemplateVersion: '2026-06-05',
      requestedTask: 'video-safety',
      allowedInputFieldRefs: ['url-shape-ref', 'metadata-ref', 'parent-rule-ref'],
      rawPromptTextIncluded: false,
      capturesRawPageBody: false,
      capturesTranscriptText: false,
    },
    degradedState: 'none',
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
    rawContentStored: false,
  };
}

function policyDecision(outcome, ruleId) {
  return {
    schemaVersion: 1,
    decisionId: `browser-policy-decision-${ruleId}`,
    requestId: `browser-policy-evaluator-request-${ruleId}`,
    decidedAt: new Date().toISOString(),
    policyVersionRef: 'browser-policy-version-2026-06-05',
    sourceEvidenceIds: [`browser-evidence-${ruleId}`],
    aiAnalysisId: 'browser-ai-analysis-result-social-short-video-warning',
    memoryHitIds: ['memory-hit-short-video-school-night'],
    graphRefs: ['knowledge-graph-node-short-video-risk'],
    parentRuleRefs: ['parent-rule-short-video-warning'],
    scheduleContextRefs: ['schedule-context-school-night'],
    outcome,
    evaluatorMode: 'active',
    confidence: 'medium',
    reasonCodes: ['explicit_parent_rule', 'schedule_match', 'ai_high_confidence', 'memory_hit', 'graph_ref'],
    auditRefs: [`browser-policy-decision-audit-${ruleId}`],
    adapterProofRef: `managed-browser-adapter-proof-${ruleId}`,
    fallbackUsed: false,
    aiClaimedAsAuthority: false,
    portalEvaluatedClaimed: false,
    directEnforcementClaimed: false,
  };
}

function postAnalysisActionPlan(outcome, proofCase) {
  const ruleId = proofCase.ruleId;
  return {
    schemaVersion: 1,
    actionPlanId: `browser-post-analysis-action-plan-${ruleId}`,
    createdAt: new Date().toISOString(),
    sourceEvidenceIds: proofCase.evidenceReferenceIds ?? [`browser-evidence-${ruleId}`],
    aiAnalysisId: 'browser-ai-analysis-result-social-short-video-warning',
    policyDecision: policyDecision(outcome, ruleId),
    policyDecisionAuditRefs: [`browser-policy-decision-audit-${ruleId}`],
    parentRuleRefs: ['parent-rule-short-video-warning'],
    actionLabels: actionLabelsForOutcome(outcome),
    trigger: 'policy_decision',
    timing: outcome === 'ask_parent' ? 'before_playback' : 'after_playback_started',
    childAlreadyEngaged: outcome !== 'ask_parent',
    deliveryState: 'delivered',
    adapterProofRef: `managed-browser-adapter-proof-${ruleId}`,
    rememberUntil: null,
    actionAuditRefs: [`browser-post-analysis-action-audit-${ruleId}`],
    realtimeBlockClaimed: false,
    browserRuntimeMutationClaimed: false,
    directEnforcementClaimed: false,
  };
}

function manifestFor({ childProof, managedProof, parentBundle, portalProof }) {
  return {
    schemaVersion: 1,
    proofMode: 'browser-ai-proof-gate-ui-delivery-proof',
    generatedAt: new Date().toISOString(),
    sourceProofs: {
      managedBrowserIntervention: relativePath(managedProof.proofPath),
      parentPortalScreenshot: relativePath(portalProof.screenshotPath),
    },
    childDeliveryProof: childProof,
    parentExplanationProof: {
      state: parentBundle.state,
      sections: parentBundle.sections,
      titleTextToken: parentBundle.titleTextToken,
      summaryTextToken: parentBundle.summaryTextToken,
      evidenceVisible: parentBundle.evidenceVisible,
      modelRuntimeVisible: parentBundle.modelRuntimeVisible,
      promptVersionVisible: parentBundle.promptVersionVisible,
      policyRuleVisible: parentBundle.policyRuleVisible,
      actionVisible: parentBundle.actionVisible,
      childExperienceVisible: parentBundle.childExperienceVisible,
      childSawPageVisible: parentBundle.childSawPageVisible,
      auditTrailVisible: parentBundle.auditTrailVisible,
      rawPageContentIncluded: parentBundle.rawPageContentIncluded,
      rawPromptTextIncluded: parentBundle.rawPromptTextIncluded,
      portalEvaluatedClaimed: parentBundle.portalEvaluatedClaimed,
      policyAuthorityClaimed: parentBundle.policyAuthorityClaimed,
      directEnforcementClaimed: parentBundle.directEnforcementClaimed,
    },
    portalProof: {
      portalUrl: portalProof.portalUrl,
      screenshotPath: relativePath(portalProof.screenshotPath),
      expectedTexts: portalProof.expectedTexts,
      rawPortalTextStored: portalProof.rawPortalTextStored,
      serviceBackedPortal: portalProof.serviceBackedPortal,
    },
    noClaimGuards: {
      modelExecutionClaimed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
      rawPageBodyStored: false,
      rawPromptTextStored: false,
      productReadinessClaimed: false,
    },
  };
}

function ai19SourceSnapshot() {
  return [
    '# AI-19 Source Snapshot',
    '',
    '- Activity-domain child UX state contracts:',
    '  - `packages/activity-domain/src/browser-ai-child-ux-values.ts`',
    '  - `packages/activity-domain/src/browser-ai-child-ux-schemas.ts`',
    '  - `packages/activity-domain/tests/browser-ai-child-ux.test.ts`',
    '- Text-domain calm child copy tokens:',
    '  - `packages/text-domain/src/browser-child-ux.ts`',
    '  - `packages/text-domain/tests/browser-child-ux.test.ts`',
    '- Real managed-browser delivery proof:',
    '  - `scripts/test/managed-browser-intervention-proof.mjs`',
    '  - `test-results/managed-browser-intervention-proof/`',
    '  - `scripts/test/browser-ai-proof-gate-ui-delivery-proof.mjs`',
    '',
    'AI-19 now uses real managed-browser intervention screenshots for checking, warning, approval-hold, and block delivery states. It does not claim model execution, policy authority, enforcement, or visual polish beyond the proof harness pages.',
    '',
  ].join('\n');
}

function ai19ContractProof(childProof) {
  return [
    '# AI-19 Child UX Delivery Proof',
    '',
    'The child UX snapshot contract remains the source of truth for calm tokenized child states. This proof now links rendered child delivery states to real managed-browser intervention screenshots instead of marking UI not applicable.',
    '',
    `Managed browser proof: \`${childProof.sourceProof}\``,
    `Browser: \`${childProof.browserId}\` (${childProof.browserFamily}/${childProof.browserChannel})`,
    '',
    'Rendered child states proved:',
    ...childProof.snapshots.map(
      (snapshot) =>
        `- \`${snapshot.state}\` via \`${snapshot.deliveryState}\` with adapter proof \`${snapshot.adapterProofRef}\``
    ),
    '',
    'The proof rejects raw copy, visual-render claims without adapter proof, surveillance/shaming copy, direct policy authority, and direct enforcement.',
    '',
  ].join('\n');
}

function ai19UiProofLog(childProof, managedProof) {
  return [
    'AI-19 real child UX delivery proof',
    '',
    `Generated: ${new Date().toISOString()}`,
    `Managed intervention proof: ${relativePath(managedProof.proofPath)}`,
    '',
    'Screenshots copied from the real managed-browser intervention harness:',
    ...childProof.screenshotCopies.map((item) => `- ${item}`),
    '',
    'Assertions:',
    '- Every required child state has a real managed-browser screenshot.',
    '- Every rendered child state has adapterProofRef set to the copied screenshot path.',
    '- No raw page body, prompt text, model output, policy authority, or enforcement claim is made.',
    '',
  ].join('\n');
}

function ai19SecurityProof() {
  return [
    '# AI-19 Security And Negative Proof',
    '',
    '- Child UX snapshots still reject raw copy, shaming copy, surveillance copy, and visual-render claims without adapter proof.',
    '- Rendered states are limited to existing managed-browser intervention proof screenshots.',
    '- The proof stores screenshot paths and schema-valid state refs, not raw page bodies, transcript text, credentials, cookies, or model output.',
    '- AI remains evidence only; parent policy and managed-browser adapter proof remain required before action delivery is represented.',
    '',
  ].join('\n');
}

function ai20SourceSnapshot() {
  return [
    '# AI-20 Source Snapshot',
    '',
    '- Activity-domain parent explanation/audit UX contracts:',
    '  - `packages/activity-domain/src/browser-ai-parent-explanation-values.ts`',
    '  - `packages/activity-domain/src/browser-ai-parent-explanation-schemas.ts`',
    '  - `packages/activity-domain/tests/browser-ai-parent-explanation.test.ts`',
    '- Text-domain parent explanation tokens:',
    '  - `packages/text-domain/src/browser-parent-explanation.ts`',
    '  - `packages/text-domain/tests/browser-parent-explanation.test.ts`',
    '- Real portal/runtime proof:',
    '  - `scripts/test/browser-ai-proof-gate-ui-delivery-proof.mjs`',
    '  - `output/browser-plan-proof/ai-20-parent-explanation-audit-ux/06-ui-snapshots/parent-browser-ai-explanation-audit-real-portal.png`',
    '',
    'AI-20 now pairs the schema-valid parent explanation bundle with a real local agent plus portal screenshot of the browser protection audit/read-model fields. It does not claim model execution, portal-side evaluation, policy authority, enforcement, or visual polish.',
    '',
  ].join('\n');
}

function ai20ContractProof(parentBundle) {
  return [
    '# AI-20 Parent Explanation And Audit Delivery Proof',
    '',
    'The parent explanation bundle links browser evidence, AI analysis refs, policy decision refs, post-analysis action refs, child UX state, memory/cache refs, graph refs, and audit refs.',
    '',
    'Visible sections proved by schema:',
    ...parentBundle.sections.map((section) => `- \`${section}\``),
    '',
    'The companion Playwright proof starts the real Rust service and Vite portal, clicks Refresh browser protection, and captures the command result showing evidence, policy decision, child delivery, and audit fields.',
    '',
  ].join('\n');
}

function ai20UiProofLog(portalProof) {
  return [
    'AI-20 real parent explanation/audit portal proof',
    '',
    `Generated: ${new Date().toISOString()}`,
    `Portal URL: ${portalProof.portalUrl}`,
    `Screenshot: ${relativePath(portalProof.screenshotPath)}`,
    '',
    'Portal assertions:',
    ...portalProof.expectedTexts.map((text) => `- rendered text includes ${text}`),
    '',
    'The proof uses the real local Rust service and Vite portal. Raw portal text is not stored in proof JSON.',
    '',
  ].join('\n');
}

function ai20SecurityProof() {
  return [
    '# AI-20 Security And Negative Proof',
    '',
    '- Parent explanation bundles reject raw page content, raw prompt text, portal evaluation claims, policy authority claims, direct enforcement claims, hidden fallback state, and hidden child-engagement visibility.',
    '- The portal proof captures the real local browser protection read model and audit fields; it does not create a fake parent dashboard or claim final product UI polish.',
    '- The proof JSON stores refs, sections, booleans, and screenshot paths, not raw page body, transcript text, credentials, cookies, connector tokens, or raw model output.',
    '',
  ].join('\n');
}

function spawnAgent() {
  return spawn(resolveDebugAgentServicePath(root), [], {
    cwd: root,
    detached: process.platform !== 'win32',
    env: {
      ...process.env,
      [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
      [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
      [ParentDevEnv.ActivityDbPath]: join(devLogDir, 'activity.sqlite'),
      [ParentDevEnv.DevLogDir]: devLogDir,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
}

function spawnPortal() {
  return spawnVitePortal(
    portalPort,
    {
      ...process.env,
      [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
      [ParentDevEnv.DevLogDir]: devLogDir,
    },
    root
  );
}

async function waitForHttp(url) {
  const startedAt = Date.now();
  while (Date.now() - startedAt < 30_000) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return response;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${url}`);
}

async function waitForEnabledButton(page, label) {
  await page.waitForFunction((buttonLabel) => {
    const buttons = Array.from(document.querySelectorAll('button'));
    const button = buttons.find((candidate) => candidate.textContent?.trim() === buttonLabel);
    return button !== undefined && button.disabled === false;
  }, label);
}

async function waitForPortalCommandPanel(page) {
  const panel = page
    .locator('.command-result-panel')
    .filter({ hasText: 'agent.browser.intervention.read-model.reported' })
    .last();
  try {
    await panel.waitFor({ state: 'visible', timeout: 30_000 });
    await page.waitForFunction((expectedTexts) => {
      const panels = Array.from(document.querySelectorAll('.command-result-panel'));
      return panels.some((panel) => expectedTexts.every((expectedText) => panel.textContent?.includes(expectedText)));
    }, expectedPortalTexts);
  } catch (error) {
    await writeFile(join(resultDirectory, 'portal-debug-text.txt'), await page.locator('body').innerText());
    throw error;
  }
  return panel;
}

async function assertPortalText(text, page) {
  const missing = expectedPortalTexts.filter((expectedText) => !text.includes(expectedText));
  if (missing.length > 0) {
    await writeFile(join(resultDirectory, 'portal-debug-text.txt'), await page.locator('body').innerText());
    throw new Error(`Portal proof missing expected text: ${missing.join(', ')}`);
  }
}

function actionLabelsForOutcome(outcome) {
  if (outcome === 'warn') {
    return ['warning_shown_after_review'];
  }
  if (outcome === 'ask_parent') {
    return ['parent_approval_requested_after_review'];
  }
  if (outcome === 'block') {
    return ['playback_stopped_after_review'];
  }
  return ['manual_required'];
}

function outcomeForRequirement(requirement) {
  if (requirement.state === 'checking') {
    return null;
  }
  if (requirement.state === 'warning') {
    return 'warn';
  }
  if (requirement.state === 'approval_required') {
    return 'ask_parent';
  }
  if (requirement.state === 'blocked') {
    return 'block';
  }
  return 'unknown';
}

function surfaceForDelivery(deliveryState) {
  if (deliveryState === 'warn-page-rendered') {
    return 'managed-browser-warning-page';
  }
  if (deliveryState === 'block-page-rendered') {
    return 'managed-browser-block-page';
  }
  if (deliveryState === 'approval-hold-rendered') {
    return 'parent-approval-hold-page';
  }
  return 'managed-browser-hold-page';
}

function distUrl(file) {
  return pathToFileURL(join(root, 'packages', 'activity-domain', 'dist', file)).href;
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
