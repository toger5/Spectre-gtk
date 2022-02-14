pub fn get_save_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".spectre.d");
    path
}
