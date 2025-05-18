import Database from "@tauri-apps/plugin-sql";
import { useState } from "react";
import { useQuery } from "react-query";
import { getSettings } from "../db/settings";

export const Settings = () => {
  const [routerIp, setRouterIp] = useState("192.168.1.1");
  const [username, setUsername] = useState("admin");
  const [password, setPassword] = useState("admin");
  const [showPassword, setShowPassword] = useState(false);
  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    console.log(routerIp, username, password);
    const db = await Database.load("sqlite:data.db");
    const result = await db.execute(
      `INSERT OR REPLACE INTO settings (id, router_ip, user, pass)
      VALUES (?, ?, ?, ?)
      ON CONFLICT (id) DO UPDATE SET
      router_ip = excluded.router_ip,
      user = excluded.user,
      pass = excluded.pass
      `,
      [1, routerIp, username, password]
    );
    console.log(result);
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
      return res;
    },
  });
  return (
    <div className="bg-amber-400">
      <form onSubmit={handleSubmit} className="flex flex-col gap-2">
        <div className="flex gap-4">
          <label htmlFor="routerIp">Router IP: </label>
          <input
            type="text"
            placeholder="Router IP"
            value={routerIp}
            onChange={(e) => setRouterIp(e.target.value)}
          />
        </div>
        <div className="flex gap-4">
          <label htmlFor="username">Username: </label>
          <input
            type="text"
            placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          />
        </div>
        <div className="flex gap-4">
          <label htmlFor="password">Password: </label>
          <div className="flex">
            <input
              type={showPassword ? "text" : "password"}
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
            <button
              type="button"
              className="px-2"
              onClick={() => setShowPassword(!showPassword)}
            >
              {showPassword ? "👁️" : "👁️‍🗨️"}
            </button>
          </div>
        </div>
        <button
          type="submit"
          className="bg-amber-500 rounded-md p-2 max-w-fit px-4"
        >
          Save
        </button>
      </form>
    </div>
  );
};
