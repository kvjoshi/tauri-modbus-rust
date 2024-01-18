// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate modbus;
use modbus::Coil;
//importing and using modbus
use modbus::tcp;
use modbus::Client;
//use serde_json::Value::Bool;

#[tauri::command]
fn handle_modbus(
    host: &str,
    port: u16,
    command: &str,
    uid: u8,
    reg: u16,
    value: u16,
) -> Option<Vec<u16>> {
    let configuration: tcp::Config = tcp::Config {
        tcp_port: port,
        tcp_connect_timeout: Some(std::time::Duration::from_secs(5)), // Adjust the timeout value as needed
        tcp_read_timeout: Some(std::time::Duration::from_secs(5)), // Adjust the timeout value as needed
        tcp_write_timeout: Some(std::time::Duration::from_secs(5)), // Adjust the timeout value as needed
        modbus_uid: uid,
    };

    match tcp::Transport::new_with_cfg(host, configuration) {
        Ok(mut client) => match command {
            // "01" => match client.read_coils(reg, 1) {
            //     Ok(result) => {
            //         println!("result: {:?}", result);
            //         Some(Vec::new())
            //     }
            //     Err(err) => {
            //         eprintln!("Error writing Modbus: {:?}", err);
            //         None
            //     }
            // },
            // "02" => match client.read_discrete_inputs(reg, 1) {
            //     Ok(result) => {
            //         println!("result: {:?}", result);
            //         Some(Vec::new())
            //         //Some(result)
            //     }
            //     Err(err) => {
            //         eprintln!("Error writing Modbus: {:?}", err);
            //         None
            //     }
            // },
            "03" => match client.read_holding_registers(reg, 1) {
                Ok(result) => {
                    println!("result: {:?}", result);
                    Some(result)
                }
                Err(err) => {
                    eprintln!("Error reading Modbus: {:?}", err);
                    None
                }
            },
            // "04" => match client.read_input_registers(reg, 1) {
            //     Ok(result) => {
            //         println!("result: {:?}", result);
            //         Some(result)
            //     }
            //     Err(err) => {
            //         eprintln!("Error writing Modbus: {:?}", err);
            //         None
            //     }
            // },
            // "05" => match client.write_single_coil(reg, Coil::On) {
            //     Ok(result) => {
            //         println!("result: {:?}", result);
            //         None
            //     }
            //     Err(err) => {
            //         eprintln!("Error writing Modbus: {:?}", err);
            //         None
            //     }
            // },
            "06" => match client.write_single_register(reg, value) {
                Ok(result) => {
                    println!("result: {:?}", result);
                    Some(vec![value])
                }
                Err(err) => {
                    eprintln!("Error writing Modbus: {:?}", err);
                    None
                }
            },
            // "15" => match client.write_multiple_coils(reg, value) {
            //     Ok(result) => {
            //         println!("result: {:?}", result);
            //     }
            //     Err(err) => {
            //         eprintln!("Error writing Modbus: {:?}", err);
            //     }
            // },
            // "16" => match client.write_multiple_registers(reg, value) {
            //     Ok(result) => {
            //         println!("result: {:?}", result);
            //     }
            //     Err(err) => {
            //         eprintln!("Error writing Modbus: {:?}", err);
            //     }
            // },
            _ => {
                eprintln!("Invalid choice");
                None
            }
        },
        Err(err) => {
            eprintln!("Error establishing Modbus connection: {:?}", err);
            None
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
        .invoke_handler(tauri::generate_handler![greet, handle_modbus])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
