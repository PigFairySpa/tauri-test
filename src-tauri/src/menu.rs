use tauri::{Menu, MenuItem, CustomMenuItem, Submenu};

fn quit() -> CustomMenuItem {
    let quit = CustomMenuItem::new("quit".to_string(), "quit");
    quit
}

fn close() -> CustomMenuItem {
    let close = CustomMenuItem::new("close".to_string(), "close");
    close
}

pub fn custom_menu() -> Menu {
    let submenu = Submenu::new("File", Menu::new().add_item(quit()).add_item(close()));

    let menu = Menu::new().add_submenu(submenu);
    menu
}
