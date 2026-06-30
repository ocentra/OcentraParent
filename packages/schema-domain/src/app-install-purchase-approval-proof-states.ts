import {
  AppInstallPurchaseApprovalReportRefsGenerated,
  appInstallPurchaseApprovalAuditReportIntegrationGenerated,
  appInstallPurchaseApprovalChildFacingStatesGenerated,
} from './generated/app-install-purchase-proof-helpers';

export const AppInstallPurchaseApprovalReportRefs = AppInstallPurchaseApprovalReportRefsGenerated;

export function appInstallPurchaseApprovalChildFacingStates(input: {
  readonly requestAuditEvent: unknown;
  readonly decisionAuditEvent: unknown;
}) {
  return appInstallPurchaseApprovalChildFacingStatesGenerated(input);
}

export function appInstallPurchaseApprovalAuditReportIntegration(input: {
  readonly requestAuditEvent: unknown;
  readonly decisionAuditEvent: unknown;
}) {
  return appInstallPurchaseApprovalAuditReportIntegrationGenerated(input);
}
