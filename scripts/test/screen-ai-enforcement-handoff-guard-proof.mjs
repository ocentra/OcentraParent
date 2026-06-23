import { strict as assert } from 'node:assert';
import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));
const outputDir = join(repoRoot, 'test-results', 'screen-ai-enforcement-handoff-guard-proof');
const outputPath = join(outputDir, 'proof.json');
const planOutputDir = join(repoRoot, 'output', 'screen-plan-proof', 'screen-ai-enforcement-handoff-guard');
const planOutputPath = join(planOutputDir, 'proof-summary.json');
const screenAiGuardOwnerPath = 'packages/screen-domain/src/screen-ai-enforcement-handoff-guard-proof.ts';

run('npm', ['run', 'build', '--workspace=@ocentra-parent/schema-domain']);
run('npm', ['run', 'build', '--workspace=@ocentra-parent/screen-domain']);
run('npm', [
  'run',
  'test',
  '--workspace=@ocentra-parent/screen-domain',
  '--',
  'screen-ai-enforcement-handoff-guard-proof.test.ts',
]);

const guard = await import('../../packages/screen-domain/dist/screen-ai-enforcement-handoff-guard-proof.js');
const payload = guard.buildScreenAiEnforcementHandoffGuardPayload(validInput(readScreenAiHandoffAcceptedEventType()));

assert.equal(payload.requestedAction, 'time-limit');
assert.equal(payload.handoffMode, 'dry-run');
assert.equal(payload.sourcePolicyDecision.dryRun, true);
assert.equal(payload.sourcePolicyDecision.enforcementHandoffState, 'disabled');
assert.equal(payload.summaryReference.kind, 'query-store-summary');
assert.equal(payload.localAiResultReference.kind, 'local-ai-result');
assert.equal(payload.auditReference.kind, 'journal-event');
assert.equal(payload.rawPixelsIncluded, false);
assert.equal(payload.rawModelTextIncluded, false);
assert.equal(payload.rawScreenshotRetained, false);
assert.equal(payload.localAiAuthorityClaimed, false);

const proof = {
  proofId: 'screen-ai-enforcement-handoff-guard-proof',
  generatedAt: '2026-06-06T22:02:00Z',
  source: '@ocentra-parent/screen-domain screen AI enforcement handoff guard builder with schema-domain contracts',
  assertions: [
    'screen enforcement handoff payloads require a dry-run policy decision that has not been handed off',
    'payloads carry parent policy rule, summary reference, local AI result reference, confidence state, and audit event',
    'summary, local AI, and audit refs must already be present in the policy decision evidence list',
    'raw pixels, raw model text, retained screenshots, and local AI authority claims are rejected',
    'the proof does not claim adapter execution; adapter readiness remains a separate downstream gate',
  ],
  parsed: {
    payloadId: payload.payloadId,
    requestedAction: payload.requestedAction,
    confidenceState: payload.confidenceState,
    handoffMode: payload.handoffMode,
    auditEventType: payload.auditEvent.eventType,
    rawMaterialIncluded: payload.rawPixelsIncluded || payload.rawModelTextIncluded || payload.rawScreenshotRetained,
  },
};

mkdirSync(outputDir, { recursive: true });
mkdirSync(planOutputDir, { recursive: true });
writeFileSync(outputPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(planOutputPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`screen-ai-enforcement-handoff-guard-proof-ok: ${outputPath}`);

function validInput(acceptedEventType) {
  const generatedAt = '2026-06-06T22:02:00.000Z';
  const summaryReference = {
    evidenceReferenceId: 'screen-summary-parent-readable-school-research',
    kind: 'query-store-summary',
    observedAt: generatedAt,
  };
  const localAiResultReference = {
    evidenceReferenceId: 'screen-local-ai-result-school-research',
    kind: 'local-ai-result',
    observedAt: generatedAt,
  };
  const auditReference = {
    evidenceReferenceId: 'screen-policy-audit-school-research',
    kind: 'journal-event',
    observedAt: generatedAt,
  };

  return {
    schemaVersion: 'v0.6',
    payloadId: 'screen-ai-enforcement-handoff-school-research',
    generatedAt,
    sourcePolicyDecision: {
      schemaVersion: 'v0.6',
      decisionId: 'screen-ai-policy-school-research-time-limit',
      action: 'time-limit',
      reasonCodes: ['screen-ai-policy-school-research'],
      evidenceReferences: [summaryReference, localAiResultReference, auditReference],
      ruleIds: ['screen-ai-parent-rule-school-research-time-limit'],
      localAiResultId: 'screen-local-ai-result-school-research',
      dryRun: true,
      enforcementHandoffState: 'disabled',
      expiresAt: null,
    },
    parentPolicyRule: {
      ruleId: 'screen-ai-parent-rule-school-research-time-limit',
      target: {
        targetId: 'screen-ai-school-research-category',
        targetType: 'category',
        targetValue: 'school-research',
      },
      action: 'time-limit',
      scheduleId: null,
      priority: 100,
      reasonCode: 'screen-ai-policy-school-research',
      createdBy: {
        actorId: 'parent-policy-author',
        role: 'parent',
      },
      enabled: true,
      effectiveFrom: null,
      effectiveUntil: null,
    },
    requestedAction: 'time-limit',
    confidenceState: 'medium',
    handoffMode: 'dry-run',
    inputMaterial: {
      summaryReference,
      localAiResultReference,
      auditReference,
      rawPixelsIncluded: false,
      rawModelTextIncluded: false,
      rawScreenshotRetained: false,
      localAiAuthorityClaimed: false,
    },
    auditEvent: {
      auditEventId: 'screen-ai-enforcement-handoff-school-research-audit',
      eventType: acceptedEventType,
      emittedAt: generatedAt,
      evidenceReference: auditReference,
    },
    claimBoundary: 'Screen AI handoff guard carries refs only; adapter execution remains a separate proof gate.',
  };
}

function readScreenAiHandoffAcceptedEventType() {
  const ownerSource = readFileSync(join(repoRoot, screenAiGuardOwnerPath), 'utf8');
  const match = ownerSource.match(/Accepted:\s*'([^']+)'/u);
  if (match === null) {
    throw new Error(`missing Accepted contract literal in ${screenAiGuardOwnerPath}`);
  }
  return match[1];
}

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    shell: process.platform === 'win32',
    stdio: 'inherit',
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${result.status}`);
  }
}
