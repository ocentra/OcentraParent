import {
  AgentEvent,
  isAgentProtocolLogText,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import {
  AppGameTimerParentPreferenceSetupRequestResultSchema,
  type AppGameTimerParentPreferenceSetupRequestResult,
} from '@ocentra-parent/schema-domain/app-game-timer-parent-preference-setup-request';
import {
  GeneratedDevLogField as DevLogField,
  GeneratedDevLogMessage as DevLogMessage,
} from '@ocentra-parent/schema-domain/generated/logging-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom, PortalTiming } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails, PortalReadableValues } from '@ocentra-parent/portal-domain/details';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import { decodeParentPortalClipboardText, type ParentRouteEventSnapshot } from '../generated/parent-ui-bridge';
import { writeClipboardText } from './clipboard';
import { writePortalDevLog } from './dev-logger';
import { latestParentRouteEventSnapshot } from './parent-route-event-snapshot';
import type { PortalRuntimeState } from './portal-state';

type CommandResultDetail = {
  readonly label: string;
  readonly value: string;
};

type TimerParentPreferenceSetupRequestParseResult =
  | {
      readonly ok: true;
      readonly value: AppGameTimerParentPreferenceSetupRequestResult;
    }
  | {
      readonly ok: false;
      readonly reason: 'wrong-event' | 'missing-json-field' | 'invalid-json' | 'invalid-payload';
    };

const CommandResultDetailSeparator = ' | ';

const CommandResultReadable = {
  HandoffReady: 'Handoff ready',
  ManualRequired: requiredReadableValue('manual-required'),
  NotClaimed: requiredReadableValue('not-claimed'),
  Pending: 'Pending',
  Persisted: 'Persisted',
  Ready: requiredReadableValue('ready'),
  Required: 'Required',
  Review: requiredReadableValue('warn'),
} as const;

const TimerParentCommandResultLabels = {
  ChildRuntimeDispatchRefs: 'Child runtime dispatch refs',
  ChildRuntimeDispatchStatus: 'Child runtime dispatch status',
  ChildRuntimeHandoffRefs: 'Child runtime handoff refs',
  ChildRuntimeHandoffStatus: 'Child runtime handoff status',
  ChildRuntimeQueueRefs: 'Child runtime queue refs',
  ChildRuntimeQueueStatus: 'Child runtime queue status',
  ChildRuntimeReceiptIngestedRefs: 'Child runtime receipt-ingested refs',
  ChildRuntimeReceiptIngestedStatus: 'Child runtime receipt-ingested status',
  ChildRuntimeReceiptPendingRefs: 'Child runtime receipt-pending refs',
  ChildRuntimeReceiptPendingStatus: 'Child runtime receipt-pending status',
  ChildRuntimeReceiptRequirementRefs: 'Child runtime receipt-required refs',
  ChildRuntimeReceiptRequirementStatus: 'Child runtime receipt-required status',
  DurableOutboxRefs: 'Durable local outbox refs',
  DurableOutboxStatus: 'Durable local outbox status',
  Mutation: 'Parent preference setup mutation',
  MutationReceiptRefs: 'Parent preference setup mutation receipt refs',
  MutationReceiptStatus: 'Parent preference setup mutation receipt status',
  NotificationRuleMutation: 'Notification rule mutation',
  ParentPreferenceSetupAcceptedAt: 'Parent preference setup accepted at',
  ParentPreferenceSetupActionResultRefs: 'Parent preference setup action-result refs',
  ParentPreferenceSetupActionResultStatus: 'Parent preference setup action-result status',
  ParentPreferenceSetupRequestRefs: 'Parent preference setup request refs',
  ProviderDeliveryAdapterRequirementRefs: 'Provider delivery adapter requirement refs',
  ProviderDeliveryAdapterRequirementStatus: 'Provider delivery adapter requirement status',
  ProviderDeliveryAggregateStatus: 'Provider delivery aggregate status',
  ProviderDeliveryAttemptRefs: 'Provider delivery attempt refs',
  ProviderDeliveryAttemptStatus: 'Provider delivery attempt status',
  ProviderDeliveryCredentialRequirementRefs: 'Provider delivery credential requirement refs',
  ProviderDeliveryCredentialRequirementStatus: 'Provider delivery credential requirement status',
  ProviderDeliveryNextAction: 'Provider delivery next action',
  ProviderDeliveryNoClaimBoundary: 'Provider delivery no-claim boundary',
  ProviderDeliveryProofState: 'Provider delivery proof state',
  ProviderDeliveryQueueRefs: 'Provider delivery queue refs',
  ProviderDeliveryQueueStatus: 'Provider delivery queue status',
  ProviderDeliveryReadinessRefs: 'Provider delivery readiness refs',
  ProviderDeliveryReadinessStatus: 'Provider delivery readiness status',
  ProviderDeliveryReceiptIngestedRefs: 'Provider delivery receipt-ingested refs',
  ProviderDeliveryReceiptIngestedStatus: 'Provider delivery receipt-ingested status',
  ProviderDeliveryReceiptPendingRefs: 'Provider delivery receipt-pending refs',
  ProviderDeliveryReceiptPendingStatus: 'Provider delivery receipt-pending status',
  ProviderDeliveryReceiptRequirementRefs: 'Provider delivery receipt-required refs',
  ProviderDeliveryReceiptRequirementStatus: 'Provider delivery receipt-required status',
} as const;

const ProviderDeliveryAggregateValues = {
  ManualProviderSetupRequired:
    'Manual provider setup required; local outbox, queue, and receipt tracking are recorded.',
  NextAction: 'Configure provider adapter and credential proof before external delivery.',
  NoClaimBoundary: 'Provider delivery execution and external provider receipt ingestion are not claimed.',
  ProofState: 'Local durable outbox, provider queue, receipt-required, pending, and ingested refs are visible.',
} as const;

const ParentPreferenceSetupReadyResultStatuses = new Set([
  'queued',
  'dispatch-ready',
  'receipt-required',
  'receipt-pending',
  'receipt-ingested',
  'outbox-recorded',
  'provider-delivery-queued',
]);

const ParentPreferenceSetupManualRequiredResultStatuses = new Set([
  'provider-manual-required',
  'provider-delivery-manual-required',
  'provider-adapter-required',
  'provider-credential-proof-required',
]);

export function renderCommandResultPanel(container: HTMLElement, state: PortalRuntimeState): void {
  const panel = document.createElement(PortalDom.Tags.Division);
  panel.className = PortalDom.Classes.CommandResultPanel;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.CommandResult);

  panel.append(title, renderSelectedResult(state));
  container.append(panel);
}

function renderSelectedResult(state: PortalRuntimeState): HTMLElement {
  const event = latestParentRouteEventSnapshot(state.events, state.selectedCommandResultEvent);
  const panel = document.createElement(PortalDom.Tags.Division);

  if (event === null) {
    const empty = document.createElement(PortalDom.Tags.Paragraph);
    empty.className = PortalDom.Classes.CommandResultEmpty;
    empty.textContent = resolvePortalDevText(PortalDevTextToken.NoCommandResult);
    panel.append(empty);
    return panel;
  }

  panel.append(renderResultEvent(event));
  return panel;
}

function renderResultEvent(event: ParentRouteEventSnapshot): HTMLElement {
  const card = document.createElement(PortalDom.Tags.Division);
  card.className = [
    PortalDom.Classes.Log,
    `${PortalDom.Classes.LogLevelPrefix}${event.severity ?? 'info'}`,
  ].join(PortalDom.Classes.ClassNameSeparator);

  const header = document.createElement(PortalDom.Tags.Division);
  header.className = PortalDom.Classes.CommandResultHeader;

  const message = document.createElement(PortalDom.Tags.Strong);
  message.textContent = event.event ?? 'unknown-event';

  const copyButton = document.createElement(PortalDom.Tags.Button);
  copyButton.type = PortalDom.ButtonType.Button;
  copyButton.className = PortalDom.Classes.CopyResultButton;
  copyButton.textContent = resolvePortalDevText(PortalDevTextToken.CopyResult);
  copyButton.addEventListener(PortalDom.Events.Click, () => {
    void copyResultEvent(copyButton, event);
  });

  const detail = document.createElement(PortalDom.Tags.Span);
  detail.textContent = [
    event.sentAt ?? 'not-reported',
    event.sourcePeerId ?? 'not-reported',
    `${PortalFormatting.CorrelationPrefix}${event.correlationId ?? event.eventId ?? 'not-reported'}`,
  ].join(PortalFormatting.EventDetailSeparator);

  const fields = document.createElement(PortalDom.Tags.Code);
  fields.textContent = JSON.stringify(event.payload ?? {}, null, 2);

  header.append(message, copyButton);
  const resultSummary = renderAppGameTimerParentPreferenceSetupCommandResult(event);

  card.append(header, detail);
  if (resultSummary !== null) {
    card.append(resultSummary);
  }
  card.append(fields);
  return card;
}

function renderAppGameTimerParentPreferenceSetupCommandResult(
  event: ParentRouteEventSnapshot
): HTMLElement | null {
  if (event.event !== AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested) {
    return null;
  }

  return renderDetailList(timerParentPreferenceSetupCommandResultDetails(event));
}

function renderDetailList(details: readonly CommandResultDetail[]): HTMLElement {
  const list = document.createElement(PortalDom.Tags.DefinitionList);
  for (const item of details) {
    const term = document.createElement(PortalDom.Tags.DefinitionTerm);
    term.textContent = item.label;

    const description = document.createElement(PortalDom.Tags.DefinitionDescription);
    description.textContent = item.value;

    list.append(term, description);
  }
  return list;
}

function timerParentPreferenceSetupCommandResultDetails(
  event: ParentRouteEventSnapshot
): readonly CommandResultDetail[] {
  const result = parseTimerParentPreferenceSetupRequestEvent(event);

  if (!result.ok) {
    return [
      detail(PortalDetails.Status, CommandResultReadable.Review),
      detail(PortalDetails.Reason, result.reason),
    ];
  }

  return [
    detail(PortalDetails.Status, CommandResultReadable.Ready),
    detail(PortalDetails.EventId, result.value.requestId),
    detail(TimerParentCommandResultLabels.ParentPreferenceSetupAcceptedAt, result.value.acceptedAt),
    detail(
      TimerParentCommandResultLabels.ParentPreferenceSetupRequestRefs,
      joinedOrNotReported(result.value.requestReferenceIds)
    ),
    detail(
      TimerParentCommandResultLabels.ParentPreferenceSetupActionResultRefs,
      joinedOrNotReported(result.value.actionResultReferenceIds)
    ),
    detail(
      TimerParentCommandResultLabels.ParentPreferenceSetupActionResultStatus,
      parentPreferenceSetupResultStatus(result.value.actionResultPersistenceStatus)
    ),
    detail(
      TimerParentCommandResultLabels.MutationReceiptRefs,
      joinedOrNotReported(result.value.parentPreferenceMutationReceiptIds)
    ),
    detail(
      TimerParentCommandResultLabels.MutationReceiptStatus,
      parentPreferenceSetupResultStatus(result.value.parentPreferenceMutationReceiptStatus)
    ),
    ...parentPreferenceSetupChildRuntimeDetails(result.value),
    detail(TimerParentCommandResultLabels.Mutation, CommandResultReadable.NotClaimed),
    detail(TimerParentCommandResultLabels.NotificationRuleMutation, CommandResultReadable.NotClaimed),
    detail(PortalDetails.ChildDelivery, claimedValue(result.value.childRuntimeDeliveryClaimed)),
    detail(PortalDetails.AdapterDispatch, claimedValue(result.value.adapterDispatchClaimed)),
    detail(PortalDetails.PlatformState, claimedValue(result.value.platformEnforcementClaimed)),
  ];
}

function parseTimerParentPreferenceSetupRequestEvent(
  event: ParentRouteEventSnapshot
): TimerParentPreferenceSetupRequestParseResult {
  if (event.event !== AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested) {
    return { ok: false, reason: 'wrong-event' };
  }

  const payload = event.payload ?? {};
  const raw = payload[AgentProtocolDefaults.Field.ActivityAppGameTimerParentPreferenceSetupRequest];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-json-field' };
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return { ok: false, reason: 'invalid-json' };
  }

  const parsed = AppGameTimerParentPreferenceSetupRequestResultSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return { ok: false, reason: 'invalid-payload' };
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function parentPreferenceSetupChildRuntimeDetails(
  result: AppGameTimerParentPreferenceSetupRequestResult
): readonly CommandResultDetail[] {
  return [
    detail(
      TimerParentCommandResultLabels.ChildRuntimeHandoffRefs,
      joinedOrNotReported(result.childRuntimeDeliveryHandoffIds)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeHandoffStatus,
      parentPreferenceSetupResultStatus(result.childRuntimeDeliveryHandoffStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeQueueRefs,
      joinedOrNotReported(result.childRuntimeDeliveryQueueIds)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeQueueStatus,
      parentPreferenceSetupResultStatus(result.childRuntimeDeliveryQueueStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeDispatchRefs,
      joinedOrNotReported(result.childRuntimeDeliveryDispatchIds)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeDispatchStatus,
      parentPreferenceSetupResultStatus(result.childRuntimeDeliveryDispatchStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeReceiptRequirementRefs,
      joinedOrNotReported(result.childRuntimeDeliveryReceiptRequirementIds)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeReceiptRequirementStatus,
      parentPreferenceSetupResultStatus(result.childRuntimeDeliveryReceiptRequirementStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeReceiptPendingRefs,
      joinedOrNotReported(result.childRuntimeDeliveryReceiptPendingIds)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeReceiptPendingStatus,
      parentPreferenceSetupResultStatus(result.childRuntimeDeliveryReceiptPendingStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeReceiptIngestedRefs,
      joinedOrNotReported(result.childRuntimeDeliveryReceiptIngestedIds)
    ),
    detail(
      TimerParentCommandResultLabels.ChildRuntimeReceiptIngestedStatus,
      parentPreferenceSetupResultStatus(result.childRuntimeDeliveryReceiptIngestedStatus)
    ),
    detail(TimerParentCommandResultLabels.DurableOutboxRefs, joinedOrNotReported(result.durableOutboxRecordIds)),
    detail(
      TimerParentCommandResultLabels.DurableOutboxStatus,
      parentPreferenceSetupResultStatus(result.durableOutboxStatus)
    ),
    ...parentPreferenceSetupProviderDeliveryDetails(result),
  ];
}

function parentPreferenceSetupProviderDeliveryDetails(
  result: AppGameTimerParentPreferenceSetupRequestResult
): readonly CommandResultDetail[] {
  return [
    ...parentPreferenceSetupProviderDeliveryAggregateDetails(result),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryReadinessRefs,
      joinedOrNotReported(result.providerDeliveryReadinessIds)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryReadinessStatus,
      parentPreferenceSetupResultStatus(result.providerDeliveryReadinessStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryAttemptRefs,
      joinedOrNotReported(result.providerDeliveryAttemptIds)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryAttemptStatus,
      parentPreferenceSetupResultStatus(result.providerDeliveryAttemptStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryAdapterRequirementRefs,
      joinedOrNotReported(result.providerDeliveryAdapterRequirementIds)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryAdapterRequirementStatus,
      parentPreferenceSetupResultStatus(result.providerDeliveryAdapterRequirementStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryCredentialRequirementRefs,
      joinedOrNotReported(result.providerDeliveryCredentialRequirementIds)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryCredentialRequirementStatus,
      parentPreferenceSetupResultStatus(result.providerDeliveryCredentialRequirementStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryQueueRefs,
      joinedOrNotReported(result.providerDeliveryQueueIds)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryQueueStatus,
      parentPreferenceSetupResultStatus(result.providerDeliveryQueueStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryReceiptRequirementRefs,
      joinedOrNotReported(result.providerDeliveryReceiptRequirementIds)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryReceiptRequirementStatus,
      parentPreferenceSetupResultStatus(result.providerDeliveryReceiptRequirementStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryReceiptPendingRefs,
      joinedOrNotReported(result.providerDeliveryReceiptPendingIds)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryReceiptPendingStatus,
      parentPreferenceSetupResultStatus(result.providerDeliveryReceiptPendingStatus)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryReceiptIngestedRefs,
      joinedOrNotReported(result.providerDeliveryReceiptIngestedIds)
    ),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryReceiptIngestedStatus,
      parentPreferenceSetupResultStatus(result.providerDeliveryReceiptIngestedStatus)
    ),
  ];
}

function parentPreferenceSetupProviderDeliveryAggregateDetails(
  result: AppGameTimerParentPreferenceSetupRequestResult
): readonly CommandResultDetail[] {
  const aggregateStatus =
    result.providerDeliveryClaimed || result.providerReceiptIngestionClaimed
      ? CommandResultReadable.Review
      : ProviderDeliveryAggregateValues.ManualProviderSetupRequired;

  return [
    detail(TimerParentCommandResultLabels.ProviderDeliveryAggregateStatus, aggregateStatus),
    detail(TimerParentCommandResultLabels.ProviderDeliveryNextAction, ProviderDeliveryAggregateValues.NextAction),
    detail(TimerParentCommandResultLabels.ProviderDeliveryProofState, ProviderDeliveryAggregateValues.ProofState),
    detail(
      TimerParentCommandResultLabels.ProviderDeliveryNoClaimBoundary,
      ProviderDeliveryAggregateValues.NoClaimBoundary
    ),
  ];
}

function parentPreferenceSetupResultStatus(status: string): string {
  if (status === 'handoff-ready') {
    return CommandResultReadable.HandoffReady;
  }
  if (status === 'persisted') {
    return CommandResultReadable.Persisted;
  }
  if (ParentPreferenceSetupReadyResultStatuses.has(status) || status === 'accepted') {
    return CommandResultReadable.Ready;
  }
  if (ParentPreferenceSetupManualRequiredResultStatuses.has(status)) {
    return CommandResultReadable.ManualRequired;
  }
  if (status === 'provider-delivery-receipt-required') {
    return CommandResultReadable.Required;
  }
  if (status === 'provider-delivery-receipt-pending') {
    return CommandResultReadable.Pending;
  }
  if (status === 'provider-delivery-receipt-ingested') {
    return CommandResultReadable.Ready;
  }
  return readableValue(status);
}

function claimedValue(value: boolean): string {
  return value ? CommandResultReadable.Ready : CommandResultReadable.NotClaimed;
}

function joinedOrNotReported(values: readonly string[]): string {
  if (values.length === 0) {
    return resolvePortalDevText(PortalDevTextToken.NotReported);
  }
  return values.join(CommandResultDetailSeparator);
}

function readableValue(value: unknown): string {
  const key = String(value);
  return PortalReadableValues[key] ?? key;
}

function requiredReadableValue(key: string): string {
  const value = PortalReadableValues[key];
  if (value === undefined) {
    throw new Error(`Missing portal readable value: ${key}`);
  }
  return value;
}

function detail(label: string, value: string): CommandResultDetail {
  return { label, value };
}

async function copyResultEvent(button: HTMLButtonElement, event: ParentRouteEventSnapshot): Promise<void> {
  button.disabled = true;
  try {
    const didCopy = await writeClipboardText(decodeParentPortalClipboardText(JSON.stringify(event, null, 2)));
    if (!didCopy) {
      button.textContent = resolvePortalDevText(PortalDevTextToken.CopyResultFailed);
      return;
    }
    writePortalDevLog(DevLogMessage.PortalResultCopied, {
      [DevLogField.Event]: event.event ?? 'unknown-event',
    });
    button.textContent = resolvePortalDevText(PortalDevTextToken.CopiedResult);
  } catch {
    button.textContent = resolvePortalDevText(PortalDevTextToken.CopyResultFailed);
  } finally {
    button.disabled = false;
    window.setTimeout(() => {
      button.textContent = resolvePortalDevText(PortalDevTextToken.CopyResult);
    }, PortalTiming.CopyFeedbackMs);
  }
}
