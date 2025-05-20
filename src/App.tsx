import { Settings } from "./components/Settings";
import { Check } from "./components/Check";
import { Logs } from "./components/Logs";
import { Roll } from "./components/Roll";
import { CheckNetwork } from "./components/CheckNetwork";

function App() {
  return (
    <main className="flex min-h-screen flex-col gap-6 bg-amber-200 p-8 text-4xl">
      <h1 className="font-bold">IP Gacha</h1>

      <div className="container mx-auto flex flex-col gap-8">
        <Settings />
        <Check />
        <div className="flex items-center justify-between gap-4">
          <Roll />
          <CheckNetwork />
        </div>
        <Logs />
      </div>
    </main>
  );
}

export default App;
