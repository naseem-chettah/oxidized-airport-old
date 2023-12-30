// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn run_surreal(title: &str, first_name: &str, last_name: &str) {
    // Create a tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Block the runtime and run the asynchronous code
    runtime.block_on(async {
        if let Err(err) = app::run_database(title, first_name, last_name).await {
            eprintln!("Error running the database: {}", err);
        }
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_surreal])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
