import { Settings } from "./components/Settings";
import { Check } from "./components/Check";
import { Logs } from "./components/Logs";
import { Roll } from "./components/Roll";
import { CheckNetwork } from "./components/CheckNetwork";

function App() {
  return (
    <main className="bg-amber-200 text-4xl flex flex-col gap-6 min-h-screen p-8">
      <h1 className="font-bold">IP Gacha</h1>

      <div className="container mx-auto flex flex-col gap-8">
        <Settings />
        <Check />
        <Roll />
        <CheckNetwork />
        <Logs />
      </div>
    </main>
  );
}

export default App;
