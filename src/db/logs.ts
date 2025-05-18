import Database from "@tauri-apps/plugin-sql";

export interface LogEntry {
  parse: {
    prompt: string;
    status: {
      status: string;
      code: string;
      ident: string;
      source: string;
      warning: string;
      message: string;
    }[];
  };
}

export async function writeLog(logs: LogEntry[]) {
  // @ts-ignore
  const db = await Database.load("sqlite:data.db");
  console.log(logs);
  // TODO: parse logs and save to db
  //   await db.insert("logs", { logs });
}
