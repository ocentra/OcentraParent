export const RequiredParentOwnedLocalExportRuntimeStates = [
  'export-queued',
  'export-running',
  'export-written',
  'delete-requested',
  'delete-confirmed',
  'delete-failed',
  'offline-queued',
  'manual-required',
] as const;

export const RequiredParentOwnedLocalExportRuntimeNonClaims = [
  'no-cloud-transfer-runtime',
  'no-connector-oauth',
  'no-provider-api',
  'no-portal-ui',
  'no-ocentra-family-data-custody',
  'no-remote-report-compiler',
  'no-child-device-mutation',
  'no-raw-evidence-upload',
] as const;

export const ParentOwnedLocalExportRuntimeKnownGaps = [
  'No cloud transfer, connector OAuth, provider API, or remote report compiler runtime is implemented by this rust-parent-runtime proof.',
  'No portal UI, CLI control, account backend, or subscription enforcement path is claimed.',
  'No Ocentra-hosted custody of family activity data, generated reports, exported bundles, or source evidence is claimed.',
  'Retention scheduler and parent-visible status controls remain future work before broader product export/delete claims.',
] as const;
