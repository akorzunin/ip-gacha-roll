import { invoke } from "@tauri-apps/api/core";
import { getSettings } from "../db/settings";
import { LogEntry, writeLog } from "../db/logs";
import { useState } from "react";
import { useQueryClient } from "react-query";

export const Roll = () => {
  const [timesRolled, setTimesRolled] = useState(0);
  const queryClient = useQueryClient();
  return (
    <div className="flex gap-2 items-center">
      <button
        className="bg-amber-500 text-white px-4 py-2 rounded-md"
        onClick={async () => {
          const settings = await getSettings();
          const res = (await invoke("keen_run", {
            settings: settings,
            command: "reroll",
          })) as string;
          const logs = JSON.parse(res) as LogEntry[];
          if (!Array.isArray(logs)) {
            throw new Error(res);
          }
          await writeLog(logs);
          setTimesRolled(timesRolled + 1);
          queryClient.invalidateQueries({ queryKey: ["checkInterface"] });
        }}
      >
        Roll
      </button>
      <p>Times rolled: {timesRolled}</p>
      <p> Pity: {20 - timesRolled}/20</p>
    </div>
  );
};
