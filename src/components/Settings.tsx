import Database from "@tauri-apps/plugin-sql";
import { useState, useEffect } from "react";
import { useQuery, useQueryClient } from "react-query";
import { getSettings } from "../db/settings";

export const Settings = () => {
  const [routerIp, setRouterIp] = useState("192.168.1.1");
  const [username, setUsername] = useState("admin");
  const [password, setPassword] = useState("admin");
  const [dryRun, setDryRun] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [isOpen, setIsOpen] = useState(false);
  const queryClient = useQueryClient();

  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        setIsOpen(false);
      }
    };

    window.addEventListener("keydown", handleEscape);
    return () => window.removeEventListener("keydown", handleEscape);
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
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
    queryClient.invalidateQueries({ queryKey: ["settings"] });
  };

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
        className="fixed top-4 right-4 rounded bg-amber-400 p-4 font-bold text-black hover:bg-amber-500"
      >
        ⚙️ Settings
      </button>
      <div
        className={`bg-opacity-50 fixed inset-0 flex items-center justify-center bg-black ${
          isOpen ? "block" : "hidden"
        }`}
      >
        <div className="relative w-full max-w-md rounded-lg bg-amber-400 p-6 shadow-lg">
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
                className="rounded border p-2"
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
                className="rounded border p-2"
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
                  className="min-w-0 flex-1 rounded-l border p-2"
                />
                <button
                  type="button"
                  className="shrink-0 rounded-r border border-l-0 bg-white px-4 hover:bg-gray-50"
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
            <div className="mt-4 flex justify-end gap-2">
              <button
                type="submit"
                className="rounded-md bg-amber-500 px-4 py-2 text-white hover:bg-amber-600"
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
