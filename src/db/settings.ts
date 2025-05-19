import Database from "@tauri-apps/plugin-sql";

export interface Settings {
  id: number;
  router_ip: string;
  user: string;
  pass: string;
  dry_run: boolean;
}

export async function getSettings(): Promise<Settings | undefined> {
  const db = await Database.load("sqlite:data.db");
  const result = (await db.select("SELECT * FROM settings")) as Settings[];
  if (result.length < 0) return;
  const r = result[0];
  // @ts-ignore
  r.dry_run = r.dry_run === "true";
  return r;
}
