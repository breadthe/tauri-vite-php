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

    // execute the index.phar binary
    let output = std::process::Command::new("./index.phar")
        .arg(message)
        .output()
        .expect("failed to execute index.phar");

    // convert the output to a string
    let output = String::from_utf8(output.stdout).expect("failed to convert PHP output to string");

    output
}
