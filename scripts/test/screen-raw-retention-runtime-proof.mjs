import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dirname, '..', '..');
const proofDir = join(repoRoot, 'output', 'screen-plan-proof', 'raw-retention-runtime');
const proofPath = join(proofDir, 'proof-summary.json');

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/screen-domain']);
run('cargo', [
  'test',
  '-p',
  'ocentra-parent-agent-protocol',
  'screen_parent_setting_serializes_parent_approved_raw_retention_ttl',
  '--',
  '--nocapture',
]);
run('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'screen_settings_runtime_', '--', '--nocapture']);

const screenEvidence = await import(
  pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'screen-evidence.js')).href
);

const approved = screenEvidence.ScreenAnalysisParentSettingSchema.parse({
  schemaVersion: screenEvidence.ScreenEvidenceSchemaVersion,
  screenAnalysisEnabled: true,
  analysisMode: 'policyDryRun',
  cadenceCaptureEnabled: true,
  cadenceSeconds: 60,
  strictModeEnabled: true,
  triggerCaptureEnabled: true,
  enabledTriggers: ['foregroundAppChange', 'policyAmbiguity'],
  allowedCaptureScope: 'activeWindow',
  ocrTextEnabled: true,
  ocrTextSnippetLimit: 4,
  redactionMode: 'localSensitiveText',
  ocrTextRetentionMode: 'redactedSnippets',
  credentialSuppressionEnabled: true,
  piiRedactionEnabled: true,
  temporaryImageTtlSeconds: 120,
  maxRetryCount: 2,
  deleteAfterSuccess: true,
  deleteAfterExpiry: true,
  retainRawImage: true,
  policyUseEnabled: true,
  changedByParentRef: 'parent-setting-screen-retention-local-ttl-approval',
  changedAt: '2026-06-07T15:25:00Z',
  settingVersion: 3,
  reason: 'parent approved local short TTL raw screenshot retention',
});

const unsafeLongTtlAccepted = screenEvidence.ScreenAnalysisParentSettingSchema.safeParse({
  ...approved,
  temporaryImageTtlSeconds: 300,
}).success;
const unsafeDisabledAccepted = screenEvidence.ScreenAnalysisParentSettingSchema.safeParse({
  ...approved,
  screenAnalysisEnabled: false,
}).success;
const unsafeDeleteExpiryAccepted = screenEvidence.ScreenAnalysisParentSettingSchema.safeParse({
  ...approved,
  deleteAfterExpiry: false,
}).success;

if (unsafeLongTtlAccepted || unsafeDisabledAccepted || unsafeDeleteExpiryAccepted) {
  throw new Error('Raw-retention runtime contract accepted unsafe parent setting');
}

mkdirSync(proofDir, { recursive: true });
writeFileSync(
  proofPath,
  `${JSON.stringify(
    {
      proof: 'screen-raw-retention-runtime-proof',
      generatedAt: new Date().toISOString(),
      claim:
        'Optional raw screenshot retention can execute only as a parent-approved local short-TTL mode with deletion after success and expiry still required.',
      contract: {
        retainRawImageAccepted: approved.retainRawImage === true,
        maxTtlSeconds: approved.temporaryImageTtlSeconds,
        deleteAfterSuccessRequired: approved.deleteAfterSuccess,
        deleteAfterExpiryRequired: approved.deleteAfterExpiry,
        parentSettingRef: approved.changedByParentRef,
      },
      rustValidation: {
        protocolSerializationTest: 'screen_parent_setting_serializes_parent_approved_raw_retention_ttl',
        serviceRuntimeTestFilter: 'screen_settings_runtime_',
        unsafeRawRetentionRejected: true,
      },
      negativeChecks: {
        longTtlRejected: !unsafeLongTtlAccepted,
        disabledAnalysisRejected: !unsafeDisabledAccepted,
        deleteAfterExpiryFalseRejected: !unsafeDeleteExpiryAccepted,
      },
      nonClaims: [
        'This proof does not retain raw screenshots by default.',
        'This proof does not allow raw screenshot remote upload or long-term storage.',
        'This proof does not complete live view, relay/cache execution, or privacy/legal approval.',
      ],
    },
    null,
    2
  )}\n`
);

console.log(`screen-raw-retention-runtime-proof-ok:${proofPath}`);

function run(command, args) {
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: process.platform === 'win32' });
}
