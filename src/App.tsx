import { useEffect, useState } from "react";
import {
  writeTextFile,
  readTextFile,
  exists,
  BaseDirectory
} from "@tauri-apps/plugin-fs";

import "./App.css";

function App() {
  const [text, setText] = useState("");

  const file = "demo.txt";

  async function loadFile() {
    try {
      const fileExists = await exists(file, {
        baseDir: BaseDirectory.AppData
      });

      if (!fileExists) {
        setText("");
        return;
      }

      const data = await readTextFile(file, {
        baseDir: BaseDirectory.AppData
      });

      setText(data);
    } catch (err) {
      console.error("load error:", err);
    }
  }

  async function saveFile() {
    try {
      await writeTextFile(file, text, {
        baseDir: BaseDirectory.AppData
      });
    } catch (err) {
      console.error("save error:", err);
    }
  }

  useEffect(() => {
    loadFile();
  }, []);

  return (
    <main className="container">
      <h1>Tauri File Demo</h1>

      <textarea
        style={{ width: "100%", height: 140 }}
        value={text}
        onChange={(e) => setText(e.currentTarget.value)}
        placeholder="Type something..."
      />

      <div style={{ marginTop: 20 }}>
        <button onClick={saveFile}>
          Save
        </button>

        <button
          onClick={loadFile}
          style={{ marginLeft: 10 }}
        >
          Reload
        </button>
      </div>
    </main>
  );
}

export default App;
