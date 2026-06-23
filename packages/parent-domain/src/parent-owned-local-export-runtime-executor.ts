/// <reference types="node" />

import { createCipheriv, createHash, randomBytes } from 'node:crypto';
import { appendFile, mkdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import {
  ParentEvidenceReferenceSchema,
  type ParentEvidenceReference,
} from '@ocentra-parent/schema-domain/family-references';
import type { StackTrace } from '@ocentra-parent/schema-domain/logging-contracts';
import {
  ParentOwnedLocalExportRuntimeDeleteReceiptSchema,
  ParentOwnedLocalExportRuntimeJobSchema,
  ParentOwnedLocalExportRuntimeOutputSchema,
  ParentOwnedLocalExportRuntimeScopeSchema,
  type ParentOwnedLocalExportRuntimeJob,
  type ParentOwnedLocalExportRuntimeOutput,
  type ParentOwnedLocalExportRuntimeScope,
} from '@ocentra-parent/schema-domain/parent-owned-local-export-runtime';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';

const log = Logger.instance;
const moduleUrl =
  (import.meta as ImportMeta & { readonly url?: string }).url ??
  'packages/parent-domain/src/parent-owned-local-export-runtime-executor.ts';
log.register(moduleUrl);

const logInfo = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  log.logInfo(message, stackTrace, data, enabled);
};

const logWarn = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  log.logWarn(message, stackTrace, data, enabled);
};

const logError = (message: string, stackTrace: StackTrace, data?: unknown): void => {
  log.logError(message, stackTrace, data);
};

type JsonCompatible =
  | null
  | boolean
  | number
  | string
  | readonly JsonCompatible[]
  | { readonly [key: string]: JsonCompatible };

export interface ParentOwnedLocalExportRuntimeExecutorOptions {
  readonly runtimeRoot: string;
  readonly encryptionSecret: string | Uint8Array;
  readonly now?: () => Date;
  readonly loggingEnabled?: boolean;
}

export interface ParentOwnedLocalExportRuntimeExportRequest {
  readonly scope: ParentOwnedLocalExportRuntimeScope;
  readonly payload: JsonCompatible | Uint8Array | string;
  readonly sourceEvidenceRefs?: readonly ParentEvidenceReference[];
  readonly auditRefs?: readonly ParentEvidenceReference[];
  readonly jobId?: string;
  readonly queueRef?: string;
  readonly requestedAt?: string;
}

export interface ParentOwnedLocalExportRuntimeDeleteRequest {
  readonly scope: ParentOwnedLocalExportRuntimeScope;
  readonly output: ParentOwnedLocalExportRuntimeOutput;
  readonly auditRefs?: readonly ParentEvidenceReference[];
  readonly jobId?: string;
  readonly queueRef?: string;
  readonly requestedAt?: string;
}

export interface ParentOwnedLocalExportRuntimeAuditEntry {
  readonly schemaVersion: 1;
  readonly operation: 'export' | 'delete';
  readonly jobId: string;
  readonly state: string;
  readonly bundleRef: string;
  readonly outputRef: string | null;
  readonly queueRef: string;
  readonly auditState: 'audit-recorded' | 'audit-pending' | 'manual-audit-required';
  readonly recordedAt: string;
  readonly bytesWritten: number | null;
  readonly failureReasonRef: string | null;
  readonly sourceEvidenceReferenceIds: readonly string[];
}

export interface ParentOwnedLocalExportRuntimeExecutionResult {
  readonly job: ParentOwnedLocalExportRuntimeJob;
  readonly bundlePath: string;
  readonly outputPath: string;
  readonly auditLogPath: string;
  readonly auditEntries: readonly ParentOwnedLocalExportRuntimeAuditEntry[];
}

export interface ParentOwnedLocalExportRuntimeExecutor {
  readonly runtimeRoot: string;
  executeExport(
    request: ParentOwnedLocalExportRuntimeExportRequest
  ): Promise<ParentOwnedLocalExportRuntimeExecutionResult>;
  executeDelete(
    request: ParentOwnedLocalExportRuntimeDeleteRequest
  ): Promise<ParentOwnedLocalExportRuntimeExecutionResult>;
  readAuditEntries(): Promise<readonly ParentOwnedLocalExportRuntimeAuditEntry[]>;
}

interface ExportArtifactEnvelope {
  readonly schemaVersion: 1;
  readonly bundleRef: string;
  readonly scopeFamilyId: string;
  readonly scopeDeviceId: string;
  readonly dataClasses: readonly string[];
  readonly cipherAlgorithm: 'aes-256-gcm';
  readonly initializationVectorBase64: string;
  readonly authTagBase64: string;
  readonly ciphertextBase64: string;
  readonly createdAt: string;
}

interface ParentOwnedLocalExportRuntimeExecutionContext {
  readonly runtimeRoot: string;
  readonly auditLogPath: string;
  readonly encryptionSecret: string | Uint8Array;
  readonly now: () => Date;
  readonly loggingEnabled: boolean;
}

interface PreparedExportExecution {
  readonly startedAt: string;
  readonly scope: ParentOwnedLocalExportRuntimeScope;
  readonly sourceEvidenceRefs: readonly ParentEvidenceReference[];
  readonly auditRefs: readonly ParentEvidenceReference[];
  readonly jobId: string;
  readonly bundleRef: string;
  readonly outputRef: string;
  readonly queueRef: string;
}

interface WrittenExportArtifacts {
  readonly bundlePath: string;
  readonly outputPath: string;
  readonly writtenAt: string;
  readonly output: ParentOwnedLocalExportRuntimeOutput;
  readonly ciphertextByteLength: number;
}

interface PreparedDeleteExecution {
  readonly startedAt: string;
  readonly scope: ParentOwnedLocalExportRuntimeScope;
  readonly output: ParentOwnedLocalExportRuntimeOutput;
  readonly auditRefs: readonly ParentEvidenceReference[];
  readonly jobId: string;
  readonly queueRef: string;
  readonly bundlePath: string;
  readonly outputPath: string;
}

interface DeleteTargetState {
  readonly bundleExists: boolean;
  readonly outputExists: boolean;
  readonly updatedAt: string;
  readonly auditState: ParentOwnedLocalExportRuntimeAuditEntry['auditState'];
}

const BundlesDirectory = 'bundles';
const OutputsDirectory = 'outputs';
const AuditDirectory = 'audit';
const AuditLogFileName = 'parent-owned-local-export-runtime.ndjson';

export function createParentOwnedLocalExportRuntimeExecutor(
  options: ParentOwnedLocalExportRuntimeExecutorOptions
): ParentOwnedLocalExportRuntimeExecutor {
  const context = createExecutionContext(options);

  return {
    runtimeRoot: context.runtimeRoot,
    executeExport(request) {
      return executeExport(context, request);
    },
    executeDelete(request) {
      return executeDelete(context, request);
    },
    async readAuditEntries() {
      return readAuditEntriesFromPath(context.auditLogPath);
    },
  };
}

function createExecutionContext(
  options: ParentOwnedLocalExportRuntimeExecutorOptions
): ParentOwnedLocalExportRuntimeExecutionContext {
  return {
    runtimeRoot: options.runtimeRoot,
    auditLogPath: join(options.runtimeRoot, AuditDirectory, AuditLogFileName),
    encryptionSecret: options.encryptionSecret,
    now: options.now ?? (() => new Date()),
    loggingEnabled: options.loggingEnabled ?? false,
  };
}

function prepareExportExecution(
  context: ParentOwnedLocalExportRuntimeExecutionContext,
  request: ParentOwnedLocalExportRuntimeExportRequest
): PreparedExportExecution {
  const startedAt = isoTimestamp(context.now);
  const scope = ParentOwnedLocalExportRuntimeScopeSchema.parse(request.scope);
  const jobId = request.jobId ?? `local-export-runtime-job-${safeToken(scope.device.deviceId)}-${startedAt}`;
  return {
    startedAt,
    scope,
    sourceEvidenceRefs: parseEvidenceRefs(request.sourceEvidenceRefs, startedAt, 'source'),
    auditRefs: parseEvidenceRefs(request.auditRefs, startedAt, 'audit'),
    jobId,
    bundleRef: `local-export-bundle-${safeToken(jobId)}`,
    outputRef: `local-export-output-${safeToken(jobId)}`,
    queueRef: request.queueRef ?? `local-export-runtime-queue-${safeToken(jobId)}`,
  };
}

async function executeExport(
  context: ParentOwnedLocalExportRuntimeExecutionContext,
  request: ParentOwnedLocalExportRuntimeExportRequest
): Promise<ParentOwnedLocalExportRuntimeExecutionResult> {
  const execution = prepareExportExecution(context, request);
  logInfo(
    'Executing parent-owned local export runtime export',
    getStackTrace(),
    {
      jobId: execution.jobId,
      queueRef: execution.queueRef,
      familyId: execution.scope.family.familyId,
      deviceId: execution.scope.device.deviceId,
      outputRef: execution.outputRef,
    },
    context.loggingEnabled
  );

  try {
    const artifacts = await writeExportArtifacts(context, execution, request.payload);
    const job = buildExportJob(execution, request, artifacts);
    const auditEntries = await appendAndReadAuditEntries(
      context.auditLogPath,
      buildAuditEntry({
        operation: 'export',
        jobId: execution.jobId,
        state: job.state,
        bundleRef: execution.bundleRef,
        outputRef: execution.outputRef,
        queueRef: execution.queueRef,
        auditState: 'audit-recorded',
        recordedAt: artifacts.writtenAt,
        bytesWritten: artifacts.ciphertextByteLength,
        failureReasonRef: null,
        sourceEvidenceRefs: execution.sourceEvidenceRefs,
      })
    );

    logInfo(
      'Parent-owned local export runtime export finished',
      getStackTrace(),
      {
        jobId: execution.jobId,
        outputRef: execution.outputRef,
        bundleRef: execution.bundleRef,
        auditEntries: auditEntries.length,
      },
      context.loggingEnabled
    );
    return buildExecutionResult(job, artifacts.bundlePath, artifacts.outputPath, context.auditLogPath, auditEntries);
  } catch (error) {
    logError(
      'Parent-owned local export runtime export failed',
      getStackTrace(),
      error instanceof Error ? { message: error.message } : { error: String(error) }
    );
    throw error;
  }
}

async function writeExportArtifacts(
  context: ParentOwnedLocalExportRuntimeExecutionContext,
  execution: PreparedExportExecution,
  payload: JsonCompatible | Uint8Array | string
): Promise<WrittenExportArtifacts> {
  await mkdir(join(context.runtimeRoot, BundlesDirectory), { recursive: true });
  await mkdir(join(context.runtimeRoot, OutputsDirectory), { recursive: true });
  await mkdir(join(context.runtimeRoot, AuditDirectory), { recursive: true });

  const payloadBuffer = normalizePayload(payload);
  const encryptionKey = deriveEncryptionKey(context.encryptionSecret);
  const initializationVector = randomBytes(12);
  const cipher = createCipheriv('aes-256-gcm', encryptionKey, initializationVector);
  const ciphertext = Buffer.concat([cipher.update(payloadBuffer), cipher.final()]);
  const authTag = cipher.getAuthTag();
  const checksum = createHash('sha256').update(payloadBuffer).digest('hex');
  const bundlePath = bundlePathFor(context.runtimeRoot, execution.bundleRef);
  const outputPath = outputPathFor(context.runtimeRoot, execution.outputRef);
  const writtenAt = isoTimestamp(context.now);

  const bundleEnvelope: ExportArtifactEnvelope = {
    schemaVersion: 1,
    bundleRef: execution.bundleRef,
    scopeFamilyId: execution.scope.family.familyId,
    scopeDeviceId: execution.scope.device.deviceId,
    dataClasses: [...execution.scope.requestedDataClasses],
    cipherAlgorithm: 'aes-256-gcm',
    initializationVectorBase64: initializationVector.toString('base64'),
    authTagBase64: authTag.toString('base64'),
    ciphertextBase64: ciphertext.toString('base64'),
    createdAt: writtenAt,
  };
  await writeFile(bundlePath, `${JSON.stringify(bundleEnvelope, null, 2)}\n`);

  const output = ParentOwnedLocalExportRuntimeOutputSchema.parse({
    bundleRef: execution.bundleRef,
    outputRef: execution.outputRef,
    outputFormat: execution.scope.outputFormat,
    destinationOwnership: execution.scope.destinationOwnership,
    encryptedAtRest: true,
    schemaVersionLabel: 'parent-owned-local-export-runtime-schema-v2',
    byteCountRange: byteCountRangeLabel(payloadBuffer.byteLength),
    checksumRef: `sha256-${checksum}`,
    createdAt: writtenAt,
    sourceEvidenceRefs: execution.sourceEvidenceRefs,
    childDetailMinimized: true,
    rawEvidenceIncludedByDefault: false,
    ocentraHostedCopyRetained: false,
  });
  await writeOutputArtifact(outputPath, execution.outputRef, execution.bundleRef, output, execution.sourceEvidenceRefs);

  return {
    bundlePath,
    outputPath,
    writtenAt,
    output,
    ciphertextByteLength: ciphertext.byteLength,
  };
}

async function writeOutputArtifact(
  outputPath: string,
  outputRef: string,
  bundleRef: string,
  output: ParentOwnedLocalExportRuntimeOutput,
  sourceEvidenceRefs: readonly ParentEvidenceReference[]
): Promise<void> {
  await writeFile(
    outputPath,
    `${JSON.stringify(
      {
        schemaVersion: 1,
        outputRef,
        bundleRef,
        checksumRef: output.checksumRef,
        byteCountRange: output.byteCountRange,
        createdAt: output.createdAt,
        sourceEvidenceReferenceIds: sourceEvidenceRefs.map((reference) => reference.evidenceReferenceId),
      },
      null,
      2
    )}\n`
  );
}

function buildExportJob(
  execution: PreparedExportExecution,
  request: ParentOwnedLocalExportRuntimeExportRequest,
  artifacts: WrittenExportArtifacts
): ParentOwnedLocalExportRuntimeJob {
  return ParentOwnedLocalExportRuntimeJobSchema.parse({
    jobId: execution.jobId,
    operation: 'export',
    state: 'export-written',
    queueRef: execution.queueRef,
    storageState: 'local-folder-ready',
    scope: execution.scope,
    output: artifacts.output,
    deleteReceipt: null,
    queuedAt: request.requestedAt ?? execution.startedAt,
    updatedAt: artifacts.writtenAt,
    auditRefs: execution.auditRefs,
    localEvidenceMutated: false,
    parentOwnedOutputMutatedByFailure: false,
    localSafetyStatePreserved: true,
    manualActionRequired: false,
  });
}

function prepareDeleteExecution(
  context: ParentOwnedLocalExportRuntimeExecutionContext,
  request: ParentOwnedLocalExportRuntimeDeleteRequest
): PreparedDeleteExecution {
  const startedAt = isoTimestamp(context.now);
  const output = ParentOwnedLocalExportRuntimeOutputSchema.parse(request.output);
  const jobId = request.jobId ?? `local-delete-runtime-job-${safeToken(output.bundleRef)}-${startedAt}`;
  return {
    startedAt,
    scope: ParentOwnedLocalExportRuntimeScopeSchema.parse(request.scope),
    output,
    auditRefs: parseEvidenceRefs(request.auditRefs, startedAt, 'delete-audit'),
    jobId,
    queueRef: request.queueRef ?? `local-export-runtime-queue-${safeToken(jobId)}`,
    bundlePath: bundlePathFor(context.runtimeRoot, output.bundleRef),
    outputPath: outputPathFor(context.runtimeRoot, output.outputRef),
  };
}

async function executeDelete(
  context: ParentOwnedLocalExportRuntimeExecutionContext,
  request: ParentOwnedLocalExportRuntimeDeleteRequest
): Promise<ParentOwnedLocalExportRuntimeExecutionResult> {
  const execution = prepareDeleteExecution(context, request);
  logInfo(
    'Executing parent-owned local export runtime delete',
    getStackTrace(),
    {
      jobId: execution.jobId,
      queueRef: execution.queueRef,
      familyId: execution.scope.family.familyId,
      deviceId: execution.scope.device.deviceId,
      outputRef: execution.output.outputRef,
    },
    context.loggingEnabled
  );

  try {
    await mkdir(join(context.runtimeRoot, AuditDirectory), { recursive: true });
    const targetState = await inspectDeleteTargetState(context.now, execution);
    if (!targetState.bundleExists || !targetState.outputExists) {
      return handleMissingDeleteTarget(context, execution, request, targetState);
    }
    return handleConfirmedDelete(context, execution, request, targetState);
  } catch (error) {
    logError(
      'Parent-owned local export runtime delete failed',
      getStackTrace(),
      error instanceof Error ? { message: error.message } : { error: String(error) }
    );
    throw error;
  }
}

async function inspectDeleteTargetState(
  now: () => Date,
  execution: PreparedDeleteExecution
): Promise<DeleteTargetState> {
  const bundleExists = await fileExists(execution.bundlePath);
  const outputExists = await fileExists(execution.outputPath);
  return {
    bundleExists,
    outputExists,
    updatedAt: isoTimestamp(now),
    auditState: bundleExists && outputExists ? 'audit-recorded' : 'manual-audit-required',
  };
}

async function handleMissingDeleteTarget(
  context: ParentOwnedLocalExportRuntimeExecutionContext,
  execution: PreparedDeleteExecution,
  request: ParentOwnedLocalExportRuntimeDeleteRequest,
  targetState: DeleteTargetState
): Promise<ParentOwnedLocalExportRuntimeExecutionResult> {
  logWarn(
    'Delete target missing for parent-owned local export runtime',
    getStackTrace(),
    {
      jobId: execution.jobId,
      bundleExists: targetState.bundleExists,
      outputExists: targetState.outputExists,
      bundleRef: execution.output.bundleRef,
      outputRef: execution.output.outputRef,
    },
    context.loggingEnabled
  );

  const deleteReceipt = buildDeleteReceipt(execution, request, targetState, false, 'delete-target-missing');
  const job = buildDeleteJob(
    execution,
    request,
    targetState,
    deleteReceipt,
    'delete-failed',
    'delete-target-missing',
    true
  );
  const auditEntries = await appendAndReadAuditEntries(
    context.auditLogPath,
    buildAuditEntry({
      operation: 'delete',
      jobId: execution.jobId,
      state: job.state,
      bundleRef: execution.output.bundleRef,
      outputRef: execution.output.outputRef,
      queueRef: execution.queueRef,
      auditState: targetState.auditState,
      recordedAt: targetState.updatedAt,
      bytesWritten: null,
      failureReasonRef: 'delete-target-missing',
      sourceEvidenceRefs: execution.output.sourceEvidenceRefs,
    })
  );
  return buildExecutionResult(job, execution.bundlePath, execution.outputPath, context.auditLogPath, auditEntries);
}

async function handleConfirmedDelete(
  context: ParentOwnedLocalExportRuntimeExecutionContext,
  execution: PreparedDeleteExecution,
  request: ParentOwnedLocalExportRuntimeDeleteRequest,
  targetState: DeleteTargetState
): Promise<ParentOwnedLocalExportRuntimeExecutionResult> {
  await rm(execution.bundlePath, { force: true });
  await rm(execution.outputPath, { force: true });

  const deleteReceipt = buildDeleteReceipt(execution, request, targetState, true, null);
  const job = buildDeleteJob(
    execution,
    request,
    targetState,
    deleteReceipt,
    'delete-confirmed',
    'local-folder-ready',
    false
  );
  const auditEntries = await appendAndReadAuditEntries(
    context.auditLogPath,
    buildAuditEntry({
      operation: 'delete',
      jobId: execution.jobId,
      state: job.state,
      bundleRef: execution.output.bundleRef,
      outputRef: execution.output.outputRef,
      queueRef: execution.queueRef,
      auditState: targetState.auditState,
      recordedAt: targetState.updatedAt,
      bytesWritten: null,
      failureReasonRef: null,
      sourceEvidenceRefs: execution.output.sourceEvidenceRefs,
    })
  );

  logInfo(
    'Parent-owned local export runtime delete finished',
    getStackTrace(),
    {
      jobId: execution.jobId,
      bundleRef: execution.output.bundleRef,
      auditEntries: auditEntries.length,
    },
    context.loggingEnabled
  );
  return buildExecutionResult(job, execution.bundlePath, execution.outputPath, context.auditLogPath, auditEntries);
}

function buildDeleteReceipt(
  execution: PreparedDeleteExecution,
  request: ParentOwnedLocalExportRuntimeDeleteRequest,
  targetState: DeleteTargetState,
  deleteConfirmed: boolean,
  failureReasonRef: string | null
) {
  return ParentOwnedLocalExportRuntimeDeleteReceiptSchema.parse({
    deleteRequestRef: `local-export-delete-request-${safeToken(execution.jobId)}`,
    targetBundleRef: execution.output.bundleRef,
    requestedAt: request.requestedAt ?? execution.startedAt,
    deletedAt: deleteConfirmed ? targetState.updatedAt : null,
    deleteConfirmed,
    auditState: targetState.auditState,
    sourceEvidenceRetained: false,
    exportedOutputDeleted: deleteConfirmed,
    localSafetyStatePreserved: true,
    failureReasonRef,
  });
}

function buildDeleteJob(
  execution: PreparedDeleteExecution,
  request: ParentOwnedLocalExportRuntimeDeleteRequest,
  targetState: DeleteTargetState,
  deleteReceipt: ReturnType<typeof buildDeleteReceipt>,
  state: 'delete-failed' | 'delete-confirmed',
  storageState: 'delete-target-missing' | 'local-folder-ready',
  manualActionRequired: boolean
): ParentOwnedLocalExportRuntimeJob {
  return ParentOwnedLocalExportRuntimeJobSchema.parse({
    jobId: execution.jobId,
    operation: 'delete',
    state,
    queueRef: execution.queueRef,
    storageState,
    scope: execution.scope,
    output: execution.output,
    deleteReceipt,
    queuedAt: request.requestedAt ?? execution.startedAt,
    updatedAt: targetState.updatedAt,
    auditRefs: execution.auditRefs,
    localEvidenceMutated: false,
    parentOwnedOutputMutatedByFailure: false,
    localSafetyStatePreserved: true,
    manualActionRequired,
  });
}

async function appendAndReadAuditEntries(
  auditLogPath: string,
  auditEntry: ParentOwnedLocalExportRuntimeAuditEntry
): Promise<readonly ParentOwnedLocalExportRuntimeAuditEntry[]> {
  await appendAuditEntry(auditLogPath, auditEntry);
  return readAuditEntriesFromPath(auditLogPath);
}

function buildExecutionResult(
  job: ParentOwnedLocalExportRuntimeJob,
  bundlePath: string,
  outputPath: string,
  auditLogPath: string,
  auditEntries: readonly ParentOwnedLocalExportRuntimeAuditEntry[]
): ParentOwnedLocalExportRuntimeExecutionResult {
  return {
    job,
    bundlePath,
    outputPath,
    auditLogPath,
    auditEntries,
  };
}

function normalizePayload(payload: JsonCompatible | Uint8Array | string): Buffer {
  if (payload instanceof Uint8Array) {
    return Buffer.from(payload);
  }
  if (typeof payload === 'string') {
    return Buffer.from(payload, 'utf8');
  }
  return Buffer.from(JSON.stringify(payload), 'utf8');
}

function deriveEncryptionKey(secret: string | Uint8Array): Buffer {
  return createHash('sha256').update(secret).digest();
}

function bundlePathFor(runtimeRoot: string, bundleRef: string): string {
  return join(runtimeRoot, BundlesDirectory, `${safeToken(bundleRef)}.json`);
}

function outputPathFor(runtimeRoot: string, outputRef: string): string {
  return join(runtimeRoot, OutputsDirectory, `${safeToken(outputRef)}.json`);
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9._-]/g, '-');
}

function byteCountRangeLabel(byteCount: number): string {
  if (byteCount <= 1024) {
    return 'size-range-0-1kb';
  }
  if (byteCount <= 10 * 1024) {
    return 'size-range-1-10kb';
  }
  return 'size-range-over-10kb';
}

function isoTimestamp(now: () => Date): string {
  return now().toISOString();
}

function parseEvidenceRefs(
  input: readonly ParentEvidenceReference[] | undefined,
  observedAt: string,
  label: string
): readonly ParentEvidenceReference[] {
  if (input != null && input.length > 0) {
    return input.map((reference) => ParentEvidenceReferenceSchema.parse(reference));
  }
  return [
    ParentEvidenceReferenceSchema.parse({
      evidenceReferenceId: `${label}-proof-${observedAt}`,
      kind: 'journal-event',
      observedAt,
    }),
  ];
}

function buildAuditEntry(input: {
  readonly operation: 'export' | 'delete';
  readonly jobId: string;
  readonly state: string;
  readonly bundleRef: string;
  readonly outputRef: string | null;
  readonly queueRef: string;
  readonly auditState: 'audit-recorded' | 'audit-pending' | 'manual-audit-required';
  readonly recordedAt: string;
  readonly bytesWritten: number | null;
  readonly failureReasonRef: string | null;
  readonly sourceEvidenceRefs: readonly ParentEvidenceReference[];
}): ParentOwnedLocalExportRuntimeAuditEntry {
  return {
    schemaVersion: 1,
    operation: input.operation,
    jobId: input.jobId,
    state: input.state,
    bundleRef: input.bundleRef,
    outputRef: input.outputRef,
    queueRef: input.queueRef,
    auditState: input.auditState,
    recordedAt: input.recordedAt,
    bytesWritten: input.bytesWritten,
    failureReasonRef: input.failureReasonRef,
    sourceEvidenceReferenceIds: input.sourceEvidenceRefs.map((reference) => reference.evidenceReferenceId),
  };
}

async function appendAuditEntry(auditLogPath: string, entry: ParentOwnedLocalExportRuntimeAuditEntry): Promise<void> {
  await appendFile(auditLogPath, `${JSON.stringify(entry)}\n`);
}

async function readAuditEntriesFromPath(
  auditLogPath: string
): Promise<readonly ParentOwnedLocalExportRuntimeAuditEntry[]> {
  if (!(await fileExists(auditLogPath))) {
    return [];
  }
  const fileContents = await readFile(auditLogPath, 'utf8');
  return fileContents
    .split(/\r?\n/)
    .filter((line: string) => line.trim().length > 0)
    .map((line: string) => JSON.parse(line) as ParentOwnedLocalExportRuntimeAuditEntry);
}

async function fileExists(path: string): Promise<boolean> {
  try {
    await stat(path);
    return true;
  } catch {
    return false;
  }
}
