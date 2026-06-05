import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright';
import { parseBrowserUrlShape } from '../../packages/activity-domain/dist/browser-url-intelligence.js';
import { buildBrowserSocialAccountFlowEvidenceFromRoute } from '../../packages/activity-domain/dist/browser-social-account-flow-schemas.js';
import { planBrowserSocialAccountCreationGate } from '../../packages/activity-domain/dist/browser-social-account-creation-gate.js';
import { BrowserSocialAccountCreationGatePlanSchema } from '../../packages/activity-domain/dist/browser-social-account-creation-gate.js';
import { detectBrowserSocialFormShape } from '../../packages/activity-domain/dist/browser-social-form-shape-detector.js';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '../../packages/activity-domain/dist/browser-social-url-patterns.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/social-13-managed-browser-account-creation-gate');
const screenshotRoot = join(proofRoot, '06-live-screenshots');
const testResultPath = join(repoRoot, 'test-results/social-account-creation-live-proof/proof.json');
const outputProofPath = join(proofRoot, '11-live-proof.json');
const observedAt = new Date().toISOString();

const sourceFiles = [
  'packages/activity-domain/src/browser-url-intelligence.ts',
  'packages/activity-domain/src/browser-social-url-patterns.ts',
  'packages/activity-domain/src/browser-social-account-flow-schemas.ts',
  'packages/activity-domain/src/browser-social-form-shape-detector.ts',
  'packages/activity-domain/src/browser-social-account-creation-gate.ts',
];
const builtFiles = [
  'packages/activity-domain/dist/browser-url-intelligence.js',
  'packages/activity-domain/dist/browser-social-url-patterns.js',
  'packages/activity-domain/dist/browser-social-account-flow-schemas.js',
  'packages/activity-domain/dist/browser-social-form-shape-detector.js',
  'packages/activity-domain/dist/browser-social-account-creation-gate.js',
];

const liveTargets = [
  {
    id: 'facebook-signup',
    url: 'https://www.facebook.com/r.php',
    gateAction: 'hold-for-parent-approval',
    parentApprovalRequired: true,
    policyDecisionCandidateRef: null,
    parentApprovalRequestRef: 'social-parent-approval-request-facebook-signup',
    reasons: ['signup-flow', 'form-shape-detected', 'parent-policy-requires-approval'],
  },
  {
    id: 'pinterest-login',
    url: 'https://www.pinterest.com/login/',
    gateAction: 'allow-navigation-candidate',
    parentApprovalRequired: false,
    policyDecisionCandidateRef: 'social-policy-decision-candidate-pinterest-login',
    parentApprovalRequestRef: null,
    reasons: ['login-flow', 'form-shape-detected'],
  },
  {
    id: 'reddit-register',
    url: 'https://www.reddit.com/register/',
    gateAction: 'block-submit-candidate',
    parentApprovalRequired: false,
    policyDecisionCandidateRef: 'social-policy-decision-candidate-reddit-register',
    parentApprovalRequestRef: null,
    reasons: ['signup-flow', 'form-shape-detected', 'policy-block-candidate'],
  },
  {
    id: 'instagram-signup',
    url: 'https://www.instagram.com/accounts/emailsignup/',
    gateAction: 'manual-review-required',
    parentApprovalRequired: false,
    policyDecisionCandidateRef: 'social-policy-decision-candidate-instagram-signup',
    parentApprovalRequestRef: null,
    reasons: ['signup-flow', 'form-shape-detected', 'manual-required'],
  },
];

assertBuiltContractsAreFresh();
mkdirSync(screenshotRoot, { recursive: true });

const browser = await chromium.launch({ headless: true });
const captures = [];
try {
  for (const target of liveTargets) {
    captures.push(await captureLiveTarget(browser, target));
  }
} finally {
  await browser.close();
}

const plannedCaptures = captures.filter((capture) => capture.contractPlanCreated);
if (plannedCaptures.length < 2) {
  throw new Error(`Expected at least 2 live social account gate plans, received ${plannedCaptures.length}`);
}

const planParseChecks = plannedCaptures.map((capture) => ({
  targetId: capture.targetId,
  accepted: BrowserSocialAccountCreationGatePlanSchema.safeParse(capture.planSummary).success,
}));
if (!planParseChecks.every((check) => check.accepted)) {
  throw new Error('Expected all SOCIAL-13 live gate plans to parse through the contract schema');
}

const negativeChecks = buildNegativeChecks(plannedCaptures);
if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected all SOCIAL-13 negative checks to reject dishonest runtime claims');
}

const proof = {
  schemaVersion: 1,
  proofId: 'social-account-creation-live-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  liveCaptureSummary: {
    realPublicSocialSurfacesUsed: true,
    generatedOrFixturePageUsed: false,
    passiveNavigationOnly: true,
    formValuesCaptured: false,
    credentialsCaptured: false,
    formsSubmitted: false,
    accountCreated: false,
    rawPageBodyPersisted: false,
    rawDomPersisted: false,
    screenshotsPersisted: true,
    plannedCaptureCount: plannedCaptures.length,
  },
  captures,
  planParseChecks,
  plannedGateSummaries: plannedCaptures.map((capture) => capture.planSummary),
  negativeChecks,
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('social-account-creation-live-proof-ok=true');
console.log(`proof=${testResultPath}`);
console.log(`outputProof=${outputProofPath}`);
console.log(`plannedCaptureCount=${plannedCaptures.length}`);
console.log(`plannedPlatforms=${plannedCaptures.map((capture) => capture.planSummary.platform).join(',')}`);

async function captureLiveTarget(browserInstance, target) {
  const page = await browserInstance.newPage({
    viewport: { width: 1280, height: 900 },
    userAgent:
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125 Safari/537.36',
  });
  let responseStatus = null;
  let captureError = null;
  try {
    const response = await page.goto(target.url, { waitUntil: 'domcontentloaded', timeout: 30_000 });
    responseStatus = response?.status() ?? null;
    await page.waitForLoadState('networkidle', { timeout: 8_000 }).catch(() => undefined);
  } catch (error) {
    captureError = error instanceof Error ? error.message : String(error);
  }

  const finalUrl = page.url();
  const proofFinalUrl = sanitizeUrlForProof(finalUrl);
  const title = await page.title().catch(() => '');
  const controls = await sanitizedControlKinds(page).catch(() => []);
  const screenshotPath = join(screenshotRoot, `${target.id}.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false }).catch(() => undefined);
  await page.close();

  const base = {
    targetId: target.id,
    requestedUrl: target.url,
    finalUrl: proofFinalUrl,
    finalUrlSha256: sha256(proofFinalUrl),
    finalUrlQueryOrHashRemoved: finalUrl !== proofFinalUrl,
    responseStatus,
    navigationError: captureError,
    titleLength: title.length,
    titleSha256: sha256(title),
    screenshotPath: relative(repoRoot, screenshotPath).replaceAll('\\', '/'),
    screenshotSha256: existsSync(screenshotPath) ? sha256File(screenshotPath) : null,
    screenshotBytes: existsSync(screenshotPath) ? statSync(screenshotPath).size : 0,
    sanitizedControlKinds: controls,
    rawPageBodyPersisted: false,
    rawDomPersisted: false,
    fieldValuesCaptured: false,
    credentialsCaptured: false,
    formSubmitted: false,
  };

  try {
    const planSummary = buildGatePlanFromLiveCapture(target, finalUrl, controls);
    return {
      ...base,
      contractPlanCreated: true,
      planSummary,
    };
  } catch (error) {
    return {
      ...base,
      contractPlanCreated: false,
      contractError: error instanceof Error ? error.message : String(error),
      planSummary: null,
    };
  }
}

async function sanitizedControlKinds(page) {
  const controls = await page
    .locator('input,textarea,select,button,a,[role="button"],[role="textbox"],[contenteditable="true"]')
    .evaluateAll((nodes) => {
      const controlKinds = new Set();
      for (const node of nodes) {
        const element = node;
        if (!(element instanceof HTMLElement)) {
          continue;
        }
        const rect = element.getBoundingClientRect();
        const style = window.getComputedStyle(element);
        if (rect.width === 0 || rect.height === 0 || style.visibility === 'hidden' || style.display === 'none') {
          continue;
        }
        const tag = element.tagName.toLowerCase();
        const type = (element.getAttribute('type') ?? '').toLowerCase();
        const autocomplete = (element.getAttribute('autocomplete') ?? '').toLowerCase();
        const role = (element.getAttribute('role') ?? '').toLowerCase();
        const explicitLabels =
          element.id.length > 0
            ? [...document.querySelectorAll('label')]
                .filter((label) => label.getAttribute('for') === element.id)
                .map((label) => label.textContent ?? '')
            : [];
        const data = [
          type,
          autocomplete,
          element.getAttribute('name') ?? '',
          element.getAttribute('id') ?? '',
          element.getAttribute('aria-label') ?? '',
          element.getAttribute('placeholder') ?? '',
          element.textContent ?? '',
          element.closest('label')?.textContent ?? '',
          element.parentElement?.textContent ?? '',
          element.parentElement?.previousElementSibling?.textContent ?? '',
          element.parentElement?.parentElement?.previousElementSibling?.textContent ?? '',
          ...explicitLabels,
        ]
          .join(' ')
          .toLowerCase();

        if (type === 'password' || data.includes('password')) {
          controlKinds.add('password-input');
        }
        if (type === 'email' || data.includes('email') || autocomplete.includes('email')) {
          controlKinds.add('email-input');
        }
        if (type === 'tel' || data.includes('phone') || data.includes('mobile')) {
          controlKinds.add('phone-input');
        }
        if (data.includes('username') || autocomplete.includes('username')) {
          controlKinds.add('username-input');
        }
        if (
          data.includes('full name') ||
          data.includes('display name') ||
          data.includes('first name') ||
          data.includes('last name') ||
          data.includes('firstname')
        ) {
          controlKinds.add('display-name-input');
        }
        if (data.includes('birth') || data.includes('birthday') || data.includes('birthdate')) {
          controlKinds.add('birthdate-input');
        }
        if (
          tag === 'button' ||
          type === 'submit' ||
          role === 'button' ||
          data.includes('sign up') ||
          data.includes('signup') ||
          data.includes('register') ||
          data.includes('log in') ||
          data.includes('login') ||
          data.includes('continue') ||
          data.includes('next') ||
          data.includes('join')
        ) {
          controlKinds.add('submit-button');
        }
        if (data.includes('switch account') || data.includes('use another account')) {
          controlKinds.add('account-switch-link');
        }
      }
      return [...controlKinds];
    });

  return controls.filter((control) => control !== 'unknown-control').sort();
}

function buildGatePlanFromLiveCapture(target, finalUrl, controlKinds) {
  const routeUrl = urlForContract(finalUrl, target.url);
  const classification = parseBrowserUrlShape({
    classificationId: `social-account-live-url-shape-${target.id}`,
    classifiedAt: observedAt,
    sourceEvidenceIds: [`social-account-live-browser-evidence-${target.id}`],
    sourceKind: 'managed-browser-exact-url',
    url: routeUrl,
    title: 'Live social account route evidence',
  });
  const routeEvidence = buildBrowserSocialRouteEvidenceFromUrlPattern({
    socialRouteEvidenceId: `social-account-live-route-${target.id}`,
    observedAt,
    sourceEvidenceIds: [`social-account-live-route-evidence-${target.id}`],
    classification,
  });
  const accountFlowEvidence = buildBrowserSocialAccountFlowEvidenceFromRoute({
    accountFlowEvidenceId: `social-account-live-flow-${target.id}`,
    observedAt,
    sourceEvidenceIds: [`social-account-live-flow-evidence-${target.id}`],
    routeEvidence,
  });
  const formShapeEvidence = detectBrowserSocialFormShape({
    formShapeEvidenceId: `social-account-live-form-shape-${target.id}`,
    observedAt,
    sourceEvidenceIds: [`social-account-live-form-evidence-${target.id}`],
    accountFlowEvidence,
    controls: controlKinds.map((controlKind) => ({ controlKind, valueCaptured: false })),
  });
  const plan = planBrowserSocialAccountCreationGate({
    gatePlanId: `social-account-live-gate-${target.id}`,
    plannedAt: observedAt,
    sourceEvidenceIds: [`social-account-live-gate-evidence-${target.id}`],
    accountFlowEvidence,
    formShapeEvidence,
    policyDecisionCandidateRef: target.policyDecisionCandidateRef,
    parentApprovalRequestRef: target.parentApprovalRequestRef,
    gateAction: target.gateAction,
    parentApprovalRequired: target.parentApprovalRequired,
    reasons: target.reasons,
  });

  return {
    schemaVersion: plan.schemaVersion,
    gatePlanId: plan.gatePlanId,
    plannedAt: plan.plannedAt,
    sourceEvidenceIds: plan.sourceEvidenceIds,
    accountFlowEvidenceId: plan.accountFlowEvidenceId,
    formShapeEvidenceId: plan.formShapeEvidenceId,
    socialRouteEvidenceId: plan.socialRouteEvidenceId,
    platform: plan.platform,
    accountFlowKind: plan.accountFlowKind,
    formShapeKind: plan.formShapeKind,
    gateState: plan.gateState,
    gateAction: plan.gateAction,
    parentApprovalRequired: plan.parentApprovalRequired,
    policyDecisionCandidateRef: plan.policyDecisionCandidateRef,
    parentApprovalRequestRef: plan.parentApprovalRequestRef,
    reasons: plan.reasons,
    navigationPausedClaimed: plan.navigationPausedClaimed,
    formSubmitBlockedClaimed: plan.formSubmitBlockedClaimed,
    childUiRenderedClaimed: plan.childUiRenderedClaimed,
    parentUiNotifiedClaimed: plan.parentUiNotifiedClaimed,
    policyDecisionClaimed: plan.policyDecisionClaimed,
    enforcementClaimed: plan.enforcementClaimed,
    nativeAppControlClaimed: plan.nativeAppControlClaimed,
    platformConnectorClaimed: plan.platformConnectorClaimed,
    credentialCaptured: plan.credentialCaptured,
    formSubmittedClaimed: plan.formSubmittedClaimed,
    accountCreatedClaimed: plan.accountCreatedClaimed,
  };
}

function urlForContract(finalUrl, requestedUrl) {
  const routeOnlyFinalUrl = sanitizeUrlForProof(finalUrl);
  if (isSupportedSocialAccountRoute(routeOnlyFinalUrl)) {
    return routeOnlyFinalUrl;
  }
  return requestedUrl;
}

function isSupportedSocialAccountRoute(url) {
  try {
    const parsed = new URL(url);
    const path = parsed.pathname.toLowerCase();
    return (
      /(^|\.)((facebook|pinterest|reddit|instagram|tiktok)\.com)$/.test(parsed.hostname.toLowerCase()) &&
      (path.includes('login') ||
        path.includes('signup') ||
        path.includes('register') ||
        path.includes('r.php') ||
        path.includes('emailsignup'))
    );
  } catch {
    return false;
  }
}

function buildNegativeChecks(plannedCaptures) {
  const valid = plannedCaptures[0]?.planSummary;
  if (valid === undefined) {
    return [];
  }
  if (!BrowserSocialAccountCreationGatePlanSchema.safeParse(valid).success) {
    throw new Error('Expected SOCIAL-13 negative checks to start from a valid parsed live plan');
  }
  const dishonestRows = [
    ['navigationPausedClaimed', { ...valid, navigationPausedClaimed: true }],
    ['formSubmitBlockedClaimed', { ...valid, formSubmitBlockedClaimed: true }],
    ['childUiRenderedClaimed', { ...valid, childUiRenderedClaimed: true }],
    ['parentUiNotifiedClaimed', { ...valid, parentUiNotifiedClaimed: true }],
    ['policyDecisionClaimed', { ...valid, policyDecisionClaimed: true }],
    ['enforcementClaimed', { ...valid, enforcementClaimed: true }],
    ['nativeAppControlClaimed', { ...valid, nativeAppControlClaimed: true }],
    ['platformConnectorClaimed', { ...valid, platformConnectorClaimed: true }],
    ['credentialCaptured', { ...valid, credentialCaptured: true }],
    ['formSubmittedClaimed', { ...valid, formSubmittedClaimed: true }],
    ['accountCreatedClaimed', { ...valid, accountCreatedClaimed: true }],
    ['manualReviewWithPlannedState', { ...valid, gateAction: 'manual-review-required', gateState: 'planned' }],
  ];
  return dishonestRows.map(([name, row]) => ({
    name,
    rejected: !BrowserSocialAccountCreationGatePlanSchema.safeParse(row).success,
  }));
}

function sanitizeUrlForProof(value) {
  try {
    const parsed = new URL(value);
    parsed.username = '';
    parsed.password = '';
    parsed.search = '';
    parsed.hash = '';
    return parsed.toString();
  } catch {
    return 'unparseable-url';
  }
}

function assertBuiltContractsAreFresh() {
  const newestSourceMtime = Math.max(...sourceFiles.map((file) => statSync(join(repoRoot, file)).mtimeMs));
  for (const builtFile of builtFiles) {
    const builtPath = join(repoRoot, builtFile);
    const builtMtime = statSync(builtPath).mtimeMs;
    if (builtMtime < newestSourceMtime) {
      throw new Error(`Build output is stale: ${builtFile}. Run cmd /c npm run build:contracts first.`);
    }
  }
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function sha256File(path) {
  return sha256(readFileSync(path));
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
