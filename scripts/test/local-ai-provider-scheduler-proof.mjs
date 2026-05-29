import { spawn } from 'node:child_process';
import { mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import {
  AgentCommand,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  ParentDevEnv,
  ParentDevPort,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  isLikelyParentAgentOccupant,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import { resolveDebugAgentServicePath, stopProcessTreeAndWait } from './agent-service-process.mjs';

const proofPort = ParentDevPort.WebSocketSmokeAgent;
const healthUrl = createAgentHealthUrl(proofPort);
const wsUrl = createAgentWebSocketUrl(proofPort);
const devLogDir = await mkdtemp(join(tmpdir(), 'ocentra-parent-local-ai-scheduler-proof-'));
const proofOutputDir = join(process.cwd(), 'test-results', 'local-ai-provider-scheduler-proof');
const proofOutputPath = join(proofOutputDir, 'proof.json');
const successfulCommands = [];

await runPackageCommand(['run', 'build:contracts']);
await runPackageCommand([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'local-ai-provider-scheduler.test.ts',
]);
await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'local_ai_provider_scheduler']);
await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'local_ai_provider_scheduler']);
await ensurePortFree(proofPort, isLikelyParentAgentOccupant, console.log);

const service = spawn(resolveDebugAgentServicePath(), [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(proofPort),
    [ParentDevEnv.ActivityDbPath]: join(devLogDir, 'activity.sqlite'),
    [ParentDevEnv.DevLogDir]: devLogDir,
    OCENTRA_PARENT_LOCAL_AI_EXECUTION_ENABLED: 'false',
  },
  stdio: ['ignore', 'pipe', 'pipe'],
});

const serviceOutput = collectOutput(service);

try {
  await waitForHttp(healthUrl);
  await runUnavailableLifecycleProof();
  await writeSchedulerProof();
  console.log(
    `local-ai-provider-scheduler-proof-ok: unavailable lifecycle, singleton scheduler, and priority queue tests passed (${proofOutputPath})`
  );
} finally {
  await stopProcessTreeAndWait(service);
  await rm(devLogDir, { recursive: true, force: true });
}

function runUnavailableLifecycleProof() {
  return new Promise((resolve, reject) => {
    const socket = new WebSocket(wsUrl);
    let sawStatus = false;
    let settled = false;
    const timer = setTimeout(() => {
      fail(new Error('Local AI provider scheduler proof timed out'));
    }, 30000);

    const fail = (error) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      socket.close();
      reject(error);
    };

    const complete = () => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      socket.close();
      resolve();
    };

    socket.addEventListener('open', () => {
      socket.send(
        JSON.stringify(commandEnvelope('cmd-local-ai-runtime-status', AgentCommand.LocalAiRuntimeStatusGet, {}))
      );
    });

    socket.addEventListener('message', (message) => {
      try {
        const parsed = AgentEventEnvelopeSchema.parse(JSON.parse(String(message.data)));
        if (parsed.event === AgentEvent.LocalAiRuntimeStatusReported) {
          assertRuntimeUnavailable(parsed.payload);
          sawStatus = true;
          socket.send(
            JSON.stringify(
              commandEnvelope('cmd-local-ai-parent-assistant', AgentCommand.LocalAiChatGenerate, {
                [AgentProtocolDefaults.Field.LocalAiPrompt]: 'Summarize current local provider state.',
                [AgentProtocolDefaults.Field.LocalAiMaxOutputTokens]: 16,
                [AgentProtocolDefaults.Field.LocalAiTimeoutMs]: 30000,
              })
            )
          );
        }

        if (parsed.event === AgentEvent.LocalAiChatGenerationReported) {
          if (!sawStatus) {
            fail(new Error('Chat generation reported before runtime status proof'));
            return;
          }
          assertGenerationUnavailable(parsed.payload);
          complete();
        }
      } catch (error) {
        fail(error instanceof Error ? error : new Error(String(error)));
      }
    });

    socket.addEventListener('error', () => {
      fail(new Error('Local AI provider scheduler proof WebSocket failed'));
    });
  });
}

function assertRuntimeUnavailable(payload) {
  if (payload[AgentProtocolDefaults.Field.LocalAiExecutionAllowed] !== false) {
    throw new Error(`Local AI runtime unexpectedly allowed execution: ${JSON.stringify(payload)}`);
  }
  if (payload[AgentProtocolDefaults.Field.LocalAiExecutionState] !== 'disabled') {
    throw new Error(`Local AI runtime did not report disabled execution: ${JSON.stringify(payload)}`);
  }
  if (payload[AgentProtocolDefaults.Field.LocalAiUnavailableReason] === null) {
    throw new Error(`Local AI runtime did not report unavailable reason: ${JSON.stringify(payload)}`);
  }
}

function assertGenerationUnavailable(payload) {
  if (payload[AgentProtocolDefaults.Field.LocalAiGenerationState] !== 'unavailable') {
    throw new Error(`Local AI generation unexpectedly completed: ${JSON.stringify(payload)}`);
  }
  if (payload[AgentProtocolDefaults.Field.LocalAiOutputText] !== null) {
    throw new Error(`Local AI generation leaked output while unavailable: ${JSON.stringify(payload)}`);
  }
  if (payload[AgentProtocolDefaults.Field.LocalAiUnavailableReason] === null) {
    throw new Error(`Local AI generation did not report unavailable reason: ${JSON.stringify(payload)}`);
  }
}

function commandEnvelope(messageId, command, payload) {
  return {
    schemaVersion: 1,
    messageId,
    sentAt: new Date().toISOString(),
    source: { peerId: 'portal-dev', role: 'portal' },
    target: { deviceId: 'local-dev-agent', platform: 'windows', route: 'localhost' },
    command,
    payload,
  };
}

async function writeSchedulerProof() {
  await mkdir(proofOutputDir, { recursive: true });
  const proof = {
    proofGeneratedAt: new Date().toISOString(),
    proofTopic: 'local-ai-provider-runtime-scheduler',
    commands: successfulCommands,
    claimsProven: [
      'one-ai-provider-role-per-physical-device',
      'one-local-model-runtime-access-lane-per-device',
      'child-safety-jobs-prioritized-above-parent-assistant-jobs',
      'queued-degraded-unavailable-provider-states',
      'no-duplicate-local-model-load-for-same-physical-device',
      'parent-assistant-job-submission-to-local-provider-when-allowed',
      'parent-and-child-roles-share-provider-runtime-on-one-physical-device',
    ],
    evidence: {
      typescriptContractTest: '@ocentra-parent/parent-domain local-ai-provider-scheduler.test.ts',
      rustProtocolParityTest: 'cargo test -p ocentra-parent-agent-protocol local_ai_provider_scheduler',
      rustServiceSchedulerTest: 'cargo test -p ocentra-parent-agent-service local_ai_provider_scheduler',
      liveUnavailableLifecycle: {
        healthUrl,
        webSocketUrl: wsUrl,
        executionEnabled: false,
      },
    },
  };
  await writeFile(proofOutputPath, `${JSON.stringify(proof, null, 2)}\n`, 'utf8');
}

async function waitForHttp(url) {
  const startedAt = Date.now();
  while (Date.now() - startedAt < 30000) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${url}\n${serviceOutput()}`);
}

function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: process.cwd(),
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const output = collectOutput(child);
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        successfulCommands.push(`${command} ${args.join(' ')}`);
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} failed with ${code}\n${output()}`));
    });
  });
}

function runPackageCommand(args) {
  if (process.platform === 'win32') {
    return runCommand('cmd', ['/c', 'npm', ...args]);
  }

  return runCommand('npm', args);
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}
