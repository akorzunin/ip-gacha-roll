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
        await writeLog(
          {
            message: `Error rerolling: ${res}`,
            level: "error",
            status: "error",
          },
          queryClient,
        );
        throw new Error(res);
      }
      await writeLog(
        {
          message: `Rerolled ${timesRolled + 1} time${timesRolled + 1 === 1 ? "" : "s"}`,
          level: "info",
          status: "success",
        },
        queryClient,
      );
      setTimesRolled(timesRolled + 1);
      queryClient.invalidateQueries({
        queryKey: ["checkInterface", "checkNetwork"],
      });
    } finally {
      // Add a small delay before stopping the animation
      setTimeout(() => setIsRolling(false), 800);
    }
  };

  return (
    <div className="z-100 flex max-h-14 gap-4">
      <button
        className={`rounded-lg bg-amber-500 px-6 text-2xl font-medium shadow-lg transition-transform duration-200 hover:-translate-y-0.5 hover:bg-amber-600 hover:shadow-amber-500/25 active:bg-amber-700 ${
          isRolling ? "animate-spin" : "animate-wiggle"
        }`}
        onClick={handleRoll}
        disabled={isRolling}
      >
        ROLL
      </button>
      <div className="">
        <div className="flex-col items-center justify-between rounded-lg bg-amber-200/30 px-2 text-xl text-amber-200 backdrop-blur-sm">
          <p className="z-20 font-medium">
            Times rolled:{" "}
            <span className="text-amber-500 text-shadow-lg/30 text-shadow-black">
              {timesRolled}
            </span>
          </p>
          <p className="z-20 font-medium">
            Pity:{" "}
            <span className="text-amber-500 text-shadow-lg/30 text-shadow-black">
              {20 - (timesRolled % 20)}/20
            </span>
          </p>
        </div>
      </div>
    </div>
  );
};
