import Database from "@tauri-apps/plugin-sql";
import { QueryClient } from "react-query";

export type LogLevel = "debug" | "info" | "warning" | "error";
export type LogStatus = "success" | "error" | null;

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

export async function getLogs() {
  const db = await Database.load("sqlite:data.db");
  const logs = (await db.select(`
    SELECT * FROM logs l
    ORDER BY l.created_at DESC
    LIMIT 100
  `)) as unknown as LogEntry[];
  return logs;
}
