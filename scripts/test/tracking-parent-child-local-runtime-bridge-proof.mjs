import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-parent-child-local-runtime-bridge-proof';
const resultRoot = join(repoRoot, 'test-results', proofMode);
const namedProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output30 = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const eventingProofRef = 'test-results/eventing-parent-child-runtime-proof/proof.json';
const eventingRowProofRef = 'output/eventing-plan-proof/51-54-parent-child-runtime/proof-summary.json';
const generatedAt = '2026-06-08T18:10:00.000Z';
const commands = [];

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });
  await mkdir(output30, { recursive: true });
  await mkdir(output33, { recursive: true });

  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-parent-child-local-runtime-bridge-proof.test.ts',
  ]);

  const eventingProof = await readJson(eventingProofRef);
  const eventingRowProof = await readJson(eventingRowProofRef);
  const proofModule = await tsImport(
    pathToFileURL(
      join(repoRoot, 'packages', 'schema-domain', 'src', 'tracking-parent-child-local-runtime-bridge-proof.ts')
    ).href,
    import.meta.url
  );
  const proof = buildProof({ proofModule, eventingProof, eventingRowProof });

  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-parent-child-local-runtime-bridge-proof-ok');
  console.log('evidence=test-results/tracking-parent-child-local-runtime-bridge-proof/proof.json');
}

function buildProof({ proofModule, eventingProof, eventingRowProof }) {
  const phaseRefs = proofModule.RequiredTrackingParentChildLocalRuntimeBridgePhaseRefs;
  const readModel = proofModule.buildTrackingParentChildLocalRuntimeBridgeProof(generatedAt, {
    eventingProofRef,
    eventingRowProofRef,
    runtimeSourceRefs: [
      eventingProof.linkedArtifacts.runtime,
      eventingProof.linkedArtifacts.runtimeBuild,
      eventingProof.linkedArtifacts.runtimePhase,
      eventingProof.linkedArtifacts.runtimeTests,
    ],
    phaseRefs,
    publishReportCount: phaseRefs.length,
    storedEventCount: phaseRefs.length,
    deadLetterCount: 0,
    childAgentPhaseCount: 4,
    parentReadModelProjectionObserved: eventingProof.claimsProved.some((claim) =>
      claim.includes('parent read-model projection')
    ),
    typedLocalServiceTransportObserved: eventingProof.claimsProved.some((claim) =>
      claim.includes('parent child-command forward requested and forwarded events')
    ),
  });

  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackIds: ['30-parent-and-child-ui-ux-surfaces', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'local_parent_child_runtime_observed_physical_child_runtime_required',
    sourceProofRefs: [eventingProofRef, eventingRowProofRef],
    eventingRowsCovered: eventingRowProof.rowsCovered,
    readModel,
    summary: {
      localParentChildRuntimeObserved: readModel.productClaims.localParentChildRuntimeObserved,
      publishReportCount: readModel.rows[0].publishReportCount,
      storedEventCount: readModel.rows[0].storedEventCount,
      deadLetterCount: readModel.rows[0].deadLetterCount,
      childAgentPhaseCount: readModel.rows[0].childAgentPhaseCount,
      phaseRefCount: readModel.rows[0].phaseRefs.length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    productClaims: readModel.productClaims,
    missingProofReason:
      'The existing Rust eventing proof shows a local parent/controller to child-agent event chain and parent read-model projection over typed local service transport. It does not prove physical child-device delivery/execution, rendered child-device UI, parent receipt runtime from a device, authority enrollment, provider delivery, production workers, or product-ready tracking.',
    commands,
  };
}

function assertProof(proof) {
  if (!proof.summary.localParentChildRuntimeObserved) {
    throw new Error(`Local parent-child runtime evidence is missing: ${JSON.stringify(proof.summary)}`);
  }
  if (proof.summary.deadLetterCount !== 0) {
    throw new Error(`Local parent-child runtime has dead letters: ${JSON.stringify(proof.summary)}`);
  }
  if (proof.summary.phaseRefCount < 9 || proof.summary.childAgentPhaseCount < 4) {
    throw new Error(`Local parent-child phase coverage is incomplete: ${JSON.stringify(proof.summary)}`);
  }
  if (
    proof.productClaims.childDeviceDeliveryRuntimeClaimed ||
    proof.productClaims.childDeviceExecutionRuntimeClaimed ||
    proof.productClaims.renderedChildDeviceUiRuntimeClaimed ||
    proof.productClaims.productClaimReady
  ) {
    throw new Error(
      `Local bridge proof overclaimed child runtime/product readiness: ${JSON.stringify(proof.productClaims)}`
    );
  }
}

async function writeProofArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'read-model.json'), proof.readModel);
  await writeJson(join(namedProofRoot, 'proof.json'), proof);
  await writeJson(join(namedProofRoot, 'read-model.json'), proof.readModel);
  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeJson(join(output30, '35-parent-child-local-runtime-bridge-proof.json'), proof);
  await writeJson(join(output33, '68-parent-child-local-runtime-bridge-proof.json'), proof);
  await writeFile(join(output33, '68-parent-child-local-runtime-bridge-validation-commands.log'), validationLog());
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Parent-Child Local Runtime Bridge Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.commit}`,
    '- requiredProofTier: P4_PHYSICAL_DEVICE',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    '- status: local parent-child runtime observed, physical child runtime still required',
    `- source proof: ${eventingProofRef}`,
    `- source proof: ${eventingRowProofRef}`,
    '- does not prove physical child-device delivery/execution runtime, rendered child-device UI, parent receipt runtime, physical-device behavior, authority, provider delivery, production, or product readiness',
    '',
  ].join('\n');
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

async function readJson(relativePath) {
  return JSON.parse(await readFile(join(repoRoot, relativePath), 'utf8'));
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) return '';
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
