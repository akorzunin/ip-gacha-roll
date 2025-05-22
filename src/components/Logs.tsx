import { useState, useRef, useEffect } from "react";
import { getLogs, LogEntry, LogLevel, LogStatus } from "../db/logs";
import { useInfiniteQuery } from "react-query";

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
  const [showFilter, setShowFilter] = useState(false);
  const [selectedLevel, setSelectedLevel] = useState<LogLevel>("debug");
  const logsRef = useRef<HTMLDivElement>(null);
  const observerRef = useRef<IntersectionObserver | null>(null);
  const loadMoreRef = useRef<HTMLDivElement>(null);

  const { data, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useInfiniteQuery({
      queryKey: ["logs", selectedLevel],
      queryFn: async ({ pageParam = 0 }) => {
        const logs = await getLogs(selectedLevel, 50, pageParam * 50);
        return logs;
      },
      getNextPageParam: (lastPage, allPages) => {
        return lastPage.length === 50 ? allPages.length : undefined;
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

  useEffect(() => {
    if (!isExpanded) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasNextPage && !isFetchingNextPage) {
          fetchNextPage();
        }
      },
      {
        root: null,
        rootMargin: "100px",
        threshold: 0.5,
      },
    );

    if (loadMoreRef.current) {
      observer.observe(loadMoreRef.current);
    }

    observerRef.current = observer;

    return () => {
      if (observerRef.current) {
        observerRef.current.disconnect();
      }
    };
  }, [hasNextPage, isFetchingNextPage, fetchNextPage, isExpanded]);

  const allLogs = data?.pages.flat() ?? [];
  const lastLog = allLogs[0];

  return (
    <div className="z-100 px-4">
      <div className="flex items-center justify-between text-xl font-semibold text-amber-50">
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="rounded-t-md bg-black px-4"
        >
          <span>{isExpanded ? "⌄" : ">"}</span>Logs:
        </button>
        <div className="relative">
          <button
            onClick={() => setShowFilter(!showFilter)}
            className="rounded-t-md bg-black px-6"
          >
            Filter {selectedLevel !== "debug" && `(${selectedLevel})`}
          </button>
          {showFilter && (
            <div className="absolute right-0 mt-1 w-32 rounded-md bg-black p-2 shadow-lg">
              {["debug", "info", "warning", "error"].map((level) => (
                <button
                  key={level}
                  onClick={() => {
                    setSelectedLevel(level as LogLevel);
                    setShowFilter(false);
                  }}
                  className={`block w-full px-4 py-2 text-left text-sm ${
                    selectedLevel === level
                      ? "text-amber-50"
                      : getLevelColor(level as LogLevel)
                  }`}
                >
                  {level.charAt(0).toUpperCase() + level.slice(1)}
                </button>
              ))}
            </div>
          )}
        </div>
      </div>
      <div
        ref={logsRef}
        className={`rounded-b-lg bg-black px-4 py-6 font-mono text-sm transition-discrete duration-300 ${isExpanded ? "h-[400px]" : "h-fit"}`}
      >
        {isExpanded ? (
          <div className="h-full space-y-2 overflow-y-scroll">
            {allLogs.map((log) => (
              <LogLine key={log.id} log={log} />
            ))}
            <div ref={loadMoreRef} className="h-4">
              {isFetchingNextPage && (
                <div className="text-center text-gray-500">Loading more...</div>
              )}
            </div>
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
