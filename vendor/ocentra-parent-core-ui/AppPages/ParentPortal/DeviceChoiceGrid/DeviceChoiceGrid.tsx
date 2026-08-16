import { useEffect, useId, useMemo, useState, type CSSProperties, type ReactElement, type WheelEvent } from 'react';
import { defaultDeviceChoiceGridConfig, mergeDeviceChoiceGridConfig } from './DeviceChoiceGridConfig';
import { DeviceChoiceGridCell } from './DeviceChoiceGridCell';
import { DeviceChoiceGridConnectors } from './DeviceChoiceGridConnectors';
import { DeviceChoiceGridDefs } from './DeviceChoiceGridDefs';
import {
  createDeviceChoiceGridGridPlan,
  createDeviceChoiceGridLayout,
  createDeviceChoiceGridShape,
} from './DeviceChoiceGridLayout';
import { DeviceChoiceGridLegend } from './DeviceChoiceGridLegend';
import { DeviceChoiceGridScopeSelector } from './DeviceChoiceGridScopeSelector';
import { DeviceChoiceGridSelectedInfo } from './DeviceChoiceGridSelectedInfo';
import { getLanSlots, makePortalSlots } from './DeviceChoiceGridSlots';
import { DEVICE_CHOICE_DEFAULT_SCOPE_VALUES } from './DeviceChoiceGridTypes';
import type {
  DeviceChoiceGridCellPosition,
  DeviceChoiceGridIds,
  DeviceChoiceGridProps,
  DeviceStatus,
  DeviceSlot,
  ScopeValue,
} from './DeviceChoiceGridTypes';

export function DeviceChoiceGrid({
  value,
  defaultValue,
  scope,
  defaultScope = 'lan',
  portalDeviceIds,
  defaultPortalDeviceIds = [],
  devices,
  slots,
  options,
  rows,
  columns,
  parentRows,
  parentColumns,
  disabled = false,
  deviceSelectionDisabled = false,
  showScopeSelector = true,
  showAddControls = true,
  scopeValues,
  className,
  style,
  onChange,
  onScopeChange,
  onAddToPortal,
  onEditDevice,
  scopeIcons,
  config: override,
}: DeviceChoiceGridProps): ReactElement {
  const cfg = useMemo(() => mergeDeviceChoiceGridConfig(defaultDeviceChoiceGridConfig, override), [override]);
  const rawUid = useId();
  const uid = `${cfg.ids.root}-${rawUid.replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const ids: DeviceChoiceGridIds = {
    cell: `${uid}-${cfg.ids.cell}`,
    selected: `${uid}-${cfg.ids.selected}`,
    shine: `${uid}-${cfg.ids.shine}`,
    glow: `${uid}-${cfg.ids.glow}`,
    gridClip: `${uid}-grid-clip`,
    titleGlow: `${uid}-${cfg.ids.titleGlow}`,
    selectedGlow: `${uid}-${cfg.ids.selectedGlow}`,
  };

  const [hover, setHover] = useState(false);
  const [hovered, setHovered] = useState<number | null>(null);
  const [pressed, setPressed] = useState<number | null>(null);
  const [scrollY, setScrollY] = useState(0);
  const [internalScope, setInternalScope] = useState<ScopeValue>(defaultScope);
  const [internalPortalIds, setInternalPortalIds] = useState<string[]>(defaultPortalDeviceIds);
  const [internalValue, setInternalValue] = useState<string | null>(() => defaultValue ?? null);

  const explicitLanSourceCount =
    slots !== undefined
      ? slots.length
      : options !== undefined
        ? options.length
        : devices !== undefined
          ? devices.length
          : cfg.text.options.length;
  const lanSourceCount = Math.max(explicitLanSourceCount, cfg.layout.columns);
  const currentScope = scope ?? internalScope;
  const activeScopeValues = scopeValues ?? DEVICE_CHOICE_DEFAULT_SCOPE_VALUES;
  const lanPlan = createDeviceChoiceGridGridPlan({
    availableH: cfg.svg.height,
    availableW: cfg.svg.width,
    cfg,
    fallbackColumns: cfg.layout.columns,
    fallbackRows: cfg.layout.rows,
    itemCount: lanSourceCount,
    legendCount: cfg.statusOrder.lan.length,
    requestedColumns: columns,
    requestedRows: rows,
  });
  const portalPlan = createDeviceChoiceGridGridPlan({
    availableH: cfg.svg.height,
    availableW: cfg.svg.width,
    cfg,
    fallbackColumns: cfg.layout.parentColumns,
    fallbackRows: cfg.layout.parentRows,
    itemCount: cfg.layout.parentRows * cfg.layout.parentColumns,
    legendCount: cfg.statusOrder.parent.length,
    requestedColumns: parentColumns,
    requestedRows: parentRows,
  });
  const shape = createDeviceChoiceGridShape({
    cfg,
    currentScope,
    lanRows: lanPlan.rows,
    lanColumns: lanPlan.columns,
    portalRows: portalPlan.rows,
    portalColumns: portalPlan.columns,
  });

  const rawLanItems = useMemo(
    () => getLanSlots(slots, devices, options, shape.lanTotalSlots, cfg.text.options),
    [slots, devices, options, shape.lanTotalSlots, cfg.text.options]
  );
  const activePortalIds = portalDeviceIds ?? internalPortalIds;
  const lanItems = useMemo(
    () =>
      rawLanItems.map((slot) =>
        activePortalIds.includes(slot.value) && slot.status === 'available'
          ? { ...slot, status: 'connected' as DeviceStatus }
          : slot
      ),
    [rawLanItems, activePortalIds]
  );
  const items = useMemo(
    () =>
      currentScope === 'parent' || currentScope === 'portal'
        ? makePortalSlots(lanItems, activePortalIds, shape.totalSlots)
        : lanItems,
    [currentScope, lanItems, activePortalIds, shape.totalSlots]
  );

  useEffect(() => {
    if (value !== undefined || internalValue === null) {
      return;
    }

    const selectedItem = items.find((slot) => slot.value === internalValue);
    if (!selectedItem || selectedItem.status === 'empty') {
      setInternalValue(null);
    }
  }, [internalValue, items, value]);

  useEffect(() => {
    if (value !== undefined || internalValue !== null) {
      return;
    }

    const firstSelectable = firstSelectableDeviceSlot(items);
    if (firstSelectable) {
      setInternalValue(firstSelectable.value);
    }
  }, [internalValue, items, value]);

  const selectedValue = value ?? internalValue;
  const selectedIndexRaw = selectedValue ? items.findIndex((slot) => slot.value === selectedValue) : -1;
  const selectedIndex = selectedIndexRaw >= 0 ? selectedIndexRaw : -1;
  const selected = selectedIndex >= 0 ? (items[selectedIndex] ?? null) : null;
  const layout = createDeviceChoiceGridLayout(cfg, shape, items, activeScopeValues);
  const clampedScrollY = Math.min(scrollY, layout.maxScrollY);

  useEffect(() => {
    setScrollY((current) => Math.min(current, layout.maxScrollY));
  }, [layout.maxScrollY, currentScope, items.length]);

  const cellPos = (index: number): DeviceChoiceGridCellPosition => ({
    row: Math.floor(index / shape.columnCount),
    col: index % shape.columnCount,
    x: layout.activeGridX + (index % shape.columnCount) * (layout.cellW + cfg.layout.gapX),
    y: layout.activeGridY + Math.floor(index / shape.columnCount) * (layout.cellH + cfg.layout.gapY),
  });

  const select = (index: number) => {
    const item = items[index];
    if (disabled || deviceSelectionDisabled || !item || item.status === 'empty') {
      return;
    }
    const position = cellPos(index);
    if (value === undefined) {
      setInternalValue(item.value);
    }
    onChange?.(item, index, position.row, position.col);
  };

  const addToPortal = (slot: DeviceSlot) => {
    if (!canAddToPortal(slot)) {
      return;
    }
    const nextIds = activePortalIds.includes(slot.value) ? activePortalIds : [...activePortalIds, slot.value];
    if (!portalDeviceIds) {
      setInternalPortalIds(nextIds);
    }
    onAddToPortal?.(slot, nextIds);
    if (!scope) {
      setInternalScope('parent');
    }
    onScopeChange?.('parent');
  };

  const canAddToPortal = (slot: DeviceSlot): boolean =>
    slot.status === 'available' && slot.device?.portalEligible !== false;

  const editDevice = (slot: DeviceSlot) => {
    if (disabled || slot.status === 'empty' || !slot.device) {
      return;
    }
    onEditDevice?.(slot);
  };

  const selectScope = (scopeValue: ScopeValue) => {
    if (!scope) {
      setInternalScope(scopeValue);
    }
    onScopeChange?.(scopeValue);
  };
  const selectScopeFromOverlay = (scopeValue: ScopeValue) => {
    if (disabled) {
      return;
    }
    selectScope(scopeValue);
  };

  const onWheel = (event: WheelEvent<SVGSVGElement>) => {
    if (layout.maxScrollY <= 0) {
      return;
    }
    event.preventDefault();
    setScrollY((current) => Math.max(0, Math.min(layout.maxScrollY, current + event.deltaY)));
  };

  const legendItems: DeviceStatus[] = cfg.statusOrder[currentScope];
  const cellsDisabled = disabled || deviceSelectionDisabled;
  const root: CSSProperties = {
    width: layout.svgW,
    height: layout.svgH,
    position: 'relative',
    opacity: disabled ? cfg.opacity.disabled : 1,
    transition: cfg.transition.root,
    cursor: disabled ? 'not-allowed' : 'default',
    ...style,
  };
  const viewBoxWWithInset = layout.viewBoxW + cfg.svg.inset * 2;
  const viewBoxHWithInset = layout.viewBoxH + cfg.svg.inset * 2;
  const svgScaleX = layout.svgW / viewBoxWWithInset;
  const svgScaleY = layout.svgH / viewBoxHWithInset;
  const scopeOverlayStyle: CSSProperties = {
    position: 'absolute',
    left: (layout.titleX + cfg.svg.inset) * svgScaleX,
    top: (cfg.layout.titleY + cfg.svg.inset) * svgScaleY,
    width: layout.titleW * svgScaleX,
    height: cfg.layout.titleH * svgScaleY,
    display: 'flex',
    pointerEvents: disabled ? 'none' : 'auto',
  };
  const scopeOverlayButtonStyle: CSSProperties = {
    appearance: 'none',
    background: 'transparent',
    border: 0,
    cursor: disabled ? 'not-allowed' : 'pointer',
    height: '100%',
    margin: 0,
    opacity: 0,
    padding: 0,
    width: layout.scopeOptionW * svgScaleX,
  };
  const outerOpacity = hover ? cfg.opacity.outerHover : cfg.opacity.outer;
  const outerGlowOpacity = hover ? cfg.opacity.outerGlowHover : cfg.opacity.outerGlow;
  const titleGlowOpacity = hover ? cfg.opacity.titleGlowHover : cfg.opacity.titleGlow;
  const selectedGlowOpacity = hover ? cfg.opacity.selectedGlowHover : cfg.opacity.selectedGlow;
  const shineOpacity = hover ? cfg.opacity.selectedShineHover : cfg.opacity.selectedShine;

  return (
    <div
      className={className}
      style={root}
      onPointerEnter={() => setHover(true)}
      onPointerLeave={() => {
        setHover(false);
        setHovered(null);
        setPressed(null);
      }}
    >
      <svg
        viewBox={`${-cfg.svg.inset} ${-cfg.svg.inset} ${layout.viewBoxW + cfg.svg.inset * 2} ${layout.viewBoxH + cfg.svg.inset * 2}`}
        width={layout.svgW}
        height={layout.svgH}
        preserveAspectRatio="xMidYMin meet"
        role="img"
        aria-label={`${cfg.text.scopeOptions[currentScope]}: ${selected?.label ?? 'no device selected'}`}
        onWheel={onWheel}
      >
        <defs>
          <clipPath id={ids.gridClip}>
            <rect
              x={layout.gridViewportX}
              y={layout.gridViewportY}
              width={layout.gridViewportW}
              height={layout.gridViewportH}
              rx={Math.max(0, cfg.radius.cell - 1)}
            />
          </clipPath>
        </defs>
        {cfg.debug.showBounds ? (
          <rect
            x={0}
            y={0}
            width={layout.viewBoxW}
            height={layout.viewBoxH}
            fill="none"
            stroke={cfg.debug.boundsColor}
            strokeWidth={cfg.debug.boundsStroke}
            strokeDasharray={cfg.debug.boundsDash}
            opacity={cfg.debug.boundsOpacity}
            pointerEvents="none"
          />
        ) : null}

        <DeviceChoiceGridDefs
          cfg={cfg}
          ids={ids}
          outerGlowOpacity={outerGlowOpacity}
          selectedGlowOpacity={selectedGlowOpacity}
          titleGlowOpacity={titleGlowOpacity}
        />
        <DeviceChoiceGridLegend cfg={cfg} statuses={legendItems} />
        <path
          d={layout.gridOuterPath}
          fill="none"
          stroke={cfg.colors.outerGlow}
          strokeWidth={cfg.stroke.outerGlow}
          opacity={outerGlowOpacity}
          filter={`url(#${ids.glow})`}
        />
        <path
          d={layout.gridOuterPath}
          fill="none"
          stroke={cfg.colors.outer}
          strokeWidth={cfg.stroke.outer}
          opacity={outerOpacity}
        />
        {showScopeSelector ? (
          <DeviceChoiceGridScopeSelector
            cfg={cfg}
            currentScope={currentScope}
            disabled={disabled}
            hover={hover}
            ids={ids}
            scopeOptionW={layout.scopeOptionW}
            scopeSliderX={layout.scopeSliderX}
            titleW={layout.titleW}
            titleX={layout.titleX}
            scopeValues={activeScopeValues}
            {...(scopeIcons ? { scopeIcons } : {})}
          />
        ) : null}
        <DeviceChoiceGridConnectors
          cfg={cfg}
          ids={ids}
          activeGridY={layout.activeGridY}
          cellH={layout.cellH}
          chainClipId={ids.gridClip}
          columnCount={shape.columnCount}
          connectorY={layout.connectorY}
          rowCount={shape.rowCount}
          scrollY={clampedScrollY}
          titleBottom={layout.titleBottom}
          titleCenterX={layout.titleX + layout.titleW / 2}
          topCenters={layout.topCenters}
        />
        <DeviceChoiceGridSelectedInfo
          cfg={cfg}
          ids={ids}
          infoX={layout.gridOuterX}
          infoY={layout.infoY}
          infoW={layout.gridOuterW}
          selected={selected}
        />

        <rect
          x={layout.gridViewportX}
          y={layout.gridViewportY}
          width={layout.gridViewportW}
          height={layout.gridViewportH}
          fill="transparent"
          pointerEvents={layout.maxScrollY > 0 ? 'all' : 'none'}
        />
        <g clipPath={`url(#${ids.gridClip})`}>
          <g transform={clampedScrollY > 0 ? `translate(0 ${-clampedScrollY})` : undefined}>
            {items.map((item, index) => {
              const status = item.status ?? 'available';
              const active = index === selectedIndex && status !== 'empty';
              return (
                <DeviceChoiceGridCell
                  key={item.value}
                  active={active}
                  cellH={layout.cellH}
                  cellW={layout.cellW}
                  cfg={cfg}
                  disabled={cellsDisabled}
                  ids={ids}
                  index={index}
                  item={item}
                  over={index === hovered}
                  position={cellPos(index)}
                  press={index === pressed}
                  shineOpacity={shineOpacity}
                  showAdd={
                    showAddControls &&
                    currentScope === 'lan' &&
                    active &&
                    canAddToPortal(item) &&
                    !activePortalIds.includes(item.value)
                  }
                  showEdit={false}
                  onAddToPortal={addToPortal}
                  onEditDevice={editDevice}
                  onHoverChange={setHovered}
                  onPressChange={setPressed}
                  onSelect={select}
                />
              );
            })}
          </g>
        </g>
        {layout.maxScrollY > 0 ? (
          <rect
            x={layout.gridViewportX + layout.gridViewportW - 3}
            y={
              layout.gridViewportY +
              (layout.gridViewportH -
                Math.max(18, layout.gridViewportH * (layout.gridViewportH / layout.gridContentH))) *
                (clampedScrollY / layout.maxScrollY)
            }
            width={1.5}
            height={Math.max(18, layout.gridViewportH * (layout.gridViewportH / layout.gridContentH))}
            rx={0.75}
            fill={cfg.colors.outer}
            opacity={0.45}
            pointerEvents="none"
          />
        ) : null}
      </svg>
      {showScopeSelector ? (
        <div style={scopeOverlayStyle}>
          {activeScopeValues.map((scopeValue) => (
            <button
              key={`scope-target:${scopeValue}`}
              type="button"
              aria-label={`Select ${cfg.text.scopeOptions[scopeValue]}`}
              disabled={disabled}
              onPointerDown={(event) => {
                event.preventDefault();
                event.stopPropagation();
                selectScopeFromOverlay(scopeValue);
              }}
              onClick={(event) => {
                event.stopPropagation();
                selectScopeFromOverlay(scopeValue);
              }}
              style={scopeOverlayButtonStyle}
            />
          ))}
        </div>
      ) : null}
    </div>
  );
}

function firstSelectableDeviceSlot(items: readonly DeviceSlot[]): DeviceSlot | undefined {
  return items.find((slot) => slot.status !== 'empty');
}
