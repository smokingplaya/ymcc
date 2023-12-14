// Yandex Music Custom Client YMCC
// coded by smokingplaya in 2023
//
// features
// TODO: Discord RPC
//
// <3

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//#[allow(unused_imports)] // :))))

use std::{process, error::Error};

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use tauri::{Manager, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use window_shadows::set_shadow;

fn connect_discord() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("1184924340312092683")?;
    loop {
        if client.connect().is_ok() {
            break;
        }
    }

    loop {
        let payload = activity::Activity::new()
            .state("part 1 (test)")
            .details("a placeholder")
            .assets(
                activity::Assets::new()
                    .large_image("large-image")
                    .large_text("a thing"),
            );

        if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
            continue;
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn main() {
    //let client = connect_discord();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");

            let handle = app.handle();
            let _tray_handle = SystemTray::new()
                .with_id("yandexmusic")
                .with_tooltip("Yandex Music")
                .with_menu(
                    SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("quit", "Закрыть клиент"))
                )
                .on_event(move |event| {
                    match event {
                    SystemTrayEvent::LeftClick { .. } => {
                        let window = handle.get_window("main").unwrap();

                        if !window.is_visible().unwrap() {
                            window.show().expect("Failed to show the application");
                            window.set_focus().expect("Failed to set focus on window");
                        } else {
                            window.hide().expect("Failed to hide the application");
                        }
                    },
                    SystemTrayEvent::MenuItemClick {id, ..} => {
                        if id == "quit" {
                            process::exit(0);
                        }
                    }
                    _ => {}
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
