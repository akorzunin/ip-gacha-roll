import Database from "@tauri-apps/plugin-sql";
import { useState, useEffect } from "react";
import { useQuery } from "react-query";
import { getSettings } from "../db/settings";

export const Settings = () => {
  const [routerIp, setRouterIp] = useState("192.168.1.1");
  const [username, setUsername] = useState("admin");
  const [password, setPassword] = useState("admin");
  const [dryRun, setDryRun] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        setIsOpen(false);
      }
    };

    window.addEventListener("keydown", handleEscape);
    return () => window.removeEventListener("keydown", handleEscape);
  }, []);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    console.log(routerIp, username, password, dryRun);
    const db = await Database.load("sqlite:data.db");
    const result = await db.execute(
      `INSERT OR REPLACE INTO settings (id, router_ip, user, pass, dry_run)
      VALUES (?, ?, ?, ?, ?)
      ON CONFLICT (id) DO UPDATE SET
      router_ip = excluded.router_ip,
      user = excluded.user,
      pass = excluded.pass,
      dry_run = excluded.dry_run
      `,
      [1, routerIp, username, password, dryRun],
    );
    console.log(result);
    setIsOpen(false);
  };

  // @ts-ignore
  const data = useQuery({
    queryKey: ["settings"],
    queryFn: async () => {
      const res = await getSettings();
      console.log(res);
      if (!res) return;
      setRouterIp(res.router_ip);
      setUsername(res.user);
      setPassword(res.pass);
      setDryRun(res.dry_run);
      return res;
    },
  });

  return (
    <div>
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="fixed top-4 right-4 bg-amber-400 hover:bg-amber-500 text-black font-bold p-4 rounded"
      >
        ⚙️ Settings
      </button>
      <div
        className={`fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center ${
          isOpen ? "block" : "hidden"
        }`}
      >
        <div className="bg-amber-400 p-6 rounded-lg shadow-lg max-w-md w-full relative">
          <button
            type="button"
            onClick={() => setIsOpen(false)}
            className="absolute top-2 right-2 text-gray-600 hover:text-gray-800"
          >
            ✕
          </button>
          <form onSubmit={handleSubmit} className="flex flex-col gap-4">
            <div className="flex flex-col gap-2">
              <label htmlFor="routerIp" className="font-medium">
                Router IP:
              </label>
              <input
                type="text"
                id="routerIp"
                placeholder="Router IP"
                value={routerIp}
                onChange={(e) => setRouterIp(e.target.value)}
                className="p-2 rounded border"
              />
            </div>
            <div className="flex flex-col gap-2">
              <label htmlFor="username" className="font-medium">
                Username:
              </label>
              <input
                type="text"
                id="username"
                placeholder="Username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                className="p-2 rounded border"
              />
            </div>
            <div className="flex flex-col gap-2">
              <label htmlFor="password" className="font-medium">
                Password:
              </label>
              <div className="flex max-w-full">
                <input
                  type={showPassword ? "text" : "password"}
                  id="password"
                  placeholder="Password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className="p-2 rounded-l border flex-1 min-w-0"
                />
                <button
                  type="button"
                  className="px-4 bg-white border border-l-0 rounded-r hover:bg-gray-50 shrink-0"
                  onClick={() => setShowPassword(!showPassword)}
                >
                  {showPassword ? "👁️" : "👁️‍🗨️"}
                </button>
              </div>
            </div>
            <div className="flex items-baseline gap-2">
              <label htmlFor="dryRun" className="font-medium">
                Dry Run:
              </label>
              <input
                type="checkbox"
                id="dryRun"
                checked={dryRun}
                onChange={(e) => setDryRun(e.target.checked)}
                className="h-7 w-7"
              />
            </div>
            <div className="flex justify-end gap-2 mt-4">
              <button
                type="submit"
                className="bg-amber-500 hover:bg-amber-600 text-white rounded-md py-2 px-4"
              >
                Save
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
};
