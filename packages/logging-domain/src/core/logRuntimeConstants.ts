export const LoggerRuntimeEnvironment = {
  RunId: 'OCENTRA_PARENT_LOG_RUN_ID',
  TestName: 'OCENTRA_PARENT_LOG_TEST_NAME',
  Scope: 'OCENTRA_PARENT_LOG_SCOPE',
  RunType: 'OCENTRA_PARENT_LOG_RUN_TYPE',
  SuiteType: 'OCENTRA_PARENT_LOG_SUITE_TYPE',
  Origin: 'OCENTRA_PARENT_LOG_ORIGIN',
  Environment: 'OCENTRA_PARENT_LOG_ENVIRONMENT',
} as const;

export const LoggerRuntimeDefaults = {
  GeneratedRunIdPrefix: 'parent-log-run-',
  TestName: 'parent-runtime-logger',
  UnknownModule: 'UnknownModule',
  ModuleContextSuffix: 'module',
} as const;
