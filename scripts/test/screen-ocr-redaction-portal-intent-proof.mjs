import { strict as assert } from 'node:assert';
import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));
const outputDir = join(repoRoot, 'output', 'screen-plan-proof', '19-sensitive-text-and-redaction-model');
const testResultDir = join(repoRoot, 'test-results', 'screen-ocr-redaction-portal-intent-proof');
const outputPath = join(outputDir, 'portal-intent-proof-summary.json');
const testResultPath = join(testResultDir, 'proof.json');

run('npm', ['run', 'build', '--workspace=@ocentra-parent/activity-domain']);
run('npm', ['run', 'build', '--workspace=@ocentra-parent/portal-domain']);
run('npm', ['run', 'test', '--workspace=@ocentra-parent/activity-domain', '--', 'tests/activity-surface.test.ts']);
run('npm', ['run', 'test', '--workspace=@ocentra-parent/portal-domain', '--', 'tests/screen-summary-panel.test.ts']);

const { ActivityScreenReadModelSchema, ActivitySurfaceSchemaVersion } =
  await import('@ocentra-parent/activity-domain/activity-surface');
const { ActivityEvidenceKind } = await import('@ocentra-parent/activity-domain/kinds');
const { createScreenSummaryPanelIntent } = await import('@ocentra-parent/portal-domain/screen-summary-panel');

const evidenceRef = {
  evidenceId: 'screen-ocr-redaction-portal-evidence',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:screen-ocr-redaction-portal-evidence',
  uri: null,
};

const readModel = ActivityScreenReadModelSchema.parse({
  schemaVersion: ActivitySurfaceSchemaVersion,
  request: {
    schemaVersion: ActivitySurfaceSchemaVersion,
    scope: {
      scopeKind: 'device',
      familyId: null,
      deviceId: 'child-device-ocr-redaction',
    },
    requestedAt: '2026-06-06T23:18:00Z',
    rangeStart: '2026-06-06T23:00:00Z',
    rangeEnd: '2026-06-06T23:18:00Z',
  },
  state: 'ready',
  generatedAt: '2026-06-06T23:18:01Z',
  summary: 'Redacted OCR Activity Screen row is ready',
  rows: [
    {
      rowId: 'screen-ocr-redaction-portal-row',
      label: 'School portal OCR summary',
      deviceId: 'child-device-ocr-redaction',
      state: 'ready',
      totalMs: 1000,
      foregroundMs: 1000,
      backgroundMs: 0,
      captureReason: 'timedCadence',
      captureScope: 'activeWindow',
      capabilityStatus: 'available',
      queueJobId: 'screen-ocr-redaction-queue-job',
      modelRuntimeRef: 'windows-winrt-ocr-local-runtime',
      modelId: 'windows-winrt-ocr',
      providerKind: 'localOcr',
      promptOrTemplateVersion: 'screen-ocr-worker-winrt-v1',
      primaryCategory: 'school',
      confidence: 0.92,
      imageDeletionState: 'deleted',
      rawImageRetained: false,
      policyEligible: true,
      imageDigest: 'sha256:screen-ocr-redaction-image',
      custodyState: 'child-device-query-store',
      evidence: [evidenceRef],
      policyDecisionRef: 'screen-ocr-redaction-policy-decision',
      policyAction: 'allow',
      policyReasonCodes: ['ocr-redaction-safe-summary'],
      parentRuleRefs: ['screen-parent-rule-school'],
      localModelRuntimeRefs: ['windows-winrt-ocr-local-runtime'],
      parentExplanationRefs: ['screen-ocr-redaction-parent-explanation'],
      explanationReasons: ['redacted-ocr-snippet-cited'],
      deletionReasons: ['screen-image-deleted'],
      ocrTextSnippets: ['School portal account [redacted-email]', 'Homework note [redacted-phone]'],
      redactionNotes: ['piiLikeTextRedacted', 'credentialLikeTextRedacted'],
    },
  ],
});

const intent = createScreenSummaryPanelIntent({
  ok: true,
  state: 'ready',
  value: {
    state: readModel.state,
    generatedAt: readModel.generatedAt,
    returned: readModel.rows.length,
    rows: readModel.rows,
  },
});

const rendered = JSON.stringify(intent);
assert.equal(intent.rows.length, 1);
assert.match(rendered, /OCR snippets/);
assert.match(rendered, /School portal account \[redacted-email\]/);
assert.match(rendered, /Homework note \[redacted-phone\]/);
assert.match(rendered, /Redaction notes/);
assert.match(rendered, /piiLikeTextRedacted/);
assert.match(rendered, /credentialLikeTextRedacted/);
assert.doesNotMatch(rendered, /student@example\.com/);
assert.doesNotMatch(rendered, /555-010-1234/);
assert.doesNotMatch(rendered, /hunter2/);

const proof = {
  proof: 'screen-ocr-redaction-portal-intent-proof',
  proofTier: 'P2_CONTRACT_SCREEN_OCR_REDACTION_PORTAL_INTENT',
  artifact: 'output/screen-plan-proof/19-sensitive-text-and-redaction-model/portal-intent-proof-summary.json',
  source: {
    activityReadModelSchema: '@ocentra-parent/activity-domain/activity-surface',
    portalIntent: '@ocentra-parent/portal-domain/screen-summary-panel',
  },
  assertions: {
    activityScreenReadModelCarriesRedactedOcrSnippets: readModel.rows[0]?.ocrTextSnippets.length === 2,
    activityScreenReadModelCarriesRedactionNotes: readModel.rows[0]?.redactionNotes.length === 2,
    portalIntentRendersRedactedOcrSnippets: rendered.includes('School portal account [redacted-email]'),
    portalIntentRendersRedactionNotes: rendered.includes('piiLikeTextRedacted'),
    rawEmailOmitted: !rendered.includes('student@example.com'),
    rawPhoneOmitted: !rendered.includes('555-010-1234'),
    rawCredentialOmitted: !rendered.includes('hunter2'),
    rawImageRetained: readModel.rows[0]?.rawImageRetained ?? true,
  },
  nonClaims: [
    'This is Activity Screen read-model and portal intent proof, not a live service persistence proof.',
    'This proof does not claim a real portal screenshot from a service-emitted redaction row.',
    'This proof does not retain raw OCR text, raw screenshots, or use remote AI.',
  ],
  validationCommands: [
    'node scripts/test/screen-ocr-redaction-portal-intent-proof.mjs',
    'cmd /c npm run build --workspace=@ocentra-parent/activity-domain',
    'cmd /c npm run build --workspace=@ocentra-parent/portal-domain',
    'cmd /c npm run test --workspace=@ocentra-parent/activity-domain -- tests/activity-surface.test.ts',
    'cmd /c npm run test --workspace=@ocentra-parent/portal-domain -- tests/screen-summary-panel.test.ts',
  ],
};

mkdirSync(outputDir, { recursive: true });
mkdirSync(testResultDir, { recursive: true });
writeFileSync(outputPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(testResultPath, `${JSON.stringify(proof, null, 2)}\n`);

console.log(`screen-ocr-redaction-portal-intent-proof-ok:${proof.proofTier}`);
console.log(`artifact=${outputPath}`);

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
