import type { AgentLogEntry, AgentLogSnapshot } from '@ocentra-parent/schema-domain/logging-contracts';
import { decodePortalDetailValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { PortalDiagnostics } from '@ocentra-parent/portal-domain/diagnostics';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import { appendDetail } from './detail-list';

export function renderDevLogPanel(container: HTMLElement, snapshot: AgentLogSnapshot | null): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.DevLog);
  panel.append(title);

  if (snapshot === null || snapshot.entries.length === 0) {
    panel.append(emptyMessage());
    container.append(panel);
    return;
  }

  const list = document.createElement(PortalDom.Tags.OrderedList);
  list.className = PortalDom.Classes.LogList;
  for (const entry of snapshot.entries.slice(0, PortalDiagnostics.DevLogEntryLimit)) {
    list.append(renderLogEntry(entry));
  }

  panel.append(list);
  container.append(panel);
}

function renderLogEntry(entry: AgentLogEntry): HTMLLIElement {
  const item = document.createElement(PortalDom.Tags.ListItem);
  item.className = [PortalDom.Classes.Log, `${PortalDom.Classes.LogLevelPrefix}${entry.level}`].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const message = document.createElement(PortalDom.Tags.Strong);
  message.textContent = entry.message;

  const detail = document.createElement(PortalDom.Tags.Span);
  detail.textContent = [entry.timestamp, entry.source, entry.id].join(PortalFormatting.EventDetailSeparator);

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.EntryId, decodePortalDetailValue(entry.id));
  appendDetail(metadata, PortalDetails.Level, decodePortalDetailValue(entry.level));
  appendDetail(metadata, PortalDetails.Observer, decodePortalDetailValue(entry.source));

  const fields = document.createElement(PortalDom.Tags.Code);
  fields.textContent = JSON.stringify(entry.fields, null, PortalDiagnostics.JsonIndent);

  item.append(message, detail, metadata, fields);
  return item;
}

function emptyMessage(): HTMLElement {
  const message = document.createElement(PortalDom.Tags.Paragraph);
  message.className = PortalDom.Classes.CommandResultEmpty;
  message.textContent = resolvePortalDevText(PortalDevTextToken.NoDevLog);
  return message;
}
