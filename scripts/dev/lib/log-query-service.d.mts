export interface AgentEvidenceRun {
  readonly runId: string;
  readonly commandId: string | null;
  readonly laneId: string | null;
  readonly machine: string;
  readonly workspace: string;
  readonly cwd: string;
  readonly command: readonly string[];
  readonly startedAt: string;
  readonly endedAt: string | null;
  readonly durationMs: number;
  readonly status: string;
  readonly exitCode: number | null;
  readonly stdoutArtifact: string | null;
  readonly stderrArtifact: string | null;
  readonly summary: string | null;
}

export interface AgentEvidenceDiagnostic {
  readonly diagnosticId: string;
  readonly kind: string;
  readonly severity: string;
  readonly signature: string;
  readonly file: string | null;
  readonly line: number | null;
  readonly column: number | null;
  readonly message: string;
  readonly rawArtifact: string | null;
  readonly rawStartLine: number | null;
  readonly rawEndLine: number | null;
  readonly hitCount: number;
}

export interface AgentEvidenceArtifact {
  readonly artifactId: string;
  readonly runId: string | null;
  readonly commandId: string | null;
  readonly path: string;
  readonly kind: string;
  readonly sha256: string | null;
  readonly byteLength: number | null;
  readonly lineCount: number | null;
  readonly createdAt: string | null;
}

export interface AgentRunEvidence {
  readonly run: AgentEvidenceRun;
  readonly diagnostics: readonly AgentEvidenceDiagnostic[];
  readonly artifacts: readonly AgentEvidenceArtifact[];
}

export interface LocalLogRow {
  readonly recordType: string;
  readonly scope: string;
  readonly timestamp: number | null;
  readonly level: string | null;
  readonly source: string | null;
  readonly context: string | null;
  readonly message: string | null;
  readonly runId: string | null;
  readonly commandId: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
  readonly correlationId: string | null;
  readonly tags: readonly string[];
  readonly data: string | null;
  readonly rawArtifact: string | null;
  readonly hitCount: number;
}

export interface LogStatsResult {
  readonly scope: string;
  readonly logLevels: Readonly<Record<string, number>>;
  readonly sources: Readonly<Record<string, number>>;
  readonly contexts: Readonly<Record<string, number>>;
  readonly agentEvidence: {
    readonly totalRuns: number;
    readonly failedRuns: number;
    readonly passedRuns: number;
    readonly newestStartedAt: string | null;
    readonly totalDiagnostics: number;
    readonly uniqueDiagnosticSignatures: number;
  } | null;
}

export interface ProofTraceRow extends LocalLogRow {
  readonly proofId: string;
  readonly testId: string | null;
  readonly causationId: string | null;
  readonly traceStep: string | null;
  readonly eventType: string | null;
  readonly action: string | null;
  readonly command: string | null;
  readonly status: string | null;
  readonly expectedNext: string | null;
  readonly artifactRef: string | null;
}

export interface ProofTraceResult {
  readonly proofId: string;
  readonly scope: string;
  readonly rows: readonly ProofTraceRow[];
}

export type ExpectedProofTraceStep =
  | string
  | {
      readonly traceStep?: string;
      readonly source?: string;
      readonly context?: string;
      readonly eventType?: string;
      readonly action?: string;
      readonly command?: string;
      readonly status?: string;
      readonly contains?: string;
    };

export interface ProofTraceStepMatch {
  readonly expected: ExpectedProofTraceStep;
  readonly matchedRow: ProofTraceRow;
}

export interface ProofTraceGapResult extends ProofTraceResult {
  readonly matchedSteps: readonly ProofTraceStepMatch[];
  readonly missingSteps: readonly ExpectedProofTraceStep[];
  readonly outOfOrderSteps: readonly ProofTraceStepMatch[];
  readonly unexpectedErrorRows: readonly ProofTraceRow[];
}

export function getLatestFailures(options?: { readonly limit?: number }): Promise<
  readonly Array<{
    readonly runId: string;
    readonly commandId: string | null;
    readonly laneId: string | null;
    readonly machine: string;
    readonly workspace: string;
    readonly cwd: string;
    readonly command: readonly string[];
    readonly startedAt: string;
    readonly endedAt: string | null;
    readonly durationMs: number;
    readonly status: string;
    readonly exitCode: number | null;
    readonly stdoutArtifact: string | null;
    readonly stderrArtifact: string | null;
    readonly summary: string | null;
    readonly diagnostics: readonly AgentEvidenceDiagnostic[];
    readonly artifacts: readonly AgentEvidenceArtifact[];
  }>
>;

export function getRunDiagnostics(options: {
  readonly runId: string;
  readonly limit?: number;
  readonly includeArtifactRefs?: boolean;
}): Promise<
  | readonly AgentEvidenceDiagnostic[]
  | {
      readonly run: AgentEvidenceRun;
      readonly diagnostics: readonly AgentEvidenceDiagnostic[];
      readonly artifacts: readonly AgentEvidenceArtifact[];
    }
>;

export function getArtifactSlice(options: {
  readonly artifactId?: string;
  readonly path?: string;
  readonly startLine?: number;
  readonly endLine?: number;
  readonly maxLines?: number;
}): Promise<{
  readonly path: string;
  readonly startLine: number;
  readonly endLine: number;
  readonly lineCount: number;
  readonly lines: readonly string[];
}>;

export function getErrors(options?: {
  readonly scope?: string;
  readonly limit?: number;
  readonly since?: string;
}): Promise<readonly LocalLogRow[]>;

export function getRecentLogs(options?: {
  readonly scope?: string;
  readonly limit?: number;
  readonly level?: string | null;
  readonly since?: string;
}): Promise<readonly LocalLogRow[]>;

export function getLogsBySource(options: {
  readonly scope?: string;
  readonly source: string;
  readonly level?: string | null;
  readonly limit?: number;
}): Promise<readonly LocalLogRow[]>;

export function getLogsByContext(options: {
  readonly scope?: string;
  readonly context: string;
  readonly level?: string | null;
  readonly limit?: number;
}): Promise<readonly LocalLogRow[]>;

export function queryLogs(options?: {
  readonly scope?: string;
  readonly limit?: number;
  readonly level?: string | null;
  readonly source?: string | null;
  readonly context?: string | null;
  readonly runId?: string | null;
  readonly contains?: string | null;
  readonly from?: string;
  readonly to?: string;
}): Promise<readonly LocalLogRow[]>;

export function getLogStats(options?: {
  readonly scope?: string;
  readonly from?: string;
  readonly to?: string;
}): Promise<LogStatsResult>;

export function getProofTrace(options?: {
  readonly scope?: string;
  readonly proofId?: string;
  readonly proof_id?: string;
  readonly limit?: number;
}): Promise<ProofTraceResult>;

export function getProofTraceGaps(options?: {
  readonly scope?: string;
  readonly proofId?: string;
  readonly proof_id?: string;
  readonly expectedSteps?: readonly ExpectedProofTraceStep[];
  readonly expected_steps?: readonly ExpectedProofTraceStep[];
  readonly limit?: number;
}): Promise<ProofTraceGapResult>;

export function queryProofTrace(options?: {
  readonly scope?: string;
  readonly proofId?: string;
  readonly proof_id?: string;
  readonly expectedSteps?: readonly ExpectedProofTraceStep[];
  readonly expected_steps?: readonly ExpectedProofTraceStep[];
  readonly limit?: number;
}): Promise<ProofTraceResult | ProofTraceGapResult>;
