import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import "./App.css";

function App() {
  const [status, setStatus] = useState("");
  const [path, setPath] = useState("");


  const pickFile = async () => {
    try {
      const selectPath = await open({ multiple: false, directory: false });
      invoke<string>('convert', { file: selectPath, path: path }).then((response: any) => {
        setStatus(response);
      })
    } catch (err) {
      console.error(err);
    }
  };


  const pickPath = async () => {
    try {
      const selectPath = await open({ directory: true, multiple: false });
      if (!selectPath) return;
      setPath(selectPath?.toString());
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="container">
      <button onClick={pickPath}>Выбрать путь для сохранения</button>
      <br />
      <button onClick={pickFile}>Выбрать файл для конвертации</button>
      <br />
      <h3>{status}</h3>
    </div>
  );
}

export default App;
