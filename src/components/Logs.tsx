import { useState, useRef, useEffect } from "react";
import { getLogs, LogEntry, LogLevel, LogStatus } from "../db/logs";
import { useQuery } from "react-query";

function getLevelColor(level: LogLevel) {
  switch (level) {
    case "debug":
      return "text-blue-400";
    case "info":
      return "text-green-400";
    case "warning":
      return "text-yellow-400";
    case "error":
      return "text-red-400";
    default:
      return "text-gray-400";
  }
}

function getStatusEmoji(status: LogStatus) {
  switch (status) {
    case "success":
      return "✅";
    case "error":
      return "❌";
    default:
      return "";
  }
}

export const Logs = () => {
  const [isExpanded, setIsExpanded] = useState(false);
  const logsRef = useRef<HTMLDivElement>(null);

  const data = useQuery({
    queryKey: ["logs"],
    queryFn: async () => {
      const logs = await getLogs();
      return logs;
    },
    refetchInterval: 1000,
  });

  useEffect(() => {
    if (isExpanded) {
      setTimeout(() => {
        logsRef.current?.scrollIntoView({ behavior: "smooth" });
      }, 300);
    } else {
      window.scrollTo({
        top: 0,
        behavior: "smooth",
      });
    }
  }, [isExpanded]);

  const lastLog = data.data?.[data.data.length - 1];

  return (
    <div className="z-100 px-4">
      <div className="flex items-center justify-between text-xl font-semibold text-amber-50">
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="rounded-t-md bg-black px-4"
        >
          <span>{isExpanded ? "⌄" : ">"}</span>Logs:
        </button>
        <button className="rounded-t-md bg-black px-6">Filter</button>
      </div>
      <div
        ref={logsRef}
        className={`rounded-b-lg bg-black px-4 py-6 font-mono text-sm transition-discrete duration-300 ${isExpanded ? "h-[400px]" : "h-fit"}`}
      >
        {isExpanded && Array.isArray(data.data) ? (
          <div className="h-full space-y-2 overflow-y-scroll">
            {data.data.map((log) => (
              <LogLine key={log.id} log={log} />
            ))}
          </div>
        ) : (
          <div className="rounded-lg bg-black px-4">
            <div className="">
              <LogLine log={lastLog} />
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

const LogLine = ({ log }: { log: LogEntry | undefined }) => {
  if (!log) return null;

  return (
    <div className="flex gap-2 text-gray-300">
      <span className="text-nowrap text-gray-500">[{log.created_at}]</span>
      <span className={getLevelColor(log.level)}>
        [{log?.level?.toUpperCase()}]
      </span>
      {log.message}
      {log.status && <span>{getStatusEmoji(log.status)}</span>}
    </div>
  );
};
