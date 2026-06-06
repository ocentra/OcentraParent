import type { ReactElement } from 'react';
import {
  PortalDetails,
  PortalDom,
  PortalFormatting,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import type { NetworkPlatformClaimManifestEntrySummary } from './network-product-readiness-status';

export function NetworkPlatformClaimManifestCard({
  entries,
}: {
  readonly entries: readonly NetworkPlatformClaimManifestEntrySummary[];
}): ReactElement {
  return (
    <article className={networkPlatformClaimManifestCardClassName()}>
      <h2>{PortalDetails.PlatformClaimManifest}</h2>
      {entries.length === 0 ? (
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <NetworkPlatformClaimManifestDetail label={PortalDetails.Status} value={notReported()} />
        </dl>
      ) : (
        entries.map((entry) => (
          <div key={networkPlatformClaimManifestEntryKey(entry)}>
            <h3>{entry.target}</h3>
            <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
              <NetworkPlatformClaimManifestDetail label={PortalDetails.Platform} value={entry.target} />
              <NetworkPlatformClaimManifestDetail label={PortalDetails.State} value={entry.state} />
              <NetworkPlatformClaimManifestDetail label={PortalDetails.PolicyPreview} value={entry.policyDecisionRef} />
              <NetworkPlatformClaimManifestDetail
                label={PortalDetails.ParentRuleContextReferences}
                value={entry.parentRuleRef}
              />
              <NetworkPlatformClaimManifestDetail label={PortalDetails.EvidenceReferences} value={entry.evidenceRefs} />
              <NetworkPlatformClaimManifestDetail label={PortalDetails.DeviceOrOsRefs} value={entry.deviceOrOsRefs} />
              <NetworkPlatformClaimManifestDetail
                label={PortalDetails.PermissionOrEntitlementRefs}
                value={entry.permissionOrEntitlementRefs}
              />
              <NetworkPlatformClaimManifestDetail
                label={PortalDetails.AdapterCapabilityRefs}
                value={entry.adapterCapabilityRefs}
              />
              <NetworkPlatformClaimManifestDetail
                label={PortalDetails.MissingProof}
                value={entry.missingRequiredArtifacts}
              />
              <NetworkPlatformClaimManifestDetail label={PortalDetails.Audit} value={entry.auditRefs} />
              <NetworkPlatformClaimManifestDetail
                label={PortalDetails.AdapterAuthorizedByProof}
                value={entry.adapterAuthorizedByProof}
              />
              <NetworkPlatformClaimManifestDetail
                label={PortalDetails.EnforcementCommandPublished}
                value={entry.enforcementCommandPublished}
              />
            </dl>
          </div>
        ))
      )}
    </article>
  );
}

function NetworkPlatformClaimManifestDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: PortalDetailValue;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function networkPlatformClaimManifestEntryKey(entry: NetworkPlatformClaimManifestEntrySummary): string {
  return [entry.target, entry.policyDecisionRef, entry.state].join(PortalFormatting.EventDetailSeparator);
}

function networkPlatformClaimManifestCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}
