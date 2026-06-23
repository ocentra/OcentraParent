#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import {
  getArtifactSlice,
  getErrors,
  getLatestFailures,
  getLogStats,
  getLogsByContext,
  getLogsBySource,
  getProofInventoryStatus,
  getProofTrace,
  getProofTraceGaps,
  getRecentLogs,
  queryProofTrace,
  getRunDiagnostics,
  queryLogs,
} from './lib/log-query-service.mjs';

const TOOLS = [
  {
    name: 'get_errors',
    description: 'Return recent error-level logs for a scope.',
    inputSchema: {
      type: 'object',
      properties: {
        scope: { type: 'string' },
        since: { type: 'string' },
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'get_recent_logs',
    description: 'Return recent logs with bounded limits.',
    inputSchema: {
      type: 'object',
      properties: {
        scope: { type: 'string' },
        level: { type: 'string' },
        since: { type: 'string' },
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'get_logs_by_source',
    description: 'Return logs filtered by source.',
    inputSchema: {
      type: 'object',
      required: ['source'],
      properties: {
        scope: { type: 'string' },
        source: { type: 'string' },
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'get_logs_by_context',
    description: 'Return logs filtered by context.',
    inputSchema: {
      type: 'object',
      required: ['context'],
      properties: {
        scope: { type: 'string' },
        context: { type: 'string' },
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'query_logs',
    description: 'Flexible bounded log query.',
    inputSchema: {
      type: 'object',
      properties: {
        scope: { type: 'string' },
        level: { type: 'string' },
        source: { type: 'string' },
        context: { type: 'string' },
        runId: { type: 'string' },
        contains: { type: 'string' },
        from: { type: 'string' },
        to: { type: 'string' },
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'get_log_stats',
    description: 'Return local log and evidence statistics.',
    inputSchema: {
      type: 'object',
      properties: {
        scope: { type: 'string' },
        from: { type: 'string' },
        to: { type: 'string' },
      },
    },
  },
  {
    name: 'get_latest_failures',
    description: 'Return compact failed validation rows.',
    inputSchema: {
      type: 'object',
      properties: {
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'get_run_diagnostics',
    description: 'Return diagnostics for one run id.',
    inputSchema: {
      type: 'object',
      required: ['runId'],
      properties: {
        runId: { type: 'string' },
        includeArtifactRefs: { type: 'boolean' },
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'get_artifact_slice',
    description: 'Return a bounded line slice from a local artifact.',
    inputSchema: {
      type: 'object',
      properties: {
        artifactId: { type: 'string' },
        path: { type: 'string' },
        startLine: { type: 'number' },
        endLine: { type: 'number' },
        maxLines: { type: 'number' },
      },
    },
  },
  {
    name: 'get_proof_inventory_status',
    description: 'Return current logging proof-root presence and stale-claim gaps for logging-domain-parity.',
    inputSchema: {
      type: 'object',
      properties: {},
    },
  },
  {
    name: 'get_proof_trace',
    description: 'Return ordered proof-trace rows for one proof id.',
    inputSchema: {
      type: 'object',
      properties: {
        scope: { type: 'string' },
        proofId: { type: 'string' },
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'get_proof_trace_gaps',
    description: 'Validate proof-trace rows against ordered expected steps.',
    inputSchema: {
      type: 'object',
      properties: {
        scope: { type: 'string' },
        proofId: { type: 'string' },
        expectedSteps: { type: 'array' },
        limit: { type: 'number' },
      },
    },
  },
  {
    name: 'query_proof_trace',
    description: 'Return a proof trace or proof-trace gap analysis when expected steps are provided.',
    inputSchema: {
      type: 'object',
      properties: {
        scope: { type: 'string' },
        proofId: { type: 'string' },
        expectedSteps: { type: 'array' },
        limit: { type: 'number' },
      },
    },
  },
];

const proofTraceSmokeScript = fileURLToPath(new URL('./logging-proof-trace-smoke.mjs', import.meta.url));

function optionValue(argv, flag) {
  const exactIndex = argv.indexOf(flag);
  if (exactIndex !== -1) {
    return argv[exactIndex + 1] ?? null;
  }
  const prefix = `${flag}=`;
  const inline = argv.find((value) => value.startsWith(prefix));
  return inline == null ? null : inline.slice(prefix.length);
}

function runProofTraceSmoke(argv) {
  const smokeRoot = optionValue(argv, '--smoke-root');
  const childArgs = ['--import', 'tsx', proofTraceSmokeScript];
  if (smokeRoot != null && smokeRoot.trim().length > 0) {
    childArgs.push(`--root=${smokeRoot}`, '--keep-root');
  }

  const result = spawnSync(process.execPath, childArgs, {
    cwd: process.cwd(),
    env: process.env,
    encoding: 'utf8',
    windowsHide: true,
  });
  if (result.status !== 0) {
    throw new Error(`proof-trace smoke failed\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`.trim());
  }

  const output = result.stdout.trim();
  if (output.length === 0) {
    throw new Error('proof-trace smoke did not produce any output.');
  }
  return JSON.parse(output);
}

async function callTool(name, argumentsValue = {}) {
  switch (name) {
    case 'get_errors':
      return getErrors(argumentsValue);
    case 'get_recent_logs':
      return getRecentLogs(argumentsValue);
    case 'get_logs_by_source':
      return getLogsBySource(argumentsValue);
    case 'get_logs_by_context':
      return getLogsByContext(argumentsValue);
    case 'query_logs':
      return queryLogs(argumentsValue);
    case 'get_log_stats':
      return getLogStats(argumentsValue);
    case 'get_latest_failures':
      return getLatestFailures(argumentsValue);
    case 'get_run_diagnostics':
      return getRunDiagnostics(argumentsValue);
    case 'get_artifact_slice':
      return getArtifactSlice(argumentsValue);
    case 'get_proof_inventory_status':
      return getProofInventoryStatus(argumentsValue);
    case 'get_proof_trace':
      return getProofTrace(argumentsValue);
    case 'get_proof_trace_gaps':
      return getProofTraceGaps(argumentsValue);
    case 'query_proof_trace':
      return queryProofTrace(argumentsValue);
    default:
      throw new Error(`Unknown tool: ${name}`);
  }
}

async function runCli(argv) {
  if (argv.includes('--list-tools')) {
    process.stdout.write(`${JSON.stringify(TOOLS, null, 2)}\n`);
    return true;
  }

  const smokeIndex = argv.indexOf('--smoke');
  if (smokeIndex !== -1) {
    const target = argv[smokeIndex + 1] ?? 'latest-failures';
    if (target === 'latest-failures') {
      process.stdout.write(`${JSON.stringify(await callTool('get_latest_failures', { limit: 1 }), null, 2)}\n`);
      return true;
    }
    if (target === 'run-diagnostics') {
      const latest = await callTool('get_latest_failures', { limit: 1 });
      const runId = latest[0]?.runId;
      if (runId == null) {
        throw new Error('No failed run available for run-diagnostics smoke.');
      }
      process.stdout.write(
        `${JSON.stringify(await callTool('get_run_diagnostics', { runId, includeArtifactRefs: true, limit: 20 }), null, 2)}\n`
      );
      return true;
    }
    if (target === 'artifact-slice') {
      const latest = await callTool('get_latest_failures', { limit: 1 });
      const runId = latest[0]?.runId;
      if (runId == null) {
        throw new Error('No failed run available for artifact-slice smoke.');
      }
      const evidence = await callTool('get_run_diagnostics', {
        runId,
        includeArtifactRefs: true,
        limit: 20,
      });
      const stderrArtifact = evidence.artifacts.find((artifact) => artifact.kind === 'stderr');
      if (stderrArtifact == null) {
        throw new Error('No stderr artifact available for artifact-slice smoke.');
      }
      process.stdout.write(
        `${JSON.stringify(await callTool('get_artifact_slice', { path: stderrArtifact.path, startLine: 1, maxLines: 20 }), null, 2)}\n`
      );
      return true;
    }
    if (target === 'proof-trace') {
      process.stdout.write(`${JSON.stringify(runProofTraceSmoke(argv), null, 2)}\n`);
      return true;
    }
    if (target === 'proof-inventory') {
      process.stdout.write(`${JSON.stringify(await callTool('get_proof_inventory_status', {}), null, 2)}\n`);
      return true;
    }
    throw new Error(`Unknown smoke target: ${target}`);
  }

  return false;
}

function encodeMessage(payload) {
  const body = JSON.stringify(payload);
  return `Content-Length: ${Buffer.byteLength(body, 'utf8')}\r\n\r\n${body}`;
}

async function handleRequest(message) {
  if (message.method === 'initialize') {
    return {
      jsonrpc: '2.0',
      id: message.id,
      result: {
        protocolVersion: '2024-11-05',
        capabilities: {
          tools: {},
        },
        serverInfo: {
          name: 'ocentra-parent-logging',
          version: '0.1.1',
        },
      },
    };
  }

  if (message.method === 'tools/list') {
    return {
      jsonrpc: '2.0',
      id: message.id,
      result: {
        tools: TOOLS,
      },
    };
  }

  if (message.method === 'tools/call') {
    try {
      const result = await callTool(message.params.name, message.params.arguments ?? {});
      return {
        jsonrpc: '2.0',
        id: message.id,
        result: {
          content: [
            {
              type: 'text',
              text: JSON.stringify(result, null, 2),
            },
          ],
          structuredContent: result,
        },
      };
    } catch (error) {
      return {
        jsonrpc: '2.0',
        id: message.id,
        error: {
          code: -32000,
          message: error.message,
        },
      };
    }
  }

  return {
    jsonrpc: '2.0',
    id: message.id,
    error: {
      code: -32601,
      message: `Unknown method: ${message.method}`,
    },
  };
}

async function runServer() {
  process.stdin.setEncoding('utf8');
  let buffer = '';

  process.stdin.on('data', async (chunk) => {
    buffer += chunk;
    while (true) {
      const headerEnd = buffer.indexOf('\r\n\r\n');
      if (headerEnd === -1) {
        break;
      }

      const header = buffer.slice(0, headerEnd);
      const lengthMatch = header.match(/Content-Length:\s*(\d+)/i);
      if (lengthMatch == null) {
        buffer = '';
        break;
      }

      const contentLength = Number(lengthMatch[1]);
      const messageStart = headerEnd + 4;
      if (buffer.length < messageStart + contentLength) {
        break;
      }

      const body = buffer.slice(messageStart, messageStart + contentLength);
      buffer = buffer.slice(messageStart + contentLength);
      const request = JSON.parse(body);
      if (request.id == null) {
        continue;
      }
      const response = await handleRequest(request);
      process.stdout.write(encodeMessage(response));
    }
  });
}

async function main() {
  const argv = process.argv.slice(2);
  if (await runCli(argv)) {
    return;
  }
  await runServer();
}

void main();
