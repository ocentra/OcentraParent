pub fn run() {
    let result = tauri::Builder::default().run(tauri::generate_context!());
    if let Err(error) = result {
        panic!("{error}");
    }
}
