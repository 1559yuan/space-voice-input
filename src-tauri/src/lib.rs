mod commands;
mod kb;

use tauri::{Manager, WindowEvent};

pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_kb_templates,
            commands::match_kb,
            commands::copy_text,
        ])
        .setup(|app| {
            if let Some(win) = app.get_webview_window("main") {
                win.on_event(|win, event| {
                    if let WindowEvent::CloseRequested = event {
                        let _ = win.hide();
                    }
                });
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .run(|_app| {});
}