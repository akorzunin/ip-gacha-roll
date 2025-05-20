import { invoke } from "@tauri-apps/api/core";
import { getSettings } from "../db/settings";
import { LogEntry, writeLog } from "../db/logs";
import { useState } from "react";
import { useQueryClient } from "react-query";

export const Roll = () => {
  const [timesRolled, setTimesRolled] = useState(0);
  const [isRolling, setIsRolling] = useState(false);
  const queryClient = useQueryClient();

  const handleRoll = async () => {
    setIsRolling(true);
    try {
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
      queryClient.invalidateQueries({ queryKey: ["checkNetwork"] });
    } finally {
      // Add a small delay before stopping the animation
      setTimeout(() => setIsRolling(false), 800);
    }
  };

  return (
    <div className="flex gap-4 rounded-lg bg-white/5 backdrop-blur-sm">
      <button
        className={`animate-wiggle rounded-lg bg-amber-500 px-6 text-2xl font-medium shadow-lg transition-transform duration-200 hover:-translate-y-0.5 hover:bg-amber-600 hover:shadow-amber-500/25 active:bg-amber-700 ${
          isRolling ? "animate-spin" : ""
        }`}
        onClick={handleRoll}
        disabled={isRolling}
      >
        ROLL
      </button>
      <div className="flex-col items-center justify-between text-xl">
        <p className="font-medium">
          Times rolled: <span className="text-amber-600">{timesRolled}</span>
        </p>
        <p className="font-medium">
          Pity:{" "}
          <span className="text-amber-600">{20 - (timesRolled % 20)}/20</span>
        </p>
      </div>
    </div>
  );
};
