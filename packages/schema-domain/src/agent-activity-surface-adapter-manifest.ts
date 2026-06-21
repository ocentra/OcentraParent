import { ActivityReadModelStateSchema } from './activity-surface';
import { type Infer, Schema, withParser } from './effect';
import { AgentCommand, AgentEvent } from './agent-command-event-contracts';
import { AgentProtocolDefaults } from './agent-protocol-defaults';

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

export const ActivitySurfaceAdapterCommandBuilder = {
  ReportGenerate: 'createActivityReportGenerateCommand',
  ReportSave: 'createActivityReportSaveCommand',
  ReportHistory: 'createActivityReportHistoryCommand',
  ReadModel: 'createActivityReadModelCommand',
} as const;

export const ActivitySurfaceAdapterEventParser = {
  ReportDocument: 'parseActivityReportDocumentEvent',
  ReportHistory: 'parseActivityReportHistoryEvent',
  ReadModel: 'parseActivityReadModelEvent',
} as const;

export const ActivitySurfaceAdapterManifestReadModelKindSchema = withParser(
  Schema.Literal('screen', 'app-use', 'browser', 'games', 'network')
);
export const ActivitySurfaceAdapterCommandBuilderSchema = withParser(
  Schema.Literal(
    ActivitySurfaceAdapterCommandBuilder.ReportGenerate,
    ActivitySurfaceAdapterCommandBuilder.ReportSave,
    ActivitySurfaceAdapterCommandBuilder.ReportHistory,
    ActivitySurfaceAdapterCommandBuilder.ReadModel
  )
);
export const ActivitySurfaceAdapterEventParserSchema = withParser(
  Schema.Literal(
    ActivitySurfaceAdapterEventParser.ReportDocument,
    ActivitySurfaceAdapterEventParser.ReportHistory,
    ActivitySurfaceAdapterEventParser.ReadModel
  )
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
    commandBuilder: ActivitySurfaceAdapterCommandBuilderSchema,
    eventParser: ActivitySurfaceAdapterEventParserSchema,
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
export type ActivitySurfaceAdapterCommandBuilderName = Infer<typeof ActivitySurfaceAdapterCommandBuilderSchema>;
export type ActivitySurfaceAdapterEventParserName = Infer<typeof ActivitySurfaceAdapterEventParserSchema>;
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
    commandBuilder: commandBuilderForOperation(operationId),
    eventParser: eventParserForResponse(responseKind),
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

function commandBuilderForOperation(
  operationId: ActivitySurfaceAdapterOperation['operationId']
): ActivitySurfaceAdapterCommandBuilderName {
  if (operationId === ActivitySurfaceAdapterOperationId.SaveActivityReport) {
    return ActivitySurfaceAdapterCommandBuilder.ReportSave;
  }
  if (operationId === ActivitySurfaceAdapterOperationId.ListHistoricalReports) {
    return ActivitySurfaceAdapterCommandBuilder.ReportHistory;
  }
  if (
    operationId === ActivitySurfaceAdapterOperationId.GetScreenActivity ||
    operationId === ActivitySurfaceAdapterOperationId.GetAppUseActivity ||
    operationId === ActivitySurfaceAdapterOperationId.GetBrowserActivity ||
    operationId === ActivitySurfaceAdapterOperationId.GetGamesActivity ||
    operationId === ActivitySurfaceAdapterOperationId.GetNetworkActivity
  ) {
    return ActivitySurfaceAdapterCommandBuilder.ReadModel;
  }

  return ActivitySurfaceAdapterCommandBuilder.ReportGenerate;
}

function eventParserForResponse(
  responseKind: ActivitySurfaceAdapterResponseKind
): ActivitySurfaceAdapterEventParserName {
  if (responseKind === 'report-history') {
    return ActivitySurfaceAdapterEventParser.ReportHistory;
  }
  if (responseKind === 'tab-read-model') {
    return ActivitySurfaceAdapterEventParser.ReadModel;
  }

  return ActivitySurfaceAdapterEventParser.ReportDocument;
}
