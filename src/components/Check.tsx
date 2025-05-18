import { invoke } from "@tauri-apps/api/core";
import { getSettings } from "../db/settings";
import { Interface } from "./Interface";
import { useQuery } from "react-query";

export interface CheckData {
  id: string;
  state: string;
  address: string;
  uptime: number;
}

interface CheckError {
  detail: string;
}

export const Check = () => {
  const data = useQuery<CheckData, CheckError>({
    queryKey: ["checkInterface"],
    queryFn: async () => {
      const settings = await getSettings();
      const res = (await invoke("keen_run", {
        settings: settings,
        command: "get_interface",
      })) as string;
      const data = JSON.parse(res) as CheckData | CheckError;
      console.log(data);
      if ("detail" in data) {
        throw new Error(data.detail);
      }
      return data;
    },
    refetchInterval: 10000,
  });
  return (
    <div>
      <button
        className="bg-amber-500 text-gray-700 p-2 rounded-md"
        onClick={() => data.refetch()}
      >
        Check interface
      </button>
      {data.isLoading && <div>Loading...</div>}
      {data.isError && <div>Error: {data.error.detail}</div>}
      {data.data && <Interface data={data.data} />}
    </div>
  );
};
