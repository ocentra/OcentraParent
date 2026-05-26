import { defaultGridToggleConfig } from './GridToggleConfig';
import { clampGridToggleInt } from './GridToggleGeometry';
import { calculateGridToggleMetrics } from './GridToggleMetrics';
import { getNextGridToggleValue, getSelectedGridToggleIndex, normalizeGridToggleOptions } from './GridToggleOptions';

type AssertFn = (condition: boolean, message: string) => void;

export function runGridToggleSelfTests(assert: AssertFn = console.assert): void {
  const config = defaultGridToggleConfig;
  const options = normalizeGridToggleOptions(undefined, 15, config.text.options);
  const metrics = calculateGridToggleMetrics(config, config.text.title, options, 3, 5);
  const clampedRows = clampGridToggleInt(99, 1, config.layout.maxRows);
  const clampedColumns = clampGridToggleInt(99, 1, config.layout.maxColumns);
  const longOptions = normalizeGridToggleOptions(
    [
      { value: 'device-1', label: 'Very Long Device Name 1' },
      { value: 'device-2', label: 'Very Long Device Name 2' },
    ],
    6,
    config.text.options
  );
  const longMetrics = calculateGridToggleMetrics(config, 'Very Long Grid Title', longOptions, 2, 3);

  assert(options.length === 15, 'Default 3x5 grid should normalize to 15 options.');
  assert(metrics.rowCount === 3, 'Grid should use 3 rows by default preview request.');
  assert(metrics.columnCount === 5, 'Grid should use 5 columns by default preview request.');
  assert(metrics.verticalDividerXs.length === 4, 'Five columns should create four vertical dividers.');
  assert(metrics.horizontalDividerYs.length === 2, 'Three rows should create two horizontal dividers.');
  assert(clampedRows === config.layout.maxRows, 'Rows should clamp to maxRows.');
  assert(clampedColumns === config.layout.maxColumns, 'Columns should clamp to maxColumns.');
  assert(longMetrics.cellWidth > metrics.cellWidth, 'Long labels should grow cell width.');
  assert(longMetrics.titleBoxWidth > metrics.titleBoxWidth, 'Long title should grow title box width.');
  assert(
    getNextGridToggleValue('device-15', options) === 'device-1',
    'Grid helper should cycle from last option to first when used programmatically.'
  );
  assert(
    getSelectedGridToggleIndex('device-7', options) === 6,
    'Direct cell selection should resolve the clicked value to its index.'
  );
}
