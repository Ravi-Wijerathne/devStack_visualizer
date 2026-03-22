mod analyzer;
mod commands;
mod graph;
mod language_detector;
mod output;
mod parser;
mod scanner;

#[cfg(test)]
mod tests {
    mod analyzer_test;
    mod language_detector_test;
    mod scanner_test;
    mod parser {
        mod js_parser_test;
        mod python_parser_test;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::analyze_project,
            commands::get_file_details,
            commands::export_graph,
            commands::detect_stack,
            commands::get_complexity,
            commands::detect_layers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
