import type { ParentPortalRow } from './parent-portal-data';
import type { ParentPortalServiceStateInput } from './parent-portal-service-state';
import { PRODUCT_SHELL_ROW_SPECS } from './parent-portal-product-shell-row-specs';
import { productShellSignals, type ProductShellSignal } from './parent-portal-product-shell-row-signals';

export function parentPortalProductShellRows(input: ParentPortalServiceStateInput): ParentPortalRow[] {
  const signals = productShellSignals(input);
  return PRODUCT_SHELL_ROW_SPECS.map((spec) => rowFromSignal(spec, signals[spec.signalKind]));
}

function rowFromSignal(spec: (typeof PRODUCT_SHELL_ROW_SPECS)[number], signal: ProductShellSignal): ParentPortalRow {
  return {
    label: spec.label,
    order: spec.order,
    signalScore: signal.signalScore,
    readyCount: signal.readyCount,
    gapCount: signal.gapCount,
    primaryArea: spec.primaryArea,
    trend: signal.trend,
    tone: spec.tone,
  };
}
