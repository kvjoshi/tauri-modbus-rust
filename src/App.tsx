import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [regMsg, setRegMsg] = useState("");
  const [writeMsg, setWriteMsg] = useState("");
  const [wValue, setWValue] = useState("");
  const [reg_addr, setReg_Addr] = useState("");
  const [wreg_addr, setWReg_Addr] = useState("");

  async function read_holding() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      setRegMsg(await invoke("read_modbus", { reg: parseInt(reg_addr) }));
  }

    async function write_reg() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setWriteMsg(await invoke("write_modbus", { reg: parseInt(wreg_addr), value: parseInt(wValue) }));
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

      <p>{regMsg}</p>
</div>
        <div>

            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    write_reg();
                }}
            >
                <input
                    id="reg-address"
                    type={"text"}
                    onChange={(e) => setWReg_Addr(e.target.value)}
                    placeholder={"Register Address"}
                />

                <input
                    id="reg-address"
                    type={"text"}
                    onChange={(e) => setWValue(e.target.value)}
                    placeholder={"Value to Write to Register"}
                />
                <button type="submit">Write</button>
            </form>

            <p>{writeMsg}</p>
        </div>
    </div>
  );
}

export default App;