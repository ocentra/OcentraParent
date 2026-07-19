import { ParentHostBridgeRuntime } from '../generated/parent-ui-bridge';

export function parentRouteRfc3339TimestampMs(value: unknown): number | null {
  if (typeof value !== ParentHostBridgeRuntime.StringType) return null;
  const timestamp = String(value);
  const match = /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})(?:\.\d+)?(?:Z|([+-])(\d{2}):(\d{2}))$/.exec(
    timestamp
  );
  if (match === null) return null;
  const [, yearText, monthText, dayText, hourText, minuteText, secondText, , offsetHourText, offsetMinuteText] = match;
  const year = Number(yearText);
  const month = Number(monthText);
  const day = Number(dayText);
  const hour = Number(hourText);
  const minute = Number(minuteText);
  const second = Number(secondText);
  const offsetHour = offsetHourText === undefined ? 0 : Number(offsetHourText);
  const offsetMinute = offsetMinuteText === undefined ? 0 : Number(offsetMinuteText);
  const calendar = new Date(Date.UTC(year, month - 1, day));
  if (
    ![
      calendar.getUTCFullYear() === year,
      calendar.getUTCMonth() === month - 1,
      calendar.getUTCDate() === day,
      hour <= 23,
      minute <= 59,
      second <= 59,
      offsetHour <= 23,
      offsetMinute <= 59,
    ].every(Boolean)
  )
    return null;
  const parsed = Date.parse(timestamp);
  return Number.isFinite(parsed) ? parsed : null;
}
