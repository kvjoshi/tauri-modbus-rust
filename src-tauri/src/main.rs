// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate modbus;
//importing and using modbus
use modbus::{Client,Coil};
use modbus::tcp;
use serde_json::Value::Bool;


const SLAVE_IP: &str = "192.168.1.13";

#[tauri::command]
fn read_modbus(reg: u16) -> Vec<u16> {
    let mut client = tcp::Transport::new_with_cfg(SLAVE_IP, tcp::Config::default()).unwrap();
    // let mut client =  Client::new(client);
    let result = client.read_holding_registers(reg, 1).unwrap();
    println!("result: {:?}", result);
    result

}

#[tauri::command]
fn write_modbus(reg: u16, value: u16) {
    let mut client = tcp::Transport::new_with_cfg(SLAVE_IP, tcp::Config::default()).unwrap();
    // let mut client =  Client::new(client);
    let result = client.write_single_register(reg, value).unwrap();
    println!("result: {:?}", result);
    result

}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet , read_modbus, write_modbus])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}