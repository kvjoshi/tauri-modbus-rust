import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import "./App.css";

interface IOutput {
  sent: number[];
  pub: number[];
  value: number;
}

function App() {
  const [host, setHost] = useState<string>("127.0.0.1");
  const [port, setPort] = useState<number>(5020);
  const [msg, setMsg] = useState("");
  const [device, setDevice] = useState<number>(1);
  const [address, setAddress] = useState<number>(128);
  const [value, setValue] = useState<number>(1);
  const [modbusFunction, setModbusFunction] = useState("06");


  async function handle_modbus(host: string, port: number, command: string, uid: number = 1, reg: number, value: number = 0) {
    const response: IOutput = await invoke("handle_modbus", { host, port, command, uid, reg, value })
    console.log({response})
    setMsg(`${response.value ?? ''}`);
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri Modbus Controller!</h1>
      <h1>Configuration</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
        }}
      >
        <input
          id="reg-address21"
          type={"text"}
          onChange={(e) => setHost(e.target.value)}
          placeholder={"Host"}
          defaultValue={host}
        />
        <input
          id="reg-address22"
          type={"number"}
          onChange={(e) => setPort(parseInt(e.target.value))}
          placeholder={"Port"}
          defaultValue={port}
        />
      </form>
      <div>
        <form
          className="column"
          onSubmit={(e) => {
            e.preventDefault();
            handle_modbus(host, port, modbusFunction, device, address, value);
          }}
        >
          <div>
            <label htmlFor="reg-address2">Command:</label>
            <select value={modbusFunction} onChange={(e) => setModbusFunction(e.target.value)}>              
              <option value="03">03</option> {/* Read Multiple holding registers */}              
              <option value="06">06</option> {/* Write single holding register */}              
            </select>
          </div>
          <div>
            <label htmlFor="reg-address2">Device:</label>
            <input
              id="reg-address1"
              type={"number"}
              onChange={(e) => setDevice(parseInt(e.target.value))}
              placeholder={"Device"}
              defaultValue={device}
            />
          </div>
          <div>
            <label htmlFor="reg-address2">Register Address:</label>
            <input
              id="reg-address2"
              type={"text"}
              onChange={(e) => setAddress(parseInt(e.target.value))}
              placeholder={"Register Address"}
              defaultValue={address}
            />
          </div>
          <div>
            <label htmlFor="reg-address2">Value:</label>
            <input
              id="reg-address3"
              type={"text"}
              onChange={(e) => setValue(parseInt(e.target.value))}
              placeholder={"Value to Write to Register"}
              defaultValue={value}
            />
          </div>
          <button type="submit">Send</button>
        </form>

        <p>{msg}</p>
      </div>
    </div>
  );
}

export default App;