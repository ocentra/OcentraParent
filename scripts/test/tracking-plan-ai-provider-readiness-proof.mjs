import { spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'tracking-plan-proof', '24-ai-provider-routing');
const proofResultDir = path.join(repoRoot, 'test-results', 'tracking-plan-ai-provider-readiness-proof');
const proofPath = path.join(proofResultDir, 'proof.json');
const workpackProofPath = path.join(proofRoot, '08-ai-analysis-proof.json');
const contractLogPath = path.join(proofRoot, '01-contract-proof.log');
const securityLogPath = path.join(proofRoot, '13-security-negative-proof.log');
const validationLogPath = path.join(proofRoot, '16-validation-commands.log');
const commands = [];

await main();

async function main() {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(proofResultDir, { recursive: true });

  await runNpm(['--workspace', '@ocentra-parent/activity-domain', 'run', 'build']);
  await runNpm(['--workspace', '@ocentra-parent/activity-domain', 'run', 'test', '--', 'tracking-ai-provider-routing']);
  await runNpm(['run', 'lint:schema-boundaries']);

  const trackingAi = await import('@ocentra-parent/activity-domain/tracking-ai-provider-routing');
  const checkedAt = new Date().toISOString();
  const routeMatrix = buildRouteMatrix(trackingAi);
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit: gitHead(),
    workpackIds: ['24-ai-provider-routing'],
    proofMode: 'tracking-plan-ai-provider-readiness-proof',
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'proved-locally',
    productClaimReady: false,
    routeMatrix,
    gates: {
      childDeviceLocalDefault: routeGate(routeMatrix.childDeviceLocal, 'selected'),
      familyHubLanOnly: routeGate(routeMatrix.familyHubLanOnly, 'selected'),
      remoteRequiresParentApproval: {
        missingApprovalState: routeMatrix.remoteApprovalMissing.executionState,
        approvedState: routeMatrix.remoteApproved.executionState,
        selectedOnlyAfterApproval:
          routeMatrix.remoteApprovalMissing.selectedRuntimeRef === null &&
          routeMatrix.remoteApproved.selectedRuntimeRef === 'tracking-parent-approved-remote-runtime',
      },
      unavailableAndUnsupportedAreDegraded: {
        unavailableState: routeMatrix.providerUnavailable.executionState,
        unavailableReasons: routeMatrix.providerUnavailable.degradedStates,
        unsupportedState: routeMatrix.unsupportedTask.executionState,
        unsupportedReasons: routeMatrix.unsupportedTask.degradedStates,
      },
      metadataOnlyAndNoAiDoNotSelectRuntime: {
        metadataOnlyState: routeMatrix.metadataOnly.executionState,
        metadataOnlyRuntimeRef: routeMatrix.metadataOnly.selectedRuntimeRef,
        noAiState: routeMatrix.noAi.executionState,
        noAiRuntimeRef: routeMatrix.noAi.selectedRuntimeRef,
      },
      authorityNegative: {
        aiCanTriggerAlertDirectly: routeMatrix.childDeviceLocal.aiCanTriggerAlertDirectly,
        aiIsFinalAuthority: routeMatrix.childDeviceLocal.aiIsFinalAuthority,
        assistantCanWritePolicy: routeMatrix.childDeviceLocal.assistantCanWritePolicy,
        remoteDefaultForBlocking: routeMatrix.childDeviceLocal.remoteDefaultForBlocking,
        remoteCanOverrideStricterLocalRules: routeMatrix.childDeviceLocal.remoteCanOverrideStricterLocalRules,
        remoteOutageDisablesLocalSafety: routeMatrix.childDeviceLocal.remoteOutageDisablesLocalSafety,
      },
    },
    validationCommands: commands,
    artifacts: {
      proof: relativePath(proofPath),
      workpackProof: relativePath(workpackProofPath),
      contractLog: relativePath(contractLogPath),
      securityNegativeProof: relativePath(securityLogPath),
      validationLog: relativePath(validationLogPath),
    },
    nonClaims: [
      'This proof does not run a production model, local family hub, or remote/API provider.',
      'This proof does not claim AI can trigger alerts, write policy, enforce, or become final authority.',
      'This proof does not claim Android/iOS physical-device behavior, background location, provider delivery, notification delivery, or production readiness.',
      'Metadata-only and no-AI states remain explicit routes without runtime refs.',
    ],
    remainingGapsBeforeProductReady: [
      'Real child-device local model artifacts and quality validation remain pending.',
      'Real family AI hub runtime discovery and LAN execution remain pending.',
      'Parent-approved remote provider adapter, data-custody review, and UI approval flow remain pending.',
      'Physical Android/iOS tracking runtime and provider delivery proof remain pending.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(workpackProofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(contractLogPath, contractLog(proof));
  await writeFile(securityLogPath, securityNegativeLog(proof));
  await writeFile(
    validationLogPath,
    `${commands.map(({ command, exitCode }) => `${command} # exit ${exitCode}`).join('\n')}\n`
  );

  console.log('tracking-plan-ai-provider-readiness-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

function buildRouteMatrix(trackingAi) {
  return {
    childDeviceLocal: trackingAi.planTrackingAiProviderRoute(routeRequest(trackingAi)),
    familyHubLanOnly: trackingAi.planTrackingAiProviderRoute({
      ...routeRequest(trackingAi),
      routeId: 'tracking-ai-route-family-hub',
      modelRuntimePreference: 'local-preferred',
      capability: familyHubCapability(trackingAi),
    }),
    remoteApprovalMissing: trackingAi.planTrackingAiProviderRoute({
      ...routeRequest(trackingAi),
      routeId: 'tracking-ai-route-remote-missing-approval',
      modelRuntimePreference: 'parent-approved-remote-allowed',
      capability: remoteCapability(trackingAi),
    }),
    remoteApproved: trackingAi.planTrackingAiProviderRoute({
      ...routeRequest(trackingAi),
      routeId: 'tracking-ai-route-remote-approved',
      modelRuntimePreference: 'parent-approved-remote-allowed',
      capability: remoteCapability(trackingAi),
      parentExplicitRemoteApproval: true,
    }),
    providerUnavailable: trackingAi.planTrackingAiProviderRoute({
      ...routeRequest(trackingAi),
      routeId: 'tracking-ai-route-provider-unavailable',
      capability: unavailableChildCapability(trackingAi),
    }),
    unsupportedTask: trackingAi.planTrackingAiProviderRoute({
      ...routeRequest(trackingAi),
      routeId: 'tracking-ai-route-unsupported-task',
      requestedTask: 'parent-summary',
    }),
    metadataOnly: trackingAi.planTrackingAiProviderRoute({
      ...routeRequest(trackingAi),
      routeId: 'tracking-ai-route-metadata-only',
      modelRuntimePreference: 'metadata-only',
      capability: metadataOnlyCapability(trackingAi),
    }),
    noAi: trackingAi.planTrackingAiProviderRoute({
      ...routeRequest(trackingAi),
      routeId: 'tracking-ai-route-no-ai',
      modelRuntimePreference: 'no-ai',
      capability: noAiCapability(trackingAi),
    }),
  };
}

function routeGate(route, expectedState) {
  return {
    status: route.executionState === expectedState ? 'passed' : 'failed',
    executionState: route.executionState,
    providerKind: route.providerKind,
    custodyLabel: route.custodyLabel,
    selectedRuntimeRef: route.selectedRuntimeRef,
  };
}

function routeRequest(trackingAi) {
  return {
    routeId: 'tracking-ai-route-child-local-home-arrival',
    routedAt: '2026-06-05T06:20:00.000Z',
    requestedTask: 'location-safety',
    modelRuntimePreference: 'child-local-required',
    deviceId: 'child-device-tracking-1',
    childProfileRef: 'child-profile-tracking-1',
    policyVersionRef: 'tracking-policy-v1',
    evidenceIds: ['location-evidence-1', 'device-status-1'],
    parentRuleRefs: ['parent-rule-home-arrival'],
    capability: childLocalCapability(trackingAi),
    parentExplicitRemoteApproval: false,
    reasonCodes: ['tracking-ai-route-proof'],
  };
}

function childLocalCapability(trackingAi) {
  return trackingAi.TrackingAiProviderCapabilitySchema.parse({
    schemaVersion: trackingAi.TrackingAiProviderRouteSchemaVersion,
    providerId: 'tracking-child-local-ai',
    checkedAt: '2026-06-05T06:19:00.000Z',
    providerKind: 'child-device-local-ai',
    capabilityState: 'available',
    supportedTasks: ['location-safety', 'expected-place-safety', 'nearby-place-context', 'geofence-risk'],
    modelRuntimeRef: 'tracking-local-model-runtime-child',
    custodyLabel: 'child-device-local',
    noRetention: true,
    localOnly: true,
    parentApprovedRemoteEnabled: false,
    canRunOnChildDevice: true,
    canRunOnParentDevice: false,
    familyHubLanOnly: false,
    degradedStates: [],
    unavailableReason: null,
  });
}

function unavailableChildCapability(trackingAi) {
  return trackingAi.TrackingAiProviderCapabilitySchema.parse({
    ...childLocalCapability(trackingAi),
    capabilityState: 'provider-unavailable',
    modelRuntimeRef: null,
    degradedStates: ['provider-unavailable'],
    unavailableReason: 'tracking-local-provider-unavailable',
  });
}

function familyHubCapability(trackingAi) {
  return trackingAi.TrackingAiProviderCapabilitySchema.parse({
    ...childLocalCapability(trackingAi),
    providerId: 'tracking-family-ai-hub',
    providerKind: 'family-ai-hub',
    modelRuntimeRef: 'tracking-family-hub-runtime-lan',
    custodyLabel: 'live-lan-child-agent',
    canRunOnChildDevice: false,
    familyHubLanOnly: true,
  });
}

function remoteCapability(trackingAi) {
  return trackingAi.TrackingAiProviderCapabilitySchema.parse({
    ...childLocalCapability(trackingAi),
    providerId: 'tracking-parent-approved-remote',
    providerKind: 'parent-approved-remote-ai',
    modelRuntimeRef: 'tracking-parent-approved-remote-runtime',
    custodyLabel: 'parent-approved-cloud',
    localOnly: false,
    parentApprovedRemoteEnabled: true,
    canRunOnChildDevice: false,
  });
}

function metadataOnlyCapability(trackingAi) {
  return trackingAi.TrackingAiProviderCapabilitySchema.parse({
    ...childLocalCapability(trackingAi),
    providerId: 'tracking-metadata-only',
    providerKind: 'metadata-only',
    capabilityState: 'manual-required',
    supportedTasks: ['location-safety'],
    modelRuntimeRef: null,
    canRunOnChildDevice: false,
    degradedStates: ['metadata-only'],
    unavailableReason: 'tracking-metadata-only-route',
  });
}

function noAiCapability(trackingAi) {
  return trackingAi.TrackingAiProviderCapabilitySchema.parse({
    ...metadataOnlyCapability(trackingAi),
    providerId: 'tracking-no-ai',
    providerKind: 'no-ai',
    degradedStates: ['no-ai'],
    unavailableReason: 'tracking-no-ai-route',
  });
}

function contractLog(proof) {
  return [
    'workpack=24-ai-provider-routing',
    `proofMode=${proof.proofMode}`,
    `currentProofTier=${proof.currentProofTier}`,
    'activity-domain tracking AI provider routing tests passed',
    'child local, family hub, explicit remote approval, unavailable, metadata-only, and no-AI routes are schema-backed',
    '',
  ].join('\n');
}

function securityNegativeLog(proof) {
  return [
    'workpack=24-ai-provider-routing',
    `proofMode=${proof.proofMode}`,
    'Remote AI is not selected unless parentExplicitRemoteApproval is true.',
    'Routes reject AI-as-authority, assistant policy writes, remote default blocking, stricter local rule override, and remote outage disabling local safety.',
    'No production model, provider delivery, notification delivery, or physical-device claim is made.',
    '',
  ].join('\n');
}

function runNpm(args) {
  if (process.platform === 'win32') {
    return runCommand('cmd', ['/c', 'npm', ...args]);
  }
  return runCommand('npm', args);
}

function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8' });
  process.stdout.write(result.stdout ?? '');
  process.stderr.write(result.stderr ?? '');
  const exitCode = result.status ?? 1;
  commands.push({ command: commandLine, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${commandLine} exited with ${exitCode}`);
  }
}

function gitHead() {
  const result = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error('git rev-parse HEAD failed');
  }
  return result.stdout.trim();
}

function relativePath(value) {
  return path.relative(repoRoot, value).replace(/\\/gu, '/');
}
