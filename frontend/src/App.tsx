import { api } from "./lib/api";

function App() {
  const meow = async () => {
    const result: any = await api.call.sum(3, 4);

    // Получение сложного объекта
    const greet: any = await api.call.greet("meow");
    alert(greet);
    alert(result);
  };

  meow();
  return (
    <div>
      <h1>Wellcome to Crosslic</h1>
    </div>
  );
}

export default App;
