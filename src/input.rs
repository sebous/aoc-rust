use std::env;
use std::fs;
use std::path::PathBuf;

pub fn load_input(path: &str) -> String {
    let dir = env::current_dir().unwrap();
    let home_dir_path = dir.as_path();
    let target_path = PathBuf::from(path);
    let joined_path = home_dir_path.join(target_path);

    let contents = fs::read_to_string(joined_path.as_os_str()).unwrap();
    return contents;
}
