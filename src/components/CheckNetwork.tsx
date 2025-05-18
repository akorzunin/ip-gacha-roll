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
  });

  return (
    <div>
      <button
        className="bg-amber-500 text-cyan-700 p-2 rounded-md"
        onClick={() => refetch()}
      >
        Check Network
      </button>
      {isLoading && <div>Loading...</div>}
      {isFetching && <div>Fetching...</div>}
      {isError && <div>Error: {error.detail}</div>}
      {data && (
        <div>
          <p>IP: {data.ip}</p>
          <p>NAT: {data.nat ? "Yes" : "No"}</p>
        </div>
      )}
    </div>
  );
};
