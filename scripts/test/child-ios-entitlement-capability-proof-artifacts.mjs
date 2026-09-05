import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';

export const CHILD_IOS_CANONICAL_PROOF_ROOT =
  'output/child-agent-runtime-distribution-plan-proof/06-child-ios-agent-capability-package';
export const CHILD_IOS_LEGACY_PROOF_PATH = 'test-results/child-ios-entitlement-capability-proof/proof.json';

const CANONICAL_PROOF_FILES = Object.freeze([
  '00-scope-summary.md',
  '01-negative-case-proof.md',
  '02-no-claim-boundary.md',
  '16-validation-commands.log',
]);

export function classifyIosXctestResult({ command, exitCode, output, platform }) {
  const skipMatch = output.match(/# SKIP\s+([^\r\n]+)/);
  if (exitCode !== 0) {
    return Object.freeze({
      command,
      exitCode,
      platform,
      status: 'failed',
      reason: lastOutputLine(output) ?? 'XCTest command failed without output',
    });
  }
  if (skipMatch) {
    return Object.freeze({
      command,
      exitCode,
      platform,
      status: 'skipped',
      reason: skipMatch[1].trim(),
    });
  }
  return Object.freeze({
    command,
    exitCode,
    platform,
    status: 'passed',
    reason: 'The real child iOS XCTest command completed successfully on this host.',
  });
}

export function renderChildIosProofArtifacts(proof) {
  const outcome = proof.xctestOutcome;
  const scopeSummary =
    `# WP06 Child iOS Capability Package\n\n` +
    `- checked_at: ${proof.checkedAt}\n` +
    `- commit: ${proof.commit}\n` +
    `- project: OcentraChildAgent\n` +
    `- bundle_id: ${proof.runtimeReadModel.bundleId}\n` +
    `- xctest_status: ${outcome.status}\n` +
    `- xctest_reason: ${outcome.reason}\n\n` +
    `The retained result proves only the checked-in child identity and the capability-status boundary observed by the ` +
    `real XCTest harness on the available host. A skipped result is explicit host-blocked evidence, not a pass.\n`;
  const negativeCaseProof =
    `# WP06 Negative-Case Proof\n\n` +
    `The runner rejects parent product, project, scheme, bundle, and artifact identity. It also rejects capability ` +
    `contracts that claim external transport, daemon behavior, supervision proof, or recovery implementation.\n\n` +
    `XCTest outcome: **${outcome.status}** (${outcome.reason}).\n`;
  const noClaimBoundary =
    `# WP06 No-Claim Boundary\n\n` +
    `This result does not claim Apple signing, provisioning or entitlement approval, physical-device launch, ` +
    `TestFlight or App Store distribution, supervision, persistent background execution, daemon or hidden-control ` +
    `behavior, external transport, recovery, tracking ingress, or parent-client parity.\n`;
  const commandLog = proof.commandResults
    .map(
      (result) =>
        `command: ${result.command}\n` +
        `exit: ${result.exitCode}\n` +
        `result: ${result.result}\n` +
        `artifact: ${result.artifact ?? 'n/a'}\n` +
        `notes: ${result.notes}\n`
    )
    .join('\n');

  return new Map([
    ['00-scope-summary.md', scopeSummary],
    ['01-negative-case-proof.md', negativeCaseProof],
    ['02-no-claim-boundary.md', noClaimBoundary],
    ['16-validation-commands.log', commandLog],
  ]);
}

export async function writeChildIosProofOutputs(repoRoot, proof) {
  const canonicalDirectory = join(repoRoot, ...CHILD_IOS_CANONICAL_PROOF_ROOT.split('/'));
  const legacyProofPath = join(repoRoot, ...CHILD_IOS_LEGACY_PROOF_PATH.split('/'));
  const artifacts = renderChildIosProofArtifacts(proof);
  assertExactArtifactNames([...artifacts.keys()]);

  await rm(canonicalDirectory, { recursive: true, force: true });
  await mkdir(canonicalDirectory, { recursive: true });
  await mkdir(dirname(legacyProofPath), { recursive: true });
  await Promise.all([
    ...[...artifacts].map(([name, contents]) => writeFile(join(canonicalDirectory, name), contents, 'utf8')),
    writeFile(legacyProofPath, `${JSON.stringify(proof, null, 2)}\n`, 'utf8'),
  ]);
}

function assertExactArtifactNames(actual) {
  if (JSON.stringify(actual) !== JSON.stringify(CANONICAL_PROOF_FILES)) {
    throw new Error(`unexpected child iOS proof artifacts: ${JSON.stringify(actual)}`);
  }
}

function lastOutputLine(output) {
  return output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean)
    .at(-1);
}
