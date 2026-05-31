import { defaultDeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import { bottomRoundRectPath, topRoundRectPath } from './DeviceChoiceGridGeometry';
import {
  createDeviceChoiceGridGridPlan,
  createDeviceChoiceGridLayout,
  createDeviceChoiceGridShape,
} from './DeviceChoiceGridLayout';
import {
  emptyLanSlot,
  emptyPortalSlot,
  getLanSlots,
  makeLanDeviceSlots,
  makePortalSlots,
  toDeviceSlot,
  unsupportedSlot,
} from './DeviceChoiceGridSlots';

type AssertFn = (condition: boolean, message: string) => void;

export function runDeviceChoiceGridSelfTests(assert: AssertFn = console.assert): void {
  const cfg = defaultDeviceChoiceGridConfig;
  const lanItems = getLanSlots(undefined, undefined, undefined, cfg.layout.rows * cfg.layout.columns, cfg.text.options);
  const explicitEmptyLanItems = getLanSlots([], undefined, undefined, cfg.layout.columns, cfg.text.options);
  const oneDeviceLanItems = makeLanDeviceSlots(
    [{ id: 'parent', name: 'Local device', platform: 'windows', status: 'connected' }],
    4
  );
  const connectedCount = lanItems.filter((slot) => slot.status === 'connected').length;
  const manuallyAddedIds = lanItems
    .filter((slot) => slot.status === 'available')
    .slice(0, 2)
    .map((slot) => slot.value);
  const portalItems = makePortalSlots(lanItems, manuallyAddedIds, cfg.layout.parentRows * cfg.layout.parentColumns);
  const oneRowGridPlan = createDeviceChoiceGridGridPlan({
    availableH: 360,
    availableW: 2600,
    cfg,
    fallbackColumns: cfg.layout.columns,
    fallbackRows: cfg.layout.rows,
    itemCount: 10,
    legendCount: cfg.statusOrder.lan.length,
  });
  const compactGridPlan = createDeviceChoiceGridGridPlan({
    availableH: 313,
    availableW: 376,
    cfg,
    fallbackColumns: cfg.layout.columns,
    fallbackRows: cfg.layout.rows,
    itemCount: 15,
    legendCount: cfg.statusOrder.lan.length,
  });
  const constrainedCfg = {
    ...cfg,
    svg: { ...cfg.svg, width: 260, height: 220 },
  };
  const constrainedShape = createDeviceChoiceGridShape({
    cfg: constrainedCfg,
    currentScope: 'lan',
    lanRows: compactGridPlan.rows,
    lanColumns: compactGridPlan.columns,
    portalRows: cfg.layout.parentRows,
    portalColumns: cfg.layout.parentColumns,
  });
  const constrainedLayout = createDeviceChoiceGridLayout(constrainedCfg, constrainedShape, lanItems);
  const manyDevicePlan = createDeviceChoiceGridGridPlan({
    availableH: 300,
    availableW: 1320,
    cfg,
    fallbackColumns: cfg.layout.columns,
    fallbackRows: cfg.layout.rows,
    itemCount: 50,
    legendCount: cfg.statusOrder.lan.length,
  });
  const manyDeviceShape = createDeviceChoiceGridShape({
    cfg,
    currentScope: 'lan',
    lanRows: manyDevicePlan.rows,
    lanColumns: manyDevicePlan.columns,
    portalRows: cfg.layout.parentRows,
    portalColumns: cfg.layout.parentColumns,
  });
  const manyDeviceLayout = createDeviceChoiceGridLayout(
    { ...cfg, svg: { ...cfg.svg, width: 1320, height: 300 } },
    manyDeviceShape,
    Array.from({ length: 50 }, (_, index) =>
      toDeviceSlot({ id: `device-${index + 1}`, name: `Device ${index + 1}`, status: 'available' }, index)
    )
  );

  assert(lanItems.length === 15, '3x5 LAN grid should have 15 items.');
  assert(
    cfg.layout.parentRows === 2 && cfg.layout.parentColumns === 4,
    'Parent Portal should default to a smaller 2x4 slot limit.'
  );
  assert(portalItems.length === 8, 'Parent Portal should default to 8 slots.');
  assert(
    portalItems.filter((slot) => slot.status === 'connected').length ===
      Math.min(8, connectedCount + manuallyAddedIds.length),
    'Parent Portal should auto-show connected LAN devices plus manually added available devices as connected.'
  );
  assert(
    portalItems.some((slot) => slot.status === 'empty'),
    'Parent Portal should show unused slots as actual empty cells.'
  );
  assert(
    cfg.connector.chainWidth === cfg.connector.width && cfg.connector.chainGlowWidth === cfg.connector.glowWidth,
    'Vertical row connectors should use the same thickness as the main connector lines.'
  );
  assert(
    cfg.colors.connected === '#57f287' && cfg.colors.available === '#38dfff' && cfg.colors.offline === '#ff4f5e',
    'Status colors should be connected green, available cyan, and offline red.'
  );
  assert(
    cfg.text.scopeOptions.lan === 'LAN Devices' &&
      cfg.text.scopeOptions.parent === 'Parent Portal' &&
      cfg.text.scopeOptions.portal === 'Portal',
    'Header should support LAN Devices / Parent Portal / Portal scopes.'
  );
  assert(
    cfg.layout.scopeIconSize > 0 && cfg.layout.scopeIconGap > 0,
    'Scope toggle should reserve room for LAN and portal icons.'
  );
  assert(
    unsupportedSlot(0).status === 'unsupported' && emptyPortalSlot(0).status === 'empty',
    'Unsupported and empty are separate concepts.'
  );
  assert(
    emptyLanSlot(0).label === '' && emptyLanSlot(0).status === 'empty',
    'LAN placeholders should render as blank empty boxes.'
  );
  assert(
    explicitEmptyLanItems.every((slot) => slot.status === 'empty' && slot.label === ''),
    'An explicitly empty LAN discovery result should not fall back to demo devices.'
  );
  assert(
    oneDeviceLanItems.slice(1).every((slot) => slot.status === 'empty' && slot.label === ''),
    'LAN discovery padding should use blank placeholders, not unsupported fake devices.'
  );
  assert(oneDeviceLanItems[0]?.platform === 'windows', 'LAN device slots should preserve discovered platform icons.');
  assert(
    cfg.stroke.cellHoverGlow > cfg.stroke.cell && cfg.opacity.cellHoverGlow > cfg.opacity.cellHover,
    'Hover should add a stronger glowing cell edge.'
  );
  assert(
    makePortalSlots(lanItems, [], 15).filter((slot) => slot.status === 'connected').length === connectedCount,
    'Parent Portal should include connected LAN devices even when no manual portal ids are passed.'
  );
  assert(
    cfg.layout.addButtonSize >= 24 && cfg.layout.addButtonInset === 0,
    'Add button should be prominent and sit on the bottom-right cell edge.'
  );
  assert(
    cfg.layout.addButtonCutoutPad > 0 && cfg.colors.addButtonCutout.includes('rgba'),
    'Add button should have a dark cutout backing.'
  );
  assert(
    cfg.colors.addButtonEdge === cfg.colors.connected && cfg.stroke.addButtonGlow > cfg.stroke.addButton,
    'Add button should have a thin green edge/glow treatment.'
  );
  assert(
    bottomRoundRectPath(0, 0, 100, 20, 8).startsWith('M0 0H100V'),
    'Bottom-rounded rectangle helper should keep straight top corners for optional overrides.'
  );
  assert(
    topRoundRectPath(0, 0, 100, 20, 8).endsWith('V20Z'),
    'Grid outer box should have straight bottom corners and rounded top corners.'
  );
  assert(
    cfg.statusOrder.lan.includes('available') && cfg.statusOrder.parent.includes('empty'),
    'Legend order should be configurable by scope.'
  );
  assert(
    cfg.ids.root.length > 0 && cfg.effects.selectedInset > 0,
    'SVG ids and key geometry values should be configurable for Codex overrides.'
  );
  assert(
    oneRowGridPlan.rows === 1 && oneRowGridPlan.columns === 10,
    'Very wide boxes should allow all ten items in one row.'
  );
  assert(
    compactGridPlan.rows > 1 && compactGridPlan.columns < 10,
    'Compact boxes should add rows instead of shrinking cells below the minimum.'
  );
  assert(
    manyDevicePlan.rows === 5 && manyDevicePlan.columns === 10,
    'Wide fifty-device grids should fit ten columns and overflow vertically.'
  );
  assert(
    manyDeviceLayout.maxScrollY > 0,
    'Overflowing device grids should scroll vertically inside the fixed top-half frame.'
  );
  assert(
    manyDeviceLayout.gridViewportH < manyDeviceLayout.gridContentH,
    'The selected-device footer should stay pinned while the grid content is clipped above it.'
  );
  assert(
    constrainedLayout.viewBoxW > constrainedLayout.svgW || constrainedLayout.viewBoxH > constrainedLayout.svgH,
    'Constrained boxes should preserve cell aspect by growing the viewBox for proportional SVG fitting.'
  );
}
