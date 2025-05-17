import { CheckData } from "./Check";

function parseUpTime(uptime: number): string {
  const days = Math.floor(uptime / 86400);
  const hours = Math.floor((uptime % 86400) / 3600);
  const minutes = Math.floor((uptime % 3600) / 60);
  const seconds = uptime % 60;
  return `${days}d ${hours}h ${minutes}m ${seconds}s`;
}

export const Interface: React.FC<{ data: CheckData }> = ({ data }) => {
  return (
    <div className="bg-amber-400 flex flex-col gap-2">
      <p>Interface: {data.id}</p>
      <p>
        Status:{" "}
        <span
          className={`${
            data.state === "up" ? "text-green-500" : "text-red-500"
          }`}
        >
          {data.state}
        </span>
      </p>
      <p>
        IP Address: <span className="text-blue-500">{data.address}</span>
      </p>
      <p>uptime: {parseUpTime(data.uptime)}</p>
    </div>
  );
};
