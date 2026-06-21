import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '19-sensitive-text-and-redaction-model');
const testResultRoot = resolve(repoRoot, 'test-results', 'screen-ocr-redaction-proof');

await Promise.all([mkdir(outputRoot, { recursive: true }), mkdir(testResultRoot, { recursive: true })]);

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/screen-domain']));
runCommand(
  ...npmCommand(['run', 'test', '--workspace', '@ocentra-parent/screen-domain', '--', 'screen-ocr-redaction'])
);

const {
  ScreenOcrRedactionPolicySchema,
  ScreenOcrRedactionProofSchema,
  ScreenOcrRedactionSchemaVersion,
  redactScreenOcrText,
} = await import('@ocentra-parent/schema-domain/screen-ocr-redaction');

const evidenceRef = {
  evidenceId: 'screen-ocr-redaction-proof-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-ocr-redaction-proof-image',
  uri: null,
};

const policy = ScreenOcrRedactionPolicySchema.parse({
  schemaVersion: ScreenOcrRedactionSchemaVersion,
  policyId: 'screen-ocr-redaction-proof-policy',
  updatedAt: new Date('2026-06-06T22:55:00.000Z').toISOString(),
  ocrTextEnabled: true,
  snippetLimit: 2,
  redactionMode: 'localSensitiveText',
  textRetentionMode: 'redactedSnippets',
  credentialSuppressionEnabled: true,
  piiRedactionEnabled: true,
  parentControlled: true,
  rawTextRetentionAllowed: false,
});

const result = redactScreenOcrText({
  policy,
  processedAt: new Date('2026-06-06T22:56:00.000Z').toISOString(),
  lines: [
    {
      text: 'School portal account jane@example.com',
      confidence: 0.92,
      evidenceRefs: [evidenceRef],
    },
    {
      text: 'password reset token visible',
      confidence: 0.89,
      evidenceRefs: [evidenceRef],
    },
    {
      text: 'Parent phone 555-010-1234',
      confidence: 0.84,
      evidenceRefs: [evidenceRef],
    },
  ],
});

const disabledPolicy = ScreenOcrRedactionPolicySchema.parse({
  ...policy,
  ocrTextEnabled: false,
  snippetLimit: 0,
  redactionMode: 'disabled',
  textRetentionMode: 'disabled',
  piiRedactionEnabled: false,
});
const disabledResult = redactScreenOcrText({
  policy: disabledPolicy,
  processedAt: new Date('2026-06-06T22:57:00.000Z').toISOString(),
  lines: [
    {
      text: 'Disabled OCR should not retain text',
      confidence: 0.9,
      evidenceRefs: [evidenceRef],
    },
  ],
});

const proof = ScreenOcrRedactionProofSchema.parse({
  schemaVersion: ScreenOcrRedactionSchemaVersion,
  proofId: 'screen-ocr-redaction-proof',
  proofTier: 'P2_CONTRACT_SCREEN_OCR_REDACTION',
  policy,
  result,
  credentialSuppressed: result.suppressed.some((row) => row.sensitiveKind === 'credentialLikeText'),
  piiRedacted: result.redactionNotes.includes('piiLikeTextRedacted'),
  disabledStateProved: disabledResult.redactionNotes.includes('ocrDisabled') && disabledResult.snippets.length === 0,
  localOnly: true,
  rawTextRetained: false,
  rawImageRetained: false,
  remoteAiUsed: false,
  portalRuntimeClaimed: false,
  servicePersistenceClaimed: false,
});

const proofSummary = {
  proof: 'screen-ocr-redaction-proof',
  proofTier: proof.proofTier,
  artifact: 'output/screen-plan-proof/19-sensitive-text-and-redaction-model/proof-summary.json',
  assertions: {
    snippetLimitEnforced: proof.result.snippets.length <= proof.policy.snippetLimit,
    credentialSuppressed: proof.credentialSuppressed,
    piiRedacted: proof.piiRedacted,
    disabledStateProved: proof.disabledStateProved,
    rawTextRetained: proof.rawTextRetained,
    rawImageRetained: proof.rawImageRetained,
    remoteAiUsed: proof.remoteAiUsed,
  },
  redactedSnippets: proof.result.snippets.map((snippet) => snippet.text),
  suppressedKinds: proof.result.suppressed.map((row) => row.sensitiveKind),
  redactionNotes: proof.result.redactionNotes,
  disabledRedactionNotes: disabledResult.redactionNotes,
  nonClaims: [
    'This is OCR sensitive-text redaction contract proof, not service persistence.',
    'This proof does not claim portal runtime rendering or a live OCR adapter run.',
    'This proof does not retain raw OCR text, raw screenshots, or send OCR text to remote AI.',
  ],
  validationCommands: [
    'node scripts/test/screen-ocr-redaction-proof.mjs',
    'cmd /c npm run build --workspace @ocentra-parent/screen-domain',
    'cmd /c npm run test --workspace @ocentra-parent/screen-domain -- screen-ocr-redaction',
  ],
};

await writeFile(resolve(outputRoot, 'proof-summary.json'), `${JSON.stringify(proofSummary, null, 2)}\n`);
await writeFile(resolve(testResultRoot, 'proof.json'), `${JSON.stringify(proofSummary, null, 2)}\n`);

console.log(`screen-ocr-redaction-proof-ok:${proof.proofTier}`);
console.log(`artifact=${resolve(outputRoot, 'proof-summary.json')}`);

function runCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: false,
  });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
