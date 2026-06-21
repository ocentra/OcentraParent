export const RequiredStatelessReportCompilerStatuses = [
  'queued',
  'running',
  'succeeded',
  'failed',
  'expired',
  'manual-required',
] as const;

export const RequiredStatelessReportCompilerNonClaims = [
  'no-report-compiler-runtime',
  'no-cloud-worker',
  'no-connector-oauth-provider-api',
  'no-portal-ui',
  'no-ocentra-family-data-custody',
  'no-upload-download-implementation',
  'no-child-device-mutation',
  'no-retained-temp-child-evidence',
] as const;

export const StatelessReportCompilerKnownGaps = [
  'No report compiler runtime or cloud worker is implemented by this production-domain contract proof.',
  'No connector OAuth, token vault, provider API, upload, or download implementation is claimed.',
  'No portal UI, CLI control, or account/subscription backend is implemented.',
  'No Ocentra-hosted custody of family activity data, generated reports, source bundles, or temporary child evidence is claimed.',
  'Real parent-owned storage reads, report rendering, deletion execution, and audit persistence remain future work.',
] as const;
