import { CheckData } from "./Check";
import { useState, useEffect } from "react";

function parseUpTime(uptime: number): string {
  if (Number.isNaN(uptime)) {
    return "Unknown";
  }
  const days = Math.floor(uptime / 86400);
  const hours = Math.floor((uptime % 86400) / 3600);
  const minutes = Math.floor((uptime % 3600) / 60);
  const seconds = uptime % 60;
  return `${days}d ${hours}h ${minutes}m ${seconds}s`;
}

export const Interface: React.FC<{ data: CheckData }> = ({ data }) => {
  const [optimisticUptime, setOptimisticUptime] = useState(data.uptime);

  useEffect(() => {
    setOptimisticUptime(data.uptime);
    const interval = setInterval(() => {
      setOptimisticUptime((prev) => prev + 1);
    }, 1000);
    return () => clearInterval(interval);
  }, [data.uptime]);

  return (
    <div className="flex flex-col gap-2 bg-amber-400">
      <p>Interface: {data.id}</p>
      <p>
        Status:{" "}
        <span
          className={`font-semibold ${
            data.state === "up" ? "text-green-500" : "text-red-500"
          }`}
        >
          {data.state}
        </span>
      </p>
      <p>Uptime: {parseUpTime(optimisticUptime)}</p>
    </div>
  );
};
