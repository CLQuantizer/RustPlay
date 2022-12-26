use std::env;
use std::path::PathBuf;

pub struct MyStruct {
}

impl MyStruct {
    pub fn print_cur_dir() {
        let current_dir: PathBuf = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("Error getting current directory: {}", e);
                return;
            }
        };
        println!("Current directory: {}", current_dir.display());
    }
}
