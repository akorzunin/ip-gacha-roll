import { invoke } from "@tauri-apps/api/core";
import { useQuery, useQueryClient } from "react-query";
import { writeLog } from "../db/logs";

interface CheckNatResponse {
  ip: string;
  nat: boolean;
}

interface CheckNatError {
  detail: string;
}

export const CheckNetwork = () => {
  const queryClient = useQueryClient();
  const { data, isLoading, isError, error, refetch, isFetching } = useQuery<
    CheckNatResponse | null,
    CheckNatError
  >({
    queryKey: ["checkNetwork"],
    queryFn: async () => {
      const prevData = queryClient.getQueryData<CheckNatResponse>([
        "checkNetwork",
      ]);
      const res = (await invoke("check_nat")) as
        | CheckNatResponse
        | CheckNatError;
      console.info(res);
      if ("detail" in res) {
        const err = res as CheckNatError;
        await writeLog(
          {
            message: `Error checking network: ${err.detail}`,
            level: "error",
          },
          queryClient,
        );
        return null;
      }
      const status =
        res.nat === prevData?.nat ? null : res.nat ? "success" : "error";
      await writeLog(
        {
          message: `Checking network: ${JSON.stringify(res)}`,
          level: status === "success" ? "info" : "debug",
          status,
        },
        queryClient,
      );
      queryClient.invalidateQueries({
        queryKey: ["logs"],
      });
      return res as CheckNatResponse;
    },
    refetchInterval: 10000,
  });

  return (
    <div className="flex items-center gap-4 px-4">
      <div>
        {isLoading && <div>Loading...</div>}
        {data && (
          <div>
            <p>IP: {data.ip}</p>
            <p>
              NAT:{" "}
              <span className={data.nat ? "text-green-500" : "text-red-500"}>
                {data.nat ? "OK" : "Not OK"}
              </span>
              {isFetching && <span className="text-amber-500"> ...</span>}
            </p>
          </div>
        )}
        {isError && (
          <div className="max-w-xs text-sm wrap-break-word text-red-500">
            Error: {error.detail}
          </div>
        )}
      </div>
      <button
        className="h-14 w-14 rounded-md bg-amber-500"
        onClick={() => refetch()}
      >
        🔄
      </button>
    </div>
  );
};
