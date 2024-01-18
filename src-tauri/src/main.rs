// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate modbus;
//importing and using modbus
use modbus::Client;
use modbus::tcp;
//use serde_json::Value::Bool;


const SLAVE_IP: &str = "127.0.0.1";
const CONFIGURATION: tcp::Config = tcp::Config {
    tcp_port: 5020,
    tcp_connect_timeout: Some(std::time::Duration::from_secs(5)), // Adjust the timeout value as needed
    tcp_read_timeout: Some(std::time::Duration::from_secs(5)), // Adjust the timeout value as needed
    tcp_write_timeout: Some(std::time::Duration::from_secs(5)), // Adjust the timeout value as needed
    modbus_uid: 1,
};

#[tauri::command]
fn read_modbus(reg: u16) -> Vec<u16> {
    match tcp::Transport::new_with_cfg(SLAVE_IP, CONFIGURATION) {
        Ok(mut client) => {
            match client.read_holding_registers(reg, 1) {
                Ok(result) => {
                    println!("result: {:?}", result);
                    result
                }
                Err(err) => {
                    eprintln!("Error reading Modbus: {:?}", err);
                    Vec::new()
                }
            }
        }
        Err(err) => {
            eprintln!("Error establishing Modbus connection: {:?}", err);
            Vec::new()
        }
    }
}

#[tauri::command]
fn write_modbus(reg: u16, value: u16) {
    match tcp::Transport::new_with_cfg(SLAVE_IP, CONFIGURATION) {
        Ok(mut client) => {
            match client.write_single_register(reg, value) {
                Ok(result) => {
                    println!("result: {:?}", result);
                }
                Err(err) => {
                    eprintln!("Error writing Modbus: {:?}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error establishing Modbus connection: {:?}", err);
        }
    }
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