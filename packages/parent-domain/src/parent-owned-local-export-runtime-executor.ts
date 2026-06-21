/// <reference types="node" />

import { createCipheriv, createHash, randomBytes } from 'node:crypto';
import { appendFile, mkdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace, type StackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import { ParentEvidenceReferenceSchema, type ParentEvidenceReference } from '@ocentra-parent/schema-domain/family-references';
import {
  ParentOwnedLocalExportRuntimeDeleteReceiptSchema,
  ParentOwnedLocalExportRuntimeJobSchema,
  ParentOwnedLocalExportRuntimeOutputSchema,
  ParentOwnedLocalExportRuntimeScopeSchema,
  type ParentOwnedLocalExportRuntimeJob,
  type ParentOwnedLocalExportRuntimeOutput,
  type ParentOwnedLocalExportRuntimeScope,
} from '@ocentra-parent/schema-domain/parent-owned-local-export-runtime';

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

type JsonCompatible = null | boolean | number | string | readonly JsonCompatible[] | { readonly [key: string]: JsonCompatible };

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
  executeExport(request: ParentOwnedLocalExportRuntimeExportRequest): Promise<ParentOwnedLocalExportRuntimeExecutionResult>;
  executeDelete(request: ParentOwnedLocalExportRuntimeDeleteRequest): Promise<ParentOwnedLocalExportRuntimeExecutionResult>;
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

const BundlesDirectory = 'bundles';
const OutputsDirectory = 'outputs';
const AuditDirectory = 'audit';
const AuditLogFileName = 'parent-owned-local-export-runtime.ndjson';

export function createParentOwnedLocalExportRuntimeExecutor(
  options: ParentOwnedLocalExportRuntimeExecutorOptions
): ParentOwnedLocalExportRuntimeExecutor {
  const now = options.now ?? (() => new Date());
  const loggingEnabled = options.loggingEnabled ?? false;
  const runtimeRoot = options.runtimeRoot;
  const auditLogPath = join(runtimeRoot, AuditDirectory, AuditLogFileName);

  return {
    runtimeRoot,
    async executeExport(request) {
      const startedAt = isoTimestamp(now);
      const scope = ParentOwnedLocalExportRuntimeScopeSchema.parse(request.scope);
      const sourceEvidenceRefs = parseEvidenceRefs(request.sourceEvidenceRefs, startedAt, 'source');
      const auditRefs = parseEvidenceRefs(request.auditRefs, startedAt, 'audit');
      const jobId = request.jobId ?? `local-export-runtime-job-${safeToken(scope.device.deviceId)}-${startedAt}`;
      const bundleRef = `local-export-bundle-${safeToken(jobId)}`;
      const outputRef = `local-export-output-${safeToken(jobId)}`;
      const queueRef = request.queueRef ?? `local-export-runtime-queue-${safeToken(jobId)}`;

      logInfo(
        'Executing parent-owned local export runtime export',
        getStackTrace(),
        {
          jobId,
          queueRef,
          familyId: scope.family.familyId,
          deviceId: scope.device.deviceId,
          outputRef,
        },
        loggingEnabled
      );

      try {
        await mkdir(join(runtimeRoot, BundlesDirectory), { recursive: true });
        await mkdir(join(runtimeRoot, OutputsDirectory), { recursive: true });
        await mkdir(join(runtimeRoot, AuditDirectory), { recursive: true });

        const payloadBuffer = normalizePayload(request.payload);
        const encryptionKey = deriveEncryptionKey(options.encryptionSecret);
        const initializationVector = randomBytes(12);
        const cipher = createCipheriv('aes-256-gcm', encryptionKey, initializationVector);
        const ciphertext = Buffer.concat([cipher.update(payloadBuffer), cipher.final()]);
        const authTag = cipher.getAuthTag();
        const checksum = createHash('sha256').update(payloadBuffer).digest('hex');
        const bundlePath = bundlePathFor(runtimeRoot, bundleRef);
        const outputPath = outputPathFor(runtimeRoot, outputRef);
        const writtenAt = isoTimestamp(now);

        const bundleEnvelope: ExportArtifactEnvelope = {
          schemaVersion: 1,
          bundleRef,
          scopeFamilyId: scope.family.familyId,
          scopeDeviceId: scope.device.deviceId,
          dataClasses: [...scope.requestedDataClasses],
          cipherAlgorithm: 'aes-256-gcm',
          initializationVectorBase64: initializationVector.toString('base64'),
          authTagBase64: authTag.toString('base64'),
          ciphertextBase64: ciphertext.toString('base64'),
          createdAt: writtenAt,
        };

        await writeFile(bundlePath, `${JSON.stringify(bundleEnvelope, null, 2)}\n`);

        const output = ParentOwnedLocalExportRuntimeOutputSchema.parse({
          bundleRef,
          outputRef,
          outputFormat: scope.outputFormat,
          destinationOwnership: scope.destinationOwnership,
          encryptedAtRest: true,
          schemaVersionLabel: 'parent-owned-local-export-runtime-schema-v2',
          byteCountRange: byteCountRangeLabel(payloadBuffer.byteLength),
          checksumRef: `sha256-${checksum}`,
          createdAt: writtenAt,
          sourceEvidenceRefs,
          childDetailMinimized: true,
          rawEvidenceIncludedByDefault: false,
          ocentraHostedCopyRetained: false,
        });

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

        const job = ParentOwnedLocalExportRuntimeJobSchema.parse({
          jobId,
          operation: 'export',
          state: 'export-written',
          queueRef,
          storageState: 'local-folder-ready',
          scope,
          output,
          deleteReceipt: null,
          queuedAt: request.requestedAt ?? startedAt,
          updatedAt: writtenAt,
          auditRefs,
          localEvidenceMutated: false,
          parentOwnedOutputMutatedByFailure: false,
          localSafetyStatePreserved: true,
          manualActionRequired: false,
        });

        const auditEntry = buildAuditEntry({
          operation: 'export',
          jobId,
          state: job.state,
          bundleRef,
          outputRef,
          queueRef,
          auditState: 'audit-recorded',
          recordedAt: writtenAt,
          bytesWritten: ciphertext.byteLength,
          failureReasonRef: null,
          sourceEvidenceRefs,
        });
        await appendAuditEntry(auditLogPath, auditEntry);
        const auditEntries = await readAuditEntriesFromPath(auditLogPath);

        logInfo(
          'Parent-owned local export runtime export finished',
          getStackTrace(),
          { jobId, outputRef, bundleRef, auditEntries: auditEntries.length },
          loggingEnabled
        );

        return {
          job,
          bundlePath,
          outputPath,
          auditLogPath,
          auditEntries,
        };
      } catch (error) {
        logError(
          'Parent-owned local export runtime export failed',
          getStackTrace(),
          error instanceof Error ? { message: error.message } : { error: String(error) }
        );
        throw error;
      }
    },
    async executeDelete(request) {
      const startedAt = isoTimestamp(now);
      const scope = ParentOwnedLocalExportRuntimeScopeSchema.parse(request.scope);
      const output = ParentOwnedLocalExportRuntimeOutputSchema.parse(request.output);
      const auditRefs = parseEvidenceRefs(request.auditRefs, startedAt, 'delete-audit');
      const jobId = request.jobId ?? `local-delete-runtime-job-${safeToken(output.bundleRef)}-${startedAt}`;
      const queueRef = request.queueRef ?? `local-export-runtime-queue-${safeToken(jobId)}`;
      const bundlePath = bundlePathFor(runtimeRoot, output.bundleRef);
      const outputPath = outputPathFor(runtimeRoot, output.outputRef);

      logInfo(
        'Executing parent-owned local export runtime delete',
        getStackTrace(),
        {
          jobId,
          queueRef,
          familyId: scope.family.familyId,
          deviceId: scope.device.deviceId,
          outputRef: output.outputRef,
        },
        loggingEnabled
      );

      try {
        await mkdir(join(runtimeRoot, AuditDirectory), { recursive: true });
        const bundleExists = await fileExists(bundlePath);
        const outputExists = await fileExists(outputPath);
        const updatedAt = isoTimestamp(now);
        const auditState = bundleExists && outputExists ? 'audit-recorded' : 'manual-audit-required';

        if (!bundleExists || !outputExists) {
          logWarn(
            'Delete target missing for parent-owned local export runtime',
            getStackTrace(),
            {
              jobId,
              bundleExists,
              outputExists,
              bundleRef: output.bundleRef,
              outputRef: output.outputRef,
            },
            loggingEnabled
          );

          const deleteReceipt = ParentOwnedLocalExportRuntimeDeleteReceiptSchema.parse({
            deleteRequestRef: `local-export-delete-request-${safeToken(jobId)}`,
            targetBundleRef: output.bundleRef,
            requestedAt: request.requestedAt ?? startedAt,
            deletedAt: null,
            deleteConfirmed: false,
            auditState,
            sourceEvidenceRetained: false,
            exportedOutputDeleted: false,
            localSafetyStatePreserved: true,
            failureReasonRef: 'delete-target-missing',
          });

          const job = ParentOwnedLocalExportRuntimeJobSchema.parse({
            jobId,
            operation: 'delete',
            state: 'delete-failed',
            queueRef,
            storageState: 'delete-target-missing',
            scope,
            output,
            deleteReceipt,
            queuedAt: request.requestedAt ?? startedAt,
            updatedAt,
            auditRefs,
            localEvidenceMutated: false,
            parentOwnedOutputMutatedByFailure: false,
            localSafetyStatePreserved: true,
            manualActionRequired: true,
          });

          const auditEntry = buildAuditEntry({
            operation: 'delete',
            jobId,
            state: job.state,
            bundleRef: output.bundleRef,
            outputRef: output.outputRef,
            queueRef,
            auditState,
            recordedAt: updatedAt,
            bytesWritten: null,
            failureReasonRef: 'delete-target-missing',
            sourceEvidenceRefs: output.sourceEvidenceRefs,
          });
          await appendAuditEntry(auditLogPath, auditEntry);
          const auditEntries = await readAuditEntriesFromPath(auditLogPath);

          return {
            job,
            bundlePath,
            outputPath,
            auditLogPath,
            auditEntries,
          };
        }

        await rm(bundlePath, { force: true });
        await rm(outputPath, { force: true });

        const deleteReceipt = ParentOwnedLocalExportRuntimeDeleteReceiptSchema.parse({
          deleteRequestRef: `local-export-delete-request-${safeToken(jobId)}`,
          targetBundleRef: output.bundleRef,
          requestedAt: request.requestedAt ?? startedAt,
          deletedAt: updatedAt,
          deleteConfirmed: true,
          auditState,
          sourceEvidenceRetained: false,
          exportedOutputDeleted: true,
          localSafetyStatePreserved: true,
          failureReasonRef: null,
        });

        const job = ParentOwnedLocalExportRuntimeJobSchema.parse({
          jobId,
          operation: 'delete',
          state: 'delete-confirmed',
          queueRef,
          storageState: 'local-folder-ready',
          scope,
          output,
          deleteReceipt,
          queuedAt: request.requestedAt ?? startedAt,
          updatedAt,
          auditRefs,
          localEvidenceMutated: false,
          parentOwnedOutputMutatedByFailure: false,
          localSafetyStatePreserved: true,
          manualActionRequired: false,
        });

        const auditEntry = buildAuditEntry({
          operation: 'delete',
          jobId,
          state: job.state,
          bundleRef: output.bundleRef,
          outputRef: output.outputRef,
          queueRef,
          auditState,
          recordedAt: updatedAt,
          bytesWritten: null,
          failureReasonRef: null,
          sourceEvidenceRefs: output.sourceEvidenceRefs,
        });
        await appendAuditEntry(auditLogPath, auditEntry);
        const auditEntries = await readAuditEntriesFromPath(auditLogPath);

        logInfo(
          'Parent-owned local export runtime delete finished',
          getStackTrace(),
          { jobId, bundleRef: output.bundleRef, auditEntries: auditEntries.length },
          loggingEnabled
        );

        return {
          job,
          bundlePath,
          outputPath,
          auditLogPath,
          auditEntries,
        };
      } catch (error) {
        logError(
          'Parent-owned local export runtime delete failed',
          getStackTrace(),
          error instanceof Error ? { message: error.message } : { error: String(error) }
        );
        throw error;
      }
    },
    async readAuditEntries() {
      return readAuditEntriesFromPath(auditLogPath);
    },
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

async function readAuditEntriesFromPath(auditLogPath: string): Promise<readonly ParentOwnedLocalExportRuntimeAuditEntry[]> {
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
