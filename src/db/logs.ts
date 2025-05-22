import Database from "@tauri-apps/plugin-sql";
import { QueryClient } from "react-query";

export type LogLevel = "debug" | "info" | "warning" | "error";
export type LogStatus = "success" | "error" | null;

export const LOG_LEVEL_SEVERITY: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warning: 2,
  error: 3,
};

export interface LogEntry {
  id?: number;
  message: string;
  level: LogLevel;
  status?: LogStatus;
  created_at?: string;
}

export async function writeLog(log: LogEntry, client: QueryClient) {
  const db = await Database.load("sqlite:data.db");
  console.info(log);
  await db.execute(
    `INSERT INTO logs (message, level, status) VALUES (?, ?, ?)`,
    [log.message, log.level, log.status],
  );
  client.invalidateQueries({ queryKey: ["logs"] });
}

export async function getLogs(
  minLevel: LogLevel = "debug",
  limit: number = 50,
  offset: number = 0,
) {
  const db = await Database.load("sqlite:data.db");
  const logs = (await db.select(
    `
    SELECT * FROM logs l
    WHERE CASE l.level
      WHEN 'debug' THEN 0
      WHEN 'info' THEN 1
      WHEN 'warning' THEN 2
      WHEN 'error' THEN 3
    END >= CASE ?
      WHEN 'debug' THEN 0
      WHEN 'info' THEN 1
      WHEN 'warning' THEN 2
      WHEN 'error' THEN 3
    END
    ORDER BY l.created_at DESC
    LIMIT ? OFFSET ?
  `,
    [minLevel, limit, offset],
  )) as unknown as LogEntry[];
  return logs;
}
