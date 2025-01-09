use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_shell::ShellExt;
use reqwest;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn server_sidecar_handler(app: tauri::AppHandle) {
    app.shell()
        .sidecar("binaries/server")
        .unwrap()
        .spawn()
        .unwrap();
}

#[tauri::command]
async fn quit_handler(app: tauri::AppHandle) {
    // Send POST request to terminate the sidecar with an explicit empty body
    match reqwest::Client::new()
        .get("http://localhost:3000/killme")
        .header("Content-Type", "application/json") // Explicit content type
        .body("{}") // Explicit empty JSON body
        .send()
        .await
    {
        Ok(response) => {
            let text = response.text().await.unwrap_or_default();
            println!("Response from sidecar: {}", text);
        }
        Err(_) => {},
    }

    // Exit the application after the sidecar is terminated
    app.exit(0);
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![server_sidecar_handler, greet, quit_handler])
        .setup(|app| {
            // Define the menu items for the tray
            let quit_i = MenuItem::with_id(app, "quit", "Quit Jester Desktop", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide/Show", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&quit_i, &hide_i])?;

            // Build the tray icon and its menu
            TrayIconBuilder::new()
                .icon(tauri::image::Image::from_path("icons/icon-128.png").unwrap())
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        // Call the async quit_handler using a blocking thread
                        tauri::async_runtime::block_on(async {
                            quit_handler(app.clone()).await;
                        });
                    }
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        let is_visible = window.is_visible().unwrap();
                        if is_visible {
                            window.hide().unwrap(); // Hide the window
                        } else {
                            window.show().unwrap(); // Show the window
                            window.set_focus().unwrap();
                        }
                    }
                    _ => {
                        println!("Menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
