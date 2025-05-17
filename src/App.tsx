import { Settings } from "./components/Settings";
import { Check } from "./components/Check";
import { Logs } from "./components/Logs";
import { Roll } from "./components/Roll";

function App() {
  return (
    <main className="bg-amber-200 text-4xl flex flex-col gap-6">
      <h1>Roll IP</h1>

      <Settings />
      <Check />
      <Roll />
      <Logs />
    </main>
  );
}

export default App;
