use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好，{}！欢迎使用智学伴侣！", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("智学伴侣 - AI 一对一教学").ok();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动应用时发生错误");
}
