// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn greet1(name: &str) -> String {
    format!("greet1 Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let baidu = tauri::WindowBuilder::new(
                app, "baidu", tauri::WindowUrl::External("https://www.baidu.com".parse().unwrap())).build()?;
            let local = tauri::WindowBuilder::new(
                app, "local", tauri::WindowUrl::App("components/Index.vue".into())).build()?;
            Ok(())
        })
        .menu(menu::custom_menu())
        .on_menu_event(|menu_event| {
            match menu_event.menu_item_id() {
                "quit" => {
                    std::process::exit(0)
                }
                "close" => {
                    menu_event.window().close().expect("close 错误")
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet,greet1])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[test]
fn test() {}

