import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const ActivityObserverLiteral = {
  AgentService: 'agent-service',
  WindowsProcess: 'windows-process',
  WindowsWindow: 'windows-window',
  WindowsNetwork: 'windows-network',
  ManagedBrowserBridge: 'managed-browser-bridge',
  BrowserExtension: 'browser-extension',
  LocalAi: 'local-ai',
} as const;

export const ActivityEventKindLiteral = {
  ProcessObserved: 'activity.process.observed',
  WindowFocused: 'activity.window.focused',
  DomainObserved: 'activity.domain.observed',
  UrlObserved: 'activity.url.observed',
  VideoObserved: 'activity.video.observed',
  BrowserInterventionApplied: 'activity.browser.intervention.applied',
  EnforcementAuditRecorded: 'activity.enforcement.audit-recorded',
  DeviceIdleStateObserved: 'activity.device.idle-state-observed',
  ScreenAnalysisSummarized: 'activity.screen.analysis.summarized',
} as const;

export const ActivitySubjectKindLiteral = {
  Process: 'process',
  Window: 'window',
  Domain: 'domain',
  Url: 'url',
  Video: 'video',
  Device: 'device',
  Intervention: 'intervention',
} as const;

export const ActivityEvidenceKindLiteral = {
  JournalEntry: 'journal-entry',
  Screenshot: 'screenshot',
  StorageObject: 'storage-object',
  LocalDbRow: 'local-db-row',
} as const;

export const ActivityObserverSchema = withParser(
  Schema.Literal(
    ActivityObserverLiteral.AgentService,
    ActivityObserverLiteral.WindowsProcess,
    ActivityObserverLiteral.WindowsWindow,
    ActivityObserverLiteral.WindowsNetwork,
    ActivityObserverLiteral.ManagedBrowserBridge,
    ActivityObserverLiteral.BrowserExtension,
    ActivityObserverLiteral.LocalAi
  )
);

export const ActivityEventKindSchema = withParser(
  Schema.Literal(
    ActivityEventKindLiteral.ProcessObserved,
    ActivityEventKindLiteral.WindowFocused,
    ActivityEventKindLiteral.DomainObserved,
    ActivityEventKindLiteral.UrlObserved,
    ActivityEventKindLiteral.VideoObserved,
    ActivityEventKindLiteral.BrowserInterventionApplied,
    ActivityEventKindLiteral.EnforcementAuditRecorded,
    ActivityEventKindLiteral.DeviceIdleStateObserved,
    ActivityEventKindLiteral.ScreenAnalysisSummarized
  )
);

export const ActivitySubjectKindSchema = withParser(
  Schema.Literal(
    ActivitySubjectKindLiteral.Process,
    ActivitySubjectKindLiteral.Window,
    ActivitySubjectKindLiteral.Domain,
    ActivitySubjectKindLiteral.Url,
    ActivitySubjectKindLiteral.Video,
    ActivitySubjectKindLiteral.Device,
    ActivitySubjectKindLiteral.Intervention
  )
);

export const ActivityEvidenceKindSchema = withParser(
  Schema.Literal(
    ActivityEvidenceKindLiteral.JournalEntry,
    ActivityEvidenceKindLiteral.Screenshot,
    ActivityEvidenceKindLiteral.StorageObject,
    ActivityEvidenceKindLiteral.LocalDbRow
  )
);

export type ActivityObserver = Infer<typeof ActivityObserverSchema>;
export type ActivityEventKind = Infer<typeof ActivityEventKindSchema>;
export type ActivitySubjectKind = Infer<typeof ActivitySubjectKindSchema>;
export type ActivityEvidenceKind = Infer<typeof ActivityEvidenceKindSchema>;

export const ActivityObserver = {
  AgentService: ActivityObserverSchema.parse(ActivityObserverLiteral.AgentService),
  WindowsProcess: ActivityObserverSchema.parse(ActivityObserverLiteral.WindowsProcess),
  WindowsWindow: ActivityObserverSchema.parse(ActivityObserverLiteral.WindowsWindow),
  WindowsNetwork: ActivityObserverSchema.parse(ActivityObserverLiteral.WindowsNetwork),
  ManagedBrowserBridge: ActivityObserverSchema.parse(ActivityObserverLiteral.ManagedBrowserBridge),
  BrowserExtension: ActivityObserverSchema.parse(ActivityObserverLiteral.BrowserExtension),
  LocalAi: ActivityObserverSchema.parse(ActivityObserverLiteral.LocalAi),
} as const;

export const ActivityEventKind = {
  ProcessObserved: ActivityEventKindSchema.parse(ActivityEventKindLiteral.ProcessObserved),
  WindowFocused: ActivityEventKindSchema.parse(ActivityEventKindLiteral.WindowFocused),
  DomainObserved: ActivityEventKindSchema.parse(ActivityEventKindLiteral.DomainObserved),
  UrlObserved: ActivityEventKindSchema.parse(ActivityEventKindLiteral.UrlObserved),
  VideoObserved: ActivityEventKindSchema.parse(ActivityEventKindLiteral.VideoObserved),
  BrowserInterventionApplied: ActivityEventKindSchema.parse(ActivityEventKindLiteral.BrowserInterventionApplied),
  EnforcementAuditRecorded: ActivityEventKindSchema.parse(ActivityEventKindLiteral.EnforcementAuditRecorded),
  DeviceIdleStateObserved: ActivityEventKindSchema.parse(ActivityEventKindLiteral.DeviceIdleStateObserved),
  ScreenAnalysisSummarized: ActivityEventKindSchema.parse(ActivityEventKindLiteral.ScreenAnalysisSummarized),
} as const;

export const ActivitySubjectKind = {
  Process: ActivitySubjectKindSchema.parse(ActivitySubjectKindLiteral.Process),
  Window: ActivitySubjectKindSchema.parse(ActivitySubjectKindLiteral.Window),
  Domain: ActivitySubjectKindSchema.parse(ActivitySubjectKindLiteral.Domain),
  Url: ActivitySubjectKindSchema.parse(ActivitySubjectKindLiteral.Url),
  Video: ActivitySubjectKindSchema.parse(ActivitySubjectKindLiteral.Video),
  Device: ActivitySubjectKindSchema.parse(ActivitySubjectKindLiteral.Device),
  Intervention: ActivitySubjectKindSchema.parse(ActivitySubjectKindLiteral.Intervention),
} as const;

export const ActivityEvidenceKind = {
  JournalEntry: ActivityEvidenceKindSchema.parse(ActivityEvidenceKindLiteral.JournalEntry),
  Screenshot: ActivityEvidenceKindSchema.parse(ActivityEvidenceKindLiteral.Screenshot),
  StorageObject: ActivityEvidenceKindSchema.parse(ActivityEvidenceKindLiteral.StorageObject),
  LocalDbRow: ActivityEvidenceKindSchema.parse(ActivityEvidenceKindLiteral.LocalDbRow),
} as const;
