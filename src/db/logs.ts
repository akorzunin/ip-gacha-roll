import Database from "@tauri-apps/plugin-sql";

export type LogLevel = "debug" | "info" | "warning" | "error";
export type LogStatus = "success" | "error" | null;

export interface LogEntry {
  id?: number;
  message: string;
  level: LogLevel;
  status: LogStatus;
  created_at?: string;
}

export async function writeLog(log: LogEntry) {
  const db = await Database.load("sqlite:data.db");
  console.log(log);
  await db.execute(
    `INSERT INTO logs (message, level, status) VALUES (?, ?, ?)`,
    [log.message, log.level, log.status],
  );
}

export async function getLogs() {
  const db = await Database.load("sqlite:data.db");
  const logs = (await db.execute(`
    SELECT * FROM logs
    ORDER BY created_at DESC
    LIMIT 100
  `)) as unknown as LogEntry[];
  return logs;
}
