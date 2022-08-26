#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![say_hi])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn say_hi(message: String) -> String {
    println!("JS says: {}", message);

    let output = "Hi back from Rust".to_string();

    output
}
