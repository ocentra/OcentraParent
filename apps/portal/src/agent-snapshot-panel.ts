import type { AgentLogSnapshot } from '@ocentra-parent/logging-domain/contracts';
import {
  PortalDetails,
  PortalDom,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';

export function renderAgentSnapshotPanel(container: HTMLElement, snapshot: AgentLogSnapshot | null): void {
  if (snapshot === null) {
    return;
  }

  const summary = document.createElement(PortalDom.Tags.Division);
  summary.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.LatestSnapshot);

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.Device, decodePortalDetailValue(snapshot.agent.deviceId));
  appendDetail(metadata, PortalDetails.Host, decodePortalDetailValue(snapshot.agent.hostname));
  appendDetail(metadata, PortalDetails.Platform, decodePortalDetailValue(snapshot.agent.platform));
  appendDetail(metadata, PortalDetails.Version, decodePortalDetailValue(snapshot.agent.serviceVersion));
  appendDetail(metadata, PortalDetails.Schema, decodePortalDetailValue(String(snapshot.schemaVersion)));

  summary.append(title, metadata);
  container.append(summary);
}
