import { invoke } from "@tauri-apps/api/core";
import { useQuery } from "react-query";

interface CheckNatResponse {
  ip: string;
  nat: boolean;
}

interface CheckNatError {
  detail: string;
}

export const CheckNetwork = () => {
  const { data, isLoading, isError, error, refetch, isFetching } = useQuery<
    CheckNatResponse,
    CheckNatError
  >({
    queryKey: ["checkNetwork"],
    queryFn: async () => {
      const res = (await invoke("check_nat")) as
        | CheckNatResponse
        | CheckNatError;
      if ("detail" in res) {
        const err = res as CheckNatError;
        throw new Error(err.detail);
      }
      return res as CheckNatResponse;
    },
    refetchInterval: 10000,
  });

  return (
    <div className="flex items-center gap-4 px-4">
      <div>
        {isLoading && <div>Loading...</div>}
        {isError && <div>Error: {error.detail}</div>}
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
      </div>
      <button
        className="h-14 w-14 rounded-md bg-amber-500"
        onClick={() => refetch()}
      >
        🔃
      </button>
    </div>
  );
};
