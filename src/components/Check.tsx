import { invoke } from "@tauri-apps/api/core";
import { getSettings } from "../db/settings";
import { Interface } from "./Interface";
import { useQuery, useQueryClient } from "react-query";
import { writeLog } from "../db/logs";

export interface CheckData {
  id: string;
  state: string;
  address: string;
  uptime: number;
  ip: string;
}

interface CheckError {
  detail: string;
}

export const Check = () => {
  const settingsQuery = useQuery({
    queryKey: ["settings"],
    queryFn: getSettings,
  });

  const queryClient = useQueryClient();

  const data = useQuery<CheckData, CheckError>({
    queryKey: ["checkInterface"],
    queryFn: async () => {
      const settings = await getSettings();
      const prevData = queryClient.getQueryData<CheckData>(["checkInterface"]);
      const res = (await invoke("keen_run", {
        settings: settings,
        command: "get_interface",
      })) as string;
      const data = JSON.parse(res) as CheckData | CheckError;
      if ("detail" in data) {
        await writeLog(
          {
            message: `Error checking interface: ${data.detail}`,
            level: "error",
            status: "error",
          },
          queryClient,
        );
        throw new Error(data.detail);
      }
      if (data.ip !== prevData?.ip) {
        await writeLog(
          {
            message: `Interface IP changed from ${prevData?.ip} to ${data.ip}`,
            level: "info",
            status: "success",
          },
          queryClient,
        );
      }
      await writeLog(
        {
          message: `Checking interface: ${data.id} uptime: ${data.uptime}`,
          level: "debug",
        },
        queryClient,
      );
      return data;
    },
    refetchInterval: 10000,
  });

  return (
    <div className="z-20 flex justify-between rounded-md bg-amber-400 p-4">
      <div className="w-full">
        {data.isLoading && <div>Loading...</div>}
        {data.isError && <div>Error: {data.error.detail}</div>}
        {data.data && <Interface data={data.data} />}
        {settingsQuery.data && (
          <div
            className={`mt-3 flex items-center gap-2 ${
              !settingsQuery.data.dry_run ? "hidden" : ""
            }`}
          >
            <div
              className={`h-2 w-2 rounded-full ${
                settingsQuery.data.dry_run ? "bg-green-500" : "bg-red-500"
              }`}
            />
            <span className="text-sm font-medium">
              Dry Run:
              <span
                className={`ml-1 ${
                  settingsQuery.data.dry_run ? "text-green-700" : "text-red-700"
                }`}
              >
                {settingsQuery.data.dry_run ? "Active" : "Inactive"}
              </span>
            </span>
          </div>
        )}
      </div>
      <button
        className="h-fit rounded-md bg-amber-500 p-2 text-gray-700 hover:bg-amber-500/80"
        onClick={() => data.refetch()}
      >
        🔄
      </button>
    </div>
  );
};
