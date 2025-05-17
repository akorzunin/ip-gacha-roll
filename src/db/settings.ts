import Database from "@tauri-apps/plugin-sql";

export interface Settings {
  id: number;
  router_ip: string;
  user: string;
  pass: string;
}

export async function getSettings(): Promise<Settings | undefined> {
  const db = await Database.load("sqlite:data.db");
  const result = (await db.select("SELECT * FROM settings")) as Settings[];
  if (result.length < 0) return;
  return result[0];
}
