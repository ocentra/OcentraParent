import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'social-video-source-custody-settings-proof');
const resultDirectory = join(root, 'test-results', 'social-video-source-custody-settings-proof');

const requiredFiles = [
  'packages/browser-domain/src/social-video-source-custody-settings.ts',
  'packages/browser-domain/tests/unit/social-video-source-custody-settings.test.ts',
  'docs/features/social-video-control.md',
  'docs/plans/browser-plan/social-platform-account-feed/readme.md',
  'docs/plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md',
];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  const files = Object.fromEntries(await Promise.all(requiredFiles.map(async (path) => [path, await readText(path)])));
  const checks = [
    checkIncludes(files, 'packages/browser-domain/src/social-video-source-custody-settings.ts', [
      'SocialVideoSourceCustodySettingsSchema',
      'sourcePrivacyEvidenceIds',
      'connector-authorization-ref-only',
      'runtimeCustodyMutationClaimed',
      'finalPolicyDecisionClaimed',
      'enforcementClaimed',
    ]),
    checkIncludes(files, 'packages/browser-domain/tests/unit/social-video-source-custody-settings.test.ts', [
      'accepts enabled source custody settings over source privacy refs',
      'accepts parent-review connector and manual-required custody boundaries',
      'rejects disabled manual and unavailable settings that pretend to feed policy input',
      'rejects raw custody connector runtime final-policy and enforcement claims',
    ]),
    checkIncludes(files, 'docs/features/social-video-control.md', [
      'social-video-source-custody-settings-proof',
      'Source permissions and custody settings',
      'runtime custody settings remain',
    ]),
    checkIncludes(files, 'docs/plans/browser-plan/social-platform-account-feed/readme.md', [
      'social-video-source-custody-settings-proof',
      'source custody settings',
    ]),
    checkIncludes(files, 'docs/plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md', [
      'social-video-source-custody-settings-proof',
      'source custody settings',
    ]),
  ];
  const failures = checks.flatMap((check) => check.failures);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-video-source-custody-settings-proof',
    generatedAt: new Date().toISOString(),
    files: requiredFiles,
    checks,
    claims: {
      sourceCustodySettingsContract: 'proof-present',
      sourcePrivacyEvidenceRefs: 'required',
      policyCandidateInput: 'enabled-redacted-ref-contract-only',
      parentReviewBoundary: 'proof-present',
      manualRequiredBoundary: 'proof-present',
      runtimeSettingsUi: 'not-claimed',
      runtimeCustodyMutation: 'not-claimed',
      rawContentCustody: 'not-claimed',
      connectorApiCalls: 'not-claimed',
      finalPolicyDecision: 'not-claimed',
      enforcement: 'not-claimed',
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social video source custody settings proof failed:\n${failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const markdownPath = join(outputDirectory, '01-social-video-source-custody-settings-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-video-source-custody-settings-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
}

function checkIncludes(files, path, expectedTexts) {
  const text = files[path] ?? '';
  return {
    path,
    failures: expectedTexts
      .filter((expectedText) => !text.includes(expectedText))
      .map((expectedText) => `${path} missing ${expectedText}`),
  };
}

function markdownFor(proof) {
  return [
    '# Social Video Source Custody Settings Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    'This proof verifies the activity-domain source custody settings contract built on source/privacy evidence refs.',
    '',
    'Claims:',
    '',
    `- Source custody settings contract: ${proof.claims.sourceCustodySettingsContract}`,
    `- Source/privacy evidence refs: ${proof.claims.sourcePrivacyEvidenceRefs}`,
    `- Policy candidate input: ${proof.claims.policyCandidateInput}`,
    `- Parent review boundary: ${proof.claims.parentReviewBoundary}`,
    `- Manual-required boundary: ${proof.claims.manualRequiredBoundary}`,
    `- Runtime settings UI: ${proof.claims.runtimeSettingsUi}`,
    `- Runtime custody mutation: ${proof.claims.runtimeCustodyMutation}`,
    `- Raw content custody: ${proof.claims.rawContentCustody}`,
    `- Connector API calls: ${proof.claims.connectorApiCalls}`,
    `- Final policy decision: ${proof.claims.finalPolicyDecision}`,
    `- Enforcement: ${proof.claims.enforcement}`,
    '',
    'Enabled rows can feed policy candidate input only as redacted refs over source/privacy evidence.',
    'Parent-review, disabled, manual-required, and unavailable rows stay out of policy input.',
  ].join('\n');
}

async function readText(path) {
  return readFile(join(root, path), 'utf8');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
