// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod aws;
mod cmd;
mod lens;
mod model;
mod stream;

use lens::Lens;

fn main() {
    let (lens, query_exec) = Lens::new();

    tauri::Builder::default()
        .setup(|_| {
            tauri::async_runtime::spawn(query_exec.run());
            Ok(())
        })
        .manage(lens)
        .invoke_handler(tauri::generate_handler![
            cmd::list_databases,
            cmd::sql,
            cmd::sql_stream,
            cmd::stream_next,
            cmd::aws_sso_login,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
