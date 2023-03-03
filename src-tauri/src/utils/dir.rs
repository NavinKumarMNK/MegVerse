use tauri::api::path::{BaseDirs, DirType};

fn get_app_data_dir() -> Option<String> {
  match BaseDirs::new() {
    Some(base_dirs) => {
      let app_dir = base_dirs.get_dir(DirType::Data, false).unwrap();
      Some(app_dir.to_str().unwrap().to_string())
    }
    None => None,
  }
}

// Usage:
fn main () {
    let app_data_dir = get_app_data_dir();
    println!("App data directory: {:?}", app_data_dir);
}