import { api } from "crosslic-api";

function App() {
  const meow = async () => {
    const result = await api.call.sum(3, 4);
    const greet = await api.call.greet("meow");
    alert(greet);
    alert(result);
  };

  meow();

  return (
    <div>
      <h1>Welcome to Crosslic</h1>
    </div>
  );
}

export default App;