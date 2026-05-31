import { ActivityReadModelStateSchema } from '@ocentra-parent/activity-domain/activity-surface';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentCommand, AgentEvent, AgentProtocolDefaults } from './contracts';

export const ActivitySurfaceAdapterOperationId = {
  GetDailyReport: 'getDailyReport',
  GetWeeklyReport: 'getWeeklyReport',
  GetMonthlyReport: 'getMonthlyReport',
  SaveActivityReport: 'saveActivityReport',
  ListHistoricalReports: 'listHistoricalReports',
  GetScreenActivity: 'getScreenActivity',
  GetAppUseActivity: 'getAppUseActivity',
  GetBrowserActivity: 'getBrowserActivity',
  GetGamesActivity: 'getGamesActivity',
  GetNetworkActivity: 'getNetworkActivity',
} as const;

export const ActivitySurfaceAdapterManifestReadModelKindSchema = withParser(
  Schema.Literal('screen', 'app-use', 'browser', 'games', 'network')
);
export const ActivitySurfaceAdapterFailureReasonSchema = withParser(
  Schema.Literal('wrong-event', 'missing-json-field', 'invalid-json', 'invalid-payload')
);
export const ActivitySurfaceAdapterResponseKindSchema = withParser(
  Schema.Literal('report-document', 'report-history', 'tab-read-model')
);
export const ActivitySurfaceAdapterProductDataOwnerSchema = withParser(Schema.Literal('rust-service-read-model'));
export const ActivitySurfaceAdapterUiConsumerSchema = withParser(Schema.Literal('c-owned-activity-ui'));

export const ActivitySurfaceAdapterOperationSchema = withParser(
  Schema.Struct({
    operationId: Schema.Literal(
      ActivitySurfaceAdapterOperationId.GetDailyReport,
      ActivitySurfaceAdapterOperationId.GetWeeklyReport,
      ActivitySurfaceAdapterOperationId.GetMonthlyReport,
      ActivitySurfaceAdapterOperationId.SaveActivityReport,
      ActivitySurfaceAdapterOperationId.ListHistoricalReports,
      ActivitySurfaceAdapterOperationId.GetScreenActivity,
      ActivitySurfaceAdapterOperationId.GetAppUseActivity,
      ActivitySurfaceAdapterOperationId.GetBrowserActivity,
      ActivitySurfaceAdapterOperationId.GetGamesActivity,
      ActivitySurfaceAdapterOperationId.GetNetworkActivity
    ),
    command: Schema.Literal(
      AgentCommand.ActivityReportDailyGenerate,
      AgentCommand.ActivityReportWeeklyGenerate,
      AgentCommand.ActivityReportMonthlyGenerate,
      AgentCommand.ActivityReportSave,
      AgentCommand.ActivityReportHistoryList,
      AgentCommand.ActivityScreenReadModelGet,
      AgentCommand.ActivityAppUseReadModelGet,
      AgentCommand.ActivityBrowserReadModelGet,
      AgentCommand.ActivityGamesReadModelGet,
      AgentCommand.ActivityNetworkReadModelGet
    ),
    successEvent: Schema.Literal(
      AgentEvent.ActivityReportGenerated,
      AgentEvent.ActivityReportSaved,
      AgentEvent.ActivityReportHistoryReported,
      AgentEvent.ActivityScreenReadModelReported,
      AgentEvent.ActivityAppUseReadModelReported,
      AgentEvent.ActivityBrowserReadModelReported,
      AgentEvent.ActivityGamesReadModelReported,
      AgentEvent.ActivityNetworkReadModelReported
    ),
    payloadField: Schema.Literal(
      AgentProtocolDefaults.Field.ActivityReportDocument,
      AgentProtocolDefaults.Field.ActivityReports,
      AgentProtocolDefaults.Field.ActivityReadModel
    ),
    responseKind: ActivitySurfaceAdapterResponseKindSchema,
    readModelKind: Schema.Union(ActivitySurfaceAdapterManifestReadModelKindSchema, Schema.Null),
    productDataOwner: ActivitySurfaceAdapterProductDataOwnerSchema,
    uiConsumer: ActivitySurfaceAdapterUiConsumerSchema,
    viteDataOwner: Schema.Literal(false),
    supportsFamilyScope: Schema.Boolean,
    supportsDeviceScope: Schema.Boolean,
    failureState: ActivityReadModelStateSchema,
    failureReasons: Schema.Array(ActivitySurfaceAdapterFailureReasonSchema),
    unavailableState: ActivityReadModelStateSchema,
  })
);

export type ActivitySurfaceAdapterOperation = Infer<typeof ActivitySurfaceAdapterOperationSchema>;
export type ActivitySurfaceAdapterFailureReason = Infer<typeof ActivitySurfaceAdapterFailureReasonSchema>;
export type ActivitySurfaceAdapterResponseKind = Infer<typeof ActivitySurfaceAdapterResponseKindSchema>;
export type ActivitySurfaceAdapterManifestReadModelKind = Infer<
  typeof ActivitySurfaceAdapterManifestReadModelKindSchema
>;

export const ActivitySurfaceAdapterOperationManifest = [
  adapterOperation(
    ActivitySurfaceAdapterOperationId.GetDailyReport,
    AgentCommand.ActivityReportDailyGenerate,
    AgentEvent.ActivityReportGenerated,
    AgentProtocolDefaults.Field.ActivityReportDocument,
    'report-document',
    null
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.GetWeeklyReport,
    AgentCommand.ActivityReportWeeklyGenerate,
    AgentEvent.ActivityReportGenerated,
    AgentProtocolDefaults.Field.ActivityReportDocument,
    'report-document',
    null
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.GetMonthlyReport,
    AgentCommand.ActivityReportMonthlyGenerate,
    AgentEvent.ActivityReportGenerated,
    AgentProtocolDefaults.Field.ActivityReportDocument,
    'report-document',
    null
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.SaveActivityReport,
    AgentCommand.ActivityReportSave,
    AgentEvent.ActivityReportSaved,
    AgentProtocolDefaults.Field.ActivityReportDocument,
    'report-document',
    null
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.ListHistoricalReports,
    AgentCommand.ActivityReportHistoryList,
    AgentEvent.ActivityReportHistoryReported,
    AgentProtocolDefaults.Field.ActivityReports,
    'report-history',
    null
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.GetScreenActivity,
    AgentCommand.ActivityScreenReadModelGet,
    AgentEvent.ActivityScreenReadModelReported,
    AgentProtocolDefaults.Field.ActivityReadModel,
    'tab-read-model',
    'screen'
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.GetAppUseActivity,
    AgentCommand.ActivityAppUseReadModelGet,
    AgentEvent.ActivityAppUseReadModelReported,
    AgentProtocolDefaults.Field.ActivityReadModel,
    'tab-read-model',
    'app-use'
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.GetBrowserActivity,
    AgentCommand.ActivityBrowserReadModelGet,
    AgentEvent.ActivityBrowserReadModelReported,
    AgentProtocolDefaults.Field.ActivityReadModel,
    'tab-read-model',
    'browser'
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.GetGamesActivity,
    AgentCommand.ActivityGamesReadModelGet,
    AgentEvent.ActivityGamesReadModelReported,
    AgentProtocolDefaults.Field.ActivityReadModel,
    'tab-read-model',
    'games'
  ),
  adapterOperation(
    ActivitySurfaceAdapterOperationId.GetNetworkActivity,
    AgentCommand.ActivityNetworkReadModelGet,
    AgentEvent.ActivityNetworkReadModelReported,
    AgentProtocolDefaults.Field.ActivityReadModel,
    'tab-read-model',
    'network'
  ),
] as const satisfies readonly ActivitySurfaceAdapterOperation[];

function adapterOperation(
  operationId: ActivitySurfaceAdapterOperation['operationId'],
  command: ActivitySurfaceAdapterOperation['command'],
  successEvent: ActivitySurfaceAdapterOperation['successEvent'],
  payloadField: ActivitySurfaceAdapterOperation['payloadField'],
  responseKind: ActivitySurfaceAdapterResponseKind,
  readModelKind: ActivitySurfaceAdapterOperation['readModelKind']
): ActivitySurfaceAdapterOperation {
  return ActivitySurfaceAdapterOperationSchema.parse({
    operationId,
    command,
    successEvent,
    payloadField,
    responseKind,
    readModelKind,
    productDataOwner: 'rust-service-read-model',
    uiConsumer: 'c-owned-activity-ui',
    viteDataOwner: false,
    supportsFamilyScope: true,
    supportsDeviceScope: true,
    failureState: 'unavailable',
    failureReasons: ['wrong-event', 'missing-json-field', 'invalid-json', 'invalid-payload'],
    unavailableState: 'unavailable',
  });
}
