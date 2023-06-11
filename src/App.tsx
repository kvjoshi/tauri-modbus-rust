import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [reg_addr, setReg_Addr] = useState("");

  async function read_holding() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("read_modbus", { reg: parseInt(reg_addr) }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri Modbus Controller!</h1>
<div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          read_holding();
        }}
      >
        <input
          id="reg-address"
          type={"text"}
          onChange={(e) => setReg_Addr(e.target.value)}
        placeholder={"Register Address"}
        />
        <button type="submit">Read</button>
      </form>

      <p>{greetMsg}</p>
</div>
    </div>
  );
}

export default App;